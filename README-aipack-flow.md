# AIPACK Rust Code Evaluation Flow

A comprehensive AI-powered workflow for evaluating and refactoring Rust codebases using local Ollama models.

## 🏗️ Repository Structure

```
rust-analysis-tools/
├── aipack-flows/                 # AIPACK workflow definitions
│   ├── rust-evaluator.aip       # Code quality analysis
│   ├── rust-refactor.aip        # Refactoring suggestions
│   ├── rust-flow-orchestrator.aip # Workflow coordination
│   ├── multi-model-evaluation.aip # Multi-model comparison
│   └── rust-evaluation-enhanced.aip # Enhanced evaluation
├── evaluation-scripts/           # Multi-model evaluation scripts
│   ├── run-multi-model-evaluation.sh
│   ├── test-few-models.sh
│   └── .aipack/config.toml       # Local AIPACK configuration
├── pipeline-tests/              # Python integration tests
└── validation-framework/        # Rust validation library
```

## Overview

This flow uses AIPACK with Ollama to analyze Rust code through multiple specialized agents:

- **rust-evaluator.aip**: Analyzes code quality, performance, and best practices
- **rust-refactor.aip**: Generates specific refactoring suggestions with examples
- **rust-flow-orchestrator.aip**: Coordinates the entire workflow and manages memory files
- **multi-model-evaluation.aip**: Compares results across multiple AI models
- **rust-evaluation-enhanced.aip**: Advanced evaluation with detailed analysis

## Prerequisites

1. **Install AIPACK**
   ```bash
   # Mac ARM
   curl -O https://repo.aipack.ai/aip-dist/stable/latest/aarch64-apple-darwin/aip.tar.gz
   tar -xvf aip.tar.gz
   ./aip self setup
   ```

2. **Install Ollama**
   ```bash
   # Visit https://ollama.ai or use homebrew
   brew install ollama
   ```

3. **Start Ollama and pull models**
   ```bash
   ollama serve
   ollama pull llama3.2:3b
   ollama pull codellama:7b
   ```

## Usage

### Quick Start
```bash
cd rust-analysis-tools

# Single project evaluation
./run-rust-evaluation.sh

# Evaluate a specific repository
./run-rust-evaluation.sh ./path/to/rust/project my-project-name

# Multi-model evaluation
cd evaluation-scripts
./run-multi-model-evaluation.sh

# Quick validation test
./test-few-models.sh
```

### Manual Execution

#### Fix Path Issues First
```bash
cd rust-analysis-tools/evaluation-scripts

# Method 1: Copy flows to local directory
cp ../aipack-flows/*.aip .

# Method 2: Update script paths
sed -i '' 's|multi-model-evaluation.aip|../aipack-flows/multi-model-evaluation.aip|g' *.sh
```

#### Run Individual Flows
```bash
# From rust-analysis-tools directory
aip run aipack-flows/rust-evaluator.aip \
  -p repo_path="./personal-website" \
  -p target_files="src/lib.rs" \
  -p focus_areas="performance,maintainability"

aip run aipack-flows/rust-refactor.aip \
  -p evaluation_report="$(cat memories.md)" \
  -p source_code="$(cat personal-website/src/lib.rs)" \
  -p priority="high"

aip run aipack-flows/multi-model-evaluation.aip -s
```

#### Multi-Model Comparison
```bash
# Test specific models
MODELS=("llama3.2:3b" "codellama:7b" "deepseek-coder:latest")

for model in "${MODELS[@]}"; do
  echo "Testing $model..."
  sed -i '' "s/model = \".*\"/model = \"$model\"/" .aipack/config.toml
  aip run aipack-flows/rust-evaluator.aip -s > "results/${model//[:/]/-}-evaluation.md"
done
```

## Memory Files

The flow automatically maintains three memory files:

- **@memories.md**: Stores evaluation findings and analysis results
- **@scratchpad.md**: Tracks current progress and interim steps  
- **@lessons-learned.md**: Documents insights and patterns for future reference

## Configuration

### Create AIPACK Configuration

```bash
# Create .aipack directory in your working directory
mkdir -p .aipack

# Create configuration file
cat > .aipack/config.toml << 'EOF'
# AIPACK Configuration for Rust Code Evaluation Flow

[options]
model = "llama3.2:3b"  # Default model

[options.model_aliases]
# Core models for Rust evaluation
llama3 = "llama3.2:3b"
codellama = "codellama:7b" 
deepseek = "deepseek-coder:latest"

# Specialized coding models
rust-coder = "victornitu/rust-coder:latest"
cogito = "cogito:latest"
qwen3-coder = "qwen3-coder:latest"
qwen25-coder = "qwen2.5-coder:latest"

# Large language models
gemma3 = "gemma3:27b"
qwen3 = "qwen3:latest"
qwen3-8b = "qwen3:8b"
EOF
```

### Model-Specific Configurations

```bash
# Switch models dynamically
sed -i '' 's/model = ".*"/model = "codellama:7b"/' .aipack/config.toml

# For large models, increase timeout
sed -i '' 's/model = ".*"/model = "gemma3:27b"/' .aipack/config.toml
```

## Example Output

The flow generates structured reports with:

### Single Model Output
- Code quality assessment (1-10 rating)
- Specific improvement recommendations
- Before/after code examples
- Implementation priority levels
- Performance metrics and timing

### Multi-Model Comparison
- Side-by-side model performance
- Consensus recommendations
- Model-specific strengths and weaknesses
- Aggregated scoring and rankings
- Visual comparison charts

### Sample Results Structure
```
evaluation-results/
├── llama3.2-3b-evaluation.md      # Individual model results
├── codellama-7b-evaluation.md
├── deepseek-coder-evaluation.md
└── comparison-summary.md           # Aggregated comparison
```

## Benefits

- **Local Processing**: Uses Ollama for privacy and speed
- **Multi-Model Analysis**: Compare results across different AI models
- **Structured Evaluation**: Systematic assessment across multiple dimensions
- **Memory Persistence**: Maintains context across sessions
- **Actionable Output**: Concrete refactoring suggestions with examples
- **Automated Scaling**: Run evaluations across all available models
- **Performance Benchmarking**: Model-specific timing and quality metrics
- **Path Flexibility**: Works from any directory with proper configuration

## Troubleshooting

### AIPACK Flow Path Issues
```
No agent found for local path: 'multi-model-evaluation.aip'
```
**Solutions**:
1. Copy flows: `cp ../aipack-flows/*.aip .`
2. Update paths in scripts: `sed -i '' 's|flow.aip|../aipack-flows/flow.aip|g' script.sh`
3. Run from root: `cd rust-analysis-tools && aip run aipack-flows/flow.aip`

### Configuration Issues
```
AIPACK configuration is invalid
```
**Solution**: Create proper `.aipack/config.toml` in working directory

### Model Availability
```
Model 'model-name' not found
```
**Solution**:
```bash
# Check available models
ollama list

# Pull missing models
ollama pull llama3.2:3b
ollama pull codellama:7b
```

## Target Repository

Perfect for evaluating the `personal-website` Rust/WASM project which includes:
- Modern Rust patterns
- WASM integration
- Async/await usage
- DOM manipulation
- Error handling patterns
