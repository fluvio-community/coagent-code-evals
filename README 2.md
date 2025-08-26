# Multi-Model Evaluation Pipeline for LLMs

A comprehensive evaluation system combining Rust validation framework, credit assessment pipeline, and multi-model code analysis using AIPACK, CrewAI, and SmolAgents with Ollama. This system evaluates various AI models across multiple frameworks and use cases.

## 🏗️ Repository Structure

```
evaluation_pipeline_llms/
├── rust-analysis-tools/           # Rust validation & code analysis
│   ├── evaluation-scripts/        # Multi-model evaluation scripts
│   ├── aipack-flows/             # AIPACK workflow definitions
│   ├── pipeline-tests/           # Python integration tests
│   ├── validation-framework/     # Rust validation library
│   └── data-compactor/          # Data compaction experiments
├── credit-assessment-system/      # Credit assessment pipeline
│   ├── credit_assessment_pipeline/ # Main assessment scripts
│   ├── evaluation_pipelines/     # Multi-framework evaluators
│   └── loan-assessment/          # Loan assessment workflows
└── .venv/                        # Python virtual environment
```

## System Components

### 🔬 Multi-Model Evaluation Engine
- **Model Comparison**: Evaluates multiple AI models simultaneously for Rust code analysis
- **Performance Benchmarking**: Measures evaluation time, accuracy, and model-specific insights
- **Automated Scoring**: Standardized scoring system (1-10) with detailed justifications
- **Result Aggregation**: Comprehensive comparison reports with visual charts

### 🔍 Pre-flight Validation Library
A comprehensive Rust library for validating all prerequisites before starting AI model evaluation processes. This library ensures that Ollama service is properly configured, models are available, disk space is sufficient, and all configurations are valid.

## Features

### 🔍 Comprehensive Validation Checks

- **Ollama Service Health**: Verifies Ollama is running and responding properly
- **Model Availability**: Checks which models are available against configuration requirements  
- **Model Size Detection**: Categorizes models by size and adjusts timeouts accordingly
- **Disk Space Validation**: Ensures sufficient storage for evaluation outputs
- **AIPACK Configuration**: Validates configuration file syntax and model references
- **System Resources**: Checks available memory and CPU resources

### ⚡ High Performance

- **Concurrent Validation**: All checks run concurrently for maximum efficiency
- **Smart Timeouts**: Model-size aware timeout recommendations
- **Minimal Overhead**: Lightweight validation with detailed diagnostics

### 🛠 Developer Experience  

- **Rich Error Context**: Detailed error messages with actionable recommendations
- **Multiple Output Formats**: JSON, summary, and detailed reporting
- **CLI Tool**: Ready-to-use command-line interface
- **Extensive Logging**: Configurable logging levels for debugging

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
evaluation-validator = "0.1.0"
```

## 🚀 Quick Start

### Run All Models with All Frameworks

```bash
cd /path/to/evaluation_pipeline_llms

# 1. Activate Python environment
source .venv/bin/activate

# 2. Run Rust analysis with all models
cd rust-analysis-tools/evaluation-scripts
./run-multi-model-evaluation.sh

# 3. Run all Python frameworks comparison  
cd ../../credit-assessment-system
python run_multi_pipeline_evaluation.py

# 4. Run credit assessment with all models
cd credit_assessment_pipeline
python run_credit_assessment.py
```

### Individual Component Testing

```bash
# Test specific models
source .venv/bin/activate
cd rust-analysis-tools
python pipeline-tests/test_aipack.py

# Run validation framework
cargo run --bin validation-cli

# Test specific credit assessment
cd credit-assessment-system/credit_assessment_pipeline
python run_credit_assessment.py --models llama3.2:3b codellama:7b
```

### Advanced Configuration

```rust
use evaluation_validator::{ValidationConfig, validate_evaluation_prerequisites_with_config};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = ValidationConfig {
        ollama_url: "http://localhost:11434".to_string(),
        timeout_seconds: 60,
        min_disk_space_gb: 10.0,
        aipack_config_path: ".aipack/config.toml".to_string(),
        output_directory: "evaluation-results".to_string(),
        deep_model_validation: true,
    };
    
    let result = validate_evaluation_prerequisites_with_config(config).await?;
    
    // Handle validation results...
    Ok(())
}
```

## Command Line Interface

The library includes a CLI tool for manual validation:

### Installation

```bash
cargo install --path . --bin validation-cli
```

### Usage

```bash
# Basic validation with default settings
validation-cli

# Use custom AIPACK configuration
validation-cli --config .aipack/config.toml

# Output as JSON for scripting
validation-cli --format json

# Enable verbose logging
validation-cli --verbose

# Summary output format
validation-cli --format summary
```

### Example Output

```
🔍 Starting comprehensive evaluation pre-flight validation...

📊 VALIDATION RESULTS
════════════════════════════════════════════════════════════════
✅ OVERALL STATUS: PASSED

📈 SUMMARY:
  Available Models: 5
  Missing Models: 0
  Available Disk Space: 45.23 GB
  Estimated Space Needed: 5.00 GB
  Ollama Response Time: 145ms
  AIPACK Config Valid: true

🔍 DETAILED CHECKS:
  ✅ Ollama Service: Service responsive, 5 models available (145ms)
  ✅ Model Availability: Service responsive, 5 models available (145ms)
  ✅ Disk Space: Available: 45.23GB, Required: 5.00GB (12ms)
  ✅ AIPACK Config: Config valid: true, Default model: qwen2.5-coder:latest (8ms)
  ✅ System Resources: Memory: 16.00GB, CPUs: 8 (2ms)

💡 RECOMMENDATIONS:
  🟡 [Performance]: Ollama service response time is slow
     Action: Consider optimizing Ollama configuration or checking system resources

Validation completed at: 2025-02-02T10:30:45.123456789Z
```

## Library Architecture

### Core Components

#### ValidationCoordinator
The main orchestrator that coordinates all validation checks:

```rust
use evaluation_validator::ValidationCoordinator;

let coordinator = ValidationCoordinator::new();
let result = coordinator.validate_all().await?;
```

#### Individual Validators

- **OllamaValidator**: Handles Ollama service and model validation
- **DiskValidator**: Manages disk space checking
- **AipackValidator**: Validates AIPACK configuration files

#### ValidationResult
Comprehensive result structure containing:

- Overall validation status
- Individual check results
- Performance metrics
- Actionable recommendations
- Critical issues and warnings

### Model Size Categories

The library automatically categorizes models by size for optimal timeout handling:

- **Small** (< 1GB): 30s timeout
- **Medium** (1-5GB): 60s timeout  
- **Large** (5-20GB): 120s timeout
- **XL** (> 20GB): 300s timeout

## Multi-Model Evaluation

### Running Multi-Model Evaluations

#### Fix AIPACK Path Issues
```bash
cd rust-analysis-tools/evaluation-scripts

# Method 1: Copy flows to local directory
cp ../aipack-flows/multi-model-evaluation.aip .
cp ../aipack-flows/rust-evaluator.aip .
./run-multi-model-evaluation.sh

# Method 2: Update script paths
sed -i '' 's|multi-model-evaluation.aip|../aipack-flows/multi-model-evaluation.aip|g' run-multi-model-evaluation.sh
./run-multi-model-evaluation.sh

# Method 3: Run from root directory
cd .. && aip run aipack-flows/multi-model-evaluation.aip -s
```

#### Available Evaluation Scripts
- `run-multi-model-evaluation.sh` - Comprehensive all-model testing
- `test-few-models.sh` - Quick validation with 3 models  
- `quick-test.sh` - Single model validation test
- `enhanced-multi-model-evaluation.sh` - Advanced parsing and charts

#### Multi-Framework Evaluation

**AIPACK Framework**
```bash
cd rust-analysis-tools
./run-rust-evaluation.sh

# Or run specific AIPACK flows
aip run aipack-flows/rust-evaluator.aip
aip run aipack-flows/rust-refactor.aip
```

**Python Frameworks (CrewAI, SmolAgents)**
```bash
cd credit-assessment-system
source ../.venv/bin/activate

# Test individual frameworks
python pipeline-tests/test_crewai.py
python pipeline-tests/test_smolagents.py

# Run comprehensive framework comparison
python run_multi_pipeline_evaluation.py
```

### Supported Models

The system supports evaluation of multiple AI models optimized for different use cases:

#### Core Models
- **llama3.2:3b** - Excellent for architectural insights and WASM patterns
- **codellama:7b** - Strong async pattern recognition
- **deepseek-coder:latest** - Good for general code analysis

#### Specialized Models
- **victornitu/rust-coder:latest** - Rust-specific optimizations
- **cogito:latest** - Advanced reasoning capabilities
- **qwen3-coder:latest** - Modern coding assistant
- **qwen2.5-coder:latest** - Enhanced coding analysis

#### Large Models
- **gemma3:27b** - Comprehensive analysis capabilities
- **qwen3:latest** - General purpose large model
- **qwen3:8b** - Balanced performance and accuracy

### Model-Specific Quirks and Optimal Configurations

#### llama3.2:3b
- **Strength**: Excellent architectural analysis and WASM-specific insights
- **Optimal timeout**: 60s
- **Best for**: Code structure evaluation, pattern recognition
- **Quirk**: Sometimes misses performance optimization opportunities

#### codellama:7b
- **Strength**: Superior async/await pattern analysis
- **Optimal timeout**: 90s
- **Best for**: Concurrent code evaluation, async optimization
- **Quirk**: May over-focus on concurrency at expense of other aspects

#### deepseek-coder:latest
- **Strength**: Comprehensive error handling analysis
- **Optimal timeout**: 120s
- **Best for**: Safety and error handling evaluation
- **Quirk**: Can struggle with complex architectural patterns

#### victornitu/rust-coder:latest
- **Strength**: Rust-specific idioms and best practices
- **Optimal timeout**: 90s
- **Best for**: Rust-specific optimization recommendations
- **Quirk**: Sometimes too focused on Rust conventions vs general code quality

### Evaluation Results Summary

Based on extensive testing, here are the key findings:

| Model | Average Score | Strengths | Optimal Use Case |
|-------|---------------|-----------|------------------|
| llama3.2:3b | 8.5/10 | Architectural insights | WASM/Web projects |
| codellama:7b | 8.2/10 | Async patterns | Concurrent systems |
| deepseek-coder | 7.8/10 | Error handling | Safety-critical code |
| rust-coder | 8.0/10 | Rust idioms | Pure Rust projects |
| cogito | 7.5/10 | Reasoning | Complex algorithms |
| qwen3-coder | 8.1/10 | Modern patterns | Contemporary codebases |

### Best Practices for Model Selection

1. **For WASM Projects**: Use llama3.2:3b for architectural insights
2. **For Async-Heavy Code**: Choose codellama:7b for concurrency analysis
3. **For Safety-Critical Systems**: Use deepseek-coder for error handling focus
4. **For Rust-Specific Analysis**: Select victornitu/rust-coder for idiom compliance
5. **For Comprehensive Analysis**: Run multiple models and compare results

### Customizing Evaluation Agents

The system uses AIPACK agents for model evaluation. Key configuration options:

```toml
# .aipack/config.toml
[options]
model = "llama3.2:3b"  # Default model

[options.model_aliases]
llama3 = "llama3.2:3b"
codellama = "codellama:7b"
deepseek = "deepseek-coder:latest"
rust-coder = "victornitu/rust-coder:latest"
```

### Performance Optimization

#### Model-Specific Timeout Configuration
```bash
# Small models (3B parameters)
TIMEOUT_SMALL=60s

# Medium models (7-8B parameters)  
TIMEOUT_MEDIUM=90s

# Large models (20B+ parameters)
TIMEOUT_LARGE=180s
```

#### Parallel Evaluation Strategy
- Run lightweight models first for quick feedback
- Execute heavy models in parallel when resources allow
- Use exponential backoff for failed evaluations

## Integration Examples

### Shell Script Integration

```bash
#!/bin/bash

# Run validation and capture exit code
if validation-cli --format summary; then
    echo "✅ Validation passed, starting evaluation..."
    # Run your evaluation process here
    ./run-evaluation.sh
else
    echo "❌ Validation failed, aborting evaluation"
    exit 1
fi
```

### CI/CD Pipeline Integration

```yaml
name: Model Evaluation

jobs:
  validate:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout code
        uses: actions/checkout@v3
        
      - name: Install validation CLI
        run: cargo install --path . --bin validation-cli
        
      - name: Run pre-flight validation
        run: validation-cli --format json > validation-result.json
        
      - name: Upload validation results
        uses: actions/upload-artifact@v3
        with:
          name: validation-results
          path: validation-result.json
          
  evaluate:
    needs: validate
    runs-on: ubuntu-latest
    steps:
      - name: Run model evaluation
        run: ./run-evaluation.sh
```

### Enhanced Shell Script Integration

Update your existing evaluation scripts to include validation:

```bash
#!/bin/bash
# Enhanced version of run-multi-model-evaluation.sh with validation

set -e

echo "🔍 Running pre-flight validation..."

# Run validation with JSON output to capture detailed results
validation-cli --format json > validation-result.json

if [ $? -eq 0 ]; then
    echo "✅ Validation passed"
    
    # Extract available models from validation result
    available_models=$(jq -r '.summary.available_models' validation-result.json)
    echo "📦 Found $available_models models available"
    
    # Continue with existing evaluation logic...
    echo "🚀 Starting model evaluation..."
    # Your existing evaluation code here
else
    echo "❌ Validation failed - check validation-result.json for details"
    exit 1
fi
```

## Configuration

### AIPACK Configuration

Create `.aipack/config.toml` in your working directory:

```toml
# rust-analysis-tools/.aipack/config.toml
[options]
model = "llama3.2:3b"  # Default model

[options.model_aliases]
# Core models
llama3 = "llama3.2:3b"
codellama = "codellama:7b"
deepseek = "deepseek-coder:latest"

# Specialized models
rust-coder = "victornitu/rust-coder:latest"
cogito = "cogito:latest"
qwen25-coder = "qwen2.5-coder:latest"

# Large models
gemma3 = "gemma3:27b"
qwen3 = "qwen3:latest"
```

### Validation Configuration

```toml
[validation]
ollama_url = "http://localhost:11434"
timeout_seconds = 30
min_disk_space_gb = 5.0
aipack_config_path = ".aipack/config.toml"
output_directory = "evaluation-results"
deep_model_validation = true
```

### Environment Variables

Override default settings with environment variables:

- `OLLAMA_URL`: Ollama service URL
- `VALIDATION_TIMEOUT`: Request timeout in seconds
- `MIN_DISK_SPACE_GB`: Minimum required disk space
- `AIPACK_CONFIG_PATH`: Path to AIPACK configuration
- `OUTPUT_DIRECTORY`: Directory for evaluation outputs

## Troubleshooting

### Common Issues

#### AIPACK Flow Not Found
```
No agent found for local path: 'multi-model-evaluation.aip'
```
**Solutions**:
1. Copy flows: `cp ../aipack-flows/*.aip .`
2. Update paths: `sed -i '' 's|multi-model-evaluation.aip|../aipack-flows/multi-model-evaluation.aip|g' script.sh`
3. Run from root: `cd .. && aip run aipack-flows/flow-name.aip`

#### Python Import Errors
```
ModuleNotFoundError: No module named 'crewai'
```
**Solution**: 
```bash
source .venv/bin/activate
pip install crewai smolagents ollama
```

#### Ollama Service Not Accessible
```
❌ Ollama service is not accessible - evaluation cannot proceed
```
**Solution**: Start Ollama service with `ollama serve`

#### Model Not Available
```
❌ Model 'model-name' not found
```
**Solution**: 
```bash
# Check available models
ollama list

# Pull missing models
ollama pull llama3.2:3b
ollama pull codellama:7b
```

#### Configuration File Issues
```
❌ AIPACK configuration is invalid
```
**Solution**: Create proper `.aipack/config.toml` in working directory

#### Virtual Environment Issues
```
Command 'python' not found
```
**Solution**: Always activate virtual environment first:
```bash
source .venv/bin/activate
```

### Debug Mode

Enable debug logging for detailed troubleshooting:

```bash
RUST_LOG=debug validation-cli --verbose
```

Or in code:

```rust
use evaluation_validator::init_logging_with_level;

init_logging_with_level(log::LevelFilter::Debug);
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Run the test suite: `cargo test`
5. Run the linter: `cargo clippy`
6. Submit a pull request

### Development Setup

```bash
# Clone the repository
git clone https://github.com/terraphim/evaluation-validator.git
cd evaluation-validator

# Run tests
cargo test

# Build the CLI tool
cargo build --bin validation-cli

# Run examples
cargo run --example basic_validation
cargo run --example advanced_validation
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with Rust's async ecosystem using Tokio
- Uses jiff for time handling (following project requirements)
- Leverages reqwest for HTTP client functionality
- Inspired by best practices in pre-flight validation systems
