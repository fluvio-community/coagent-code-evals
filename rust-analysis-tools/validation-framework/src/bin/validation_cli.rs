/// # Validation CLI Tool
/// 
/// Command-line interface for running comprehensive evaluation pre-flight validation.
/// This tool provides an easy way to validate all prerequisites before running 
/// model evaluations.

use anyhow::Result;
use evaluation_validator::{
    ValidationConfig, 
    validate_evaluation_prerequisites, 
    validate_evaluation_prerequisites_with_config,
    init_logging_with_level
};
use std::env;
use serde_json;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    init_logging_with_level(log::LevelFilter::Info);
    
    let args: Vec<String> = env::args().collect();
    
    // Parse command line arguments
    let (config_path, output_format, verbose) = parse_args(&args);
    
    // Set verbose logging if requested
    if verbose {
        init_logging_with_level(log::LevelFilter::Debug);
    }
    
    println!("🔍 Starting comprehensive evaluation pre-flight validation...\n");
    
    // Run validation with appropriate configuration
    let result = if let Some(path) = config_path {
        println!("📋 Using custom configuration: {}", path);
        let config = ValidationConfig {
            aipack_config_path: path,
            ..ValidationConfig::default()
        };
        validate_evaluation_prerequisites_with_config(config).await?
    } else {
        println!("📋 Using default configuration");
        validate_evaluation_prerequisites().await?
    };
    
    // Display results based on format
    match output_format.as_str() {
        "json" => {
            println!("{}", serde_json::to_string_pretty(&result)?);
        },
        "summary" => {
            print_summary_format(&result);
        },
        _ => {
            print_detailed_format(&result);
        }
    }
    
    // Exit with appropriate code
    std::process::exit(if result.is_valid { 0 } else { 1 });
}

/// Parse command line arguments
fn parse_args(args: &[String]) -> (Option<String>, String, bool) {
    let mut config_path = None;
    let mut output_format = "detailed".to_string();
    let mut verbose = false;
    
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--config" | "-c" => {
                if i + 1 < args.len() {
                    config_path = Some(args[i + 1].clone());
                    i += 2;
                } else {
                    eprintln!("Error: --config requires a path argument");
                    std::process::exit(1);
                }
            },
            "--format" | "-f" => {
                if i + 1 < args.len() {
                    output_format = args[i + 1].clone();
                    i += 2;
                } else {
                    eprintln!("Error: --format requires an argument (detailed|summary|json)");
                    std::process::exit(1);
                }
            },
            "--verbose" | "-v" => {
                verbose = true;
                i += 1;
            },
            "--help" | "-h" => {
                print_help();
                std::process::exit(0);
            },
            _ => {
                eprintln!("Error: Unknown argument: {}", args[i]);
                print_help();
                std::process::exit(1);
            }
        }
    }
    
    (config_path, output_format, verbose)
}

/// Print help information
fn print_help() {
    println!("Evaluation Pre-flight Validation CLI");
    println!();
    println!("USAGE:");
    println!("    validation-cli [OPTIONS]");
    println!();
    println!("OPTIONS:");
    println!("    -c, --config <PATH>     Path to AIPACK configuration file");
    println!("    -f, --format <FORMAT>   Output format: detailed, summary, json [default: detailed]");
    println!("    -v, --verbose           Enable verbose logging");
    println!("    -h, --help              Print this help information");
    println!();
    println!("EXAMPLES:");
    println!("    validation-cli                              # Run with default settings");
    println!("    validation-cli --config .aipack/config.toml # Use custom config");
    println!("    validation-cli --format json                # Output as JSON");
    println!("    validation-cli --verbose                    # Enable debug logging");
}

/// Print detailed validation results
fn print_detailed_format(result: &evaluation_validator::ValidationResult) {
    println!("📊 VALIDATION RESULTS");
    println!("════════════════════════════════════════════════════════════════");
    
    // Overall status
    if result.is_valid {
        println!("✅ OVERALL STATUS: PASSED");
    } else {
        println!("❌ OVERALL STATUS: FAILED");
    }
    println!();
    
    // Summary metrics
    println!("📈 SUMMARY:");
    println!("  Available Models: {}", result.summary.available_models);
    println!("  Missing Models: {}", result.summary.missing_models);
    println!("  Available Disk Space: {:.2} GB", result.summary.available_disk_space_gb);
    println!("  Estimated Space Needed: {:.2} GB", result.summary.estimated_space_needed_gb);
    println!("  Ollama Response Time: {}ms", result.summary.ollama_response_time_ms);
    println!("  AIPACK Config Valid: {}", result.summary.aipack_config_valid);
    println!();
    
    // Individual check results
    println!("🔍 DETAILED CHECKS:");
    print_check_result("Ollama Service", &result.checks.ollama_service);
    print_check_result("Model Availability", &result.checks.model_availability);
    print_check_result("Disk Space", &result.checks.disk_space);
    print_check_result("AIPACK Config", &result.checks.aipack_config);
    print_check_result("System Resources", &result.checks.system_resources);
    println!();
    
    // Critical issues
    if !result.critical_issues.is_empty() {
        println!("🚨 CRITICAL ISSUES:");
        for issue in &result.critical_issues {
            println!("  ❌ {}", issue);
        }
        println!();
    }
    
    // Warnings
    if !result.warnings.is_empty() {
        println!("⚠️  WARNINGS:");
        for warning in &result.warnings {
            println!("  ⚠️  {}", warning);
        }
        println!();
    }
    
    // Recommendations
    if !result.recommendations.is_empty() {
        println!("💡 RECOMMENDATIONS:");
        for rec in &result.recommendations {
            println!("  {} [{}]: {}", 
                match rec.priority.as_str() {
                    "High" => "🔴",
                    "Medium" => "🟡", 
                    "Low" => "🟢",
                    _ => "📋"
                },
                rec.category, 
                rec.description
            );
            println!("     Action: {}", rec.action);
        }
        println!();
    }
    
    println!("Validation completed at: {}", result.timestamp);
}

/// Print summary validation results
fn print_summary_format(result: &evaluation_validator::ValidationResult) {
    println!("VALIDATION SUMMARY");
    println!("==================");
    
    let status = if result.is_valid { "PASSED ✅" } else { "FAILED ❌" };
    println!("Status: {}", status);
    println!("Models Available: {}", result.summary.available_models);
    println!("Disk Space: {:.1}GB available", result.summary.available_disk_space_gb);
    println!("Response Time: {}ms", result.summary.ollama_response_time_ms);
    
    if !result.critical_issues.is_empty() {
        println!("\nCritical Issues: {}", result.critical_issues.len());
        for issue in &result.critical_issues {
            println!("  - {}", issue);
        }
    }
    
    if !result.recommendations.is_empty() {
        println!("\nRecommendations: {}", result.recommendations.len());
        for rec in &result.recommendations {
            if rec.priority == "High" {
                println!("  - [{}] {}", rec.priority, rec.description);
            }
        }
    }
}

/// Print individual check result
fn print_check_result(name: &str, check: &evaluation_validator::validation::CheckResult) {
    let status = if check.passed { "✅" } else { "❌" };
    println!("  {} {}: {} ({}ms)", status, name, check.message, check.duration_ms);
    
    if !check.metadata.is_empty() && log::log_enabled!(log::Level::Debug) {
        for (key, value) in &check.metadata {
            println!("     {}: {}", key, value);
        }
    }
}
