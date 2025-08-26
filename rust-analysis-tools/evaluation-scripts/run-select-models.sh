#!/bin/bash

# Test a selection of available models for Rust code evaluation
set -e

TIMESTAMP=$(date +"%Y-%m-%d_%H-%M-%S")
RESULTS_DIR="select-models-$TIMESTAMP"

echo "🔬 Testing Select Models for Rust Code Evaluation"
echo "Results: $RESULTS_DIR"

mkdir -p "$RESULTS_DIR"

# Test models that we know are available
MODELS=(
    "llama3.2:3b"
    "codellama:7b"
    "deepseek-coder:latest"
    "victornitu/rust-coder:latest"
    "qwen2.5-coder:latest"
)

run_evaluation() {
    local model=$1
    local output_file="$RESULTS_DIR/${model//[:\/.]/}-evaluation.md"
    
    echo "🤖 Testing: $model"
    
    # Update config temporarily
    sed -i.bak "s/model = \".*\"/model = \"$model\"/" .aipack/config.toml
    
    local start_time=$(date +%s)
    
    if timeout 90s aip run ../aipack-flows/multi-model-evaluation.aip -s > "$output_file" 2>&1; then
        local end_time=$(date +%s)
        local duration=$((end_time - start_time))
        echo "✅ $model completed in ${duration}s"
        
        echo "" >> "$output_file"
        echo "---" >> "$output_file"
        echo "**Model:** $model" >> "$output_file"
        echo "**Duration:** ${duration}s" >> "$output_file"
    else
        echo "❌ $model failed/timeout"
        echo "**ERROR: Failed or timeout**" > "$output_file"
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

# Create summary
echo "📊 Creating summary..."
{
    echo "# Multi-Model Evaluation Results"
    echo "**Date:** $(date)"
    echo ""
    echo "## Model Comparison"
    echo ""
    echo "| Model | Status | Duration | Score |"
    echo "|-------|--------|----------|-------|"
    
    for model in "${MODELS[@]}"; do
        file="$RESULTS_DIR/${model//[:\/.]/}-evaluation.md"
        if [ -f "$file" ]; then
            if grep -q "ERROR" "$file"; then
                echo "| $model | ❌ Failed | - | - |"
            else
                duration=$(grep "Duration:" "$file" | cut -d' ' -f2 || echo "N/A")
                score=$(grep -o "Score: [0-9]\+/10" "$file" | head -1 || echo "N/A")
                echo "| $model | ✅ Success | $duration | $score |"
            fi
        fi
    done
} > "$RESULTS_DIR/summary.md"

echo "✅ Evaluation complete!"
echo "📁 Results: $RESULTS_DIR/"
echo "📊 Summary: $RESULTS_DIR/summary.md"
cat "$RESULTS_DIR/summary.md"
