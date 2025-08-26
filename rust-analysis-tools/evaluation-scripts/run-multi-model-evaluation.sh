#!/bin/bash

# Multi-Model Rust Code Evaluation using AIPACK + Ollama
# Tests all available models and compares their analysis quality

set -e

TIMESTAMP=$(date +"%Y-%m-%d_%H-%M-%S")
RESULTS_DIR="multi-model-results-$TIMESTAMP"

echo "🔬 Starting Multi-Model Rust Code Evaluation"
echo "Timestamp: $TIMESTAMP"
echo "Results will be saved to: $RESULTS_DIR"

# Create results directory
mkdir -p "$RESULTS_DIR"

# Define models to test (using actual Ollama model names, not aliases)
MODELS=(
    "llama3.2:3b"
    "codellama:7b"
    "deepseek-coder:latest"
    "victornitu/rust-coder:latest"
    "cogito:latest"
    "qwen3-coder:latest"
    "qwen2.5-coder:latest"
    "gemma3:27b"
    "qwen3:latest"
    "qwen3:8b"
)

# Pre-flight checks function
check_model_availability() {
    echo "🔍 Running pre-flight model availability checks..."
    
    # Check if Ollama is running
    if ! curl -s http://localhost:11434/api/tags > /dev/null; then
        echo "❌ Ollama is not running. Please start it with: ollama serve"
        exit 1
    fi
    
    # Get available models from Ollama
    available_models=$(ollama list | awk 'NR>1 {print $1}' | sort)
    echo "📦 Available models in Ollama:"
    echo "$available_models" | sed 's/^/  - /'
    
    # Check each model in our test list
    echo "\n🔎 Checking model availability:"
    missing_models=()
    available_test_models=()
    
    for model in "${MODELS[@]}"; do
        if echo "$available_models" | grep -q "^$model$"; then
            echo "  ✅ $model - Available"
            available_test_models+=("$model")
        else
            echo "  ❌ $model - Missing"
            missing_models+=("$model")
        fi
    done
    
    # Report results
    if [ ${#missing_models[@]} -gt 0 ]; then
        echo "\n⚠️  WARNING: ${#missing_models[@]} model(s) missing:"
        printf '  - %s\n' "${missing_models[@]}"
        echo "\n💡 To install missing models, run:"
        for model in "${missing_models[@]}"; do
            echo "  ollama pull $model"
        done
        echo "\n🔄 Continuing with available models only..."
        # Update MODELS array to only include available models
        MODELS=("${available_test_models[@]}")
    fi
    
    if [ ${#MODELS[@]} -eq 0 ]; then
        echo "\n❌ ERROR: No models available for testing!"
        echo "Please install at least one model using 'ollama pull [model-name]'"
        exit 1
    fi
    
    echo "\n✅ Pre-flight check complete. Testing ${#MODELS[@]} model(s)."
}

# Run pre-flight checks
check_model_availability

echo "📋 Models to evaluate: ${#MODELS[@]}"
printf '%s\n' "${MODELS[@]}"

# Function to run evaluation with a specific model
run_model_evaluation() {
    local model=$1
    # Clean model name for filename (replace special characters)
    local clean_model_name=$(echo "$model" | sed 's/[:\/]/-/g')
    local output_file="$RESULTS_DIR/${clean_model_name}-evaluation.md"
    
    echo "🤖 Evaluating with model: $model"
    echo "📄 Output: $output_file"
    
    # Create temporary config with the specific model
    cp .aipack/config.toml .aipack/config.toml.bak
    # Create new config with the specific model
    cat > .aipack/config.toml << EOF
# AIPACK Configuration for Rust Code Evaluation Flow

[options]
model = "$model"  # Temporary model for evaluation

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
    
    # Measure time and run evaluation
    local start_time=$(date +%s)
    
    if aip run ../aipack-flows/multi-model-evaluation.aip -s > "$output_file" 2>&1; then
        local end_time=$(date +%s)
        local duration=$((end_time - start_time))
        echo "✅ Completed $model in ${duration}s"
        
        # Add metadata to the result file
        echo "" >> "$output_file"
        echo "---" >> "$output_file"
        echo "**Model:** $model" >> "$output_file"
        echo "**Duration:** ${duration}s" >> "$output_file"
        echo "**Timestamp:** $(date)" >> "$output_file"
    else
        echo "❌ Failed or timed out: $model"
        echo "**ERROR: Evaluation failed or timed out (120s limit)**" > "$output_file"
        echo "**Model:** $model" >> "$output_file"
        echo "**Timestamp:** $(date)" >> "$output_file"
    fi
    
    # Restore original config
    mv .aipack/config.toml.bak .aipack/config.toml
}

# Log file for errors
ERROR_LOG="$RESULTS_DIR/error-log-$TIMESTAMP.txt"
touch "$ERROR_LOG"

# Function for exponential backoff
exponential_backoff() {
    local attempt=$1
    echo "Attempting model evaluation: Attempt #$attempt"
    sleep $((2**attempt))
}

# Run evaluations for each model
for model in "${MODELS[@]}"; do
    attempt=0
    success=false

    # Retry logic
    until [ $attempt -ge 3 ] || [ "$success" = true ]; do
        if run_model_evaluation "$model"; then
            success=true
        else
            echo "Retrying $model due to failure..." | tee -a "$ERROR_LOG"
            echo "Evaluation failed for $model at $(date)." >> "$ERROR_LOG"
            exponential_backoff $attempt
            ((attempt++))
        fi
    done

    # Log failure if not successful after retries
    if [ "$success" != true ]; then
        echo "Maximum attempts reached for $model; logging failure." | tee -a "$ERROR_LOG"
    fi

    echo "⏳ Pausing briefly before next model..."
    sleep 2

done

# Generate comparison report
echo "📊 Generating comparison report..."
cat > "$RESULTS_DIR/comparison-summary.md" << 'EOF'
# Multi-Model Rust Code Evaluation Comparison

## Model Performance Summary

| Model | Score | Duration | Primary Strength | Main Issue |
|-------|-------|----------|------------------|------------|
EOF

# Extract scores and create summary
for model in "${MODELS[@]}"; do
    # Clean model name for filename (same as in run_model_evaluation)
    clean_model_name=$(echo "$model" | sed 's/[:\/]/-/g')
    result_file="$RESULTS_DIR/${clean_model_name}-evaluation.md"
    if [ -f "$result_file" ]; then
        # Try to extract score from the result file
        score=$(grep -o "\*\*Score: [0-9]\+/10\*\*" "$result_file" | head -1 | sed 's/\*\*Score: \([0-9]\+\/10\)\*\*/\1/' || echo "N/A")
        duration=$(grep "\*\*Duration:\*\*" "$result_file" | cut -d' ' -f2 || echo "N/A")
        
        # Extract first strength and issue from the structured output
        strength=$(grep -A3 "\*\*Strengths:\*\*" "$result_file" | grep "^1\." | cut -c3-43 | tr -d '\n' || echo "N/A")
        issue=$(grep -A3 "\*\*Issues:\*\*" "$result_file" | grep "^1\." | cut -c3-43 | tr -d '\n' || echo "N/A")
        
        echo "| $model | $score | $duration | $strength... | $issue... |" >> "$RESULTS_DIR/comparison-summary.md"
    fi
done

echo "" >> "$RESULTS_DIR/comparison-summary.md"
echo "## Detailed Analysis" >> "$RESULTS_DIR/comparison-summary.md"
echo "See individual model result files for complete evaluations." >> "$RESULTS_DIR/comparison-summary.md"

echo "✅ Multi-model evaluation completed!"
echo "📁 Results directory: $RESULTS_DIR"
echo "📊 Summary report: $RESULTS_DIR/comparison-summary.md"
echo ""
echo "📋 Quick Summary:"
cat "$RESULTS_DIR/comparison-summary.md"
