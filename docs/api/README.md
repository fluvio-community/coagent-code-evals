# API Reference

## Overview

This section provides detailed API documentation for all modules and functions in the LLM Evaluation Pipeline system.

## Modules

- [Credit Assessment API](./credit-assessment-api.md)
- [Evaluation Framework API](./evaluation-framework-api.md)
- [Data Fetching API](./data-fetching-api.md)
- [Validation API](./validation-api.md)

## Core Components

### Credit Assessment Pipeline

```python
from credit_assessment_pipeline import run_credit_assessment

# Main evaluation function
run_credit_assessment(
    models: List[str],
    output_dir: Path,
    data_format: str = 'json',
    use_atomic: bool = False
)
```

### Multi-Framework Evaluator

```python
from evaluation_pipelines import MultiFrameworkEvaluator

evaluator = MultiFrameworkEvaluator()
results = evaluator.evaluate(
    framework: str,
    model: str,
    task: str
)
```

## Quick Reference

### Import Statements

```python
# Credit Assessment
from credit_assessment_pipeline.run_credit_assessment import (
    run_credit_assessment,
    fetch_data_from_atomic_server,
    load_data_source
)

# Framework Evaluation
from evaluation_pipelines.base_evaluator import BaseEvaluator
from evaluation_pipelines.aipack_eval import AIPACKEvaluator
from evaluation_pipelines.crewai_eval import CrewAIEvaluator
from evaluation_pipelines.smolagents_eval import SmolAgentsEvaluator

# Utilities
from pathlib import Path
import json
import time
```

### Common Patterns

#### Running Evaluations

```python
# Basic evaluation
models = ["llama3.2:3b"]
output_dir = Path("./results")
run_credit_assessment(models, output_dir)

# With Atomic Server
run_credit_assessment(
    models,
    output_dir,
    data_format="json-ad",
    use_atomic=True
)
```

#### Error Handling

```python
try:
    results = run_credit_assessment(models, output_dir)
except Exception as e:
    print(f"Evaluation failed: {e}")
    # Log error details
    with open("error_log.txt", "a") as f:
        f.write(f"{time.strftime('%Y-%m-%d %H:%M:%S')}: {e}\n")
```

## Type Definitions

```python
from typing import Dict, List, Any, Optional, Union
from pathlib import Path

# Common type aliases
ModelName = str
TaskName = str
FrameworkName = str
DataFormat = Literal["json", "json-ad", "turtle"]

# Result types
EvaluationResult = Dict[str, Any]
MetricsDict = Dict[str, float]
ConfigDict = Dict[str, Any]
```

## Return Value Schemas

### Evaluation Result

```json
{
  "model": "llama3.2:3b",
  "framework": "aipack",
  "task": "credit_assessment",
  "metrics": {
    "execution_time": 45.2,
    "quality_score": 8.5,
    "token_count": 1523
  },
  "output": "...",
  "timestamp": "2025-08-18T10:30:00Z"
}
```

### Metrics Response

```json
{
  "performance": {
    "speed": 45.2,
    "memory": 512.3,
    "cpu_usage": 65.4
  },
  "quality": {
    "accuracy": 0.92,
    "completeness": 0.88,
    "consistency": 0.95
  }
}
```

## Environment Variables

```bash
# Ollama configuration
OLLAMA_HOST=http://localhost:11434
OLLAMA_TIMEOUT=120

# Atomic Server configuration
ATOMIC_SERVER_URL=https://charm.terraphim.io
ATOMIC_SERVER_TIMEOUT=30

# Pipeline configuration
EVALUATION_OUTPUT_DIR=./results
EVALUATION_LOG_LEVEL=INFO
EVALUATION_MAX_RETRIES=3
```

## Command-Line Interface

All scripts support command-line execution via `uv run`:

```bash
# General pattern
uv run python <script_path> [options]

# Examples
uv run python credit_assessment_pipeline/run_credit_assessment.py --help
uv run python evaluation_pipelines/run_evaluation.py --help
```

## Error Codes

| Code | Description | Resolution |
|------|-------------|------------|
| E001 | Model not found | Pull model with `ollama pull` |
| E002 | Network timeout | Increase timeout or retry |
| E003 | Invalid data format | Check format parameter |
| E004 | Atomic Server unreachable | Check network/use local files |
| E005 | Insufficient memory | Use smaller model or increase RAM |
| E006 | Framework not installed | Install with `uv pip install` |

## Logging

Configure logging for debugging:

```python
import logging

# Set up logging
logging.basicConfig(
    level=logging.DEBUG,
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s',
    handlers=[
        logging.FileHandler('evaluation.log'),
        logging.StreamHandler()
    ]
)

logger = logging.getLogger(__name__)
```

## Performance Considerations

### Recommended Settings

```python
# Optimal configuration for different model sizes
CONFIG = {
    "3b_models": {
        "timeout": 60,
        "max_tokens": 2000,
        "batch_size": 5
    },
    "7b_models": {
        "timeout": 90,
        "max_tokens": 3000,
        "batch_size": 3
    },
    "13b_models": {
        "timeout": 180,
        "max_tokens": 4000,
        "batch_size": 1
    }
}
```

## Extending the API

### Creating Custom Evaluators

```python
from evaluation_pipelines.base_evaluator import BaseEvaluator

class CustomEvaluator(BaseEvaluator):
    def setup(self, config: Dict[str, Any]) -> None:
        # Custom initialization
        pass
    
    def evaluate(self, task: str, model: str) -> Dict[str, Any]:
        # Custom evaluation logic
        pass
    
    def get_metrics(self) -> Dict[str, float]:
        # Custom metrics calculation
        pass
```

### Adding New Data Formats

```python
def custom_format_handler(data: str, format_type: str) -> str:
    """Handle custom data formats"""
    if format_type == "custom":
        # Process custom format
        return processed_data
    return data
```

## Version Information

Current API version: 1.0.0

### Compatibility

- Python: 3.8+
- Ollama: 0.1.0+
- uv: 0.1.0+

### Breaking Changes

See [CHANGELOG.md](../../CHANGELOG.md) for version history and breaking changes.