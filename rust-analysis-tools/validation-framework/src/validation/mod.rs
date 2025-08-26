/// # Evaluation Pre-flight Validation Module
/// 
/// This module provides comprehensive validation before starting the evaluation process.
/// It ensures all prerequisites are met for successful model evaluation including:
/// - Ollama service health and responsiveness
/// - Model availability verification against actual Ollama models  
/// - Model size detection for timeout adjustment
/// - Disk space validation for output storage
/// - AIPACK configuration validity checks

use std::collections::HashMap;
use std::time::Duration;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use reqwest::Client;
use jiff::Zoned;

pub mod ollama;
pub mod disk;
pub mod aipack;

use ollama::OllamaValidator;
use disk::DiskValidator;
use aipack::AipackValidator;

/// Comprehensive validation result containing all pre-flight check outcomes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    /// Overall validation success status
    pub is_valid: bool,
    /// Detailed validation summary
    pub summary: ValidationSummary,
    /// Individual check results
    pub checks: ValidationChecks,
    /// Recommended configuration adjustments
    pub recommendations: Vec<ValidationRecommendation>,
    /// Critical issues that must be resolved
    pub critical_issues: Vec<String>,
    /// Warnings that should be addressed
    pub warnings: Vec<String>,
    /// Validation timestamp
    pub timestamp: Zoned,
}

/// Summary of validation results with key metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationSummary {
    /// Total number of models available for testing
    pub available_models: usize,
    /// Total number of models requested but unavailable
    pub missing_models: usize,
    /// Available disk space in GB
    pub available_disk_space_gb: f64,
    /// Estimated space requirements in GB
    pub estimated_space_needed_gb: f64,
    /// Ollama service response time in milliseconds
    pub ollama_response_time_ms: u64,
    /// AIPACK configuration validation status
    pub aipack_config_valid: bool,
}

/// Individual validation check results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationChecks {
    /// Ollama service connectivity and health
    pub ollama_service: CheckResult,
    /// Model availability verification
    pub model_availability: CheckResult,
    /// Disk space validation
    pub disk_space: CheckResult,
    /// AIPACK configuration validation
    pub aipack_config: CheckResult,
    /// System resource validation
    pub system_resources: CheckResult,
}

/// Individual check result with detailed information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckResult {
    /// Check success status
    pub passed: bool,
    /// Human-readable check description
    pub description: String,
    /// Detailed result message
    pub message: String,
    /// Check execution duration in milliseconds
    pub duration_ms: u64,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

/// Validation recommendation for configuration optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRecommendation {
    /// Recommendation category (performance, reliability, etc.)
    pub category: String,
    /// Specific recommendation description
    pub description: String,
    /// Priority level (High, Medium, Low)
    pub priority: String,
    /// Actionable steps to implement recommendation
    pub action: String,
}

/// Configuration for validation behavior
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationConfig {
    /// Ollama service URL (default: http://localhost:11434)
    pub ollama_url: String,
    /// Maximum timeout for service checks in seconds
    pub timeout_seconds: u64,
    /// Minimum required disk space in GB
    pub min_disk_space_gb: f64,
    /// Path to AIPACK configuration file
    pub aipack_config_path: String,
    /// Output directory for evaluation results
    pub output_directory: String,
    /// Whether to perform deep model validation
    pub deep_model_validation: bool,
}

impl Default for ValidationConfig {
    fn default() -> Self {
        Self {
            ollama_url: "http://localhost:11434".to_string(),
            timeout_seconds: 30,
            min_disk_space_gb: 5.0,
            aipack_config_path: ".aipack/config.toml".to_string(),
            output_directory: "evaluation-results".to_string(),
            deep_model_validation: true,
        }
    }
}

/// Main validation coordinator that orchestrates all pre-flight checks
pub struct ValidationCoordinator {
    config: ValidationConfig,
    http_client: Client,
    ollama_validator: OllamaValidator,
    disk_validator: DiskValidator,
    aipack_validator: AipackValidator,
}

impl ValidationCoordinator {
    /// Create a new validation coordinator with default configuration
    pub fn new() -> Self {
        Self::with_config(ValidationConfig::default())
    }

    /// Create a new validation coordinator with custom configuration
    pub fn with_config(config: ValidationConfig) -> Self {
        let http_client = Client::builder()
            .timeout(Duration::from_secs(config.timeout_seconds))
            .build()
            .expect("Failed to create HTTP client");

        let ollama_validator = OllamaValidator::new(&config.ollama_url, &http_client);
        let disk_validator = DiskValidator::new();
        let aipack_validator = AipackValidator::new(&config.aipack_config_path);

        Self {
            config,
            http_client,
            ollama_validator,
            disk_validator,
            aipack_validator,
        }
    }

    /// Run comprehensive validation of all prerequisites for evaluation
    /// 
    /// This is the main entry point for validation that coordinates all checks:
    /// 1. Ollama service health and responsiveness
    /// 2. Model availability and size detection
    /// 3. Disk space validation for outputs
    /// 4. AIPACK configuration validity
    /// 5. System resource checks
    /// 
    /// Returns detailed validation results with recommendations
    pub async fn validate_all(&self) -> Result<ValidationResult> {
        let start_time = jiff::Zoned::now();
        
        log::info!("🔍 Starting comprehensive evaluation pre-flight validation");
        
        // Run all validation checks concurrently for efficiency
        let (
            ollama_result,
            disk_result,
            aipack_result,
            system_result
        ) = tokio::try_join!(
            self.validate_ollama_service(),
            self.validate_disk_space(),
            self.validate_aipack_config(),
            self.validate_system_resources()
        )?;

        // Determine overall validation status
        let is_valid = ollama_result.passed && 
                      disk_result.passed && 
                      aipack_result.passed && 
                      system_result.passed;

        let checks = ValidationChecks {
            ollama_service: ollama_result.clone(),
            model_availability: ollama_result.clone(), // Model availability is part of Ollama validation
            disk_space: disk_result.clone(),
            aipack_config: aipack_result.clone(),
            system_resources: system_result,
        };

        // Generate recommendations based on check results
        let recommendations = self.generate_recommendations(&checks).await?;

        // Collect critical issues and warnings
        let (critical_issues, warnings) = self.categorize_issues(&checks);

        // Create validation summary
        let summary = self.create_summary(&checks).await?;

        let end_time = jiff::Zoned::now();
        let duration_ms = start_time.until(&end_time).unwrap().total(jiff::Unit::Millisecond).unwrap_or(0.0) as u64;
        
        let result = ValidationResult {
            is_valid,
            summary,
            checks,
            recommendations,
            critical_issues,
            warnings,
            timestamp: start_time,
        };
        
        log::info!(
            "✅ Validation complete - Status: {} (took {}ms)", 
            if is_valid { "PASSED" } else { "FAILED" },
            duration_ms
        );

        Ok(result)
    }

    /// Validate Ollama service health and model availability
    async fn validate_ollama_service(&self) -> Result<CheckResult> {
        let start_time = std::time::Instant::now();
        
        match self.ollama_validator.validate_service().await {
            Ok(ollama_result) => {
                Ok(CheckResult {
                    passed: true,
                    description: "Ollama service health and model availability".to_string(),
                    message: format!("Service responsive, {} models available", ollama_result.available_models.len()),
                    duration_ms: start_time.elapsed().as_millis() as u64,
                    metadata: ollama_result.into_metadata(),
                })
            },
            Err(e) => {
                Ok(CheckResult {
                    passed: false,
                    description: "Ollama service health and model availability".to_string(),
                    message: format!("Ollama validation failed: {}", e),
                    duration_ms: start_time.elapsed().as_millis() as u64,
                    metadata: HashMap::new(),
                })
            }
        }
    }

    /// Validate available disk space for evaluation outputs
    async fn validate_disk_space(&self) -> Result<CheckResult> {
        let start_time = std::time::Instant::now();
        
        match self.disk_validator.validate_space(&self.config.output_directory, self.config.min_disk_space_gb).await {
            Ok(disk_result) => {
                Ok(CheckResult {
                    passed: disk_result.sufficient_space,
                    description: "Disk space availability for evaluation outputs".to_string(),
                    message: format!(
                        "Available: {:.2}GB, Required: {:.2}GB", 
                        disk_result.available_space_gb, 
                        self.config.min_disk_space_gb
                    ),
                    duration_ms: start_time.elapsed().as_millis() as u64,
                    metadata: disk_result.into_metadata(),
                })
            },
            Err(e) => {
                Ok(CheckResult {
                    passed: false,
                    description: "Disk space availability for evaluation outputs".to_string(),
                    message: format!("Disk space validation failed: {}", e),
                    duration_ms: start_time.elapsed().as_millis() as u64,
                    metadata: HashMap::new(),
                })
            }
        }
    }

    /// Validate AIPACK configuration file validity
    async fn validate_aipack_config(&self) -> Result<CheckResult> {
        let start_time = std::time::Instant::now();
        
        match self.aipack_validator.validate_config().await {
            Ok(config_result) => {
                let is_valid = config_result.is_valid;
                let default_model = config_result.default_model.clone().unwrap_or("None".to_string());
                let metadata = config_result.into_metadata();
                
                Ok(CheckResult {
                    passed: is_valid,
                    description: "AIPACK configuration validity".to_string(),
                    message: format!("Config valid: {}, Default model: {}", is_valid, default_model),
                    duration_ms: start_time.elapsed().as_millis() as u64,
                    metadata,
                })
            },
            Err(e) => {
                Ok(CheckResult {
                    passed: false,
                    description: "AIPACK configuration validity".to_string(),
                    message: format!("AIPACK config validation failed: {}", e),
                    duration_ms: start_time.elapsed().as_millis() as u64,
                    metadata: HashMap::new(),
                })
            }
        }
    }

    /// Validate system resources (memory, CPU, etc.)
    async fn validate_system_resources(&self) -> Result<CheckResult> {
        let start_time = std::time::Instant::now();
        
        // Basic system resource check - can be extended with more sophisticated monitoring
        let available_memory = self.get_available_memory().await.unwrap_or(0);
        let cpu_count = num_cpus::get();
        
        let sufficient_resources = available_memory > 1_000_000_000 && cpu_count > 0; // 1GB minimum
        
        let mut metadata = HashMap::new();
        metadata.insert("available_memory_bytes".to_string(), available_memory.to_string());
        metadata.insert("cpu_count".to_string(), cpu_count.to_string());
        
        Ok(CheckResult {
            passed: sufficient_resources,
            description: "System resource availability".to_string(),
            message: format!(
                "Memory: {:.2}GB, CPUs: {}", 
                available_memory as f64 / 1_000_000_000.0,
                cpu_count
            ),
            duration_ms: start_time.elapsed().as_millis() as u64,
            metadata,
        })
    }

    /// Get available system memory (platform-specific implementation needed)
    async fn get_available_memory(&self) -> Result<u64> {
        // This is a simplified implementation - in production, use system-specific APIs
        // or crates like `sysinfo` for accurate memory information
        #[cfg(target_os = "linux")]
        {
            use tokio::fs;
            let meminfo = fs::read_to_string("/proc/meminfo").await?;
            for line in meminfo.lines() {
                if line.starts_with("MemAvailable:") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 2 {
                        let kb = parts[1].parse::<u64>()?;
                        return Ok(kb * 1024); // Convert KB to bytes
                    }
                }
            }
        }
        
        // Fallback: estimate 4GB available
        Ok(4_000_000_000)
    }

    /// Generate actionable recommendations based on validation results
    async fn generate_recommendations(&self, checks: &ValidationChecks) -> Result<Vec<ValidationRecommendation>> {
        let mut recommendations = Vec::new();

        // Ollama service recommendations
        if !checks.ollama_service.passed {
            recommendations.push(ValidationRecommendation {
                category: "Service".to_string(),
                description: "Ollama service is not responding properly".to_string(),
                priority: "High".to_string(),
                action: "Start Ollama service with 'ollama serve' and verify it's accessible at the configured URL".to_string(),
            });
        }

        // Disk space recommendations  
        if !checks.disk_space.passed {
            recommendations.push(ValidationRecommendation {
                category: "Storage".to_string(),
                description: "Insufficient disk space for evaluation outputs".to_string(),
                priority: "High".to_string(),
                action: format!("Free up disk space or change output directory. Need at least {:.2}GB", self.config.min_disk_space_gb),
            });
        }

        // AIPACK configuration recommendations
        if !checks.aipack_config.passed {
            recommendations.push(ValidationRecommendation {
                category: "Configuration".to_string(),
                description: "AIPACK configuration file has issues".to_string(),
                priority: "High".to_string(),
                action: "Review and fix AIPACK configuration file syntax and model references".to_string(),
            });
        }

        // Performance recommendations based on response times
        if let Some(response_time) = checks.ollama_service.metadata.get("response_time_ms") {
            if let Ok(time_ms) = response_time.parse::<u64>() {
                if time_ms > 5000 {
                    recommendations.push(ValidationRecommendation {
                        category: "Performance".to_string(),
                        description: "Ollama service response time is slow".to_string(),
                        priority: "Medium".to_string(),
                        action: "Consider optimizing Ollama configuration or checking system resources".to_string(),
                    });
                }
            }
        }

        Ok(recommendations)
    }

    /// Categorize validation issues into critical and warnings
    fn categorize_issues(&self, checks: &ValidationChecks) -> (Vec<String>, Vec<String>) {
        let mut critical_issues = Vec::new();
        let mut warnings = Vec::new();

        if !checks.ollama_service.passed {
            critical_issues.push("Ollama service is not accessible - evaluation cannot proceed".to_string());
        }

        if !checks.aipack_config.passed {
            critical_issues.push("AIPACK configuration is invalid - fix configuration before proceeding".to_string());
        }

        if !checks.disk_space.passed {
            critical_issues.push("Insufficient disk space for evaluation outputs".to_string());
        }

        if !checks.system_resources.passed {
            warnings.push("System resources may be insufficient for optimal performance".to_string());
        }

        (critical_issues, warnings)
    }

    /// Create validation summary with key metrics
    async fn create_summary(&self, checks: &ValidationChecks) -> Result<ValidationSummary> {
        let available_models = checks.ollama_service.metadata
            .get("available_model_count")
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);

        let missing_models = checks.ollama_service.metadata
            .get("missing_model_count")
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);

        let available_disk_space_gb = checks.disk_space.metadata
            .get("available_space_gb")
            .and_then(|s| s.parse().ok())
            .unwrap_or(0.0);

        let estimated_space_needed_gb = checks.disk_space.metadata
            .get("estimated_space_needed_gb")
            .and_then(|s| s.parse().ok())
            .unwrap_or(1.0);

        let ollama_response_time_ms = checks.ollama_service.metadata
            .get("response_time_ms")
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);

        Ok(ValidationSummary {
            available_models,
            missing_models,
            available_disk_space_gb,
            estimated_space_needed_gb,
            ollama_response_time_ms,
            aipack_config_valid: checks.aipack_config.passed,
        })
    }
}

/// Convenience function to run validation with default configuration
pub async fn validate_evaluation_prerequisites() -> Result<ValidationResult> {
    let coordinator = ValidationCoordinator::new();
    coordinator.validate_all().await
}

/// Convenience function to run validation with custom configuration
pub async fn validate_evaluation_prerequisites_with_config(config: ValidationConfig) -> Result<ValidationResult> {
    let coordinator = ValidationCoordinator::with_config(config);
    coordinator.validate_all().await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_validation_coordinator_creation() {
        let coordinator = ValidationCoordinator::new();
        assert_eq!(coordinator.config.ollama_url, "http://localhost:11434");
        assert_eq!(coordinator.config.timeout_seconds, 30);
    }

    #[tokio::test]
    async fn test_validation_config_default() {
        let config = ValidationConfig::default();
        assert_eq!(config.ollama_url, "http://localhost:11434");
        assert_eq!(config.min_disk_space_gb, 5.0);
        assert_eq!(config.aipack_config_path, ".aipack/config.toml");
    }
}
