use crate::types::{ScaffoldRequest, ScaffoldResponse};
use reqwest::Client;
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AiServiceError {
    #[error("API request failed: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("Invalid API key")]
    InvalidApiKey,
    #[error("AI response parsing failed")]
    ResponseParsingError,
    #[error("Rate limit exceeded")]
    RateLimitExceeded,
    #[error("Network error")]
    NetworkError,
}

pub struct AiService {
    client: Client,
    base_url: String,
}

impl AiService {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: "https://api.openai.com/v1".to_string(),
        }
    }

    pub async fn generate_scaffold(
        &self,
        request: ScaffoldRequest,
        api_key: &str,
    ) -> Result<ScaffoldResponse, AiServiceError> {
        let prompt = self.build_prompt(&request);
        
        let request_body = json!({
            "model": "gpt-4-turbo",
            "messages": [
                {
                    "role": "system",
                    "content": self.get_system_prompt()
                },
                {
                    "role": "user", 
                    "content": prompt
                }
            ],
            "max_tokens": 4000,
            "temperature": 0.3
        });

        let response = self.client
            .post(&format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        if response.status() == 401 {
            return Err(AiServiceError::InvalidApiKey);
        }

        if response.status() == 429 {
            return Err(AiServiceError::RateLimitExceeded);
        }

        if !response.status().is_success() {
            return Err(AiServiceError::NetworkError);
        }

        let response_text = response.text().await?;
        let parsed_response: serde_json::Value = serde_json::from_str(&response_text)
            .map_err(|_| AiServiceError::ResponseParsingError)?;

        self.parse_ai_response(&parsed_response)
    }

    fn build_prompt(&self, request: &ScaffoldRequest) -> String {
        let framework_hint = request.framework
            .as_deref()
            .unwrap_or("auto-detect from description");
            
        format!(
            "Create a {} project: {}\n\nProvide a complete project structure with all necessary files, dependencies, and configuration. Focus on creating a working, production-ready scaffold.",
            framework_hint, request.prompt
        )
    }

    fn get_system_prompt(&self) -> &'static str {
        "You are an expert software project scaffolding assistant. Given a user's project description, generate a complete project structure.\n\nWhen responding, always provide valid JSON with this exact structure:\n{\n  \"framework\": \"detected_or_specified_framework\",\n  \"project_name\": \"suggested_project_name\",\n  \"structure\": [\n    {\n      \"path\": \"relative/path/to/file\",\n      \"is_directory\": false,\n      \"size\": null\n    }\n  ],\n  \"files\": {\n    \"relative/path/to/file\": \"file contents here\"\n  },\n  \"dependencies\": {\n    \"package-name\": \"version\"\n  }\n}\n\nSupported frameworks: react, nextjs, flask, express, cli.\nAlways include package.json or requirements.txt when appropriate.\nInclude README.md with setup instructions.\nInclude .gitignore file.\nEnsure all generated code is syntactically correct and follows best practices."
    }

    fn parse_ai_response(&self, response: &serde_json::Value) -> Result<ScaffoldResponse, AiServiceError> {
        let content = response
            .get("choices")
            .and_then(|choices| choices.get(0))
            .and_then(|choice| choice.get("message"))
            .and_then(|message| message.get("content"))
            .and_then(|content| content.as_str())
            .ok_or(AiServiceError::ResponseParsingError)?;

        let scaffold_response: ScaffoldResponse = serde_json::from_str(content)
            .map_err(|_| AiServiceError::ResponseParsingError)?;

        Ok(scaffold_response)
    }

    pub async fn validate_api_key(&self, api_key: &str) -> Result<bool, AiServiceError> {
        let request_body = json!({
            "model": "gpt-3.5-turbo",
            "messages": [{"role": "user", "content": "test"}],
            "max_tokens": 1
        });

        let response = self.client
            .post(&format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        match response.status() {
            401 => Ok(false),
            200 => Ok(true),
            _ => Err(AiServiceError::NetworkError),
        }
    }
}

impl Default for AiService {
    fn default() -> Self {
        Self::new()
    }
}