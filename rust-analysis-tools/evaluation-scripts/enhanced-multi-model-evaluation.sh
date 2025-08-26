#!/bin/bash

# Enhanced Multi-Model Rust Code Evaluation using AIPACK + Ollama
# Features improved parsing, fallback strategies, additional metrics extraction, and terminal-friendly charts

set -e

TIMESTAMP=$(date +"%Y-%m-%d_%H-%M-%S")
RESULTS_DIR="multi-model-results-$TIMESTAMP"

echo "🔬 Starting Enhanced Multi-Model Rust Code Evaluation"
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

# Enhanced parsing functions for better data extraction
extract_score() {
    local file="$1"
    # Multiple regex patterns to handle different score formats
    local score=""
    
    # Pattern 1: **Score: X/10**
    score=$(grep -o "\*\*Score[: ]*[0-9]\+/10\*\*" "$file" | sed -n 's/.*\*\*Score[: ]*\([0-9]\+\)\/10\*\*.*/\1/p' | head -1)
    
    # Pattern 2: Score: X/10 (without asterisks)
    if [ -z "$score" ]; then
        score=$(grep -o "Score[: ]*[0-9]\+/10" "$file" | sed -n 's/.*Score[: ]*\([0-9]\+\)\/10.*/\1/p' | head -1)
    fi
    
    # Pattern 3: X/10 score at beginning of line
    if [ -z "$score" ]; then
        score=$(grep -o "^[0-9]\+/10" "$file" | sed 's|/10||' | head -1)
    fi
    
    # Fallback: search for any X/10 pattern
    if [ -z "$score" ]; then
        score=$(grep -o "[0-9]\+/10" "$file" | sed 's|/10||' | head -1)
    fi
    
    echo "${score:-N/A}"
}

extract_strengths() {
    local file="$1"
    local strength=""
    
    # Primary approach: numbered list after "Strengths:"
    strength=$(grep -A10 "\*\*Strengths:\*\*\|^Strengths:" "$file" | grep -E "^[0-9]+\." | head -1 | sed 's/^[0-9]*\. *//' | cut -c1-50)
    
    # Fallback 1: bullet points
    if [ -z "$strength" ]; then
        strength=$(grep -A10 "\*\*Strengths:\*\*\|^Strengths:" "$file" | grep -E "^[-*]" | head -1 | sed 's/^[-*] *//' | cut -c1-50)
    fi
    
    # Fallback 2: first line after "Strengths:" that isn't empty
    if [ -z "$strength" ]; then
        strength=$(grep -A5 "\*\*Strengths:\*\*\|^Strengths:" "$file" | grep -v "\*\*Strengths:\*\*\|^Strengths:\|^$" | head -1 | cut -c1-50)
    fi
    
    echo "${strength:-N/A}"
}

extract_issues() {
    local file="$1"
    local issue=""
    
    # Primary approach: numbered list after "Issues:"
    issue=$(grep -A10 "\*\*Issues:\*\*\|^Issues:" "$file" | grep -E "^[0-9]+\." | head -1 | sed 's/^[0-9]*\. *//' | cut -c1-50)
    
    # Fallback 1: bullet points
    if [ -z "$issue" ]; then
        issue=$(grep -A10 "\*\*Issues:\*\*\|^Issues:" "$file" | grep -E "^[-*]" | head -1 | sed 's/^[-*] *//' | cut -c1-50)
    fi
    
    # Fallback 2: first line after "Issues:" that isn't empty
    if [ -z "$issue" ]; then
        issue=$(grep -A5 "\*\*Issues:\*\*\|^Issues:" "$file" | grep -v "\*\*Issues:\*\*\|^Issues:\|^$" | head -1 | cut -c1-50)
    fi
    
    echo "${issue:-N/A}"
}

extract_metrics() {
    local file="$1"
    local prompt_tokens completion_tokens response_time model_size
    
    # Extract token counts
    prompt_tokens=$(grep -o "Prompt Tokens: [0-9]\+" "$file" | sed 's/Prompt Tokens: //' || echo "0")
    completion_tokens=$(grep -o "Completion Tokens: [0-9]\+" "$file" | sed 's/Completion Tokens: //' || echo "0")
    
    # Extract response time from Duration field
    response_time=$(grep -o "Duration: [0-9]\+[smh]" "$file" | sed 's/Duration: //' || echo "N/A")
    
    # Alternative response time extraction
    if [ "$response_time" = "N/A" ]; then
        response_time=$(grep -o "[0-9]\+s [0-9]\+ms\|[0-9]\+m [0-9]\+s" "$file" | head -1 || echo "N/A")
    fi
    
    # Model size estimation (basic heuristic based on model name)
    case "$1" in
        *3b*) model_size="3B" ;;
        *7b*) model_size="7B" ;;
        *8b*) model_size="8B" ;;
        *27b*) model_size="27B" ;;
        *latest*) model_size="Variable" ;;
        *) model_size="Unknown" ;;
    esac
    
    echo "$prompt_tokens|$completion_tokens|$response_time|$model_size"
}

generate_visual_bar() {
    local score="$1"
    local bar=""
    
    if [[ "$score" =~ ^[0-9]+$ ]]; then
        # Generate filled bars for score
        for ((i=1; i<=score; i++)); do
            bar+="█"
        done
        
        # Generate empty bars for remaining
        for ((i=score+1; i<=10; i++)); do
            bar+="░"
        done
    else
        bar="N/A      "
    fi
    
    echo "$bar"
}

# Pre-flight checks function (same as original)
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
    echo -e "\n🔎 Checking model availability:"
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
        echo -e "\n⚠️  WARNING: ${#missing_models[@]} model(s) missing:"
        printf '  - %s\n' "${missing_models[@]}"
        echo -e "\n💡 To install missing models, run:"
        for model in "${missing_models[@]}"; do
            echo "  ollama pull $model"
        done
        echo -e "\n🔄 Continuing with available models only..."
        # Update MODELS array to only include available models
        MODELS=("${available_test_models[@]}")
    fi
    
    if [ ${#MODELS[@]} -eq 0 ]; then
        echo -e "\n❌ ERROR: No models available for testing!"
        echo "Please install at least one model using 'ollama pull [model-name]'"
        exit 1
    fi
    
    echo -e "\n✅ Pre-flight check complete. Testing ${#MODELS[@]} model(s)."
}

# Enhanced function to run evaluation with specific model
run_model_evaluation() {
    local model=$1
    # Clean model name for filename (replace special characters)
    local clean_model_name=$(echo "$model" | sed 's/[:\\/]/-/g')
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
    
    if timeout 300 aip run ../aipack-flows/multi-model-evaluation.aip -s > "$output_file" 2>&1; then
        local end_time=$(date +%s)
        local duration=$((end_time - start_time))
        echo "✅ Completed $model in ${duration}s"
        
        # Add enhanced metadata to the result file
        echo "" >> "$output_file"
        echo "---" >> "$output_file"
        echo "**Model:** $model" >> "$output_file"
        echo "**Duration:** ${duration}s" >> "$output_file"
        echo "**Timestamp:** $(date)" >> "$output_file"
        echo "**Status:** SUCCESS" >> "$output_file"
    else
        echo "❌ Failed or timed out: $model"
        echo "**ERROR: Evaluation failed or timed out (300s limit)**" > "$output_file"
        echo "**Model:** $model" >> "$output_file"
        echo "**Timestamp:** $(date)" >> "$output_file"
        echo "**Status:** FAILED" >> "$output_file"
    fi
    
    # Restore original config
    mv .aipack/config.toml.bak .aipack/config.toml
}

# Run pre-flight checks
check_model_availability

echo "📋 Models to evaluate: ${#MODELS[@]}"
printf '%s\n' "${MODELS[@]}"

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
    local attempt=0
    local success=false

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
    sleep 3

done

# Generate enhanced comparison report
echo "📊 Generating enhanced comparison report..."
cat > "$RESULTS_DIR/enhanced-comparison-summary.md" << 'EOF'
# Enhanced Multi-Model Rust Code Evaluation Comparison

## Visual Performance Overview

```
Model Performance Chart (Score out of 10)
0    2    4    6    8    10
|    |    |    |    |    |
EOF

# Enhanced summary table header
echo "" >> "$RESULTS_DIR/enhanced-comparison-summary.md"
echo "## Detailed Performance Summary" >> "$RESULTS_DIR/enhanced-comparison-summary.md"
echo "" >> "$RESULTS_DIR/enhanced-comparison-summary.md"
echo "| Model | Visual Score | Score | Duration | Tokens (P/C) | Size | Primary Strength | Main Issue |" >> "$RESULTS_DIR/enhanced-comparison-summary.md"
echo "|-------|-------------|-------|----------|--------------|------|------------------|------------|" >> "$RESULTS_DIR/enhanced-comparison-summary.md"

# Create arrays to store data for charts
declare -a model_names=()
declare -a scores=()
declare -a durations=()

# Extract enhanced metrics and create summary
for model in "${MODELS[@]}"; do
    # Clean model name for filename (same as in run_model_evaluation)
    clean_model_name=$(echo "$model" | sed 's/[:\\/]/-/g')
    result_file="$RESULTS_DIR/${clean_model_name}-evaluation.md"
    
    if [ -f "$result_file" ]; then
        # Enhanced parsing using new functions
        score=$(extract_score "$result_file")
        strength=$(extract_strengths "$result_file")
        issue=$(extract_issues "$result_file")
        
        # Extract duration from metadata
        duration=$(grep "**Duration:**" "$result_file" | sed 's/\*\*Duration:\*\* //' | sed 's/s//' || echo "N/A")
        
        # Extract additional metrics
        metrics=$(extract_metrics "$result_file")
        IFS='|' read -r prompt_tokens completion_tokens response_time model_size <<< "$metrics"
        
        # Generate visual representation
        visual_bar=$(generate_visual_bar "$score")
        
        # Store data for charts
        model_names+=("$model")
        scores+=("$score")
        durations+=("$duration")
        
        # Create enhanced table row
        printf "| %s | %s | %s/10 | %ss | %s/%s | %s | %s | %s |\n" \
            "$model" \
            "$visual_bar" \
            "$score" \
            "$duration" \
            "$prompt_tokens" \
            "$completion_tokens" \
            "$model_size" \
            "$(echo "$strength" | cut -c1-30)..." \
            "$(echo "$issue" | cut -c1-30)..." >> "$RESULTS_DIR/enhanced-comparison-summary.md"
        
        # Add to visual chart
        printf "%-25s %s %s/10\n" "$model" "$visual_bar" "$score" >> "$RESULTS_DIR/enhanced-comparison-summary.md.tmp"
    fi
done

# Insert visual chart into report
echo "" >> "$RESULTS_DIR/enhanced-comparison-summary.md"
if [ -f "$RESULTS_DIR/enhanced-comparison-summary.md.tmp" ]; then
    cat "$RESULTS_DIR/enhanced-comparison-summary.md.tmp" >> "$RESULTS_DIR/enhanced-comparison-summary.md"
    rm "$RESULTS_DIR/enhanced-comparison-summary.md.tmp"
fi

# Add statistics section
echo "" >> "$RESULTS_DIR/enhanced-comparison-summary.md"
echo "## Performance Statistics" >> "$RESULTS_DIR/enhanced-comparison-summary.md"
echo "" >> "$RESULTS_DIR/enhanced-comparison-summary.md"

# Calculate statistics
if [ ${#scores[@]} -gt 0 ]; then
    # Calculate average score (excluding N/A values)
    total_score=0
    valid_scores=0
    for score in "${scores[@]}"; do
        if [[ "$score" =~ ^[0-9]+$ ]]; then
            total_score=$((total_score + score))
            valid_scores=$((valid_scores + 1))
        fi
    done
    
    if [ $valid_scores -gt 0 ]; then
        avg_score=$((total_score / valid_scores))
        echo "- **Average Score:** $avg_score/10" >> "$RESULTS_DIR/enhanced-comparison-summary.md"
    fi
    
    # Find best and worst performing models
    best_score=0
    worst_score=10
    best_model=""
    worst_model=""
    
    for i in "${!scores[@]}"; do
        score="${scores[$i]}"
        model="${model_names[$i]}"
        
        if [[ "$score" =~ ^[0-9]+$ ]]; then
            if [ "$score" -gt "$best_score" ]; then
                best_score="$score"
                best_model="$model"
            fi
            if [ "$score" -lt "$worst_score" ]; then
                worst_score="$score"
                worst_model="$model"
            fi
        fi
    done
    
    if [ -n "$best_model" ]; then
        echo "- **Best Performer:** $best_model ($best_score/10)" >> "$RESULTS_DIR/enhanced-comparison-summary.md"
    fi
    if [ -n "$worst_model" ]; then
        echo "- **Needs Improvement:** $worst_model ($worst_score/10)" >> "$RESULTS_DIR/enhanced-comparison-summary.md"
    fi
fi

echo "- **Total Models Evaluated:** ${#MODELS[@]}" >> "$RESULTS_DIR/enhanced-comparison-summary.md"
echo "- **Evaluation Date:** $(date)" >> "$RESULTS_DIR/enhanced-comparison-summary.md"

echo "" >> "$RESULTS_DIR/enhanced-comparison-summary.md"
echo "## Detailed Analysis" >> "$RESULTS_DIR/enhanced-comparison-summary.md"
echo "See individual model result files for complete evaluations and recommendations." >> "$RESULTS_DIR/enhanced-comparison-summary.md"

echo "" >> "$RESULTS_DIR/enhanced-comparison-summary.md"
echo "---" >> "$RESULTS_DIR/enhanced-comparison-summary.md"
echo "*Generated by Enhanced Multi-Model Evaluation System*" >> "$RESULTS_DIR/enhanced-comparison-summary.md"

echo "✅ Enhanced multi-model evaluation completed!"
echo "📁 Results directory: $RESULTS_DIR"
echo "📊 Enhanced summary report: $RESULTS_DIR/enhanced-comparison-summary.md"
echo ""
echo "📋 Quick Summary:"
echo "$(head -20 "$RESULTS_DIR/enhanced-comparison-summary.md")"
