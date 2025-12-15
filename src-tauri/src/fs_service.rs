use std::fs;
use std::path::Path;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FsServiceError {
    #[error("Path traversal attack detected")]
    InvalidPath,
    #[error("File system error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Permission denied")]
    PermissionDenied,
    #[error("Disk full")]
    DiskFull,
}

pub struct FsService;

impl FsService {
    pub fn new() -> Self {
        Self
    }

    pub fn create_project_structure(
        &self,
        project_path: &Path,
        files: &std::collections::HashMap<String, String>,
    ) -> Result<(), FsServiceError> {
        // Security check: ensure project path is within allowed boundaries
        self.validate_project_path(project_path)?;
        
        // Create project directory
        fs::create_dir_all(project_path)?;
        
        // Create all files
        for (relative_path, content) in files {
            let full_path = project_path.join(relative_path);
            
            // Ensure parent directory exists
            if let Some(parent) = full_path.parent() {
                fs::create_dir_all(parent)?;
            }
            
            // Write file content
            fs::write(&full_path, content)?;
        }
        
        Ok(())
    }

    pub fn get_project_info(&self, project_path: &Path) -> Result<ProjectInfo, FsServiceError> {
        let mut total_size = 0u64;
        let mut file_count = 0u32;
        let mut directory_count = 0u32;
        
        if project_path.exists() && project_path.is_dir() {
            for entry in fs::read_dir(project_path)? {
                let entry = entry?;
                let path = entry.path();
                
                if path.is_dir() {
                    directory_count += 1;
                } else if path.is_file() {
                    file_count += 1;
                    if let Ok(metadata) = fs::metadata(&path) {
                        total_size += metadata.len();
                    }
                }
            }
        }
        
        Ok(ProjectInfo {
            path: project_path.to_path_buf(),
            total_size,
            file_count,
            directory_count,
            exists: project_path.exists(),
        })
    }

    pub fn delete_project(&self, project_path: &Path) -> Result<(), FsServiceError> {
        // Security check
        self.validate_project_path(project_path)?;
        
        if project_path.exists() {
            fs::remove_dir_all(project_path)?;
        }
        
        Ok(())
    }

    pub fn validate_project_path(&self, project_path: &Path) -> Result<(), FsServiceError> {
        // Prevent path traversal attacks
        let canonical_path = project_path.canonicalize()
            .map_err(|_| FsServiceError::InvalidPath)?;
            
        // Check if path contains parent directory references
        let path_str = canonical_path.to_string_lossy();
        if path_str.contains("..") {
            return Err(FsServiceError::InvalidPath);
        }
        
        // Additional security checks can be added here
        // such as restricting to user's home directory
        
        Ok(())
    }

    pub fn find_writable_directory(&self) -> Result<PathBuf, FsServiceError> {
        // Try common writable locations
        let candidates = [
            dirs::home_dir().map(|p| p.join("Projects")),
            dirs::desktop_dir(),
            dirs::downloads_dir(),
            dirs::document_dir(),
            dirs::data_dir(),
        ];
        
        for candidate in candidates {
            if let Some(path) = candidate {
                if let Ok(_) = fs::create_dir_all(&path) {
                    if let Ok(_) = self.test_write_permission(&path) {
                        return Ok(path);
                    }
                }
            }
        }
        
        Err(FsServiceError::PermissionDenied)
    }

    fn test_write_permission(&self, path: &Path) -> Result<(), FsServiceError> {
        let test_file = path.join(".scaffold-ai-write-test");
        
        match fs::write(&test_file, "test") {
            Ok(_) => {
                fs::remove_file(&test_file)?;
                Ok(())
            }
            Err(_) => Err(FsServiceError::PermissionDenied),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ProjectInfo {
    pub path: PathBuf,
    pub total_size: u64,
    pub file_count: u32,
    pub directory_count: u32,
    pub exists: bool,
}

impl Default for FsService {
    fn default() -> Self {
        Self::new()
    }
}