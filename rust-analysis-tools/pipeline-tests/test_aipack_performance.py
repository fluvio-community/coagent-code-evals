#!/usr/bin/env python3
"""
Test script to investigate AIPACK performance issues.
"""

import asyncio
import time
from evaluation_pipelines import AIPackEvaluator

async def test_aipack_performance():
    """Test AIPACK performance with multiple runs."""
    model = "llama3.2:3b"
    
    print("🔍 Testing AIPACK performance...")
    
    for i in range(3):
        print(f"\n--- Run {i+1} ---")
        start_time = time.time()
        
        try:
            evaluator = AIPackEvaluator(model)
            code = evaluator.parse_rust_code()
            
            print(f"Starting AIPACK evaluation {i+1}...")
            result = await evaluator.evaluate_code(code)
            
            end_time = time.time()
            total_time = end_time - start_time
            
            print(f"AIPACK Run {i+1} Score: {result.score}/10")
            print(f"AIPACK Run {i+1} Duration: {result.duration:.2f}s")
            print(f"AIPACK Run {i+1} Total Time: {total_time:.2f}s")
            
            if result.error:
                print(f"AIPACK Run {i+1} Error: {result.error}")
                
        except Exception as e:
            end_time = time.time()
            total_time = end_time - start_time
            print(f"AIPACK Run {i+1} Exception: {e}")
            print(f"AIPACK Run {i+1} Total Time: {total_time:.2f}s")
        
        # Wait between runs
        if i < 2:
            print("Waiting 5 seconds before next run...")
            await asyncio.sleep(5)

if __name__ == "__main__":
    asyncio.run(test_aipack_performance()) 