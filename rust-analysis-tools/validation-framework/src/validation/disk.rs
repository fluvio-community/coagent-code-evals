/// # Disk Space Validation Module
/// 
/// This module checks for sufficient available disk space to store evaluation outputs.
/// It provides a set of utility functions for:
/// - Checking available disk space for a given directory
/// - Estimating space requirements based on model outputs

use std::collections::HashMap;
use std::path::Path;
use anyhow::{Result, Context};
use tokio::fs;
use tokio::process::Command;

/// Disk space validation result
#[derive(Debug, Clone)]
pub struct DiskValidationResult {
    /// Available space in GB
    pub available_space_gb: f64,
    /// Required space in GB
    pub required_space_gb: f64,
    /// Whether the required space is available
    pub sufficient_space: bool,
}

impl DiskValidationResult {
    /// Convert result to metadata for diagnostics
    pub fn into_metadata(self) -> HashMap<String, String> {
        let mut metadata = HashMap::new();
        metadata.insert("available_space_gb".to_string(), format!("{:.2}", self.available_space_gb));
        metadata.insert("required_space_gb".to_string(), format!("{:.2}", self.required_space_gb));
        metadata.insert("sufficient_space".to_string(), self.sufficient_space.to_string());
        metadata
    }
}

/// Disk space validation logic
pub struct DiskValidator;

impl DiskValidator {
    /// Create a new disk space validator
    pub fn new() -> Self {
        DiskValidator
    }
    
    /// Validate if the directory has sufficient space for evaluation results
    pub async fn validate_space(&self, path: &str, space_needed_gb: f64) -> Result<DiskValidationResult> {
        let path_obj = Path::new(path);
        let available_space_gb = self.get_available_disk_space_gb(path_obj).await?;
        let sufficient_space = available_space_gb >= space_needed_gb;

        Ok(DiskValidationResult {
            available_space_gb,
            required_space_gb: space_needed_gb,
            sufficient_space,
        })
    }

    /// Get the available disk space in GB for a given path
    async fn get_available_disk_space_gb(&self, path: &Path) -> Result<f64> {
        // Create directory if it doesn't exist for checking
        if !path.exists() {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).await.context("Failed to create output directory")?;
            }
        }

        // Use `df` command which works on both Linux and macOS
        let output = Command::new("df")
            .arg("-k") // Output in 1K blocks
            .arg(path)
            .output()
            .await
            .context("Failed to execute df command")?;

        if !output.status.success() {
            anyhow::bail!("df command failed: {}", String::from_utf8_lossy(&output.stderr));
        }

        let output_str = String::from_utf8(output.stdout)
            .context("Failed to parse df output as UTF-8")?;

        // Parse df output - second line contains the data we need
        let lines: Vec<&str> = output_str.trim().lines().collect();
        if lines.len() < 2 {
            anyhow::bail!("Unexpected df output format");
        }

        // df output format: Filesystem 1K-blocks Used Available Use% Mounted-on
        // We want the "Available" column (index 3)
        let data_line = lines[1];
        let fields: Vec<&str> = data_line.split_whitespace().collect();
        
        if fields.len() < 4 {
            anyhow::bail!("Unable to parse available space from df output");
        }

        let available_kb = fields[3].parse::<f64>()
            .context("Failed to parse available space as number")?;
        
        // Convert from KB to GB
        Ok(available_kb / 1_048_576.0) // 1024 * 1024 = 1,048,576 KB per GB
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_disk_validation_result_metadata() {
        let result = DiskValidationResult {
            available_space_gb: 20.0,
            required_space_gb: 5.0,
            sufficient_space: true,
        };

        let metadata = result.into_metadata();
        assert_eq!(metadata["available_space_gb"], "20.00");
        assert_eq!(metadata["required_space_gb"], "5.00");
        assert!(metadata["sufficient_space"].parse::<bool>().unwrap());
    }
}
