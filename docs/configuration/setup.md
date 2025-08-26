# Setup and Configuration Guide

## System Requirements

### Minimum Requirements
- **OS**: Linux, macOS, or Windows (WSL2)
- **Python**: 3.8 or higher
- **RAM**: 8GB minimum (16GB recommended)
- **Disk Space**: 10GB for models
- **Network**: Internet connection for model downloads

### Recommended Specifications
- **CPU**: 4+ cores
- **RAM**: 32GB for large models
- **GPU**: Optional but speeds up inference
- **Storage**: SSD for better performance

## Installation

### Step 1: Install System Dependencies

#### macOS
```bash
# Install Homebrew (if not installed)
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Install Python
brew install python@3.11

# Install development tools
xcode-select --install
```

#### Linux (Ubuntu/Debian)
```bash
# Update package list
sudo apt update

# Install Python and pip
sudo apt install python3.11 python3-pip python3-venv

# Install development tools
sudo apt install build-essential curl git
```

#### Windows (WSL2)
```bash
# In WSL2 terminal
sudo apt update
sudo apt install python3.11 python3-pip python3-venv
```

### Step 2: Install uv Package Manager

```bash
# Install uv
curl -LsSf https://astral.sh/uv/install.sh | sh

# Add to PATH (if not automatically added)
export PATH="$HOME/.cargo/bin:$PATH"

# Verify installation
uv --version
```

### Step 3: Install Ollama

#### Automatic Installation
```bash
# Linux and macOS
curl -fsSL https://ollama.ai/install.sh | sh
```

#### Manual Installation
```bash
# Download from https://ollama.ai/download
# Follow platform-specific instructions
```

#### Start Ollama Service
```bash
# Start the service
ollama serve

# Verify it's running
curl http://localhost:11434/api/tags
```

### Step 4: Clone and Setup Repository

```bash
# Clone the repository
git clone https://github.com/terraphim/evaluation_pipeline_llms.git
cd evaluation_pipeline_llms

# Create virtual environment with uv
uv venv

# Install dependencies
uv pip install ollama requests

# Optional: Install all framework dependencies
uv pip install crewai aipack smolagents
```

## Configuration Files

### Pipeline Configuration

Create `config/pipeline_config.yaml`:

```yaml
# Pipeline settings
pipeline:
  max_retries: 3
  timeout_multiplier: 1.5
  parallel_execution: true
  save_intermediate: true
  
# Model settings
models:
  default: "llama3.2:3b"
  timeout_by_size:
    small: 60  # <3B parameters
    medium: 90  # 3-7B parameters
    large: 180  # 7-13B parameters
    xlarge: 300  # >13B parameters

# Data sources
data:
  local_path: "./credit_assessment_pipeline"
  atomic_server:
    base_url: "https://charm.terraphim.io"
    company_endpoint: "/charmapp7/detra-energy/account/735329000000458254"
    drs_endpoint: "/charmapp7/detra-energy/drs-assessment-record/d2025-06-05-735329000000458254"
    timeout: 30
    retry_count: 3

# Output settings
output:
  default_dir: "./credit-assessment-results"
  format: "markdown"
  include_metadata: true
  timestamp_format: "%Y-%m-%d %H:%M:%S"
```

### Ollama Configuration

Create `config/ollama_config.json`:

```json
{
  "host": "http://localhost:11434",
  "timeout": 120,
  "num_ctx": 4096,
  "temperature": 0.7,
  "top_p": 0.9,
  "repeat_penalty": 1.1,
  "seed": 42,
  "models": {
    "preferred": ["llama3.2:3b", "codellama:7b"],
    "fallback": ["qwen:1.8b", "tinyllama:1.1b"]
  }
}
```

### Environment Variables

Create `.env` file:

```bash
# Ollama settings
OLLAMA_HOST=http://localhost:11434
OLLAMA_MODELS_PATH=/usr/local/share/ollama/models
OLLAMA_TIMEOUT=120

# Atomic Server settings
ATOMIC_SERVER_URL=https://charm.terraphim.io
ATOMIC_SERVER_API_KEY=your_api_key_here  # If authentication required

# Pipeline settings
EVALUATION_LOG_LEVEL=INFO
EVALUATION_OUTPUT_DIR=./results
EVALUATION_CACHE_DIR=./cache
EVALUATION_MAX_WORKERS=4

# System settings
PYTHONPATH=${PYTHONPATH}:${PWD}
UV_SYSTEM_PYTHON=1
```

Load environment variables:

```bash
# Load in current session
source .env

# Or use direnv for automatic loading
brew install direnv  # macOS
echo 'eval "$(direnv hook bash)"' >> ~/.bashrc
direnv allow .
```

## Model Management

### Downloading Models

```bash
# Pull specific models
ollama pull llama3.2:3b
ollama pull codellama:7b
ollama pull deepseek-coder:latest
ollama pull qwen:1.8b

# List available models
ollama list

# Show model information
ollama show llama3.2:3b
```

### Model Selection Guide

| Model | Size | Best For | Speed | Quality |
|-------|------|----------|-------|---------|
| llama3.2:3b | 3GB | General tasks | Fast | Good |
| codellama:7b | 7GB | Code analysis | Medium | Excellent |
| deepseek-coder | 6GB | Technical tasks | Medium | Very Good |
| qwen:1.8b | 1.8GB | Quick tests | Very Fast | Fair |
| mistral:7b | 7GB | Reasoning | Medium | Excellent |

### Managing Model Storage

```bash
# Check model storage location
ollama list --format json | jq '.models[].size'

# Remove unused models
ollama rm model_name

# Clear model cache
rm -rf ~/.ollama/models/.cache
```

## Network Configuration

### Proxy Settings

If behind a corporate proxy:

```bash
# Set proxy environment variables
export HTTP_PROXY=http://proxy.company.com:8080
export HTTPS_PROXY=http://proxy.company.com:8080
export NO_PROXY=localhost,127.0.0.1

# Configure for Ollama
export OLLAMA_PROXY=$HTTP_PROXY
```

### Firewall Rules

Ensure these ports are open:
- **11434**: Ollama API
- **443**: HTTPS for Atomic Server
- **8080**: Optional API server

## Performance Tuning

### System Optimization

```bash
# Increase file descriptor limits (Linux/macOS)
ulimit -n 4096

# Adjust swap settings (Linux)
sudo sysctl vm.swappiness=10

# Enable huge pages (Linux)
echo 'vm.nr_hugepages=128' | sudo tee -a /etc/sysctl.conf
sudo sysctl -p
```

### Python Optimization

```bash
# Use Python optimization flags
export PYTHONOPTIMIZE=1

# Disable Python assertions
export PYTHONNODEBUGRANGES=1

# Use compiled bytecode
python -m compileall credit_assessment_pipeline/
```

### Ollama Optimization

```bash
# Set number of parallel requests
export OLLAMA_NUM_PARALLEL=2

# Adjust context window
export OLLAMA_NUM_CTX=2048

# GPU acceleration (if available)
export OLLAMA_CUDA_VISIBLE_DEVICES=0
```

## Validation

### System Check Script

Create `scripts/system_check.sh`:

```bash
#!/bin/bash

echo "=== System Check ==="

# Check Python
echo -n "Python: "
python3 --version || echo "NOT FOUND"

# Check uv
echo -n "uv: "
uv --version || echo "NOT FOUND"

# Check Ollama
echo -n "Ollama: "
ollama --version || echo "NOT FOUND"

# Check Ollama service
echo -n "Ollama service: "
curl -s http://localhost:11434/api/tags > /dev/null && echo "RUNNING" || echo "NOT RUNNING"

# Check available models
echo "Available models:"
ollama list

# Check disk space
echo "Disk space:"
df -h | grep -E "^/|Filesystem"

# Check memory
echo "Memory:"
free -h 2>/dev/null || vm_stat 2>/dev/null

echo "=== Check Complete ==="
```

Run the check:

```bash
chmod +x scripts/system_check.sh
./scripts/system_check.sh
```

## Troubleshooting Setup Issues

### Common Problems and Solutions

#### Python Version Issues
```bash
# Check Python version
python3 --version

# Install specific version with pyenv
curl https://pyenv.run | bash
pyenv install 3.11.0
pyenv global 3.11.0
```

#### uv Installation Failed
```bash
# Alternative installation method
pip install uv

# Or use pipx
pipx install uv
```

#### Ollama Won't Start
```bash
# Check if port is in use
lsof -i :11434

# Kill existing process
killall ollama

# Start with debug logging
OLLAMA_DEBUG=1 ollama serve
```

#### Permission Denied Errors
```bash
# Fix script permissions
chmod +x credit_assessment_pipeline/*.py

# Fix directory permissions
chmod -R 755 credit_assessment_pipeline/
```

## Security Configuration

### API Key Management

```python
# config/secrets.py
import os
from typing import Optional

def get_api_key(service: str) -> Optional[str]:
    """Securely retrieve API keys"""
    # Try environment variable first
    key = os.environ.get(f"{service.upper()}_API_KEY")
    
    # Fall back to secrets file
    if not key:
        try:
            with open(f".secrets/{service}.key", "r") as f:
                key = f.read().strip()
        except FileNotFoundError:
            pass
    
    return key
```

### Secure Configuration

```bash
# Create secure directories
mkdir -p .secrets
chmod 700 .secrets

# Store sensitive data
echo "your-api-key" > .secrets/atomic_server.key
chmod 600 .secrets/*.key

# Add to .gitignore
echo ".secrets/" >> .gitignore
echo ".env" >> .gitignore
```

## Docker Setup (Optional)

### Dockerfile

```dockerfile
FROM python:3.11-slim

WORKDIR /app

# Install system dependencies
RUN apt-get update && apt-get install -y \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Install uv
RUN curl -LsSf https://astral.sh/uv/install.sh | sh

# Copy requirements
COPY requirements.txt .
RUN uv pip install -r requirements.txt

# Copy application
COPY . .

CMD ["uv", "run", "python", "credit_assessment_pipeline/run_credit_assessment.py"]
```

### Docker Compose

```yaml
# docker-compose.yml
version: '3.8'

services:
  ollama:
    image: ollama/ollama:latest
    ports:
      - "11434:11434"
    volumes:
      - ollama_data:/root/.ollama
    command: serve

  evaluation:
    build: .
    depends_on:
      - ollama
    environment:
      - OLLAMA_HOST=http://ollama:11434
    volumes:
      - ./results:/app/results
    command: >
      uv run python credit_assessment_pipeline/run_credit_assessment.py
      --models llama3.2:3b
      --use-atomic

volumes:
  ollama_data:
```

## Next Steps

1. **Run First Assessment**: Follow [Getting Started Tutorial](../tutorials/getting-started.md)
2. **Configure Pipelines**: See [Pipeline Configuration](./pipeline-config.md)
3. **Optimize Performance**: Read [Performance Guide](./performance.md)
4. **Set Up Monitoring**: Check [Monitoring Setup](./monitoring.md)