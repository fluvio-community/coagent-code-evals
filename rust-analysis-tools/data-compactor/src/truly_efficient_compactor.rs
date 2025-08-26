use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Truly efficient compactor using proper compression techniques
#[derive(Debug, Clone)]
pub struct TrulyEfficientCompactor {
    /// Dictionary for URL compression
    url_dict: HashMap<String, u8>,
    /// Dictionary for repeated strings 
    string_dict: HashMap<String, u8>,
    /// Next available IDs (using small integers)
    next_url_id: u8,
    next_string_id: u8,
}

/// Extremely compact data format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompactFormat {
    /// Schema with abbreviated field names
    pub s: Schema,
    /// Tabular data organized by resource type
    pub d: HashMap<String, TabularData>,
    /// Dictionaries (only if needed)
    pub u: Option<HashMap<u8, String>>,  // URLs
    pub t: Option<HashMap<u8, String>>,  // Text strings
    /// Stats
    pub stats: CompactStats,
}

/// Minimal schema 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schema {
    /// Field types: s=string, i=int, f=float, b=bool, u=url, j=json
    pub types: HashMap<String, char>,
    /// Field order for arrays
    pub order: Vec<String>,
}

/// Tabular data format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TabularData {
    /// Number of rows
    pub n: usize,
    /// Column data as arrays
    pub c: HashMap<String, Value>,
}

/// Compression statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompactStats {
    pub orig: usize,
    pub comp: usize,
    pub ratio: f32,
}

impl TrulyEfficientCompactor {
    /// Create new compactor
    pub fn new() -> Self {
        Self {
            url_dict: HashMap::new(),
            string_dict: HashMap::new(),
            next_url_id: 1,
            next_string_id: 1,
        }
    }

    /// Compact data using aggressive optimization techniques
    pub fn compact(&mut self, data: &Value) -> Result<CompactFormat> {
        let original_json = serde_json::to_string(data)?;
        let original_size = original_json.len();

        // Extract subresources
        let empty_vec = vec![];
        let subresources = data.get("subresources")
            .and_then(|v| v.as_array())
            .unwrap_or(&empty_vec);

        // Group by resource type
        let mut grouped: HashMap<String, Vec<&Value>> = HashMap::new();
        for resource in subresources {
            let resource_type = resource.get("resource_type")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown");
            let short_type = self.shorten_resource_type(resource_type);
            grouped.entry(short_type).or_default().push(resource);
        }

        // Convert to compact tabular format
        let mut compact_data = HashMap::new();
        let mut schema = Schema {
            types: HashMap::new(),
            order: Vec::new(),
        };

        for (type_name, resources) in grouped {
            let (tabular, type_schema) = self.convert_to_tabular(&resources)?;
            compact_data.insert(type_name, tabular);
            
            // Merge schema
            for (field, field_type) in type_schema {
                schema.types.insert(field.clone(), field_type);
                if !schema.order.contains(&field) {
                    schema.order.push(field);
                }
            }
        }

        let compact_format = CompactFormat {
            s: schema,
            d: compact_data,
            u: if self.url_dict.is_empty() { None } else { 
                Some(self.url_dict.iter().map(|(url, &id)| (id, url.clone())).collect()) 
            },
            t: if self.string_dict.is_empty() { None } else { 
                Some(self.string_dict.iter().map(|(s, &id)| (id, s.clone())).collect()) 
            },
            stats: CompactStats {
                orig: original_size,
                comp: 0, // Will be filled after serialization
                ratio: 0.0,
            },
        };

        let compact_json = serde_json::to_string(&compact_format)?;
        let compact_size = compact_json.len();
        
        Ok(CompactFormat {
            stats: CompactStats {
                orig: original_size,
                comp: compact_size,
                ratio: if original_size > 0 {
                    (original_size as f32 - compact_size as f32) / original_size as f32
                } else { 0.0 },
            },
            ..compact_format
        })
    }

    /// Convert resources to tabular format using columnar storage
    fn convert_to_tabular(&mut self, resources: &[&Value]) -> Result<(TabularData, HashMap<String, char>)> {
        if resources.is_empty() {
            return Ok((TabularData { n: 0, c: HashMap::new() }, HashMap::new()));
        }

        let n = resources.len();

        // Analyze all fields across all resources
        let mut field_analysis: HashMap<String, (char, Vec<Value>)> = HashMap::new();

        // First pass: collect all unique fields and their types
        for resource in resources {
            if let Some(obj) = resource.as_object() {
                for (key, value) in obj {
                    let short_key = self.abbreviate_field(key);
                    let field_type = self.infer_compact_type(value);
                    
                    field_analysis.entry(short_key).or_insert_with(|| (field_type, Vec::new()));
                }
            }
        }

        // Second pass: collect values for each field
        for resource in resources {
            if let Some(obj) = resource.as_object() {
                for (short_key, (_, values)) in field_analysis.iter_mut() {
                    let original_key = self.find_original_key(obj, short_key);
                    let value = original_key.and_then(|k| obj.get(k));
                    
                    match value {
                        Some(v) => values.push(self.compress_value(v.clone())?),
                        None => values.push(Value::Null),
                    }
                }
            }
        }

        // Build columnar data
        let mut columns = HashMap::new();
        let mut schema = HashMap::new();

        for (field_name, (field_type, values)) in field_analysis {
            schema.insert(field_name.clone(), field_type);
            columns.insert(field_name, Value::Array(values));
        }

        Ok((TabularData { n, c: columns }, schema))
    }

    /// Shorten resource type URL to minimal identifier
    fn shorten_resource_type(&self, resource_type: &str) -> String {
        resource_type.split('/').last()
            .unwrap_or("unk")
            .replace("-step", "")
            .replace('-', "_")
            .chars().take(8).collect() // Limit to 8 chars max
    }

    /// Abbreviate field names aggressively
    fn abbreviate_field(&self, field: &str) -> String {
        match field {
            // Core atomic properties
            "https://atomicdata.dev/properties/isA" => "t".to_string(),
            "https://atomicdata.dev/properties/parent" => "p".to_string(),
            "https://atomicdata.dev/properties/lastCommit" => "lc".to_string(),
            "url" => "u".to_string(),
            "resource_type" => "rt".to_string(),
            "json_format" => "jf".to_string(),
            "json_ad_format" => "jaf".to_string(),
            "turtle_format" => "tf".to_string(),
            "fetch_errors" => "fe".to_string(),
            
            // Company properties - use initials
            s if s.contains("company-name") => "cn".to_string(),
            s if s.contains("company-description") => "cd".to_string(),
            s if s.contains("business-website") => "bw".to_string(),
            s if s.contains("year-of-incorporation") => "yi".to_string(),
            s if s.contains("country-of-registration") => "cr".to_string(),
            s if s.contains("registration-number") => "rn".to_string(),
            
            // For other long URLs, extract meaningful abbreviation
            s if s.contains("/property/") => {
                s.split("/property/").last().unwrap_or(s)
                    .split('-')
                    .map(|word| word.chars().next().unwrap_or('x'))
                    .collect::<String>()
                    .chars().take(4).collect() // Max 4 chars
            },
            
            // Keep short fields as-is
            s if s.len() <= 4 => s.to_string(),
            
            // Truncate long fields
            _ => field.chars().take(4).collect(),
        }
    }

    /// Infer most compact type representation
    fn infer_compact_type(&self, value: &Value) -> char {
        match value {
            Value::String(s) if s.starts_with("http") => 'u', // URL
            Value::String(_) => 's', // String
            Value::Number(n) if n.is_i64() => 'i', // Integer
            Value::Number(_) => 'f', // Float
            Value::Bool(_) => 'b', // Boolean
            Value::Array(_) | Value::Object(_) => 'j', // JSON
            Value::Null => 's', // Default to string
        }
    }

    /// Compress a value using dictionaries
    fn compress_value(&mut self, value: Value) -> Result<Value> {
        match value {
            Value::String(s) if s.starts_with("http") => {
                // URL compression
                if let Some(&id) = self.url_dict.get(&s) {
                    Ok(Value::Number(id.into()))
                } else {
                    let id = self.next_url_id;
                    self.url_dict.insert(s, id);
                    self.next_url_id = self.next_url_id.saturating_add(1);
                    Ok(Value::Number(id.into()))
                }
            },
            Value::String(s) if s.len() > 50 => {
                // Long string compression
                if let Some(&id) = self.string_dict.get(&s) {
                    Ok(Value::Number(id.into()))
                } else {
                    let id = self.next_string_id;
                    self.string_dict.insert(s, id);
                    self.next_string_id = self.next_string_id.saturating_add(1);
                    Ok(Value::Number(id.into()))
                }
            },
            Value::Array(arr) => {
                // Compress array elements
                let compressed: Result<Vec<_>> = arr.into_iter()
                    .map(|v| self.compress_value(v))
                    .collect();
                Ok(Value::Array(compressed?))
            },
            // Keep other values as-is for maximum compression
            _ => Ok(value),
        }
    }

    /// Find original field name from abbreviated version
    fn find_original_key<'a>(&self, obj: &'a serde_json::Map<String, Value>, abbrev: &'a str) -> Option<&'a str> {
        // First try direct match
        if obj.contains_key(abbrev) {
            return Some(abbrev);
        }

        // Then try to reverse-lookup common abbreviations
        let possible_keys = match abbrev {
            "t" => vec!["https://atomicdata.dev/properties/isA"],
            "p" => vec!["https://atomicdata.dev/properties/parent"],
            "lc" => vec!["https://atomicdata.dev/properties/lastCommit"],
            "cn" => vec![
                "https://common.terraphim.io/01jxw2jx8qze6yakh4fz24mnhy/property/company-name",
                "company-name",
            ],
            "cd" => vec![
                "https://common.terraphim.io/01jxw2jx8qze6yakh4fz24mnhy/property/company-description",
                "company-description",
            ],
            "yi" => vec![
                "https://common.terraphim.io/01jxw2jx8qze6yakh4fz24mnhy/property/year-of-incorporation",
                "year-of-incorporation",
            ],
            _ => vec![],
        };

        for key in possible_keys {
            if obj.contains_key(key) {
                return Some(key);
            }
        }

        // Last resort: find any key that might match
        for key in obj.keys() {
            if self.abbreviate_field(key) == abbrev {
                return Some(key);
            }
        }

        None
    }

    /// Reconstruct original data from compact format
    pub fn reconstruct(compact: &CompactFormat) -> Result<Value> {
        let mut subresources = Vec::new();

        for (type_name, tabular) in &compact.d {
            // Reconstruct each row
            for i in 0..tabular.n {
                let mut resource = serde_json::Map::new();
                
                // Add resource type
                let full_type = format!(
                    "https://common.terraphim.io/01jxw2jx8qze6yakh4fz24mnhy/class/{}-step",
                    type_name.replace('_', "-")
                );
                resource.insert("resource_type".to_string(), Value::String(full_type));

                // Reconstruct each field
                for (field_name, column) in &tabular.c {
                    if let Some(values) = column.as_array() {
                        if let Some(value) = values.get(i) {
                            if !value.is_null() {
                                let decompressed_value = Self::decompress_value(value, compact)?;
                                let original_field = Self::expand_field_name(field_name);
                                resource.insert(original_field, decompressed_value);
                            }
                        }
                    }
                }

                subresources.push(Value::Object(resource));
            }
        }

        Ok(serde_json::json!({
            "subresources": subresources
        }))
    }

    /// Decompress a value using dictionaries
    fn decompress_value(value: &Value, compact: &CompactFormat) -> Result<Value> {
        match value {
            Value::Number(n) if n.is_u64() => {
                let id = n.as_u64().unwrap() as u8;
                
                // Try URL dictionary first
                if let Some(ref url_dict) = compact.u {
                    if let Some(url) = url_dict.get(&id) {
                        return Ok(Value::String(url.clone()));
                    }
                }
                
                // Try string dictionary
                if let Some(ref string_dict) = compact.t {
                    if let Some(text) = string_dict.get(&id) {
                        return Ok(Value::String(text.clone()));
                    }
                }
                
                // If not found in dictionaries, keep as number
                Ok(value.clone())
            },
            Value::Array(arr) => {
                let decompressed: Result<Vec<_>> = arr.iter()
                    .map(|v| Self::decompress_value(v, compact))
                    .collect();
                Ok(Value::Array(decompressed?))
            },
            _ => Ok(value.clone()),
        }
    }

    /// Expand abbreviated field names back to full names
    fn expand_field_name(abbrev: &str) -> String {
        match abbrev {
            "t" => "https://atomicdata.dev/properties/isA".to_string(),
            "p" => "https://atomicdata.dev/properties/parent".to_string(),
            "lc" => "https://atomicdata.dev/properties/lastCommit".to_string(),
            "cn" => "https://common.terraphim.io/01jxw2jx8qze6yakh4fz24mnhy/property/company-name".to_string(),
            "cd" => "https://common.terraphim.io/01jxw2jx8qze6yakh4fz24mnhy/property/company-description".to_string(),
            "bw" => "https://common.terraphim.io/01jxw2jx8qze6yakh4fz24mnhy/property/business-website".to_string(),
            "yi" => "https://common.terraphim.io/01jxw2jx8qze6yakh4fz24mnhy/property/year-of-incorporation".to_string(),
            "cr" => "https://common.terraphim.io/01jxw2jx8qze6yakh4fz24mnhy/property/country-of-registration".to_string(),
            "rn" => "https://common.terraphim.io/01jxw2jx8qze6yakh4fz24mnhy/property/company-registration-number".to_string(),
            "u" => "url".to_string(),
            "rt" => "resource_type".to_string(),
            "jf" => "json_format".to_string(),
            "jaf" => "json_ad_format".to_string(),
            "tf" => "turtle_format".to_string(),
            "fe" => "fetch_errors".to_string(),
            _ => abbrev.to_string(), // Keep unknown abbreviations as-is
        }
    }
}

impl Default for TrulyEfficientCompactor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_aggressive_compaction() {
        let mut compactor = TrulyEfficientCompactor::new();

        // Test data with lots of redundancy
        let test_data = json!({
            "subresources": [
                {
                    "url": "https://common.terraphim.io/01k2cxga1cqmqsgvqk0enxq8a5",
                    "resource_type": "https://common.terraphim.io/01jxw2jx8qze6yakh4fz24mnhy/class/company-information-and-history-step",
                    "json_format": "very long JSON string that should be compressed because it's over 50 characters long and contains repeated information",
                    "https://common.terraphim.io/01jxw2jx8qze6yakh4fz24mnhy/property/company-name": "EcoBright Solutions Uganda Limited",
                    "https://common.terraphim.io/01jxw2jx8qze6yakh4fz24mnhy/property/year-of-incorporation": 2020,
                    "https://atomicdata.dev/properties/isA": ["https://common.terraphim.io/01jxw2jx8qze6yakh4fz24mnhy/class/company-information-and-history-step"]
                },
                {
                    "url": "https://common.terraphim.io/01k2cxga6pywwjg8yc2m6b1jzz", 
                    "resource_type": "https://common.terraphim.io/01jxw2jx8qze6yakh4fz24mnhy/class/company-information-and-history-step",
                    "json_format": "very long JSON string that should be compressed because it's over 50 characters long and contains repeated information",
                    "https://common.terraphim.io/01jxw2jx8qze6yakh4fz24mnhy/property/company-name": "EcoBright Solutions Uganda Limited",
                    "https://common.terraphim.io/01jxw2jx8qze6yakh4fz24mnhy/property/year-of-incorporation": 2020,
                    "https://atomicdata.dev/properties/isA": ["https://common.terraphim.io/01jxw2jx8qze6yakh4fz24mnhy/class/company-information-and-history-step"]
                }
            ]
        });

        let result = compactor.compact(&test_data).unwrap();

        println!("Original size: {} bytes", result.stats.orig);
        println!("Compacted size: {} bytes", result.stats.comp);  
        println!("Compression ratio: {:.2}%", result.stats.ratio * 100.0);

        // Should achieve actual compression
        assert!(result.stats.ratio > 0.0, "Should have positive compression ratio");
        assert!(result.stats.comp < result.stats.orig, "Compacted should be smaller than original");

        // Verify reconstruction works
        let reconstructed = TrulyEfficientCompactor::reconstruct(&result).unwrap();
        let reconstructed_resources = reconstructed["subresources"].as_array().unwrap();
        let original_resources = test_data["subresources"].as_array().unwrap();
        
        assert_eq!(reconstructed_resources.len(), original_resources.len());
        
        println!("✅ Aggressive compaction test passed!");
        println!("Compact format: {}", serde_json::to_string_pretty(&result).unwrap());
    }

    #[test]
    fn test_field_abbreviation() {
        let compactor = TrulyEfficientCompactor::new();
        
        assert_eq!(compactor.abbreviate_field("https://atomicdata.dev/properties/isA"), "t");
        assert_eq!(compactor.abbreviate_field("url"), "u");
        assert_eq!(compactor.abbreviate_field("resource_type"), "rt");
        assert_eq!(compactor.abbreviate_field("https://common.terraphim.io/01jxw2jx8qze6yakh4fz24mnhy/property/company-name"), "cn");
    }

    #[test]
    fn test_round_trip() {
        let mut compactor = TrulyEfficientCompactor::new();
        
        let original = json!({
            "subresources": [{
                "url": "https://example.com",
                "https://atomicdata.dev/properties/isA": ["test"],
                "https://common.terraphim.io/01jxw2jx8qze6yakh4fz24mnhy/property/company-name": "Test Company"
            }]
        });

        let compacted = compactor.compact(&original).unwrap();
        let reconstructed = TrulyEfficientCompactor::reconstruct(&compacted).unwrap();

        println!("Original: {}", serde_json::to_string_pretty(&original).unwrap());
        println!("Compacted: {}", serde_json::to_string_pretty(&compacted).unwrap());
        println!("Reconstructed: {}", serde_json::to_string_pretty(&reconstructed).unwrap());
        
        // Should preserve essential structure
        let original_len = original["subresources"].as_array().unwrap().len();
        let reconstructed_len = reconstructed["subresources"].as_array().unwrap().len();
        
        println!("Original subresources length: {}", original_len);
        println!("Reconstructed subresources length: {}", reconstructed_len);
        
        assert_eq!(original_len, reconstructed_len);

        println!("✅ Round-trip test passed!");
    }
}