# Data Compactor

A high-performance Rust library for compressing JSON data structures, specifically optimized for Atomic Data format.

## Features

- **URL Compression**: Maps long URLs to numeric IDs
- **Property Abbreviation**: Shortens common Atomic Data property names
- **String Deduplication**: Compresses repeated strings with categorical encoding
- **Boolean Optimization**: Converts booleans to compact "T"/"F" strings
- **Structural Compaction**: Groups related data by type
- **Lossless Decompression**: Full recovery of original data structure

## Usage

```rust
use data_compactor::DataCompactor;
use serde_json::json;

let mut compactor = DataCompactor::new();

let data = json!({
    "https://atomicdata.dev/properties/isA": "https://example.com/class",
    "https://common.terraphim.io/property/company-name": "Example Corp",
    "active": true,
    "verified": false
});

// Compress the data
let compacted = compactor.compact_comprehensive_data(&data)?;
println!("Compression ratio: {:.2}%", compacted.stats.compression_ratio * 100.0);

// Decompress back to original
let decompressed = compactor.decompress(&compacted)?;
assert_eq!(data, decompressed);
```

## Performance

Achieves compression ratios of **90%+** on typical Atomic Data structures while maintaining:
- Fast compression/decompression
- Memory efficiency
- Type safety

## Integration

Originally developed for the [loan_application_pipeline](../loan_application_pipeline), this library can be used standalone or integrated into other Rust projects requiring JSON data compression.

## License

MIT OR Apache-2.0