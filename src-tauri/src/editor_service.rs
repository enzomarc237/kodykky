use std::process::Command;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EditorServiceError {
    #[error("Editor not found")]
    EditorNotFound,
    #[error("Failed to launch editor: {0}")]
    LaunchError(String),
    #[error("Path validation failed")]
    InvalidPath,
    #[error("Permission denied")]
    PermissionDenied,
}

pub struct EditorService;

impl EditorService {
    pub fn new() -> Self {
        Self
    }

    pub fn open_in_vscode(&self, project_path: &str) -> Result<(), EditorServiceError> {
        self.validate_path(project_path)?;
        
        // Try to find VSCode installation
        if let Some(vscode_path) = self.find_vscode() {
            self.launch_editor(&vscode_path, project_path)
        } else {
            // Fallback to system 'open' command
            self.open_with_system_command(project_path, "Visual Studio Code")
        }
    }

    pub fn open_in_vscode_insiders(&self, project_path: &str) -> Result<(), EditorServiceError> {
        self.validate_path(project_path)?;
        
        if let Some(vscode_path) = self.find_vscode_insiders() {
            self.launch_editor(&vscode_path, project_path)
        } else {
            self.open_with_system_command(project_path, "Visual Studio Code - Insiders")
        }
    }

    pub fn open_in_default_editor(&self, project_path: &str) -> Result<(), EditorServiceError> {
        self.validate_path(project_path)?;
        
        match Command::new("open").arg(project_path).spawn() {
            Ok(_) => Ok(()),
            Err(_) => Err(EditorServiceError::LaunchError(
                "Failed to open with default editor".to_string()
            )),
        }
    }

    pub fn find_vscode() -> Option<String> {
        // Check common VSCode installation paths on macOS
        let paths = [
            "/Applications/Visual Studio Code.app/Contents/Resources/app/bin/code",
            "/Applications/Visual Studio Code.app/Contents/MacOS/Electron",
            "/usr/local/bin/code",
            "/opt/homebrew/bin/code",
        ];
        
        for path in &paths {
            if Path::new(path).exists() {
                return Some(path.to_string());
            }
        }
        
        // Try using 'which' command
        if let Ok(output) = Command::new("which").arg("code").output() {
            if output.status.success() {
                return String::from_utf8(output.stdout).ok();
            }
        }
        
        None
    }

    pub fn find_vscode_insiders() -> Option<String> {
        let paths = [
            "/Applications/Visual Studio Code - Insiders.app/Contents/Resources/app/bin/code-insiders",
            "/Applications/Visual Studio Code - Insiders.app/Contents/MacOS/Electron",
            "/usr/local/bin/code-insiders",
            "/opt/homebrew/bin/code-insiders",
        ];
        
        for path in &paths {
            if Path::new(path).exists() {
                return Some(path.to_string());
            }
        }
        
        // Try using 'which' command
        if let Ok(output) = Command::new("which").arg("code-insiders").output() {
            if output.status.success() {
                return String::from_utf8(output.stdout).ok();
            }
        }
        
        None
    }

    fn launch_editor(&self, editor_path: &str, project_path: &str) -> Result<(), EditorServiceError> {
        match Command::new(editor_path)
            .arg(project_path)
            .spawn()
        {
            Ok(_) => Ok(()),
            Err(e) => Err(EditorServiceError::LaunchError(e.to_string())),
        }
    }

    fn open_with_system_command(&self, project_path: &str, app_name: &str) -> Result<(), EditorServiceError> {
        match Command::new("open")
            .args(&["-a", app_name, project_path])
            .spawn()
        {
            Ok(_) => Ok(()),
            Err(e) => Err(EditorServiceError::LaunchError(e.to_string())),
        }
    }

    fn validate_path(&self, path: &str) -> Result<(), EditorServiceError> {
        if path.is_empty() {
            return Err(EditorServiceError::InvalidPath);
        }
        
        let path_obj = Path::new(path);
        
        if !path_obj.exists() {
            return Err(EditorServiceError::InvalidPath);
        }
        
        Ok(())
    }

    pub fn get_available_editors(&self) -> Vec<EditorInfo> {
        let mut editors = Vec::new();
        
        if let Some(_) = Self::find_vscode() {
            editors.push(EditorInfo {
                name: "Visual Studio Code".to_string(),
                executable: "code".to_string(),
                id: "vscode".to_string(),
                is_default: true,
            });
        }
        
        if let Some(_) = Self::find_vscode_insiders() {
            editors.push(EditorInfo {
                name: "Visual Studio Code Insiders".to_string(),
                executable: "code-insiders".to_string(),
                id: "vscode-insiders".to_string(),
                is_default: false,
            });
        }
        
        editors.push(EditorInfo {
            name: "System Default".to_string(),
            executable: "open".to_string(),
            id: "default".to_string(),
            is_default: false,
        });
        
        editors
    }

    pub fn open_file_with_editor(&self, editor_id: &str, file_path: &str) -> Result<(), EditorServiceError> {
        self.validate_path(file_path)?;
        
        match editor_id {
            "vscode" => {
                if let Some(path) = Self::find_vscode() {
                    self.launch_editor(&path, file_path)
                } else {
                    Err(EditorServiceError::EditorNotFound)
                }
            }
            "vscode-insiders" => {
                if let Some(path) = Self::find_vscode_insiders() {
                    self.launch_editor(&path, file_path)
                } else {
                    Err(EditorServiceError::EditorNotFound)
                }
            }
            "default" => {
                match Command::new("open").arg(file_path).spawn() {
                    Ok(_) => Ok(()),
                    Err(e) => Err(EditorServiceError::LaunchError(e.to_string())),
                }
            }
            _ => Err(EditorServiceError::EditorNotFound),
        }
    }

    pub fn reveal_in_finder(&self, path: &str) -> Result<(), EditorServiceError> {
        self.validate_path(path)?;
        
        match Command::new("open").arg("-R").arg(path).spawn() {
            Ok(_) => Ok(()),
            Err(e) => Err(EditorServiceError::LaunchError(e.to_string())),
        }
    }

    pub fn copy_path_to_clipboard(&self, path: &str) -> Result<(), EditorServiceError> {
        self.validate_path(path)?;
        
        // Use pbcopy to copy to clipboard
        use std::io::Write;
        match Command::new("pbcopy")
            .stdin(std::process::Stdio::piped())
            .spawn()
        {
            Ok(mut child) => {
                if let Some(stdin) = child.stdin.as_mut() {
                    stdin.write_all(path.as_bytes())?;
                }
                Ok(())
            }
            Err(e) => Err(EditorServiceError::LaunchError(e.to_string())),
        }
    }

    pub fn open_terminal_at_path(&self, path: &str) -> Result<(), EditorServiceError> {
        self.validate_path(path)?;
        
        // Open Terminal.app at the specified path
        match Command::new("open")
            .args(&["-a", "Terminal", path])
            .spawn()
        {
            Ok(_) => Ok(()),
            Err(e) => Err(EditorServiceError::LaunchError(e.to_string())),
        }
    }
}

#[derive(Debug, Clone)]
pub struct EditorInfo {
    pub name: String,
    pub executable: String,
    pub id: String,
    pub is_default: bool,
}

impl Default for EditorService {
    fn default() -> Self {
        Self::new()
    }
}