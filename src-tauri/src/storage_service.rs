use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use crate::types::ProjectHistory;

#[derive(Error, Debug)]
pub enum StorageServiceError {
    #[error("Failed to serialize data: {0}")]
    SerializationError(#[from] serde_json::Error),
    #[error("Failed to read/write file: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Invalid data format")]
    InvalidFormat,
    #[error("Data directory error")]
    DirectoryError,
}

pub struct StorageService {
    data_dir: PathBuf,
}

impl StorageService {
    pub fn new() -> Self {
        let data_dir = dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("/tmp"))
            .join("scaffold-ai");
            
        Self { data_dir }
    }

    pub fn get_data_dir(&self) -> &PathBuf {
        &self.data_dir
    }

    pub fn ensure_data_dir(&self) -> Result<(), StorageServiceError> {
        fs::create_dir_all(&self.data_dir)
            .map_err(StorageServiceError::IoError)?;
        Ok(())
    }

    pub fn get_history_file_path(&self) -> PathBuf {
        self.data_dir.join("history.json")
    }

    pub fn get_settings_file_path(&self) -> PathBuf {
        self.data_dir.join("settings.json")
    }

    pub fn get_project_history(&self) -> Result<Vec<ProjectHistory>, StorageServiceError> {
        self.ensure_data_dir()?;
        let history_file = self.get_history_file_path();
        
        if !history_file.exists() {
            return Ok(Vec::new());
        }
        
        let content = fs::read_to_string(&history_file)?;
        let history: Vec<ProjectHistory> = serde_json::from_str(&content)?;
        Ok(history)
    }

    pub fn add_to_history(&self, project: &ProjectHistory) -> Result<(), StorageServiceError> {
        self.ensure_data_dir()?;
        let history_file = self.get_history_file_path();
        
        let mut history = if history_file.exists() {
            let content = fs::read_to_string(&history_file)?;
            serde_json::from_str(&content)?
        } else {
            Vec::new()
        };
        
        history.push(project.clone());
        
        // Keep only the last 50 projects
        if history.len() > 50 {
            history = history.into_iter().rev().take(50).rev().collect();
        }
        
        let content = serde_json::to_string_pretty(&history)?;
        fs::write(&history_file, content)?;
        
        Ok(())
    }

    pub fn delete_from_history(&self, id: &str) -> Result<(), StorageServiceError> {
        self.ensure_data_dir()?;
        let history_file = self.get_history_file_path();
        
        if !history_file.exists() {
            return Ok(());
        }
        
        let mut history = self.get_project_history()?;
        history.retain(|project| project.id != id);
        
        let content = serde_json::to_string_pretty(&history)?;
        fs::write(&history_file, content)?;
        
        Ok(())
    }

    pub fn clear_history(&self) -> Result<(), StorageServiceError> {
        self.ensure_data_dir()?;
        let history_file = self.get_history_file_path();
        
        if history_file.exists() {
            fs::remove_file(&history_file)?;
        }
        
        Ok(())
    }

    pub fn get_settings(&self) -> Result<AppSettings, StorageServiceError> {
        self.ensure_data_dir()?;
        let settings_file = self.get_settings_file_path();
        
        if !settings_file.exists() {
            return Ok(AppSettings::default());
        }
        
        let content = fs::read_to_string(&settings_file)?;
        let settings: AppSettings = serde_json::from_str(&content)?;
        Ok(settings)
    }

    pub fn save_settings(&self, settings: &AppSettings) -> Result<(), StorageServiceError> {
        self.ensure_data_dir()?;
        let settings_file = self.get_settings_file_path();
        
        let content = serde_json::to_string_pretty(settings)?;
        fs::write(&settings_file, content)?;
        
        Ok(())
    }

    pub fn export_history(&self, output_path: &Path) -> Result<(), StorageServiceError> {
        let history = self.get_project_history()?;
        let content = serde_json::to_string_pretty(&history)?;
        fs::write(output_path, content)?;
        Ok(())
    }

    pub fn import_history(&self, input_path: &Path) -> Result<(), StorageServiceError> {
        let content = fs::read_to_string(input_path)?;
        let history: Vec<ProjectHistory> = serde_json::from_str(&content)?;
        
        // Validate the imported data
        for project in &history {
            if project.id.is_empty() || project.name.is_empty() || project.path.is_empty() {
                return Err(StorageServiceError::InvalidFormat);
            }
        }
        
        self.ensure_data_dir()?;
        let history_file = self.get_history_file_path();
        let content = serde_json::to_string_pretty(&history)?;
        fs::write(&history_file, content)?;
        
        Ok(())
    }

    pub fn get_storage_usage(&self) -> Result<StorageUsage, StorageServiceError> {
        self.ensure_data_dir()?;
        
        let mut total_size = 0u64;
        let mut file_count = 0u32;
        
        if self.data_dir.exists() {
            for entry in fs::read_dir(&self.data_dir)? {
                let entry = entry?;
                let path = entry.path();
                
                if path.is_file() {
                    file_count += 1;
                    if let Ok(metadata) = fs::metadata(&path) {
                        total_size += metadata.len();
                    }
                }
            }
        }
        
        Ok(StorageUsage {
            total_size,
            file_count,
            data_dir: self.data_dir.clone(),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub default_project_location: Option<String>,
    pub preferred_editor: EditorType,
    pub theme: Theme,
    pub auto_save_history: bool,
    pub max_history_items: usize,
    pub api_timeout_seconds: u64,
    pub confirm_before_overwrite: bool,
    pub show_tutorial: bool,
    pub telemetry_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EditorType {
    Vscode,
    VscodeInsiders,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Theme {
    Dark,
    Light,
    System,
}

#[derive(Debug, Clone)]
pub struct StorageUsage {
    pub total_size: u64,
    pub file_count: u32,
    pub data_dir: PathBuf,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            default_project_location: None,
            preferred_editor: EditorType::Vscode,
            theme: Theme::Dark,
            auto_save_history: true,
            max_history_items: 50,
            api_timeout_seconds: 30,
            confirm_before_overwrite: true,
            show_tutorial: true,
            telemetry_enabled: false,
        }
    }
}

impl Default for StorageService {
    fn default() -> Self {
        Self::new()
    }
}