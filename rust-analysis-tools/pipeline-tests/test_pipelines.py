#!/usr/bin/env python3
"""
Quick test script for the evaluation pipelines.
"""

import asyncio
import sys
from pathlib import Path
sys.path.append(str(Path(__file__).parent.parent.parent / "credit-assessment-system"))
from evaluation_pipelines import AIPackEvaluator, CrewAIEvaluator

async def test_pipelines():
    """Test the working pipelines with a simple model."""
    model = "llama3.2:3b"
    
    print("🧪 Testing evaluation pipelines...")
    
    # Test AIPACK
    try:
        print("Testing AIPACK...")
        evaluator = AIPackEvaluator(model)
        code = evaluator.parse_rust_code()
        result = await evaluator.evaluate_code(code)
        print(f"AIPACK Score: {result.score}/10")
        print(f"AIPACK Duration: {result.duration:.2f}s")
    except Exception as e:
        print(f"AIPACK Error: {e}")
    
    # Test CrewAI
    try:
        print("Testing CrewAI...")
        evaluator = CrewAIEvaluator(model)
        code = evaluator.parse_rust_code()
        result = await evaluator.evaluate_code(code)
        print(f"CrewAI Score: {result.score}/10")
        print(f"CrewAI Duration: {result.duration:.2f}s")
    except Exception as e:
        print(f"CrewAI Error: {e}")

if __name__ == "__main__":
    asyncio.run(test_pipelines())
