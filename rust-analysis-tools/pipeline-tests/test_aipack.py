#!/usr/bin/env python3
"""
Simple test script for AIPACK pipeline.
"""

import asyncio
import sys
from pathlib import Path
sys.path.append(str(Path(__file__).parent.parent.parent / "credit-assessment-system"))
from evaluation_pipelines import AIPackEvaluator

async def test_aipack():
    """Test AIPACK pipeline."""
    model = "llama3.2:3b"
    
    print("🧪 Testing AIPACK pipeline...")
    
    try:
        print("Creating AIPACK evaluator...")
        evaluator = AIPackEvaluator(model)
        
        print("Parsing Rust code...")
        code = evaluator.parse_rust_code()
        print("Code parsed successfully")
        
        print("Running AIPACK evaluation...")
        result = await evaluator.evaluate_code(code)
        
        print(f"AIPACK Score: {result.score}/10")
        print(f"AIPACK Duration: {result.duration:.2f}s")
        print(f"AIPACK Primary Recommendation: {result.primary_recommendation}")
        
        print("✅ AIPACK test completed successfully!")
        
    except Exception as e:
        print(f"❌ AIPACK Error: {e}")
        import traceback
        traceback.print_exc()

if __name__ == "__main__":
    asyncio.run(test_aipack()) 