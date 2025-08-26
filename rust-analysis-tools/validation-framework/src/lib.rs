/// # Evaluation Pre-flight Validation Library
///
/// This library provides comprehensive validation capabilities for AI model evaluation processes.
/// It ensures all prerequisites are met before starting evaluation, including:
/// - Ollama service health and model availability
/// - Disk space validation for outputs
/// - AIPACK configuration validity
/// - System resource checks
///
/// ## Usage
///
/// ### Basic validation with default configuration:
/// ```rust
/// use evaluation_validator::validation::validate_evaluation_prerequisites;
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     let result = validate_evaluation_prerequisites().await?;
///     
///     if result.is_valid {
///         println!("✅ All validation checks passed!");
///         println!("Available models: {}", result.summary.available_models);
///     } else {
///         println!("❌ Validation failed:");
///         for issue in &result.critical_issues {
///             println!("  - {}", issue);
///         }
///     }
///     
///     Ok(())
/// }
/// ```
///
/// ### Advanced validation with custom configuration:
/// ```rust
/// use evaluation_validator::validation::{ValidationConfig, validate_evaluation_prerequisites_with_config};
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     let config = ValidationConfig {
///         ollama_url: "http://localhost:11434".to_string(),
///         timeout_seconds: 60,
///         min_disk_space_gb: 10.0,
///         aipack_config_path: "custom_config.toml".to_string(),
///         output_directory: "custom_results".to_string(),
///         deep_model_validation: true,
///     };
///     
///     let result = validate_evaluation_prerequisites_with_config(config).await?;
///     
///     // Handle validation results...
///     Ok(())
/// }
/// ```

pub mod validation;
pub mod evaluation;

use std::sync::Once;

static INIT: Once = Once::new();

/// Initialize logging for the validation library
/// 
/// This sets up structured logging with appropriate levels for validation processes.
/// Call this once at the start of your application to enable detailed validation logging.
pub fn init_logging() {
    INIT.call_once(|| {
        env_logger::Builder::from_default_env()
            .filter_level(log::LevelFilter::Info)
            .format_timestamp_secs()
            .format_module_path(false)
            .init();
        
        log::info!("🔧 Evaluation validation library initialized");
    });
}

/// Initialize logging with custom level
/// 
/// Allows specifying a custom log level for more granular control over validation output.
pub fn init_logging_with_level(level: log::LevelFilter) {
    INIT.call_once(|| {
        env_logger::Builder::from_default_env()
            .filter_level(level)
            .format_timestamp_secs()
            .format_module_path(false)
            .init();
        
        log::info!("🔧 Evaluation validation library initialized with level: {:?}", level);
    });
}

// Re-export main validation types for convenience
pub use validation::{
    ValidationResult, 
    ValidationSummary, 
    ValidationConfig, 
    ValidationCoordinator,
    validate_evaluation_prerequisites,
    validate_evaluation_prerequisites_with_config
};
