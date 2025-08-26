#!/bin/bash

# Setup script for multi-pipeline Rust code evaluation system
# Uses uv for Python dependency management

set -e

echo "🔧 Setting up Multi-Pipeline Rust Code Evaluation System"
echo "========================================================"

# Check if uv is installed
if ! command -v uv &> /dev/null; then
    echo "❌ uv is not installed. Please install it first:"
    echo "   curl -LsSf https://astral.sh/uv/install.sh | sh"
    exit 1
fi

echo "✅ uv is installed"

# Check if Ollama is running
if ! curl -s http://localhost:11434/api/tags > /dev/null; then
    echo "❌ Ollama is not running. Please start it with: ollama serve"
    exit 1
fi

echo "✅ Ollama is running"

# Create virtual environment and install dependencies
echo "📦 Installing Python dependencies with uv..."

# Install dependencies using uv
uv sync

echo "✅ Dependencies installed successfully"

# Make the main script executable
chmod +x ../../credit-assessment-system/run_multi_pipeline_evaluation.py

# Create a simple test script
cat > test_pipelines.py << 'EOF'
#!/usr/bin/env python3
"""
Quick test script for the evaluation pipelines.
"""

import asyncio
import sys
from pathlib import Path
sys.path.append(str(Path(__file__).parent.parent.parent / "credit-assessment-system"))
from evaluation_pipelines import AIPackEvaluator, CrewAIEvaluator, SmolAgentsEvaluator

async def test_pipelines():
    """Test all three pipelines with a simple model."""
    model = "llama3.2:3b"
    
    print("🧪 Testing evaluation pipelines...")
    
    # Test AIPACK
    try:
        print("Testing AIPACK...")
        evaluator = AIPackEvaluator(model)
        code = evaluator.parse_rust_code()
        result = await evaluator.evaluate_code(code)
        print(f"AIPACK Score: {result.score}/10")
    except Exception as e:
        print(f"AIPACK Error: {e}")
    
    # Test CrewAI
    try:
        print("Testing CrewAI...")
        evaluator = CrewAIEvaluator(model)
        code = evaluator.parse_rust_code()
        result = await evaluator.evaluate_code(code)
        print(f"CrewAI Score: {result.score}/10")
    except Exception as e:
        print(f"CrewAI Error: {e}")
    
    # Test smol-agents
    try:
        print("Testing smol-agents...")
        evaluator = SmolAgentsEvaluator(model)
        code = evaluator.parse_rust_code()
        result = await evaluator.evaluate_code(code)
        print(f"smol-agents Score: {result.score}/10")
    except Exception as e:
        print(f"smol-agents Error: {e}")

if __name__ == "__main__":
    asyncio.run(test_pipelines())
EOF

chmod +x test_pipelines.py

echo "✅ Setup completed successfully!"
echo ""
echo "📋 Usage Examples:"
echo ""
echo "1. Run all pipelines with default models:"
echo "   uv run python ../../credit-assessment-system/run_multi_pipeline_evaluation.py"
echo ""
echo "2. Run specific pipeline:"
echo "   uv run python ../../credit-assessment-system/run_multi_pipeline_evaluation.py --pipeline crewai"
echo ""
echo "3. Run with custom models:"
echo "   uv run python ../../credit-assessment-system/run_multi_pipeline_evaluation.py --models llama3.2:3b codellama:7b"
echo ""
echo "4. Test individual pipelines:"
echo "   uv run python test_pipelines.py"
echo ""
echo "📁 Results will be saved to: multi-pipeline-results/"
echo ""
echo "🔬 Ready to evaluate Rust code with three different frameworks!" 