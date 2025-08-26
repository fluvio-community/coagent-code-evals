#!/usr/bin/env python3
"""
Debug script for smolagents pipeline.
"""

import asyncio
from evaluation_pipelines import SmolAgentsEvaluator

async def debug_smolagents():
    """Debug smolagents pipeline."""
    model = "llama3.2:3b"
    
    print("🔍 Debugging smolagents pipeline...")
    
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
        
        if result.error:
            print(f"❌ Error: {result.error}")
        
        print("✅ smolagents debug completed!")
        
    except Exception as e:
        print(f"❌ Exception: {e}")
        import traceback
        traceback.print_exc()

if __name__ == "__main__":
    asyncio.run(debug_smolagents()) 