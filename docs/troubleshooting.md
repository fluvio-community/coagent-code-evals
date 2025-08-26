# Troubleshooting Guide

This comprehensive guide addresses common issues encountered when using the LLM Evaluation Pipeline, based on real operational experience and documented solutions.

## 🚨 Critical Issues - Immediate Solutions

### System Prerequisites

#### Virtual Environment Not Activated
```
Command 'python' not found or ModuleNotFoundError
```
**Solution - Always Required:**
```bash
# MUST be done first before any Python work
source .venv/bin/activate

# Verify activation (should show project path)
which python
```

#### Ollama Service Not Running
```
❌ Connection refused to localhost:11434
❌ Ollama service is not accessible
```
**Solutions:**
```bash
# Check if Ollama is running
curl http://localhost:11434/api/version

# Start Ollama service
ollama serve

# Verify service status
ollama list

# If stuck, restart service
pkill ollama && sleep 5 && ollama serve
```

## 🔧 Configuration Issues

### AIPACK Flow Path Problems

#### "No agent found for local path"
```
No agent found for local path: 'multi-model-evaluation.aip'
(full path: /path/to/multi-model-evaluation.aip)
```

**Root Cause:** AIPACK flows are in `aipack-flows/` directory but scripts look in current directory.

**Solutions (3 methods):**

```bash
# Method 1: Copy flows to working directory
cd rust-analysis-tools/evaluation-scripts
cp ../aipack-flows/*.aip .
./run-multi-model-evaluation.sh

# Method 2: Update script paths dynamically
sed -i '' 's|multi-model-evaluation.aip|../aipack-flows/multi-model-evaluation.aip|g' run-multi-model-evaluation.sh
sed -i '' 's|rust-evaluator.aip|../aipack-flows/rust-evaluator.aip|g' test-few-models.sh

# Method 3: Run from root directory with absolute paths
cd rust-analysis-tools
aip run aipack-flows/multi-model-evaluation.aip -s
```

#### Missing AIPACK Configuration
```
❌ AIPACK configuration is invalid - fix configuration before proceeding
```

**Solution:**
```bash
# Create .aipack directory in working directory (NOT in project root)
mkdir -p .aipack

# Create proper configuration file
cat > .aipack/config.toml << 'EOF'
# AIPACK Configuration for Rust Code Evaluation Flow

[options]
model = "llama3.2:3b"  # Default model

[options.model_aliases]
# Core models for Rust evaluation
llama3 = "llama3.2:3b"
codellama = "codellama:7b"
deepseek = "deepseek-coder:latest"

# Specialized coding models
rust-coder = "victornitu/rust-coder:latest"
cogito = "cogito:latest"
qwen25-coder = "qwen2.5-coder:latest"

# Large language models
gemma3 = "gemma3:27b"
qwen3 = "qwen3:latest"
qwen3-8b = "qwen3:8b"
EOF

# Verify configuration
cat .aipack/config.toml
```

## 🤖 Model-Related Issues

### Model Availability Problems

#### Model Not Found
```
❌ Model 'model-name:tag' not found
❌ Error: model 'model-name' not found, try pulling it first
```

**Diagnosis:**
```bash
# Check currently available models
ollama list

# Check if model exists on Ollama registry
ollama search model-name
```

**Solutions:**
```bash
# Pull missing models (most common ones)
ollama pull llama3.2:3b        # 2.0GB download, 4GB RAM
ollama pull codellama:7b        # 3.8GB download, 6GB RAM
ollama pull deepseek-coder:latest  # 776MB download, 2GB RAM
ollama pull qwen2.5-coder:latest   # 4.7GB download, 7GB RAM
ollama pull victornitu/rust-coder:latest  # 7.4GB download, 8GB RAM

# For large models (ensure sufficient resources)
ollama pull gemma3:27b          # 17GB download, 20GB RAM
ollama pull devstral:latest     # 14GB download, 16GB RAM
```

#### Model Loading/Timeout Issues
```
❌ Model evaluation timed out after 60s
❌ Context deadline exceeded
```

**Root Cause:** Default timeouts too short for larger models.

**Solutions:**

```bash
# Set model-specific timeouts based on size
# Small models (< 5GB): 60s
# Medium models (5-10GB): 90s  
# Large models (> 10GB): 180s

# Method 1: Environment variables
export SMALL_MODEL_TIMEOUT=60
export MEDIUM_MODEL_TIMEOUT=90
export LARGE_MODEL_TIMEOUT=180

# Method 2: Model-specific configuration
# In your scripts, use parameter count to determine timeout:
# 3B params -> 60s, 7B params -> 90s, 20B+ params -> 180s
```

## 💾 Resource and Performance Issues

### Memory Problems

#### Out of Memory Errors
```
❌ CUDA out of memory
❌ System out of memory
❌ Process killed (OOMKilled)
```

**Diagnosis:**
```bash
# Check available memory
free -h

# Check running processes
ps aux | grep ollama
ps aux | grep python

# Monitor resource usage during evaluation
htop  # or top
```

**Solutions (in order of preference):**

```bash
# 1. Reduce concurrent model evaluations
export MAX_CONCURRENT_MODELS=1

# 2. Use smaller models
python run_evaluation.py --models llama3.2:3b deepseek-coder:latest

# 3. Sequential processing instead of parallel
python run_evaluation.py --sequential --models codellama:7b

# 4. Increase system swap (temporary fix)
sudo fallocate -l 4G /swapfile
sudo chmod 600 /swapfile
sudo mkswap /swapfile
sudo swapon /swapfile

# 5. Close other applications
# Close browsers, IDEs, unnecessary applications
```

### Disk Space Issues

#### Insufficient Disk Space
```
❌ Insufficient disk space for evaluation outputs
❌ No space left on device
```

**Diagnosis:**
```bash
# Check available disk space
df -h

# Check directory sizes
du -sh evaluation-results/
du -sh .ollama/  # Ollama model storage
du -sh .venv/    # Python packages
```

**Solutions:**
```bash
# 1. Clean up old evaluation results
find evaluation-results/ -name "*.md" -mtime +7 -delete
find multi-model-results-*/ -mtime +14 -delete

# 2. Remove unused Ollama models
ollama rm unused-model-name

# 3. Clean Python cache
find . -name "__pycache__" -type d -exec rm -rf {} +
find . -name "*.pyc" -delete

# 4. Clean Rust build artifacts
cargo clean
rm -rf target/

# 5. Move evaluation results to external storage
mv evaluation-results/ /path/to/external/storage/
ln -s /path/to/external/storage/evaluation-results/ .
```

### Network and Connectivity Issues

#### Atomic Server Connection Problems
```
❌ Failed to fetch data from https://charm.terraphim.io/
❌ Connection timeout
❌ SSL certificate verification failed
```

**Diagnosis:**
```bash
# Test basic connectivity
curl -I https://charm.terraphim.io/

# Test specific endpoints
curl -H "Accept: application/json" \
  https://charm.terraphim.io/charmapp7/detra-energy/account/735329000000458254

# Check SSL certificates
curl -I --ssl-no-revoke https://charm.terraphim.io/
```

**Solutions:**

```bash
# 1. Use fallback to local files (automatic)
# System automatically falls back when server unavailable

# 2. Bypass SSL verification (temporary, for testing only)
export PYTHONHTTPSVERIFY=0
# OR
python -c "import ssl; ssl._create_default_https_context = ssl._create_unverified_context"

# 3. Use different data format
python run_credit_assessment.py --format json  # Instead of json-ad

# 4. Test with local data only
python run_credit_assessment.py --models llama3.2:3b  # No --use-atomic flag
```

## 🐍 Python Environment Issues

### Import Errors

#### Missing Python Packages
```
ModuleNotFoundError: No module named 'ollama'
ModuleNotFoundError: No module named 'crewai'
ModuleNotFoundError: No module named 'smolagents'
```

**Solution:**
```bash
# Always ensure virtual environment is activated first
source .venv/bin/activate

# Install missing packages
pip install ollama
pip install crewai
pip install smolagents
pip install requests
pip install rich

# Or install from requirements
pip install -r requirements.txt

# Verify installations
pip list | grep ollama
pip list | grep crewai
```

#### Python Path Issues
```
ImportError: attempted relative import with no known parent package
```

**Solutions:**
```bash
# Method 1: Run from correct directory
cd credit-assessment-system/credit_assessment_pipeline
python run_credit_assessment.py

# Method 2: Update Python path in scripts (already implemented)
import sys
from pathlib import Path
sys.path.append(str(Path(__file__).parent.parent.parent / "credit-assessment-system"))
```

### Framework-Specific Issues

#### CrewAI Framework Problems
```
WARNING: pkg_resources is deprecated
DeprecationWarning: Deprecated call to pkg_resources.declare_namespace
```

**Solution (Informational - these are warnings, not errors):**
```bash
# These warnings don't affect functionality
# To suppress warnings:
export PYTHONWARNINGS="ignore::DeprecationWarning"
python your_script.py
```

## 🏃‍♀️ Performance and Optimization Issues

### Slow Performance

#### Models Taking Too Long
```
⏰ Model evaluation taking > 5 minutes
⏰ System appears frozen
```

**Diagnosis:**
```bash
# Check if model is actually running
ps aux | grep ollama

# Monitor resource usage
htop
nvidia-smi  # If using GPU

# Check Ollama logs
journalctl -u ollama -f
```

**Solutions:**

```bash
# 1. Use faster models for development
python run_evaluation.py --models llama3.2:3b deepseek-coder:latest

# 2. Optimize model selection based on use case
# Quick validation: llama3.2:3b (60s avg)
# Balanced analysis: codellama:7b (90s avg)
# Comprehensive review: gemma3:27b (180s avg)

# 3. Progressive evaluation strategy
# Run fast model first, then comprehensive analysis
python run_evaluation.py --models llama3.2:3b && \
python run_evaluation.py --models codellama:7b
```

#### High CPU/Memory Usage
```
⚠️ System load average > 8.0
⚠️ Memory usage > 90%
```

**Solutions:**
```bash
# 1. Limit concurrent processes
export OMP_NUM_THREADS=2
export MAX_CONCURRENT_MODELS=1

# 2. Use nice/ionice for background processing
nice -n 19 ionice -c3 python run_evaluation.py

# 3. Monitor and kill runaway processes
ps aux | grep python | grep -v grep
kill -9 PID_OF_RUNAWAY_PROCESS
```

## 📊 Output and Quality Issues

### Missing or Incomplete Results

#### No Output Files Generated
```
❌ Expected output file not found
❌ Evaluation completed but no results
```

**Diagnosis:**
```bash
# Check output directory
ls -la evaluation-results/
ls -la credit-assessment-results/
ls -la multi-model-results-*/

# Check permissions
ls -ld evaluation-results/

# Check for error logs
find . -name "*.log" -mtime -1
tail -50 error.log 2>/dev/null || echo "No error log found"
```

**Solutions:**
```bash
# 1. Fix directory permissions
chmod 755 evaluation-results/
chmod 644 evaluation-results/*.md

# 2. Check disk space (covered above)
df -h

# 3. Manually create output directory
mkdir -p evaluation-results
mkdir -p credit-assessment-results

# 4. Run with explicit output path
python run_evaluation.py --output-dir ./my-results/
```

#### Poor Quality Results
```
❌ Generated assessment is incomplete
❌ Score extraction failed
❌ Results contain errors or nonsense
```

**Diagnosis:**
```bash
# Check model selection
ollama list | grep -E "(llama|codellama|deepseek|qwen)"

# Verify input data quality
python -m json.tool company_information.json
python -m json.tool drs_score.json
```

**Solutions:**
```bash
# 1. Use higher quality models
python run_evaluation.py --models codellama:7b gemma3:27b

# 2. Improve input data quality
# Ensure JSON files are well-formed and complete

# 3. Use semantic data formats
python run_credit_assessment.py --format json-ad --use-atomic

# 4. Run multiple models for consensus
python run_evaluation.py --models llama3.2:3b codellama:7b qwen2.5-coder:latest

# 5. Increase model timeout for better results
export MODEL_TIMEOUT=180  # 3 minutes
```

## 🔍 Debugging and Diagnostics

### Enable Debug Mode

#### Comprehensive Logging
```bash
# Enable debug logging for Python scripts
export PYTHONVERBOSE=1
export DEBUG=1

# Run with verbose output
python -v run_evaluation.py --models llama3.2:3b

# Enable logging in scripts
import logging
logging.basicConfig(level=logging.DEBUG)
```

#### Ollama Debug Information
```bash
# Check Ollama service status
ollama ps
ollama list

# Test model directly
ollama run llama3.2:3b "Hello, test message"

# Check Ollama logs (system-dependent)
# macOS: /usr/local/var/log/ollama.log
# Linux: journalctl -u ollama
# Docker: docker logs ollama-container
```

### System Information Collection
```bash
# Collect system info for bug reports
echo "=== System Information ===" > debug_info.txt
uname -a >> debug_info.txt
python --version >> debug_info.txt
which python >> debug_info.txt
pip list >> debug_info.txt

echo "=== Ollama Information ===" >> debug_info.txt
ollama --version >> debug_info.txt
ollama list >> debug_info.txt
curl http://localhost:11434/api/version >> debug_info.txt

echo "=== Resource Usage ===" >> debug_info.txt
free -h >> debug_info.txt
df -h >> debug_info.txt
ps aux | grep -E "(ollama|python)" >> debug_info.txt
```

## 🚀 Prevention and Best Practices

### Pre-flight Validation

#### Always Run Validation First
```bash
cd rust-analysis-tools
cargo run --bin validation-cli

# Sample output:
# ✅ OVERALL STATUS: PASSED
# 📈 SUMMARY:
#   Available Models: 5
#   Available Disk Space: 45.23 GB
#   Ollama Response Time: 145ms
```

#### Automated Health Checks
```bash
#!/bin/bash
# health_check.sh - Run before any evaluation

set -e

echo "🔍 Running pre-flight health checks..."

# 1. Check virtual environment
if [[ "$VIRTUAL_ENV" == "" ]]; then
    echo "❌ Virtual environment not activated"
    echo "Run: source .venv/bin/activate"
    exit 1
fi

# 2. Check Ollama service
if ! curl -s http://localhost:11434/api/version > /dev/null; then
    echo "❌ Ollama service not running"
    echo "Run: ollama serve"
    exit 1
fi

# 3. Check disk space (need at least 5GB)
AVAILABLE_GB=$(df . | awk 'NR==2 {printf "%.0f", $4/1024/1024}')
if [ "$AVAILABLE_GB" -lt 5 ]; then
    echo "❌ Insufficient disk space ($AVAILABLE_GB GB available, need 5GB)"
    exit 1
fi

# 4. Check essential models
ESSENTIAL_MODELS="llama3.2:3b codellama:7b"
for model in $ESSENTIAL_MODELS; do
    if ! ollama list | grep -q "$model"; then
        echo "⚠️  Essential model $model not found"
        echo "Run: ollama pull $model"
    fi
done

echo "✅ Health checks passed"
```

### Error Recovery Patterns

#### Robust Retry Logic
```python
import asyncio
import logging
from typing import Optional

async def robust_evaluation(model: str, max_retries: int = 3) -> Optional[dict]:
    """Run evaluation with exponential backoff retry logic."""
    
    for attempt in range(max_retries):
        try:
            result = await run_model_evaluation(model)
            return result
            
        except (ConnectionError, TimeoutError) as e:
            if attempt == max_retries - 1:
                logging.error(f"Failed to evaluate {model} after {max_retries} attempts: {e}")
                return None
                
            wait_time = 2 ** attempt
            logging.warning(f"Retry {attempt + 1} for {model} after {wait_time}s")
            await asyncio.sleep(wait_time)
            
        except Exception as e:
            logging.error(f"Unexpected error evaluating {model}: {e}")
            return None
    
    return None
```

## 📞 Getting Help

### Documentation Resources
- [Main Documentation](./README.md) - System overview and quick start
- [Multi-Model Evaluation Guide](./multi-model-evaluation-guide.md) - Comprehensive evaluation patterns
- [Lessons Learned](./lessons-learned.md) - Development insights and patterns
- [Credit Assessment Pipeline](./pipelines/credit-assessment.md) - Specific pipeline documentation

### Community Support
- **GitHub Issues**: Report bugs and request features
- **Discussions**: Ask questions and share experiences
- **Wiki**: Community-maintained knowledge base

### Professional Support
For production deployments and enterprise use:
- **Consulting**: Architecture review and optimization
- **Training**: Team onboarding and best practices
- **Custom Development**: Feature development and integration

---

*This troubleshooting guide is continuously updated based on user feedback and operational experience. Last updated: 2025-08-26*