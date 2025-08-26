/// # Ollama Service Validation Module
/// 
/// This module handles validation of Ollama service health, responsiveness, and model availability.
/// It provides comprehensive checks for:
/// - Service connectivity and API response
/// - Model availability verification against actual Ollama models
/// - Model size detection for timeout adjustment
/// - Performance metrics collection

use std::collections::HashMap;
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};
use anyhow::{Result, Context, bail};
use reqwest::Client;
use tokio::time::timeout;

/// Ollama API response structure for model list
#[derive(Debug, Clone, Serialize, Deserialize)]
struct OllamaModelInfo {
    name: String,
    #[serde(rename = "modified_at")]
    modified_at: String,
    size: u64,
    digest: String,
    details: Option<OllamaModelDetails>,
}

/// Detailed model information from Ollama API
#[derive(Debug, Clone, Serialize, Deserialize)]
struct OllamaModelDetails {
    format: Option<String>,
    family: Option<String>,
    families: Option<Vec<String>>,
    parameter_size: Option<String>,
    quantization_level: Option<String>,
}

/// Ollama API response for model list endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
struct OllamaModelsResponse {
    models: Vec<OllamaModelInfo>,
}

/// Ollama API response for version endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
struct OllamaVersionResponse {
    version: String,
}

/// Result of Ollama service validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaValidationResult {
    /// Service is accessible and responding
    pub service_accessible: bool,
    /// Ollama version information
    pub version: Option<String>,
    /// List of available models
    pub available_models: Vec<OllamaModel>,
    /// List of requested models that are missing
    pub missing_models: Vec<String>,
    /// Service response time in milliseconds
    pub response_time_ms: u64,
    /// Total models available
    pub total_models_available: usize,
    /// Validation error messages if any
    pub error_messages: Vec<String>,
}

/// Simplified model information for validation results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaModel {
    /// Model name/identifier
    pub name: String,
    /// Model size in bytes
    pub size_bytes: u64,
    /// Model size category (Small, Medium, Large, XL)
    pub size_category: ModelSizeCategory,
    /// Recommended timeout for this model in seconds
    pub recommended_timeout_seconds: u64,
    /// Model parameter information
    pub parameter_info: Option<String>,
    /// Last modified timestamp
    pub modified_at: String,
}

/// Model size categories for timeout adjustment
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ModelSizeCategory {
    /// Models under 1GB - fast inference
    Small,
    /// Models 1-5GB - moderate inference speed
    Medium,
    /// Models 5-20GB - slower inference
    Large,
    /// Models over 20GB - very slow inference
    XL,
}

impl ModelSizeCategory {
    /// Get recommended timeout in seconds based on model size
    pub fn recommended_timeout(&self) -> u64 {
        match self {
            ModelSizeCategory::Small => 30,
            ModelSizeCategory::Medium => 60,
            ModelSizeCategory::Large => 120,
            ModelSizeCategory::XL => 300,
        }
    }

    /// Create size category from byte size
    pub fn from_size_bytes(size_bytes: u64) -> Self {
        const GB: u64 = 1_073_741_824; // 1GB in bytes
        
        match size_bytes {
            0..=GB => ModelSizeCategory::Small,
            size if size <= 5 * GB => ModelSizeCategory::Medium,
            size if size <= 20 * GB => ModelSizeCategory::Large,
            _ => ModelSizeCategory::XL,
        }
    }
}

impl OllamaValidationResult {
    /// Convert validation result to metadata HashMap for CheckResult
    pub fn into_metadata(self) -> HashMap<String, String> {
        let mut metadata = HashMap::new();
        
        metadata.insert("service_accessible".to_string(), self.service_accessible.to_string());
        metadata.insert("response_time_ms".to_string(), self.response_time_ms.to_string());
        metadata.insert("available_model_count".to_string(), self.available_models.len().to_string());
        metadata.insert("missing_model_count".to_string(), self.missing_models.len().to_string());
        metadata.insert("total_models_available".to_string(), self.total_models_available.to_string());
        
        if let Some(version) = self.version {
            metadata.insert("ollama_version".to_string(), version);
        }
        
        // Add model size distribution
        let small_count = self.available_models.iter().filter(|m| m.size_category == ModelSizeCategory::Small).count();
        let medium_count = self.available_models.iter().filter(|m| m.size_category == ModelSizeCategory::Medium).count();
        let large_count = self.available_models.iter().filter(|m| m.size_category == ModelSizeCategory::Large).count();
        let xl_count = self.available_models.iter().filter(|m| m.size_category == ModelSizeCategory::XL).count();
        
        metadata.insert("small_models_count".to_string(), small_count.to_string());
        metadata.insert("medium_models_count".to_string(), medium_count.to_string());
        metadata.insert("large_models_count".to_string(), large_count.to_string());
        metadata.insert("xl_models_count".to_string(), xl_count.to_string());
        
        // Add available model names
        let model_names: Vec<String> = self.available_models.into_iter().map(|m| m.name).collect();
        metadata.insert("available_models".to_string(), model_names.join(","));
        
        if !self.missing_models.is_empty() {
            metadata.insert("missing_models".to_string(), self.missing_models.join(","));
        }
        
        if !self.error_messages.is_empty() {
            metadata.insert("error_messages".to_string(), self.error_messages.join("; "));
        }
        
        metadata
    }
}

/// Ollama service validator with comprehensive health and model checks
pub struct OllamaValidator {
    /// Base URL for Ollama service
    ollama_url: String,
    /// HTTP client for API requests
    http_client: Client,
    /// Timeout for API requests
    request_timeout: Duration,
}

impl OllamaValidator {
    /// Create new Ollama validator with specified URL and HTTP client
    pub fn new(ollama_url: &str, http_client: &Client) -> Self {
        Self {
            ollama_url: ollama_url.to_string(),
            http_client: http_client.clone(),
            request_timeout: Duration::from_secs(30),
        }
    }

    /// Create new Ollama validator with custom timeout
    pub fn with_timeout(ollama_url: &str, http_client: &Client, timeout: Duration) -> Self {
        Self {
            ollama_url: ollama_url.to_string(),
            http_client: http_client.clone(),
            request_timeout: timeout,
        }
    }

    /// Validate Ollama service health and model availability
    /// 
    /// Performs comprehensive validation including:
    /// - Service connectivity check
    /// - Version information retrieval
    /// - Model list retrieval and analysis
    /// - Model size categorization
    /// - Timeout recommendations
    pub async fn validate_service(&self) -> Result<OllamaValidationResult> {
        let start_time = Instant::now();
        let mut error_messages = Vec::new();
        
        log::info!("🔍 Validating Ollama service at {}", self.ollama_url);

        // Check service connectivity
        let service_accessible = match self.check_service_health().await {
            Ok(true) => {
                log::debug!("✅ Ollama service is accessible");
                true
            },
            Ok(false) => {
                let msg = "Ollama service health check failed".to_string();
                error_messages.push(msg.clone());
                log::warn!("⚠️ {}", msg);
                false
            },
            Err(e) => {
                let msg = format!("Failed to connect to Ollama service: {}", e);
                error_messages.push(msg.clone());
                log::error!("❌ {}", msg);
                false
            }
        };

        // Get version information
        let version = if service_accessible {
            match self.get_ollama_version().await {
                Ok(v) => {
                    log::debug!("📋 Ollama version: {}", v);
                    Some(v)
                },
                Err(e) => {
                    let msg = format!("Failed to get Ollama version: {}", e);
                    error_messages.push(msg.clone());
                    log::warn!("⚠️ {}", msg);
                    None
                }
            }
        } else {
            None
        };

        // Get available models
        let available_models = if service_accessible {
            match self.get_available_models().await {
                Ok(models) => {
                    log::info!("📦 Found {} available models", models.len());
                    for model in &models {
                        log::debug!("  - {} ({:.2}GB, {:?})", 
                            model.name, 
                            model.size_bytes as f64 / 1_073_741_824.0,
                            model.size_category
                        );
                    }
                    models
                },
                Err(e) => {
                    let msg = format!("Failed to retrieve model list: {}", e);
                    error_messages.push(msg.clone());
                    log::error!("❌ {}", msg);
                    Vec::new()
                }
            }
        } else {
            Vec::new()
        };

        let response_time_ms = start_time.elapsed().as_millis() as u64;
        let total_models_available = available_models.len();

        // For now, we don't have a specific list of required models to check against
        // This could be enhanced to check against a configuration-defined list
        let missing_models = Vec::new();

        log::info!(
            "✅ Ollama validation completed in {}ms - {} models available", 
            response_time_ms, 
            total_models_available
        );

        Ok(OllamaValidationResult {
            service_accessible,
            version,
            available_models,
            missing_models,
            response_time_ms,
            total_models_available,
            error_messages,
        })
    }

    /// Validate specific models are available
    /// 
    /// Check if a list of required models are available in Ollama
    pub async fn validate_required_models(&self, required_models: &[String]) -> Result<OllamaValidationResult> {
        let mut result = self.validate_service().await?;
        
        if result.service_accessible && !required_models.is_empty() {
            let available_model_names: Vec<String> = result.available_models
                .iter()
                .map(|m| m.name.clone())
                .collect();

            result.missing_models = required_models
                .iter()
                .filter(|required| !available_model_names.contains(required))
                .cloned()
                .collect();

            if !result.missing_models.is_empty() {
                log::warn!(
                    "⚠️ {} required models are missing: {}", 
                    result.missing_models.len(),
                    result.missing_models.join(", ")
                );
                
                result.error_messages.push(format!(
                    "{} required models are not available: {}", 
                    result.missing_models.len(),
                    result.missing_models.join(", ")
                ));
            }
        }

        Ok(result)
    }

    /// Check if Ollama service is healthy and responding
    async fn check_service_health(&self) -> Result<bool> {
        let health_url = format!("{}/api/tags", self.ollama_url);
        
        match timeout(self.request_timeout, self.http_client.get(&health_url).send()).await {
            Ok(Ok(response)) => {
                let is_healthy = response.status().is_success();
                log::debug!("🏥 Ollama health check: {} (status: {})", 
                    if is_healthy { "HEALTHY" } else { "UNHEALTHY" }, 
                    response.status()
                );
                Ok(is_healthy)
            },
            Ok(Err(e)) => {
                log::debug!("🏥 Ollama health check failed: {}", e);
                Err(e.into())
            },
            Err(_) => {
                log::debug!("🏥 Ollama health check timed out after {}s", self.request_timeout.as_secs());
                bail!("Health check request timed out")
            }
        }
    }

    /// Get Ollama version information
    async fn get_ollama_version(&self) -> Result<String> {
        let version_url = format!("{}/api/version", self.ollama_url);
        
        let response = timeout(
            self.request_timeout,
            self.http_client.get(&version_url).send()
        ).await
            .context("Version request timed out")?
            .context("Failed to send version request")?;

        if !response.status().is_success() {
            bail!("Version endpoint returned status: {}", response.status());
        }

        let version_response: OllamaVersionResponse = response.json().await
            .context("Failed to parse version response")?;

        Ok(version_response.version)
    }

    /// Get list of available models from Ollama
    async fn get_available_models(&self) -> Result<Vec<OllamaModel>> {
        let models_url = format!("{}/api/tags", self.ollama_url);
        
        let response = timeout(
            self.request_timeout,
            self.http_client.get(&models_url).send()
        ).await
            .context("Models list request timed out")?
            .context("Failed to send models list request")?;

        if !response.status().is_success() {
            bail!("Models endpoint returned status: {}", response.status());
        }

        let models_response: OllamaModelsResponse = response.json().await
            .context("Failed to parse models response")?;

        let mut ollama_models = Vec::new();
        
        for model_info in models_response.models {
            let size_category = ModelSizeCategory::from_size_bytes(model_info.size);
            let recommended_timeout = size_category.recommended_timeout();
            
            let parameter_info = model_info.details
                .as_ref()
                .and_then(|d| d.parameter_size.clone())
                .or_else(|| {
                    // Try to extract parameter info from model name
                    if model_info.name.contains("7b") || model_info.name.contains("7B") {
                        Some("7B".to_string())
                    } else if model_info.name.contains("13b") || model_info.name.contains("13B") {
                        Some("13B".to_string())
                    } else if model_info.name.contains("3b") || model_info.name.contains("3B") {
                        Some("3B".to_string())
                    } else {
                        None
                    }
                });

            ollama_models.push(OllamaModel {
                name: model_info.name,
                size_bytes: model_info.size,
                size_category,
                recommended_timeout_seconds: recommended_timeout,
                parameter_info,
                modified_at: model_info.modified_at,
            });
        }

        // Sort models by size for better organization
        ollama_models.sort_by(|a, b| a.size_bytes.cmp(&b.size_bytes));

        Ok(ollama_models)
    }

    /// Get timeout recommendation for a specific model
    pub async fn get_model_timeout_recommendation(&self, model_name: &str) -> Result<u64> {
        let models = self.get_available_models().await?;
        
        for model in models {
            if model.name == model_name {
                return Ok(model.recommended_timeout_seconds);
            }
        }
        
        // Default timeout if model not found
        Ok(60)
    }

    /// Check if a specific model is available
    pub async fn is_model_available(&self, model_name: &str) -> Result<bool> {
        let models = self.get_available_models().await?;
        Ok(models.iter().any(|m| m.name == model_name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_size_category_from_bytes() {
        assert_eq!(ModelSizeCategory::from_size_bytes(500_000_000), ModelSizeCategory::Small);
        assert_eq!(ModelSizeCategory::from_size_bytes(2_000_000_000), ModelSizeCategory::Medium);
        assert_eq!(ModelSizeCategory::from_size_bytes(10_000_000_000), ModelSizeCategory::Large);
        assert_eq!(ModelSizeCategory::from_size_bytes(25_000_000_000), ModelSizeCategory::XL);
    }

    #[test]
    fn test_model_size_category_timeouts() {
        assert_eq!(ModelSizeCategory::Small.recommended_timeout(), 30);
        assert_eq!(ModelSizeCategory::Medium.recommended_timeout(), 60);
        assert_eq!(ModelSizeCategory::Large.recommended_timeout(), 120);
        assert_eq!(ModelSizeCategory::XL.recommended_timeout(), 300);
    }
}
