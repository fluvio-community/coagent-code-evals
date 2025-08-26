use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Efficient data compactor using structured format optimization
#[derive(Debug, Clone)]
pub struct EfficientCompactor {
    /// URL dictionary for categorical encoding
    url_dict: HashMap<String, u16>,
    /// String dictionary for repeated content
    string_dict: HashMap<String, u16>,
    /// Common property abbreviations
    property_abbrevs: HashMap<String, &'static str>,
    /// Next available ID
    next_url_id: u16,
    next_string_id: u16,
}

/// Compacted data using structured format optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EfficientCompactedData {
    /// Schema definition with type hints
    pub schema: CompactionSchema,
    /// Columnar data organized by type
    pub data: ColumnarData,
    /// Lookup dictionaries
    pub dictionaries: Dictionaries,
    /// Compression statistics
    pub stats: CompressionStats,
}

/// Schema definition with type information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompactionSchema {
    /// Resource types and their structure
    pub resource_types: HashMap<String, ResourceSchema>,
    /// Data types for each field
    pub field_types: HashMap<String, FieldType>,
}

/// Schema for a specific resource type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceSchema {
    /// Required fields
    pub required: Vec<String>,
    /// Optional fields
    pub optional: Vec<String>,
    /// Field data types
    pub types: HashMap<String, FieldType>,
}

/// Field data types for efficient storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FieldType {
    Str,
    Int,
    Float,
    Bool,
    Url,
    Json,
    Array(Box<FieldType>),
}

/// Columnar data storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnarData {
    /// Resources grouped by type
    pub resources: HashMap<String, TypedResourceGroup>,
}

/// Typed resource group with columnar storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypedResourceGroup {
    /// Count of resources
    pub count: usize,
    /// String columns
    pub str_cols: HashMap<String, Vec<Option<u16>>>,
    /// Integer columns
    pub int_cols: HashMap<String, Vec<Option<i64>>>,
    /// Float columns
    pub float_cols: HashMap<String, Vec<Option<f64>>>,
    /// Boolean columns (packed as bits)
    pub bool_cols: HashMap<String, Vec<bool>>,
    /// URL columns (references to dictionary)
    pub url_cols: HashMap<String, Vec<Option<u16>>>,
    /// JSON columns (for complex nested data)
    pub json_cols: HashMap<String, Vec<Option<u16>>>,
}

/// Lookup dictionaries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dictionaries {
    /// URL dictionary
    pub urls: HashMap<u16, String>,
    /// String dictionary
    pub strings: HashMap<u16, String>,
    /// Property abbreviations
    pub properties: HashMap<String, String>,
}

/// Compression statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionStats {
    pub original_size: usize,
    pub compacted_size: usize,
    pub compression_ratio: f32,
    pub urls_deduplicated: u32,
    pub strings_deduplicated: u32,
    pub properties_abbreviated: u32,
    pub resources_processed: u32,
}

impl EfficientCompactor {
    /// Create a new efficient compactor
    pub fn new() -> Self {
        Self {
            url_dict: HashMap::new(),
            string_dict: HashMap::new(),
            property_abbrevs: Self::init_property_abbreviations(),
            next_url_id: 1,
            next_string_id: 1,
        }
    }

    /// Initialize property abbreviations for common Atomic Data properties
    fn init_property_abbreviations() -> HashMap<String, &'static str> {
        let mut abbrevs = HashMap::new();
        
        // Core atomic properties
        abbrevs.insert("https://atomicdata.dev/properties/isA".to_string(), "t");
        abbrevs.insert("https://atomicdata.dev/properties/parent".to_string(), "p");
        abbrevs.insert("https://atomicdata.dev/properties/lastCommit".to_string(), "lc");
        
        // Common terraphim properties  
        abbrevs.insert("https://common.terraphim.io/01jxw2jx8qze6yakh4fz24mnhy/property/company-name".to_string(), "cn");
        abbrevs.insert("https://common.terraphim.io/01jxw2jx8qze6yakh4fz24mnhy/property/company-description".to_string(), "cd");
        abbrevs.insert("https://common.terraphim.io/01jxw2jx8qze6yakh4fz24mnhy/property/business-website".to_string(), "bw");
        abbrevs.insert("https://common.terraphim.io/01jxw2jx8qze6yakh4fz24mnhy/property/year-of-incorporation".to_string(), "yi");
        abbrevs.insert("https://common.terraphim.io/01jxw2jx8qze6yakh4fz24mnhy/property/company-registration-number".to_string(), "rn");
        abbrevs.insert("https://common.terraphim.io/01jxw2jx8qze6yakh4fz24mnhy/property/country-of-registration".to_string(), "cr");
        
        abbrevs
    }

    /// Compact comprehensive subresource data efficiently
    pub fn compact_comprehensive_data(&mut self, data: &Value) -> Result<EfficientCompactedData> {
        let original_json = serde_json::to_string(data)?;
        let original_size = original_json.len();
        
        let mut stats = CompressionStats {
            original_size,
            compacted_size: 0,
            compression_ratio: 0.0,
            urls_deduplicated: 0,
            strings_deduplicated: 0,
            properties_abbreviated: 0,
            resources_processed: 0,
        };

        // Extract and analyze subresources
        let empty_vec = vec![];
        let subresources = data.get("subresources")
            .and_then(|v| v.as_array())
            .unwrap_or(&empty_vec);

        // Group resources by type and extract schema
        let mut resource_groups: HashMap<String, Vec<&Value>> = HashMap::new();
        let mut schema = CompactionSchema {
            resource_types: HashMap::new(),
            field_types: HashMap::new(),
        };

        for subresource in subresources {
            if let Some(resource_type) = subresource.get("resource_type").and_then(|v| v.as_str()) {
                let type_name = self.extract_type_name(resource_type);
                resource_groups.entry(type_name.clone()).or_default().push(subresource);
                stats.resources_processed += 1;
            }
        }

        // Convert to columnar format
        let mut columnar_data = ColumnarData {
            resources: HashMap::new(),
        };

        for (type_name, resources) in resource_groups {
            let typed_group = self.convert_to_columnar(&type_name, &resources, &mut stats)?;
            columnar_data.resources.insert(type_name.clone(), typed_group);
            
            // Update schema
            schema.resource_types.insert(type_name, self.infer_schema(&resources)?);
        }

        // Calculate final compression stats
        let compacted_data = EfficientCompactedData {
            schema,
            data: columnar_data,
            dictionaries: Dictionaries {
                urls: self.create_reverse_url_dict(),
                strings: self.create_reverse_string_dict(),
                properties: self.property_abbrevs.iter().map(|(k, &v)| (v.to_string(), k.clone())).collect(),
            },
            stats: stats.clone(),
        };

        let compacted_json = serde_json::to_string(&compacted_data)?;
        let compacted_size = compacted_json.len();
        
        // Update final stats
        let compression_ratio = if original_size > 0 {
            (original_size as f32 - compacted_size as f32) / original_size as f32
        } else {
            0.0
        };

        Ok(EfficientCompactedData {
            stats: CompressionStats {
                compacted_size,
                compression_ratio,
                ..stats
            },
            ..compacted_data
        })
    }

    /// Extract clean type name from resource URL
    fn extract_type_name(&self, resource_type: &str) -> String {
        resource_type
            .split('/')
            .last()
            .unwrap_or("unknown")
            .replace("-step", "")
            .replace('-', "_")
    }

    /// Convert resources to columnar format
    fn convert_to_columnar(
        &mut self,
        _type_name: &str,
        resources: &[&Value],
        stats: &mut CompressionStats,
    ) -> Result<TypedResourceGroup> {
        let count = resources.len();
        let mut group = TypedResourceGroup {
            count,
            str_cols: HashMap::new(),
            int_cols: HashMap::new(),
            float_cols: HashMap::new(),
            bool_cols: HashMap::new(),
            url_cols: HashMap::new(),
            json_cols: HashMap::new(),
        };

        // Analyze all fields across resources
        let mut all_fields: HashMap<String, FieldType> = HashMap::new();
        for resource in resources {
            if let Some(obj) = resource.as_object() {
                for (key, value) in obj {
                    let field_name = self.abbreviate_property(key, stats);
                    let field_type = self.infer_field_type(value);
                    all_fields.insert(field_name, field_type);
                }
            }
        }

        // Initialize columns
        for (field_name, field_type) in &all_fields {
            match field_type {
                FieldType::Str => { group.str_cols.insert(field_name.clone(), Vec::with_capacity(count)); },
                FieldType::Int => { group.int_cols.insert(field_name.clone(), Vec::with_capacity(count)); },
                FieldType::Float => { group.float_cols.insert(field_name.clone(), Vec::with_capacity(count)); },
                FieldType::Bool => { group.bool_cols.insert(field_name.clone(), Vec::with_capacity(count)); },
                FieldType::Url => { group.url_cols.insert(field_name.clone(), Vec::with_capacity(count)); },
                FieldType::Json => { group.json_cols.insert(field_name.clone(), Vec::with_capacity(count)); },
                _ => { group.json_cols.insert(field_name.clone(), Vec::with_capacity(count)); },
            }
        }

        // Fill columns with data
        for resource in resources {
            if let Some(obj) = resource.as_object() {
                for (field_name, field_type) in &all_fields {
                    let original_key = self.find_original_key(obj, field_name);
                    let value = original_key.and_then(|k| obj.get(k));

                    match field_type {
                        FieldType::Str => {
                            let str_val = value.and_then(|v| v.as_str()).map(|s| self.intern_string(s, stats));
                            group.str_cols.get_mut(field_name).unwrap().push(str_val);
                        },
                        FieldType::Int => {
                            let int_val = value.and_then(|v| v.as_i64());
                            group.int_cols.get_mut(field_name).unwrap().push(int_val);
                        },
                        FieldType::Float => {
                            let float_val = value.and_then(|v| v.as_f64());
                            group.float_cols.get_mut(field_name).unwrap().push(float_val);
                        },
                        FieldType::Bool => {
                            let bool_val = value.and_then(|v| v.as_bool()).unwrap_or(false);
                            group.bool_cols.get_mut(field_name).unwrap().push(bool_val);
                        },
                        FieldType::Url => {
                            let url_val = value.and_then(|v| v.as_str()).map(|s| self.intern_url(s, stats));
                            group.url_cols.get_mut(field_name).unwrap().push(url_val);
                        },
                        FieldType::Json => {
                            let json_val = value.map(|v| {
                                let json_str = serde_json::to_string(v).unwrap_or_default();
                                self.intern_string(&json_str, stats)
                            });
                            group.json_cols.get_mut(field_name).unwrap().push(json_val);
                        },
                        _ => {},
                    }
                }
            }
        }

        Ok(group)
    }

    /// Abbreviate property name using dictionary
    fn abbreviate_property(&self, property: &str, stats: &mut CompressionStats) -> String {
        if let Some(&abbrev) = self.property_abbrevs.get(property) {
            stats.properties_abbreviated += 1;
            abbrev.to_string()
        } else {
            property.to_string()
        }
    }

    /// Intern a URL in the dictionary
    fn intern_url(&mut self, url: &str, stats: &mut CompressionStats) -> u16 {
        if let Some(&id) = self.url_dict.get(url) {
            id
        } else {
            let id = self.next_url_id;
            self.url_dict.insert(url.to_string(), id);
            self.next_url_id += 1;
            stats.urls_deduplicated += 1;
            id
        }
    }

    /// Intern a string in the dictionary
    fn intern_string(&mut self, s: &str, stats: &mut CompressionStats) -> u16 {
        if let Some(&id) = self.string_dict.get(s) {
            id
        } else {
            let id = self.next_string_id;
            self.string_dict.insert(s.to_string(), id);
            self.next_string_id += 1;
            stats.strings_deduplicated += 1;
            id
        }
    }

    /// Infer field type from JSON value
    fn infer_field_type(&self, value: &Value) -> FieldType {
        match value {
            Value::String(s) => {
                if s.starts_with("https://") || s.starts_with("http://") {
                    FieldType::Url
                } else {
                    FieldType::Str
                }
            },
            Value::Number(n) => {
                if n.is_i64() {
                    FieldType::Int
                } else {
                    FieldType::Float
                }
            },
            Value::Bool(_) => FieldType::Bool,
            Value::Array(_) | Value::Object(_) => FieldType::Json,
            Value::Null => FieldType::Str, // Default to string for nulls
        }
    }

    /// Find original property key given abbreviated name
    fn find_original_key<'a>(&'a self, obj: &'a serde_json::Map<String, Value>, abbrev_name: &'a str) -> Option<&'a str> {
        // First try exact match
        if obj.contains_key(abbrev_name) {
            return Some(abbrev_name);
        }

        // Then try to find by abbreviation
        for (full_key, &abbrev) in &self.property_abbrevs {
            if abbrev == abbrev_name && obj.contains_key(full_key) {
                return Some(full_key.as_str());
            }
        }

        None
    }

    /// Infer schema from resources
    fn infer_schema(&self, resources: &[&Value]) -> Result<ResourceSchema> {
        let mut all_fields = HashMap::new();
        let mut field_counts = HashMap::new();

        for resource in resources {
            if let Some(obj) = resource.as_object() {
                for (key, value) in obj {
                    let field_type = self.infer_field_type(value);
                    all_fields.insert(key.clone(), field_type);
                    *field_counts.entry(key.clone()).or_insert(0) += 1;
                }
            }
        }

        let total_resources = resources.len();
        let mut required = Vec::new();
        let mut optional = Vec::new();

        for (field, count) in field_counts {
            if count == total_resources {
                required.push(field);
            } else {
                optional.push(field);
            }
        }

        Ok(ResourceSchema {
            required,
            optional,
            types: all_fields,
        })
    }

    /// Create reverse URL dictionary
    fn create_reverse_url_dict(&self) -> HashMap<u16, String> {
        self.url_dict.iter().map(|(url, &id)| (id, url.clone())).collect()
    }

    /// Create reverse string dictionary
    fn create_reverse_string_dict(&self) -> HashMap<u16, String> {
        self.string_dict.iter().map(|(s, &id)| (id, s.clone())).collect()
    }
}

impl Default for EfficientCompactor {
    fn default() -> Self {
        Self::new()
    }
}

impl EfficientCompactor {
    /// Reconstruct original data from compacted format
    pub fn reconstruct_data(compacted: &EfficientCompactedData) -> Result<Value> {
        let mut reconstructed_subresources = Vec::new();

        for (type_name, group) in &compacted.data.resources {
            // Reconstruct resources of this type
            for i in 0..group.count {
                let mut resource = serde_json::Map::new();

                // Add resource type
                let full_type_url = format!(
                    "https://common.terraphim.io/01jxw2jx8qze6yakh4fz24mnhy/class/{}-step",
                    type_name.replace('_', "-")
                );
                resource.insert("resource_type".to_string(), Value::String(full_type_url));

                // Reconstruct string fields
                for (field_name, values) in &group.str_cols {
                    if let Some(Some(string_id)) = values.get(i) {
                        if let Some(string_value) = compacted.dictionaries.strings.get(string_id) {
                            let original_key = compacted.dictionaries.properties
                                .get(field_name)
                                .map(|s| s.as_str())
                                .unwrap_or(field_name);
                            resource.insert(original_key.to_string(), Value::String(string_value.clone()));
                        }
                    }
                }

                // Reconstruct URL fields  
                for (field_name, values) in &group.url_cols {
                    if let Some(Some(url_id)) = values.get(i) {
                        if let Some(url_value) = compacted.dictionaries.urls.get(url_id) {
                            let original_key = compacted.dictionaries.properties
                                .get(field_name)
                                .map(|s| s.as_str())
                                .unwrap_or(field_name);
                            resource.insert(original_key.to_string(), Value::String(url_value.clone()));
                        }
                    }
                }

                // Reconstruct int fields
                for (field_name, values) in &group.int_cols {
                    if let Some(Some(int_value)) = values.get(i) {
                        let original_key = compacted.dictionaries.properties
                            .get(field_name)
                            .map(|s| s.as_str())
                            .unwrap_or(field_name);
                        resource.insert(original_key.to_string(), Value::Number((*int_value).into()));
                    }
                }

                // Reconstruct float fields
                for (field_name, values) in &group.float_cols {
                    if let Some(Some(float_value)) = values.get(i) {
                        let original_key = compacted.dictionaries.properties
                            .get(field_name)
                            .map(|s| s.as_str())
                            .unwrap_or(field_name);
                        if let Some(num) = serde_json::Number::from_f64(*float_value) {
                            resource.insert(original_key.to_string(), Value::Number(num));
                        }
                    }
                }

                // Reconstruct bool fields
                for (field_name, values) in &group.bool_cols {
                    if let Some(bool_value) = values.get(i) {
                        let original_key = compacted.dictionaries.properties
                            .get(field_name)
                            .map(|s| s.as_str())
                            .unwrap_or(field_name);
                        resource.insert(original_key.to_string(), Value::Bool(*bool_value));
                    }
                }

                // Reconstruct JSON fields
                for (field_name, values) in &group.json_cols {
                    if let Some(Some(json_id)) = values.get(i) {
                        if let Some(json_string) = compacted.dictionaries.strings.get(json_id) {
                            let original_key = compacted.dictionaries.properties
                                .get(field_name)
                                .map(|s| s.as_str())
                                .unwrap_or(field_name);
                            if let Ok(json_value) = serde_json::from_str::<Value>(json_string) {
                                resource.insert(original_key.to_string(), json_value);
                            } else {
                                resource.insert(original_key.to_string(), Value::String(json_string.clone()));
                            }
                        }
                    }
                }

                reconstructed_subresources.push(Value::Object(resource));
            }
        }

        Ok(serde_json::json!({
            "subresources": reconstructed_subresources
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_efficient_compaction() {
        let mut compactor = EfficientCompactor::new();

        // Create test data with repeated URLs and properties
        let test_data = json!({
            "subresources": [
                {
                    "url": "https://common.terraphim.io/01k2cxga1cqmqsgvqk0enxq8a5",
                    "resource_type": "https://common.terraphim.io/01jxw2jx8qze6yakh4fz24mnhy/class/company-information-and-history-step",
                    "json_format": "{\"@id\": \"https://common.terraphim.io/01k2cxga1cqmqsgvqk0enxq8a5\", \"https://common.terraphim.io/01jxw2jx8qze6yakh4fz24mnhy/property/company-name\": \"EcoBright Solutions Uganda Limited\"}",
                    "https://common.terraphim.io/01jxw2jx8qze6yakh4fz24mnhy/property/company-name": "EcoBright Solutions Uganda Limited",
                    "https://common.terraphim.io/01jxw2jx8qze6yakh4fz24mnhy/property/year-of-incorporation": 2020,
                    "https://atomicdata.dev/properties/isA": ["https://common.terraphim.io/01jxw2jx8qze6yakh4fz24mnhy/class/company-information-and-history-step"]
                },
                {
                    "url": "https://common.terraphim.io/01k2cxga6pywwjg8yc2m6b1jzz", 
                    "resource_type": "https://common.terraphim.io/01jxw2jx8qze6yakh4fz24mnhy/class/management-governance-and-ownership",
                    "json_format": "{\"@id\": \"https://common.terraphim.io/01k2cxga6pywwjg8yc2m6b1jzz\", \"https://common.terraphim.io/01jxw2jx8qze6yakh4fz24mnhy/property/company-name\": \"EcoBright Solutions Uganda Limited\"}",
                    "https://common.terraphim.io/01jxw2jx8qze6yakh4fz24mnhy/property/company-name": "EcoBright Solutions Uganda Limited",
                    "https://common.terraphim.io/01jxw2jx8qze6yakh4fz24mnhy/property/year-of-incorporation": 2020,
                    "https://atomicdata.dev/properties/isA": ["https://common.terraphim.io/01jxw2jx8qze6yakh4fz24mnhy/class/management-governance-and-ownership"]
                }
            ]
        });

        let result = compactor.compact_comprehensive_data(&test_data).unwrap();

        // Verify compression actually happened
        println!("Original size: {} bytes", result.stats.original_size);
        println!("Compacted size: {} bytes", result.stats.compacted_size);
        println!("Compression ratio: {:.2}%", result.stats.compression_ratio * 100.0);
        println!("URLs deduplicated: {}", result.stats.urls_deduplicated);
        println!("Strings deduplicated: {}", result.stats.strings_deduplicated);
        println!("Properties abbreviated: {}", result.stats.properties_abbreviated);

        // Should have positive compression ratio (original > compacted)
        assert!(result.stats.compression_ratio > 0.0, "Data should be smaller after compression");
        assert!(result.stats.compacted_size < result.stats.original_size, "Compacted size should be less than original");
        
        // Should have deduplication
        assert!(result.stats.urls_deduplicated > 0, "Should deduplicate URLs");
        assert!(result.stats.strings_deduplicated > 0, "Should deduplicate strings");
        assert!(result.stats.properties_abbreviated > 0, "Should abbreviate properties");

        // Should have proper schema
        assert!(!result.schema.resource_types.is_empty(), "Should have resource type schemas");
        assert!(!result.dictionaries.urls.is_empty(), "Should have URL dictionary");
        assert!(!result.dictionaries.strings.is_empty(), "Should have string dictionary");
    }

    #[test]
    fn test_data_reconstruction() {
        let mut compactor = EfficientCompactor::new();

        // Create simple test data
        let original_data = json!({
            "subresources": [
                {
                    "url": "https://example.com/1",
                    "resource_type": "https://common.terraphim.io/01jxw2jx8qze6yakh4fz24mnhy/class/company-information-and-history-step",
                    "https://common.terraphim.io/01jxw2jx8qze6yakh4fz24mnhy/property/company-name": "Test Company",
                    "https://common.terraphim.io/01jxw2jx8qze6yakh4fz24mnhy/property/year-of-incorporation": 2020
                }
            ]
        });

        // Compact the data
        let compacted = compactor.compact_comprehensive_data(&original_data).unwrap();
        
        // Reconstruct the data
        let reconstructed = EfficientCompactor::reconstruct_data(&compacted).unwrap();

        // Verify essential data is preserved
        let original_resources = original_data["subresources"].as_array().unwrap();
        let reconstructed_resources = reconstructed["subresources"].as_array().unwrap();

        assert_eq!(original_resources.len(), reconstructed_resources.len());
        
        // Check that key fields were preserved
        let _orig_resource = &original_resources[0];
        let recon_resource = &reconstructed_resources[0];
        
        // Company name should be preserved
        assert!(recon_resource.get("https://common.terraphim.io/01jxw2jx8qze6yakh4fz24mnhy/property/company-name").is_some() ||
                recon_resource.get("cn").is_some(), "Company name should be preserved");
        
        // Year should be preserved  
        assert!(recon_resource.get("https://common.terraphim.io/01jxw2jx8qze6yakh4fz24mnhy/property/year-of-incorporation").is_some() ||
                recon_resource.get("yi").is_some(), "Year should be preserved");

        println!("✅ Data reconstruction test passed");
        println!("Original: {}", serde_json::to_string_pretty(&original_data).unwrap());
        println!("Reconstructed: {}", serde_json::to_string_pretty(&reconstructed).unwrap());
    }

    #[test]
    fn test_property_abbreviation() {
        let compactor = EfficientCompactor::new();
        let mut stats = CompressionStats {
            original_size: 0,
            compacted_size: 0,
            compression_ratio: 0.0,
            urls_deduplicated: 0,
            strings_deduplicated: 0,
            properties_abbreviated: 0,
            resources_processed: 0,
        };

        // Test abbreviation
        let abbreviated = compactor.abbreviate_property(
            "https://atomicdata.dev/properties/isA",
            &mut stats
        );
        
        assert_eq!(abbreviated, "t");
        assert_eq!(stats.properties_abbreviated, 1);
    }

    #[test]  
    fn test_field_type_inference() {
        let compactor = EfficientCompactor::new();
        
        assert!(matches!(compactor.infer_field_type(&json!("https://example.com")), FieldType::Url));
        assert!(matches!(compactor.infer_field_type(&json!("regular string")), FieldType::Str));
        assert!(matches!(compactor.infer_field_type(&json!(42)), FieldType::Int));
        assert!(matches!(compactor.infer_field_type(&json!(3.14)), FieldType::Float));
        assert!(matches!(compactor.infer_field_type(&json!(true)), FieldType::Bool));
        assert!(matches!(compactor.infer_field_type(&json!({"key": "value"})), FieldType::Json));
        assert!(matches!(compactor.infer_field_type(&json!([1, 2, 3])), FieldType::Json));
    }
}