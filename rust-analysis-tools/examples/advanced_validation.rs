/// # Advanced Validation Example
/// 
/// This example demonstrates advanced usage of the evaluation validation library.
/// It customizes the validation settings and handles detailed results separately.
use anyhow::Result;
use evaluation_validator::{
    init_logging_with_level,
    validate_evaluation_prerequisites_with_config,
    ValidationConfig
};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging at debug level to see detailed validation process
    init_logging_with_level(log::LevelFilter::Debug);
    
    println!("🔍 Running advanced evaluation pre-flight validation...\n");
    
    // Configure custom validation settings
    let config = ValidationConfig {
        ollama_url: "http://localhost:11434".to_string(),
        timeout_seconds: 60,
        min_disk_space_gb: 10.0,
        aipack_config_path: ".aipack/custom_config.toml".to_string(),
        output_directory: "custom_results".to_string(),
        deep_model_validation: true,
    };
    
    // Run validation with customized configuration
    let result = validate_evaluation_prerequisites_with_config(config).await?;
    
    // Handle validation result with detailed output
    println!("📊 Advanced Validation Details:");
    if result.is_valid {
        println!("\n✅ All checks passed with the following configurations:");
        println!("  • Available models: {}", result.summary.available_models);
        println!("  • Disk space available: {:.2} GB", result.summary.available_disk_space_gb);
        println!("  • Ollama response time: {}ms", result.summary.ollama_response_time_ms);
    } else {
        println!("\n❌ Validation encountered issues:");
        for issue in &result.critical_issues {
            println!("  • Critical Issue: {}", issue);
        }
    }
    
    if !result.recommendations.is_empty() {
        println!("\n💡 Suggestions:");
        for rec in result.recommendations {
            println!("  • {} [{}]: {}", rec.priority, rec.category, rec.description);
            println!("     Action: {}", rec.action);
        }
    }
    
    println!("\nValidation completed at: {}", result.timestamp);
    std::process::exit(if result.is_valid { 0 } else { 1 });
}
