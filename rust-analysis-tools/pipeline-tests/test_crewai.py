#!/usr/bin/env python3
"""
Simple test script for CrewAI pipeline.
"""

import asyncio
import sys
from pathlib import Path
sys.path.append(str(Path(__file__).parent.parent.parent / "credit-assessment-system"))
from evaluation_pipelines import CrewAIEvaluator

async def test_crewai():
    """Test CrewAI pipeline."""
    model = "llama3.2:3b"
    
    print("🧪 Testing CrewAI pipeline...")
    
    try:
        print("Creating CrewAI evaluator...")
        evaluator = CrewAIEvaluator(model)
        
        print("Parsing Rust code...")
        code = evaluator.parse_rust_code()
        print("Code parsed successfully")
        
        print("Running CrewAI evaluation...")
        result = await evaluator.evaluate_code(code)
        
        print(f"CrewAI Score: {result.score}/10")
        print(f"CrewAI Duration: {result.duration:.2f}s")
        print(f"CrewAI Primary Recommendation: {result.primary_recommendation}")
        
        print("✅ CrewAI test completed successfully!")
        
    except Exception as e:
        print(f"❌ CrewAI Error: {e}")
        import traceback
        traceback.print_exc()

if __name__ == "__main__":
    asyncio.run(test_crewai()) 