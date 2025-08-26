/// # Basic Validation Example
/// 
/// This example demonstrates the simplest usage of the evaluation validation library.
/// It runs all validation checks with default configuration and displays the results.

use anyhow::Result;
use evaluation_validator::{init_logging, validate_evaluation_prerequisites};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging to see validation progress
    init_logging();
    
    println!("🔍 Running basic evaluation pre-flight validation...\n");
    
    // Run validation with default configuration
    let result = validate_evaluation_prerequisites().await?;
    
    // Display overall validation status
    if result.is_valid {
        println!("✅ All validation checks passed!");
        println!("\n📊 Summary:");
        println!("  • Available models: {}", result.summary.available_models);
        println!("  • Disk space available: {:.2} GB", result.summary.available_disk_space_gb);
        println!("  • Ollama response time: {}ms", result.summary.ollama_response_time_ms);
        
        if !result.warnings.is_empty() {
            println!("\n⚠️ Warnings:");
            for warning in &result.warnings {
                println!("  • {}", warning);
            }
        }
        
        if !result.recommendations.is_empty() {
            println!("\n💡 Recommendations:");
            for rec in &result.recommendations {
                if rec.priority == "High" || rec.priority == "Medium" {
                    println!("  • [{}] {}: {}", rec.priority, rec.category, rec.description);
                }
            }
        }
        
    } else {
        println!("❌ Validation failed!");
        
        if !result.critical_issues.is_empty() {
            println!("\n🚨 Critical Issues:");
            for issue in &result.critical_issues {
                println!("  • {}", issue);
            }
        }
        
        if !result.recommendations.is_empty() {
            println!("\n💡 Immediate Actions Required:");
            for rec in &result.recommendations {
                if rec.priority == "High" {
                    println!("  • {}", rec.action);
                }
            }
        }
    }
    
    println!("\n🕐 Validation completed at: {}", result.timestamp);
    
    // Exit with appropriate code for shell scripting
    std::process::exit(if result.is_valid { 0 } else { 1 });
}
