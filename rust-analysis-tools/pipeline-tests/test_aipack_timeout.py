#!/usr/bin/env python3
"""
Test script to investigate AIPACK timeout issues.
"""

import asyncio
import subprocess
import tempfile
import os
from pathlib import Path

async def test_aipack_timeout():
    """Test AIPACK timeout behavior."""
    print("🔍 Testing AIPACK timeout behavior...")
    
    # Create a simple AIPACK file
    aip_content = """# Test AIPACK Timeout

You are a test agent. Please respond quickly.

What is 2+2?

## Output Format
**Answer:** [Your answer]
"""
    
    with tempfile.TemporaryDirectory() as temp_dir:
        temp_path = Path(temp_dir)
        
        # Create .aipack directory structure
        aipack_dir = temp_path / ".aipack"
        aipack_dir.mkdir()
        
        # Write config
        config_file = aipack_dir / "config.toml"
        config_content = """[options]
model = "llama3.2:3b"
"""
        with open(config_file, "w") as f:
            f.write(config_content)
        
        # Create AIPACK file
        aip_file = temp_path / "test.aip"
        with open(aip_file, "w") as f:
            f.write(aip_content)
        
        print(f"Created AIPACK file: {aip_file}")
        print(f"Config file: {config_file}")
        
        # Change to temp directory
        original_cwd = os.getcwd()
        os.chdir(temp_path)
        
        try:
            print("Starting AIPACK with 30s timeout...")
            start_time = asyncio.get_event_loop().time()
            
            # Run AIPACK with shorter timeout
            process = await asyncio.create_subprocess_exec(
                "aip", "run", "test.aip", "-s",
                stdout=asyncio.subprocess.PIPE,
                stderr=asyncio.subprocess.PIPE
            )
            
            try:
                stdout, stderr = await asyncio.wait_for(process.communicate(), timeout=30)
                end_time = asyncio.get_event_loop().time()
                
                print(f"AIPACK completed in {end_time - start_time:.2f}s")
                print(f"Return code: {process.returncode}")
                
                if process.returncode == 0:
                    print("✅ AIPACK succeeded")
                    print(f"Output: {stdout.decode('utf-8')[:200]}...")
                else:
                    print("❌ AIPACK failed")
                    print(f"Error: {stderr.decode('utf-8')}")
                    
            except asyncio.TimeoutError:
                end_time = asyncio.get_event_loop().time()
                print(f"⏰ AIPACK timed out after {end_time - start_time:.2f}s")
                process.kill()
                print("Process killed")
                
        finally:
            os.chdir(original_cwd)

if __name__ == "__main__":
    asyncio.run(test_aipack_timeout()) 