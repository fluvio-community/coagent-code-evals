#!/usr/bin/env python3
"""
Test script to compare AIPACK performance in isolation vs multi-pipeline context.
"""

import asyncio
import time
from evaluation_pipelines import AIPackEvaluator

async def test_aipack_isolation():
    """Test AIPACK performance in isolation."""
    model = "llama3.2:3b"
    
    print("🔍 Testing AIPACK performance in isolation...")
    
    # Test 1: Direct evaluation
    print("\n--- Test 1: Direct AIPACK Evaluation ---")
    start_time = time.time()
    
    evaluator = AIPackEvaluator(model)
    code = evaluator.parse_rust_code()
    
    print("Starting AIPACK evaluation...")
    result = await evaluator.evaluate_code(code)
    
    end_time = time.time()
    total_time = end_time - start_time
    
    print(f"AIPACK Score: {result.score}/10")
    print(f"AIPACK Duration (internal): {result.duration:.2f}s")
    print(f"AIPACK Total Time (external): {total_time:.2f}s")
    
    # Test 2: Simulate multi-pipeline context
    print("\n--- Test 2: Simulated Multi-Pipeline Context ---")
    start_time = time.time()
    
    # Simulate the multi-pipeline context by creating multiple evaluators
    evaluators = []
    for i in range(3):
        evaluators.append(AIPackEvaluator(model))
    
    # Run the actual evaluation
    evaluator = evaluators[0]  # Use the first one
    code = evaluator.parse_rust_code()
    
    print("Starting AIPACK evaluation in simulated context...")
    result = await evaluator.evaluate_code(code)
    
    end_time = time.time()
    total_time = end_time - start_time
    
    print(f"AIPACK Score: {result.score}/10")
    print(f"AIPACK Duration (internal): {result.duration:.2f}s")
    print(f"AIPACK Total Time (external): {total_time:.2f}s")
    
    # Test 3: Check if there are any hanging processes
    print("\n--- Test 3: Process Check ---")
    import subprocess
    try:
        result = subprocess.run(['ps', 'aux'], capture_output=True, text=True)
        aip_processes = [line for line in result.stdout.split('\n') if 'aip' in line.lower()]
        if aip_processes:
            print("Found AIPACK processes:")
            for proc in aip_processes:
                print(f"  {proc}")
        else:
            print("No AIPACK processes found")
    except Exception as e:
        print(f"Error checking processes: {e}")

if __name__ == "__main__":
    asyncio.run(test_aipack_isolation()) 