#[cfg(test)]
mod compactor_comparison_tests {
    use crate::{DataCompactor, EfficientCompactor, TrulyEfficientCompactor};
    use serde_json::json;

    /// Test all three compactors with the same realistic data
    #[test]
    fn test_all_compactors_comparison() {
        // Create realistic test data with lots of redundancy (similar to atomic server data)
        let test_data = json!({
            "subresources": [
                {
                    "url": "https://common.terraphim.io/01k2cxga1cqmqsgvqk0enxq8a5",
                    "resource_type": "https://common.terraphim.io/01jxw2jx8qze6yakh4fz24mnhy/class/company-information-and-history-step",
                    "json_format": "{\"@id\": \"https://common.terraphim.io/01k2cxga1cqmqsgvqk0enxq8a5\", \"https://common.terraphim.io/01jxw2jx8qze6yakh4fz24mnhy/property/company-name\": \"EcoBright Solutions Uganda Limited\", \"https://common.terraphim.io/01jxw2jx8qze6yakh4fz24mnhy/property/year-of-incorporation\": 2020}",
                    "json_ad_format": "{\"@id\": \"https://common.terraphim.io/01k2cxga1cqmqsgvqk0enxq8a5\", \"https://common.terraphim.io/01jxw2jx8qze6yakh4fz24mnhy/property/company-name\": \"EcoBright Solutions Uganda Limited\", \"https://atomicdata.dev/properties/isA\": [\"https://common.terraphim.io/01jxw2jx8qze6yakh4fz24mnhy/class/company-information-and-history-step\"]}",
                    "https://common.terraphim.io/01jxw2jx8qze6yakh4fz24mnhy/property/company-name": "EcoBright Solutions Uganda Limited",
                    "https://common.terraphim.io/01jxw2jx8qze6yakh4fz24mnhy/property/company-description": "EcoBright Solutions Uganda Limited is an innovative clean energy company based in Kampala, Uganda. Founded in 2020, the company specializes in providing sustainable solar energy solutions for both residential and commercial customers across East Africa.",
                    "https://common.terraphim.io/01jxw2jx8qze6yakh4fz24mnhy/property/year-of-incorporation": 2020,
                    "https://common.terraphim.io/01jxw2jx8qze6yakh4fz24mnhy/property/country-of-registration": "Uganda",
                    "https://common.terraphim.io/01jxw2jx8qze6yakh4fz24mnhy/property/business-website": "https://ecobright-uganda.com",
                    "https://atomicdata.dev/properties/isA": ["https://common.terraphim.io/01jxw2jx8qze6yakh4fz24mnhy/class/company-information-and-history-step"],
                    "https://atomicdata.dev/properties/parent": "https://common.terraphim.io/01k2cxg9ndzyyfd357hz7npa38",
                    "https://atomicdata.dev/properties/lastCommit": "https://common.terraphim.io/01k2cxga1cqmqsgvqk0enxq8a6"
                },
                {
                    "url": "https://common.terraphim.io/01k2cxga6pywwjg8yc2m6b1jzz",
                    "resource_type": "https://common.terraphim.io/01jxw2jx8qze6yakh4fz24mnhy/class/management-governance-and-ownership-step",
                    "json_format": "{\"@id\": \"https://common.terraphim.io/01k2cxga6pywwjg8yc2m6b1jzz\", \"https://common.terraphim.io/01jxw2jx8qze6yakh4fz24mnhy/property/company-name\": \"EcoBright Solutions Uganda Limited\", \"https://common.terraphim.io/01jxw2jx8qze6yakh4fz24mnhy/property/board-of-directors\": \"Sarah Nakamura (Chairperson), James Okello (CEO), Dr. Maria Gonzalez (CTO)\"}",
                    "json_ad_format": "{\"@id\": \"https://common.terraphim.io/01k2cxga6pywwjg8yc2m6b1jzz\", \"https://common.terraphim.io/01jxw2jx8qze6yakh4fz24mnhy/property/company-name\": \"EcoBright Solutions Uganda Limited\", \"https://atomicdata.dev/properties/isA\": [\"https://common.terraphim.io/01jxw2jx8qze6yakh4fz24mnhy/class/management-governance-and-ownership-step\"]}",
                    "https://common.terraphim.io/01jxw2jx8qze6yakh4fz24mnhy/property/company-name": "EcoBright Solutions Uganda Limited",
                    "https://common.terraphim.io/01jxw2jx8qze6yakh4fz24mnhy/property/board-of-directors": "Sarah Nakamura (Chairperson), James Okello (CEO), Dr. Maria Gonzalez (CTO)",
                    "https://common.terraphim.io/01jxw2jx8qze6yakh4fz24mnhy/property/key-management-personnel": "James Okello - Chief Executive Officer with 15 years experience in renewable energy; Dr. Maria Gonzalez - Chief Technology Officer, PhD in Solar Engineering; Samuel Kiprotich - Chief Financial Officer, CPA with 10 years experience",
                    "https://common.terraphim.io/01jxw2jx8qze6yakh4fz24mnhy/property/business-owners": "Sarah Nakamura (45%), James Okello (35%), EcoVest Capital (20%)",
                    "https://atomicdata.dev/properties/isA": ["https://common.terraphim.io/01jxw2jx8qze6yakh4fz24mnhy/class/management-governance-and-ownership-step"],
                    "https://atomicdata.dev/properties/parent": "https://common.terraphim.io/01k2cxg9ndzyyfd357hz7npa38",
                    "https://atomicdata.dev/properties/lastCommit": "https://common.terraphim.io/01k2cxga6pywwjg8yc2m6b1j00"
                }
            ]
        });

        let original_json = serde_json::to_string(&test_data).unwrap();
        let original_size = original_json.len();

        println!("=== COMPACTOR COMPARISON TEST ===");
        println!("Original data size: {} bytes", original_size);
        println!();

        // Test 1: Original DataCompactor
        let mut compactor1 = DataCompactor::new();
        let result1 = compactor1.compact_comprehensive_data(&test_data).unwrap();
        let compacted1_json = serde_json::to_string(&result1).unwrap();
        
        println!("1. ORIGINAL DATA COMPACTOR:");
        println!("   Compacted size: {} bytes", compacted1_json.len());
        println!("   Compression ratio: {:.2}%", result1.stats.compression_ratio * 100.0);
        println!("   Size change: {} bytes ({:+.1}%)", 
                compacted1_json.len() as i32 - original_size as i32,
                ((compacted1_json.len() as f32 - original_size as f32) / original_size as f32) * 100.0);

        // Test 2: EfficientCompactor
        let mut compactor2 = EfficientCompactor::new();
        let result2 = compactor2.compact_comprehensive_data(&test_data).unwrap();
        let compacted2_json = serde_json::to_string(&result2).unwrap();
        
        println!("2. EFFICIENT COMPACTOR (Columnar):");
        println!("   Compacted size: {} bytes", compacted2_json.len());
        println!("   Compression ratio: {:.2}%", result2.stats.compression_ratio * 100.0);
        println!("   Size change: {} bytes ({:+.1}%)", 
                compacted2_json.len() as i32 - original_size as i32,
                ((compacted2_json.len() as f32 - original_size as f32) / original_size as f32) * 100.0);

        // Test 3: TrulyEfficientCompactor
        let mut compactor3 = TrulyEfficientCompactor::new();
        let result3 = compactor3.compact(&test_data).unwrap();
        let compacted3_json = serde_json::to_string(&result3).unwrap();
        
        println!("3. TRULY EFFICIENT COMPACTOR (Aggressive):");
        println!("   Compacted size: {} bytes", result3.stats.comp);
        println!("   Compression ratio: {:.2}%", result3.stats.ratio * 100.0);
        println!("   Size change: {} bytes ({:+.1}%)", 
                result3.stats.comp as i32 - original_size as i32,
                ((result3.stats.comp as f32 - original_size as f32) / original_size as f32) * 100.0);

        println!();
        println!("=== WINNER ANALYSIS ===");
        
        let sizes = vec![
            ("Original", compacted1_json.len()),
            ("Efficient", compacted2_json.len()),
            ("Truly Efficient", result3.stats.comp),
        ];
        
        let best = sizes.iter().min_by_key(|(_, size)| *size).unwrap();
        
        println!("Best compactor: {} ({} bytes)", best.0, best.1);
        
        // Verify the best one actually compresses
        if best.1 < original_size {
            println!("✅ Best compactor achieves real compression!");
            let compression_pct = ((original_size - best.1) as f32 / original_size as f32) * 100.0;
            println!("   Compression achieved: {:.1}%", compression_pct);
        } else {
            println!("❌ Even the best compactor makes data LARGER!");
        }

        // Test data reconstruction for the best compactor
        println!();
        println!("=== RECONSTRUCTION TEST FOR BEST COMPACTOR ===");
        match best.0 {
            "Truly Efficient" => {
                let reconstructed = TrulyEfficientCompactor::reconstruct(&result3).unwrap();
                let orig_len = test_data["subresources"].as_array().unwrap().len();
                let recon_len = reconstructed["subresources"].as_array().unwrap().len();
                println!("Reconstruction: {} -> {} resources", orig_len, recon_len);
                assert_eq!(orig_len, recon_len, "Reconstruction should preserve resource count");
                println!("✅ Reconstruction successful!");
            },
            "Efficient" => {
                let reconstructed = EfficientCompactor::reconstruct_data(&result2).unwrap();
                let orig_len = test_data["subresources"].as_array().unwrap().len();
                let recon_len = reconstructed["subresources"].as_array().unwrap().len();
                println!("Reconstruction: {} -> {} resources", orig_len, recon_len);
                assert_eq!(orig_len, recon_len, "Reconstruction should preserve resource count");
                println!("✅ Reconstruction successful!");
            },
            _ => {
                println!("ℹ️  Original compactor doesn't have reconstruction test");
            }
        }
    }
}