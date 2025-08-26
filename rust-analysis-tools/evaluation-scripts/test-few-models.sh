#!/bin/bash

# Test a few models to validate the system
set -e

TIMESTAMP=$(date +"%Y-%m-%d_%H-%M-%S")
RESULTS_DIR="test-models-$TIMESTAMP"

echo "🧪 Testing Few Models for Validation"
echo "Results: $RESULTS_DIR"

mkdir -p "$RESULTS_DIR"

# Test just 3 fast models
MODELS=(
    "llama3.2:3b"
    "deepseek-coder:latest"
    "qwen2.5-coder:latest"
)

run_evaluation() {
    local model=$1
    local clean_name=$(echo "$model" | sed 's/[:\/.]/&/g')
    local output_file="$RESULTS_DIR/${clean_name}-evaluation.md"
    
    echo "🤖 Testing: $model"
    
    # Create temporary config
    cp .aipack/config.toml .aipack/config.toml.bak
    cat > .aipack/config.toml << EOF
# AIPACK Configuration for Rust Code Evaluation Flow

[options]
model = "$model"

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
    
    local start_time=$(date +%s)
    
    if aip run ../aipack-flows/multi-model-evaluation.aip -s > "$output_file" 2>&1; then
        local end_time=$(date +%s)
        local duration=$((end_time - start_time))
        echo "✅ $model completed in ${duration}s"
        
        echo "" >> "$output_file"
        echo "---" >> "$output_file"
        echo "**Model:** $model" >> "$output_file"
        echo "**Duration:** ${duration}s" >> "$output_file"
    else
        echo "❌ $model failed"
        echo "**ERROR: Evaluation failed**" > "$output_file"
        echo "**Model:** $model" >> "$output_file"
    fi
    
    # Restore config
    mv .aipack/config.toml.bak .aipack/config.toml
    sleep 1
}

# Test each model
for model in "${MODELS[@]}"; do
    run_evaluation "$model"
done

echo "✅ Test complete!"
echo "📁 Results: $RESULTS_DIR/"
ls -la "$RESULTS_DIR/"
