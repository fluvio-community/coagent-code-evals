/// # AIPACK Configuration Validation Module
///
/// This module provides utilities for validating AIPACK configuration files.
/// It ensures the configuration is correct and ready for evaluation.

use std::collections::HashMap;
use tokio::fs;
use serde::Deserialize;
use anyhow::{Result, Context};

/// AIPACK configuration structure for deserialization
#[derive(Debug, Deserialize)]
struct AipackConfig {
    options: AipackOptions,
    #[serde(default)]
    models: HashMap<String, String>,
}

/// Options section for AIPACK configuration
#[derive(Debug, Deserialize)]
struct AipackOptions {
    model: String,
}

/// Result of AIPACK configuration validation
#[derive(Debug, Clone)]
pub struct AipackValidationResult {
    /// Configuration is valid
    pub is_valid: bool,
    /// Default model specified in the configuration
    pub default_model: Option<String>,
    /// Error messages encountered during validation
    pub error_messages: Vec<String>,
}

impl AipackValidationResult {
    /// Convert validation result to metadata
    pub fn into_metadata(self) -> HashMap<String, String> {
        let mut metadata = HashMap::new();
        metadata.insert("config_valid".to_string(), self.is_valid.to_string());
        if let Some(model) = self.default_model {
            metadata.insert("default_model".to_string(), model);
        }
        if !self.error_messages.is_empty() {
            metadata.insert("error_messages".to_string(), self.error_messages.join("; "));
        }
        metadata
    }
}

/// AIPACK configuration validator
pub struct AipackValidator {
    config_path: String,
}

impl AipackValidator {
    /// Create a new AIPACK configuration validator
    pub fn new(config_path: &str) -> Self {
        Self {
            config_path: config_path.to_string(),
        }
    }

    /// Validate the AIPACK configuration file
    pub async fn validate_config(&self) -> Result<AipackValidationResult> {
        let config_content = fs::read_to_string(&self.config_path).await
            .context("Failed to read AIPACK configuration file")?;

        let config: Result<AipackConfig, _> = toml::from_str(&config_content);

        match config {
            Ok(cfg) => Ok(AipackValidationResult {
                is_valid: true,
                default_model: Some(cfg.options.model),
                error_messages: Vec::new(),
            }),
            Err(e) => Ok(AipackValidationResult {
                is_valid: false,
                default_model: None,
                error_messages: vec![format!("Configuration parsing error: {}", e)],
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use tokio::io::AsyncWriteExt;

    #[tokio::test]
    async fn test_aipack_validation_valid() {
        let tmp_dir = env::temp_dir();
        let config_path = tmp_dir.join("aipack_valid.toml");

        let mut file = fs::File::create(&config_path).await.expect("Unable to create file");
        file.write_all(b"[options]\nmodel = \"codellama:7b\"").await.expect("Unable to write data");

        let validator = AipackValidator::new(config_path.to_str().unwrap());
        let result = validator.validate_config().await.expect("Validation failed");

        assert!(result.is_valid);
        assert_eq!(result.default_model.unwrap(), "codellama:7b");
    }

    #[tokio::test]
    async fn test_aipack_validation_invalid() {
        let tmp_dir = env::temp_dir();
        let config_path = tmp_dir.join("aipack_invalid.toml");

        let mut file = fs::File::create(&config_path).await.expect("Unable to create file");
        file.write_all(b"[not_options]\nmodel = \"codellama:7b\"").await.expect("Unable to write data");

        let validator = AipackValidator::new(config_path.to_str().unwrap());
        let result = validator.validate_config().await.expect("Validation failed");

        assert!(!result.is_valid);
    }
}

