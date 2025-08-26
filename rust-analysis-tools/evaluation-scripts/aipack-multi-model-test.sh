#!/bin/bash

# AIPACK Multi-Model Rust Code Evaluation
# Tests different Ollama models directly with AIPACK

set -e

TIMESTAMP=$(date +"%Y-%m-%d_%H-%M-%S")
RESULTS_DIR="aipack-multi-model-$TIMESTAMP"

echo "🔬 AIPACK Multi-Model Rust Code Evaluation"
echo "Results: $RESULTS_DIR"

mkdir -p "$RESULTS_DIR"

# Available models from ollama list
MODELS=(
    "llama3.2:3b"
    "codellama:7b"
    "deepseek-coder:latest"
    "victornitu/rust-coder:latest"
    "qwen2.5-coder:latest"
    "cogito:latest"
)

echo "📋 Testing ${#MODELS[@]} models"

# Function to test a specific model
test_model() {
    local model=$1
    local safe_name=$(echo "$model" | tr ':/./' '-')
    local output_file="$RESULTS_DIR/${safe_name}.md"
    
    echo "🤖 Testing: $model"
    
    # Create temporary config for this model
    cat > .aipack/config_temp.toml << EOF
[options]
model = "$model"
EOF
    
    local start_time=$(date +%s)
    
    # Run AIPACK with model-specific config
    if timeout 60s aip run ../aipack-flows/multi-model-evaluation.aip -s > "$output_file" 2>&1; then
        local end_time=$(date +%s)
        local duration=$((end_time - start_time))
        echo "✅ $model completed in ${duration}s"
        
        # Add metadata
        echo "" >> "$output_file"
        echo "---" >> "$output_file"
        echo "**Model:** $model" >> "$output_file"
        echo "**Duration:** ${duration}s" >> "$output_file"
        echo "**Status:** Success" >> "$output_file"
    else
        local end_time=$(date +%s)
        local duration=$((end_time - start_time))
        echo "❌ $model failed (${duration}s)"
        echo "**ERROR: Failed or timeout**" > "$output_file"
        echo "**Model:** $model" >> "$output_file"
        echo "**Duration:** ${duration}s" >> "$output_file"
        echo "**Status:** Failed" >> "$output_file"
    fi
    
    # Restore original config
    if [ -f .aipack/config.toml.orig ]; then
        cp .aipack/config.toml.orig .aipack/config.toml
    fi
    
    sleep 1
}

# Backup original config
cp .aipack/config.toml .aipack/config.toml.orig

# Test each model
for model in "${MODELS[@]}"; do
    test_model "$model"
done

# Create comprehensive comparison
echo "📊 Generating comparison report..."

{
    echo "# AIPACK Multi-Model Rust Code Evaluation Results"
    echo ""
    echo "**Date:** $(date)"
    echo "**Code Analyzed:** Rust WASM personal website application"
    echo ""
    echo "## Model Performance Comparison"
    echo ""
    echo "| Model | Status | Duration | Score | Primary Issue |"
    echo "|-------|--------|----------|-------|---------------|"
    
    for model in "${MODELS[@]}"; do
        safe_name=$(echo "$model" | tr ':/./' '-')
        file="$RESULTS_DIR/${safe_name}.md"
        
        if [ -f "$file" ]; then
            if grep -q "ERROR" "$file"; then
                duration=$(grep "Duration:" "$file" | cut -d' ' -f2 || echo "N/A")
                echo "| $model | ❌ Failed | $duration | - | Timeout/Error |"
            else
                duration=$(grep "Duration:" "$file" | cut -d' ' -f2 || echo "N/A")
                score=$(grep -o "Score: [0-9]\+/10" "$file" | head -1 || echo "N/A")
                # Extract first issue mentioned
                issue=$(grep -A3 "Issues:" "$file" | tail -2 | head -1 | sed 's/^[0-9]*\. *//' | cut -c1-30 | tr -d '\n' || echo "N/A")
                echo "| $model | ✅ Success | $duration | $score | $issue... |"
            fi
        fi
    done
    
    echo ""
    echo "## Analysis Quality Comparison"
    echo ""
    
    for model in "${MODELS[@]}"; do
        safe_name=$(echo "$model" | tr ':/./' '-')
        file="$RESULTS_DIR/${safe_name}.md"
        
        if [ -f "$file" ] && ! grep -q "ERROR" "$file"; then
            echo "### $model Analysis"
            echo ""
            # Extract key sections
            if grep -q "Score:" "$file"; then
                echo "**Score:** $(grep -o "Score: [0-9]\+/10" "$file" | head -1)"
            fi
            if grep -A1 "Primary Recommendation" "$file" > /dev/null 2>&1; then
                echo "**Key Recommendation:** $(grep -A1 "Primary Recommendation" "$file" | tail -1)"
            fi
            echo ""
        fi
    done
    
    echo "## Summary"
    echo ""
    successful=$(find "$RESULTS_DIR" -name "*.md" -exec grep -l "Score:" {} \; | wc -l | tr -d ' ')
    total=${#MODELS[@]}
    echo "- **Models tested:** $total"
    echo "- **Successful evaluations:** $successful"
    echo "- **Success rate:** $((successful * 100 / total))%"
    echo ""
    echo "All detailed results available in individual model files."
    
} > "$RESULTS_DIR/comparison-report.md"

# Restore original config
cp .aipack/config.toml.orig .aipack/config.toml
rm .aipack/config.toml.orig

echo ""
echo "✅ Multi-model evaluation completed!"
echo "📁 Results: $RESULTS_DIR/"
echo "📊 Report: $RESULTS_DIR/comparison-report.md"
echo ""
echo "📋 Quick Summary:"
tail -10 "$RESULTS_DIR/comparison-report.md"
