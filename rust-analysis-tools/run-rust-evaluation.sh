#!/bin/bash

# Rust Code Evaluation Flow using AIPACK + Ollama
# Usage: ./run-rust-evaluation.sh [repository_path] [module_name]

set -e

REPO_PATH=${1:-"./personal-website"}
MODULE_NAME=${2:-"personal-website"}
TIMESTAMP=$(date +"%Y-%m-%d_%H-%M-%S")

echo "🦀 Starting Rust Code Evaluation Flow"
echo "Repository: $REPO_PATH"
echo "Module: $MODULE_NAME"
echo "Timestamp: $TIMESTAMP"

# Check if Ollama is running
if ! curl -s http://localhost:11434/api/tags > /dev/null; then
    echo "❌ Ollama is not running. Please start it with: ollama serve"
    exit 1
fi

# Ensure required models are available
echo "📥 Checking Ollama models..."
ollama pull llama3.2:3b
ollama pull codellama:7b

# Initialize memory files if they don't exist
touch memories.md scratchpad.md lessons-learned.md

# Run the orchestrated flow
echo "🚀 Running Rust evaluation flow..."

# First run the evaluator
echo "📊 Running code evaluation..."
aip run aipack-flows/rust-evaluator.aip -f "$REPO_PATH/src/lib.rs" -s > evaluation-result-$TIMESTAMP.md

# Then run the refactoring suggestions
echo "🔧 Generating refactoring suggestions..."
aip run aipack-flows/rust-refactor.aip -f "$REPO_PATH/src/lib.rs" -s >> evaluation-result-$TIMESTAMP.md

echo "✅ Evaluation flow completed!"
echo "📄 Results saved to: evaluation-result-$TIMESTAMP.md"
echo "📝 Check memory files for detailed progress tracking"
