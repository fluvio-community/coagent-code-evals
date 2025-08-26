#!/usr/bin/env python3
"""
Simple test script for smolagents pipeline.
"""

import asyncio
import sys
from pathlib import Path
sys.path.append(str(Path(__file__).parent.parent.parent / "credit-assessment-system"))
from evaluation_pipelines import SmolAgentsEvaluator

async def test_smolagents():
    """Test smolagents pipeline."""
    model = "llama3.2:3b"
    
    print("🧪 Testing smolagents pipeline...")
    
    try:
        print("Creating smolagents evaluator...")
        evaluator = SmolAgentsEvaluator(model)
        
        print("Parsing Rust code...")
        code = evaluator.parse_rust_code()
        print("Code parsed successfully")
        
        print("Running smolagents evaluation...")
        result = await evaluator.evaluate_code(code)
        
        print(f"smolagents Score: {result.score}/10")
        print(f"smolagents Duration: {result.duration:.2f}s")
        print(f"smolagents Primary Recommendation: {result.primary_recommendation}")
        
        print("✅ smolagents test completed successfully!")
        
    except Exception as e:
        print(f"❌ smolagents Error: {e}")
        import traceback
        traceback.print_exc()

if __name__ == "__main__":
    asyncio.run(test_smolagents()) 