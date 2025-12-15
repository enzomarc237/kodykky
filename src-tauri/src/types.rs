use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScaffoldRequest {
    pub prompt: String,
    pub framework: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScaffoldResponse {
    pub framework: String,
    pub project_name: String,
    pub structure: Vec<FileEntry>,
    pub files: std::collections::HashMap<String, String>,
    pub dependencies: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEntry {
    pub path: String,
    pub is_directory: bool,
    pub size: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectHistory {
    pub id: String,
    pub name: String,
    pub prompt: String,
    pub framework: String,
    pub path: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectStructure {
    pub name: String,
    pub framework: String,
    pub files: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    pub code: String,
    pub message: String,
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

impl std::error::Error for ApiError {}