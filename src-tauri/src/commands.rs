use crate::ai_service::{AiService, AiServiceError};
use crate::template_engine::{TemplateEngine, Template};
use crate::types::{ScaffoldRequest, ScaffoldResponse, ProjectStructure};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CommandError {
    #[error("AI service error: {0}")]
    AiServiceError(#[from] AiServiceError),
    #[error("Template rendering failed")]
    TemplateError,
    #[error("Invalid project structure")]
    InvalidStructure,
    #[error("File system error: {0}")]
    FsError(String),
}

#[tauri::command]
pub async fn generate_scaffold(
    prompt: String,
    framework: Option<String>,
    api_key: String,
) -> Result<ScaffoldResponse, String> {
    let ai_service = AiService::new();
    let template_engine = TemplateEngine::new();
    
    let request = ScaffoldRequest { prompt, framework };
    
    let scaffold_response = ai_service.generate_scaffold(request, &api_key)
        .await
        .map_err(|e| format!("AI generation failed: {}", e))?;
    
    // Load templates and merge with AI response
    let templates = template_engine.load_templates()
        .map_err(|e| format!("Template loading failed: {}", e))?;
    
    let selected_template = match templates.get(&scaffold_response.framework) {
        Some(template) => template.clone(),
        None => return Err(format!("Unsupported framework: {}", scaffold_response.framework)),
    };
    
    // Merge AI customizations with template
    let mut merged_files = selected_template.files;
    for (path, content) in scaffold_response.files.clone() {
        merged_files.insert(path, content);
    }
    
    let mut merged_dependencies = selected_template.dependencies;
    for (name, version) in scaffold_response.dependencies.clone() {
        merged_dependencies.insert(name, version);
    }
    
    // Create the final structure
    let structure = merged_files.keys()
        .map(|path| crate::types::FileEntry {
            path: path.clone(),
            is_directory: path.contains('/'),
            size: None,
        })
        .collect();
    
    Ok(ScaffoldResponse {
        framework: scaffold_response.framework,
        project_name: scaffold_response.project_name,
        structure,
        files: merged_files,
        dependencies: merged_dependencies,
    })
}

#[tauri::command]
pub async fn validate_api_key(api_key: String) -> Result<bool, String> {
    let ai_service = AiService::new();
    
    ai_service.validate_api_key(&api_key)
        .await
        .map_err(|e| format!("API key validation failed: {}", e))
}

#[tauri::command]
pub async fn save_api_key(api_key: String) -> Result<(), String> {
    use keyring::Entry;
    
    let entry = Entry::new("scaffold-ai", "openai-api-key")
        .map_err(|e| format!("Failed to create keyring entry: {}", e))?;
        
    entry.set_password(&api_key)
        .map_err(|e| format!("Failed to save API key: {}", e))?;
    
    Ok(())
}

#[tauri::command]
pub async fn get_api_key() -> Result<Option<String>, String> {
    use keyring::Entry;
    
    let entry = Entry::new("scaffold-ai", "openai-api-key")
        .map_err(|e| format!("Failed to create keyring entry: {}", e))?;
        
    match entry.get_password() {
        Ok(password) => Ok(Some(password)),
        Err(_) => Ok(None),
    }
}

#[tauri::command]
pub async fn create_project(
    path: String,
    structure: ProjectStructure,
) -> Result<(), String> {
    use std::fs;
    use std::path::Path;
    
    let project_path = Path::new(&path).join(&structure.name);
    
    // Create project directory
    fs::create_dir_all(&project_path)
        .map_err(|e| format!("Failed to create project directory: {}", e))?;
    
    // Create all files
    for (file_path, content) in &structure.files {
        let full_path = project_path.join(file_path);
        
        // Create parent directories if they don't exist
        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create directory {}: {}", parent.display(), e))?;
        }
        
        // Write file content
        fs::write(&full_path, content)
            .map_err(|e| format!("Failed to write file {}: {}", full_path.display(), e))?;
    }
    
    Ok(())
}

#[tauri::command]
pub async fn validate_project_path(path: String) -> Result<bool, String> {
    use std::fs;
    use std::path::Path;
    
    let path = Path::new(&path);
    
    // Check if parent directory exists and is writable
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            return Ok(false);
        }
        
        // Try to create a test file to check writability
        let test_file = parent.join(".scaffold-ai-test");
        match fs::write(&test_file, "test") {
            Ok(_) => {
                let _ = fs::remove_file(&test_file);
                Ok(true)
            }
            Err(_) => Ok(false),
        }
    } else {
        Ok(false)
    }
}

#[tauri::command]
pub async fn select_directory() -> Result<Option<String>, String> {
    use tauri::api::dialog;
    
    // This is a simplified implementation - in a real app you'd use
    // a native file dialog. For now, we'll return None.
    Ok(None)
}

#[tauri::command]
pub async fn open_in_vscode(project_path: String) -> Result<(), String> {
    use std::process::Command;
    
    // Use macOS 'open' command to launch VSCode
    match Command::new("open")
        .args(&["-a", "Visual Studio Code", &project_path])
        .spawn()
    {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to open VSCode: {}", e)),
    }
}

#[tauri::command]
pub async fn get_project_history() -> Result<Vec<crate::types::ProjectHistory>, String> {
    use crate::storage_service::StorageService;
    
    let storage = StorageService::new();
    storage.get_project_history()
        .map_err(|e| format!("Failed to load project history: {}", e))
}

#[tauri::command]
pub async fn add_to_history(project: crate::types::ProjectHistory) -> Result<(), String> {
    use crate::storage_service::StorageService;
    
    let storage = StorageService::new();
    storage.add_to_history(&project)
        .map_err(|e| format!("Failed to save to history: {}", e))
}

#[tauri::command]
pub async fn delete_from_history(id: String) -> Result<(), String> {
    use crate::storage_service::StorageService;
    
    let storage = StorageService::new();
    storage.delete_from_history(&id)
        .map_err(|e| format!("Failed to delete from history: {}", e))
}

#[tauri::command]
pub async fn open_in_finder(path: String) -> Result<(), String> {
    use std::process::Command;
    
    // Use macOS 'open' command to reveal in Finder
    match Command::new("open").arg(&path).spawn() {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to open in Finder: {}", e)),
    }
}