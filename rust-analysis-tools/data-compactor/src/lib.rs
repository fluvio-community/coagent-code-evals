use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

pub mod efficient_compactor;
pub use efficient_compactor::{EfficientCompactor, EfficientCompactedData};

pub mod truly_efficient_compactor;
pub use truly_efficient_compactor::{TrulyEfficientCompactor, CompactFormat};

pub mod compactor_comparison_test;

/// Data compactor for atomic server data optimization
#[derive(Debug, Clone)]
pub struct DataCompactor {
    /// URL mappings for categorical encoding
    url_mappings: HashMap<String, u32>,
    /// String mappings for repeated values
    string_mappings: HashMap<String, u32>,
    /// Property mappings for common atomic data properties
    property_mappings: HashMap<String, String>,
    /// Next available ID for mappings
    next_id: u32,
}

/// Compacted data structure with lookup tables
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompactedData {
    /// The compacted JSON data
    pub data: Value,
    /// URL lookup table
    pub url_lookup: HashMap<u32, String>,
    /// String lookup table
    pub string_lookup: HashMap<u32, String>,
    /// Property abbreviation lookup
    pub property_lookup: HashMap<String, String>,
    /// Compression statistics
    pub stats: CompressionStats,
}

/// Compression statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionStats {
    pub original_size: usize,
    pub compacted_size: usize,
    pub compression_ratio: f32,
    pub urls_compressed: u32,
    pub strings_compressed: u32,
    pub properties_abbreviated: u32,
}

impl DataCompactor {
    /// Create a new data compactor
    pub fn new() -> Self {
        Self {
            url_mappings: HashMap::new(),
            string_mappings: HashMap::new(),
            property_mappings: Self::initialize_property_mappings(),
            next_id: 1,
        }
    }

    /// Initialize common property abbreviations for Atomic Data
    fn initialize_property_mappings() -> HashMap<String, String> {
        let mut mappings = HashMap::new();

        // Atomic Data core properties
        mappings.insert(
            "https://atomicdata.dev/properties/isA".to_string(),
            "isA".to_string(),
        );
        mappings.insert(
            "https://atomicdata.dev/properties/parent".to_string(),
            "parent".to_string(),
        );
        mappings.insert(
            "https://atomicdata.dev/properties/lastCommit".to_string(),
            "lastCommit".to_string(),
        );
        mappings.insert(
            "https://atomicdata.dev/properties/subresources".to_string(),
            "subresources".to_string(),
        );

        // Company-specific properties (shortened)
        mappings.insert(
            "https://common.terraphim.io/01jxw2jx8qze6yakh4fz24mnhy/property/company-name"
                .to_string(),
            "companyName".to_string(),
        );
        mappings.insert(
            "https://common.terraphim.io/01jxw2jx8qze6yakh4fz24mnhy/property/company-description"
                .to_string(),
            "companyDesc".to_string(),
        );
        mappings.insert(
            "https://common.terraphim.io/01jxw2jx8qze6yakh4fz24mnhy/property/business-website"
                .to_string(),
            "website".to_string(),
        );
        mappings.insert("https://common.terraphim.io/01jxw2jx8qze6yakh4fz24mnhy/property/company-registration-number".to_string(), "regNumber".to_string());
        mappings.insert(
            "https://common.terraphim.io/01jxw2jx8qze6yakh4fz24mnhy/property/year-of-incorporation"
                .to_string(),
            "yearInc".to_string(),
        );
        mappings.insert("https://common.terraphim.io/01jxw2jx8qze6yakh4fz24mnhy/property/country-of-registration".to_string(), "country".to_string());
        mappings.insert(
            "https://common.terraphim.io/01jxw2jx8qze6yakh4fz24mnhy/property/trading-name"
                .to_string(),
            "tradingName".to_string(),
        );
        mappings.insert(
            "https://common.terraphim.io/01jxw2jx8qze6yakh4fz24mnhy/property/business-type"
                .to_string(),
            "bizType".to_string(),
        );
        mappings.insert(
            "https://common.terraphim.io/01jxw2jx8qze6yakh4fz24mnhy/property/annual-revenue"
                .to_string(),
            "revenue".to_string(),
        );
        mappings.insert(
            "https://common.terraphim.io/01jxw2jx8qze6yakh4fz24mnhy/property/number-of-employees"
                .to_string(),
            "employees".to_string(),
        );
        mappings.insert(
            "https://common.terraphim.io/01jxw2jx8qze6yakh4fz24mnhy/property/years-in-business"
                .to_string(),
            "yearsInBiz".to_string(),
        );
        mappings.insert("https://common.terraphim.io/01jxw2jx8qze6yakh4fz24mnhy/property/is-business-female-lead".to_string(), "femaleLed".to_string());
        mappings.insert(
            "https://common.terraphim.io/01jxw2jx8qze6yakh4fz24mnhy/property/board-of-directors"
                .to_string(),
            "board".to_string(),
        );
        mappings.insert("https://common.terraphim.io/01jxw2jx8qze6yakh4fz24mnhy/property/key-management-personnel".to_string(), "management".to_string());
        mappings.insert(
            "https://common.terraphim.io/01jxw2jx8qze6yakh4fz24mnhy/property/business-owners"
                .to_string(),
            "owners".to_string(),
        );
        mappings.insert(
            "https://common.terraphim.io/01jxw2jx8qze6yakh4fz24mnhy/property/business-auditors"
                .to_string(),
            "auditors".to_string(),
        );

        mappings
    }

    /// Compact comprehensive subresource data
    pub fn compact_comprehensive_data(&mut self, data: &Value) -> Result<CompactedData> {
        let original_json = serde_json::to_string(data)?;
        let original_size = original_json.len();

        let mut stats = CompressionStats {
            original_size,
            compacted_size: 0,
            compression_ratio: 0.0,
            urls_compressed: 0,
            strings_compressed: 0,
            properties_abbreviated: 0,
        };

        // Apply structural compaction first (before URLs are compressed)
        let structurally_compacted = self.apply_structural_compaction(data.clone())?;

        // Then process the data with compression
        let compacted_data = self.compact_value(structurally_compacted, &mut stats)?;

        let compacted_json = serde_json::to_string(&compacted_data)?;
        stats.compacted_size = compacted_json.len();
        stats.compression_ratio =
            (original_size - stats.compacted_size) as f32 / original_size as f32;

        Ok(CompactedData {
            data: compacted_data,
            url_lookup: self.create_reverse_url_lookup(),
            string_lookup: self.create_reverse_string_lookup(),
            property_lookup: self.property_mappings.clone(),
            stats,
        })
    }

    /// Compact a JSON value recursively
    fn compact_value(&mut self, value: Value, stats: &mut CompressionStats) -> Result<Value> {
        match value {
            Value::Object(obj) => {
                let mut compacted_obj = serde_json::Map::new();

                for (key, val) in obj {
                    // Apply property abbreviation
                    let compacted_key = if let Some(abbreviated) = self.property_mappings.get(&key)
                    {
                        stats.properties_abbreviated += 1;
                        abbreviated.clone()
                    } else {
                        key
                    };

                    let compacted_val = self.compact_value(val, stats)?;
                    compacted_obj.insert(compacted_key, compacted_val);
                }

                Ok(Value::Object(compacted_obj))
            }
            Value::Array(arr) => {
                let mut compacted_arr = Vec::new();
                for item in arr {
                    compacted_arr.push(self.compact_value(item, stats)?);
                }
                Ok(Value::Array(compacted_arr))
            }
            Value::String(s) => {
                // Check if it's a URL
                if self.is_url(&s) {
                    if let Some(&id) = self.url_mappings.get(&s) {
                        Ok(Value::Number(serde_json::Number::from(id)))
                    } else {
                        let id = self.next_id;
                        self.url_mappings.insert(s, id);
                        self.next_id += 1;
                        stats.urls_compressed += 1;
                        Ok(Value::Number(serde_json::Number::from(id)))
                    }
                } else {
                    // Apply categorical encoding for repeated strings
                    self.compress_string(s, stats)
                }
            }
            Value::Number(n) => {
                // Apply numerical compression
                self.compress_number(n)
            }
            Value::Bool(b) => {
                // Convert booleans to compact form
                Ok(Value::String(if b {
                    "T".to_string()
                } else {
                    "F".to_string()
                }))
            }
            _ => Ok(value),
        }
    }

    /// Check if a string is likely a URL
    fn is_url(&self, s: &str) -> bool {
        s.starts_with("http://") || s.starts_with("https://")
    }

    /// Compress repeated strings with categorical encoding
    fn compress_string(&mut self, s: String, stats: &mut CompressionStats) -> Result<Value> {
        // Only compress strings that appear multiple times or are very long
        if s.len() > 50 || self.string_mappings.contains_key(&s) {
            if let Some(&id) = self.string_mappings.get(&s) {
                Ok(Value::Number(serde_json::Number::from(id)))
            } else {
                let id = self.next_id;
                self.string_mappings.insert(s, id);
                self.next_id += 1;
                stats.strings_compressed += 1;
                Ok(Value::Number(serde_json::Number::from(id)))
            }
        } else {
            Ok(Value::String(s))
        }
    }

    /// Compress numerical values
    fn compress_number(&self, n: serde_json::Number) -> Result<Value> {
        if let Some(f) = n.as_f64() {
            // Round to reasonable precision
            let rounded = if f.abs() > 1000.0 {
                // Use scientific notation for large numbers
                format!("{:.2e}", f)
            } else if f.abs() < 0.01 {
                // Use scientific notation for small numbers
                format!("{:.2e}", f)
            } else {
                // Round to 2 decimal places
                format!("{:.2}", f)
            };

            // Try to parse back as number, fallback to string
            if let Ok(parsed) = rounded.parse::<f64>() {
                Ok(Value::Number(
                    serde_json::Number::from_f64(parsed).unwrap_or(n),
                ))
            } else {
                Ok(Value::String(rounded))
            }
        } else {
            Ok(Value::Number(n))
        }
    }

    /// Apply structural compaction by grouping related data
    fn apply_structural_compaction(&self, mut data: Value) -> Result<Value> {
        if let Value::Object(ref mut obj) = data {
            // Group subresources by type
            if let Some(Value::Array(subresources)) = obj.get("subresources").cloned() {
                let mut grouped = serde_json::Map::new();

                for subresource in subresources {
                    if let Value::Object(sub_obj) = &subresource {
                        if let Some(Value::String(resource_type)) = sub_obj.get("resource_type") {
                            // Extract the class name from URL
                            let type_name = resource_type
                                .split('/')
                                .next_back()
                                .unwrap_or("unknown")
                                .replace("-step", "")
                                .replace("-", "_");

                            grouped
                                .entry(type_name.clone())
                                .or_insert_with(|| Value::Array(Vec::new()));
                            if let Some(Value::Array(arr)) = grouped.get_mut(&type_name) {
                                arr.push(subresource);
                            }
                        }
                    }
                }

                obj.insert("subresources_grouped".to_string(), Value::Object(grouped));
                obj.remove("subresources");
            }
        }

        Ok(data)
    }

    /// Create reverse lookup for URLs
    fn create_reverse_url_lookup(&self) -> HashMap<u32, String> {
        self.url_mappings
            .iter()
            .map(|(k, &v)| (v, k.clone()))
            .collect()
    }

    /// Create reverse lookup for strings
    fn create_reverse_string_lookup(&self) -> HashMap<u32, String> {
        self.string_mappings
            .iter()
            .map(|(k, &v)| (v, k.clone()))
            .collect()
    }

    /// Decompress data back to original form (for debugging)
    pub fn decompress(&self, compacted: &CompactedData) -> Result<Value> {
        self.decompress_value(
            &compacted.data,
            &compacted.url_lookup,
            &compacted.string_lookup,
        )
    }

    /// Recursively decompress a value
    fn decompress_value(
        &self,
        value: &Value,
        url_lookup: &HashMap<u32, String>,
        string_lookup: &HashMap<u32, String>,
    ) -> Result<Value> {
        match value {
            Value::Object(obj) => {
                let mut decompressed_obj = serde_json::Map::new();

                for (key, val) in obj {
                    // Reverse property abbreviation
                    let original_key = self
                        .property_mappings
                        .iter()
                        .find(|(_, v)| *v == key)
                        .map(|(k, _)| k.clone())
                        .unwrap_or_else(|| key.clone());

                    let decompressed_val = self.decompress_value(val, url_lookup, string_lookup)?;
                    decompressed_obj.insert(original_key, decompressed_val);
                }

                Ok(Value::Object(decompressed_obj))
            }
            Value::Array(arr) => {
                let mut decompressed_arr = Vec::new();
                for item in arr {
                    decompressed_arr.push(self.decompress_value(
                        item,
                        url_lookup,
                        string_lookup,
                    )?);
                }
                Ok(Value::Array(decompressed_arr))
            }
            Value::Number(n) => {
                if let Some(id) = n.as_u64() {
                    let id = id as u32;
                    // Check if this is a compressed URL
                    if let Some(url) = url_lookup.get(&id) {
                        Ok(Value::String(url.clone()))
                    }
                    // Check if this is a compressed string
                    else if let Some(s) = string_lookup.get(&id) {
                        Ok(Value::String(s.clone()))
                    } else {
                        Ok(Value::Number(n.clone()))
                    }
                } else {
                    Ok(Value::Number(n.clone()))
                }
            }
            Value::String(s) => {
                // Convert compact booleans back
                match s.as_str() {
                    "T" => Ok(Value::Bool(true)),
                    "F" => Ok(Value::Bool(false)),
                    _ => Ok(Value::String(s.clone())),
                }
            }
            _ => Ok(value.clone()),
        }
    }
}

impl Default for DataCompactor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_url_compression() {
        let mut compactor = DataCompactor::new();
        let data = json!({
            "id": "https://common.terraphim.io/01k2cxga1cqmqsgvqk0enxq8a5",
            "parent": "https://common.terraphim.io/01k2cxg9ndzyyfd357hz7npa38"
        });

        let compacted = compactor.compact_comprehensive_data(&data).unwrap();
        assert!(compacted.stats.urls_compressed > 0);
        assert!(compacted.stats.compression_ratio > 0.0);
    }

    #[test]
    fn test_property_abbreviation() {
        let mut compactor = DataCompactor::new();
        let data = json!({
            "https://common.terraphim.io/01jxw2jx8qze6yakh4fz24mnhy/property/company-name": "Test Company"
        });

        let compacted = compactor.compact_comprehensive_data(&data).unwrap();
        assert!(compacted.stats.properties_abbreviated > 0);
    }

    #[test]
    fn test_boolean_compression() {
        let mut compactor = DataCompactor::new();
        let data = json!({
            "active": true,
            "verified": false
        });

        let compacted = compactor.compact_comprehensive_data(&data).unwrap();
        // Should compress booleans to T/F strings
        if let Value::Object(obj) = &compacted.data {
            assert_eq!(obj.get("active"), Some(&Value::String("T".to_string())));
            assert_eq!(obj.get("verified"), Some(&Value::String("F".to_string())));
        }
    }
}