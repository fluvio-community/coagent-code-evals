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
    fn test_property_abbreviation() {
        let compactor = EfficientCompactor::new();
        let mut stats = super::CompressionStats {
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
        
        assert!(matches!(compactor.infer_field_type(&json!("https://example.com")), super::FieldType::Url));
        assert!(matches!(compactor.infer_field_type(&json!("regular string")), super::FieldType::Str));
        assert!(matches!(compactor.infer_field_type(&json!(42)), super::FieldType::Int));
        assert!(matches!(compactor.infer_field_type(&json!(3.14)), super::FieldType::Float));
        assert!(matches!(compactor.infer_field_type(&json!(true)), super::FieldType::Bool));
        assert!(matches!(compactor.infer_field_type(&json!({"key": "value"})), super::FieldType::Json));
        assert!(matches!(compactor.infer_field_type(&json!([1, 2, 3])), super::FieldType::Json));
    }
}