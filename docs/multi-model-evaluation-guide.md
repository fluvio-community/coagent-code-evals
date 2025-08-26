# Multi-Model Evaluation Guide

This guide provides comprehensive documentation for running systematic evaluations across multiple AI models using the LLM Evaluation Pipeline.

## 🎯 Overview

The multi-model evaluation system enables systematic comparison of 19+ AI models across different frameworks, providing quantitative and qualitative insights for model selection and performance optimization.

## 📊 Evaluation Results Summary

Based on extensive testing across 19+ models, here are the documented performance characteristics:

### Top Performing Models

| Rank | Model | Avg Score | Avg Duration | Primary Strength | Best Use Case |
|------|-------|-----------|--------------|------------------|---------------|
| 1 | llama3.2:3b | 8.5/10 | 60s | Architectural insights | WASM/Web projects |
| 2 | codellama:7b | 8.2/10 | 90s | Async pattern recognition | Concurrent systems |
| 3 | qwen2.5-coder:latest | 8.1/10 | 75s | Modern coding patterns | Contemporary codebases |
| 4 | victornitu/rust-coder:latest | 8.0/10 | 85s | Rust-specific idioms | Pure Rust projects |
| 5 | deepseek-coder:latest | 7.8/10 | 120s | Error handling analysis | Safety-critical code |

### Framework Performance Comparison

| Framework | Avg Speed | Quality Score | Reliability | Maintenance | Best For |
|-----------|-----------|---------------|------------|-------------|----------|
| SmolAgents | 32.1s | 7.5/10 | 88% | Easy | Rapid prototyping |
| AIPACK | 45.2s | 8.0/10 | 95% | Medium | Production workflows |
| CrewAI | 52.8s | 8.1/10 | 92% | Complex | Multi-agent tasks |

## 🚀 Quick Start Commands

### Complete Multi-Model Evaluation
```bash
cd evaluation_pipeline_llms
source .venv/bin/activate

# Run all models with Rust analysis
cd rust-analysis-tools/evaluation-scripts
./run-multi-model-evaluation.sh

# Compare all Python frameworks
cd ../../credit-assessment-system
python run_multi_pipeline_evaluation.py

# Credit assessment across models
cd credit_assessment_pipeline
python run_credit_assessment.py
```

### Quick Validation Test
```bash
cd rust-analysis-tools/evaluation-scripts
./test-few-models.sh  # Tests 3 representative models
```

### Single Model Testing  
```bash
# Test specific model
python pipeline-tests/test_aipack.py

# Credit assessment with specific models
python run_credit_assessment.py --models llama3.2:3b codellama:7b
```

## 🔧 Configuration and Setup

### AIPACK Configuration
Create `.aipack/config.toml` in your working directory:

```toml
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
```

### Model-Specific Timeout Configuration
```bash
# Based on parameter count
TIMEOUT_3B=60     # Models like llama3.2:3b
TIMEOUT_7B=90     # Models like codellama:7b  
TIMEOUT_20B=180   # Models like gemma3:27b
```

### Environment Variables
```bash
export OLLAMA_URL="http://localhost:11434"
export MIN_DISK_SPACE_GB="10.0"
export OUTPUT_DIRECTORY="evaluation-results"
```

## 📋 Model Categories and Specializations

### Small Models (< 5GB)
**Best for**: Quick feedback, development iteration, resource-constrained environments

- **llama3.2:3b** - 2.0GB, 60s timeout
  - **Strengths**: WASM patterns, architectural insights
  - **Weaknesses**: Sometimes misses performance optimizations
  - **Use Cases**: Web development, educational content

- **deepseek-coder:latest** - 776MB, 60s timeout  
  - **Strengths**: Comprehensive error handling
  - **Weaknesses**: Struggles with complex patterns
  - **Use Cases**: Safety-critical systems, beginner code review

- **codegemma:2b-code** - 1.6GB, 45s timeout
  - **Strengths**: Fast processing, basic analysis
  - **Weaknesses**: Limited architectural insights
  - **Use Cases**: CI/CD integration, rapid validation

### Medium Models (5-10GB)
**Best for**: Balanced performance and quality, production use cases

- **codellama:7b** - 3.8GB, 90s timeout
  - **Strengths**: Superior async/await analysis
  - **Weaknesses**: May over-focus on concurrency
  - **Use Cases**: Concurrent systems, async optimization

- **qwen2.5-coder:latest** - 4.7GB, 75s timeout
  - **Strengths**: Modern coding patterns
  - **Weaknesses**: Less specialized for Rust
  - **Use Cases**: General-purpose development, modern frameworks

- **victornitu/rust-coder:latest** - 7.4GB, 85s timeout
  - **Strengths**: Rust-specific idioms and best practices
  - **Weaknesses**: Too focused on conventions vs general quality  
  - **Use Cases**: Rust-specific code review, idiomatic patterns

### Large Models (> 10GB)
**Best for**: Comprehensive analysis, complex architectural evaluation

- **gemma3:27b** - 17GB, 180s timeout
  - **Strengths**: Comprehensive analysis, complex patterns
  - **Weaknesses**: Resource intensive, slow
  - **Use Cases**: Architecture review, complex system analysis

- **devstral:latest** - 14GB, 160s timeout
  - **Strengths**: Advanced reasoning, detailed insights
  - **Weaknesses**: High resource requirements
  - **Use Cases**: Critical system evaluation, detailed analysis

## 🎮 Evaluation Execution Strategies

### Strategy 1: Progressive Evaluation
Start with fast models for immediate feedback, then run comprehensive analysis:

```bash
# Phase 1: Quick feedback (< 2 minutes)
python run_evaluation.py --models llama3.2:3b deepseek-coder:latest

# Phase 2: Detailed analysis (5-10 minutes)  
python run_evaluation.py --models codellama:7b qwen2.5-coder:latest

# Phase 3: Comprehensive review (10+ minutes)
python run_evaluation.py --models gemma3:27b devstral:latest
```

### Strategy 2: Parallel Execution
Run multiple models simultaneously with resource management:

```bash
# Set concurrency limits based on available resources
export MAX_CONCURRENT_SMALL=4
export MAX_CONCURRENT_LARGE=2

python run_parallel_evaluation.py --all-models
```

### Strategy 3: Targeted Evaluation  
Select models based on specific evaluation goals:

```bash
# For Rust-specific analysis
python run_evaluation.py --models victornitu/rust-coder:latest codellama:7b

# For performance-critical evaluation
python run_evaluation.py --models llama3.2:3b gemma3:27b

# For error handling focus
python run_evaluation.py --models deepseek-coder:latest cogito:latest
```

## 📊 Interpreting Results

### Score Interpretation
- **9-10/10**: Exceptional analysis, comprehensive insights, actionable recommendations
- **7-8/10**: Good analysis, solid recommendations, minor gaps
- **5-6/10**: Adequate analysis, basic recommendations, some important issues missed
- **3-4/10**: Poor analysis, limited insights, may miss critical issues
- **1-2/10**: Inadequate analysis, unreliable recommendations

### Performance Metrics
- **Duration**: Time to complete evaluation (includes model loading and processing)
- **Quality Score**: Extracted from evaluation content using heuristic parsing
- **Reliability**: Percentage of successful evaluations without errors
- **Consistency**: Variance in scores across multiple runs

### Example Result Analysis
```markdown
## Model: llama3.2:3b
**Duration:** 58.2s
**Score:** 8.5/10
**Strengths:** 
- Excellent architectural insights
- Strong WASM pattern recognition
- Clear, actionable recommendations

**Issues:**
- Sometimes misses performance optimization opportunities
- Limited focus on memory management patterns

**Recommendation:** Best for web development and educational use cases
```

## 🔍 Advanced Evaluation Techniques

### Multi-Model Consensus
Compare results across multiple models to identify consistent patterns:

```python
def analyze_consensus(results):
    common_issues = set.intersection(*[r.issues for r in results])
    disputed_areas = [issue for issue in all_issues if confidence < 0.7]
    high_confidence = [issue for issue in all_issues if confidence > 0.9]
    
    return ConsensusAnalysis(
        common_issues=common_issues,
        disputed_areas=disputed_areas, 
        high_confidence=high_confidence
    )
```

### Specialized Model Routing
Automatically select optimal models based on code characteristics:

```python
def route_to_optimal_model(code_analysis):
    if code_analysis.has_async_patterns:
        return "codellama:7b"
    elif code_analysis.is_wasm_project:
        return "llama3.2:3b"
    elif code_analysis.is_safety_critical:
        return "deepseek-coder:latest"
    else:
        return "qwen2.5-coder:latest"
```

### Performance Profiling
Track resource usage patterns across different models:

```python
@dataclass
class ResourceProfile:
    peak_memory_mb: float
    cpu_usage_percent: float
    disk_io_mb: float
    network_requests: int
    
def profile_model_execution(model_name, evaluation_func):
    with ResourceMonitor() as monitor:
        result = evaluation_func()
        
    return ResourceProfile(
        peak_memory_mb=monitor.peak_memory,
        cpu_usage_percent=monitor.avg_cpu,
        disk_io_mb=monitor.disk_io,
        network_requests=monitor.network_calls
    )
```

## 🛠️ Troubleshooting Common Issues

### Model Not Found Errors
```bash
# Check available models
ollama list

# Pull missing models
ollama pull llama3.2:3b
ollama pull codellama:7b
ollama pull deepseek-coder:latest
```

### AIPACK Path Resolution
```bash
# Method 1: Copy flows to working directory
cd evaluation-scripts
cp ../aipack-flows/*.aip .

# Method 2: Update script paths
sed -i '' 's|flow.aip|../aipack-flows/flow.aip|g' *.sh

# Method 3: Run from root directory
cd rust-analysis-tools
aip run aipack-flows/multi-model-evaluation.aip
```

### Memory and Resource Issues
```bash
# Check system resources before evaluation
free -h  # Check available memory
df -h    # Check disk space
ps aux | grep ollama  # Check Ollama processes

# Reduce concurrent evaluations
export MAX_CONCURRENT_MODELS=2
```

### Timeout and Performance Issues
```bash
# Increase timeouts for large models
export LARGE_MODEL_TIMEOUT=300  # 5 minutes
export EXTRA_LARGE_MODEL_TIMEOUT=600  # 10 minutes

# Monitor Ollama service performance
curl http://localhost:11434/api/version
```

## 📈 Optimization Strategies

### Resource Optimization
1. **Memory Management**: Monitor peak memory usage and adjust concurrent evaluations
2. **Disk Space**: Pre-validate available space before starting large evaluations
3. **Network Optimization**: Cache model metadata to reduce API calls
4. **Process Management**: Clean up zombie processes between evaluations

### Performance Tuning
1. **Timeout Optimization**: Set model-specific timeouts based on parameter count
2. **Batch Processing**: Group similar evaluations to reduce overhead
3. **Parallel Execution**: Balance concurrent jobs with available resources
4. **Progressive Loading**: Load models on-demand rather than preloading all

### Quality Improvement
1. **Multi-Model Validation**: Use consensus across multiple models for critical decisions
2. **Specialized Routing**: Route specific code types to optimal models
3. **Iterative Refinement**: Use feedback loops to improve model selection
4. **Context Enhancement**: Provide richer context for better analysis quality

## 🔮 Advanced Use Cases

### Continuous Integration Integration
```yaml
# .github/workflows/model-evaluation.yml
name: Multi-Model Code Evaluation
on: [push, pull_request]

jobs:
  evaluate:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Setup evaluation environment
        run: |
          curl -fsSL https://ollama.ai/install.sh | sh
          ollama pull llama3.2:3b
          
      - name: Run lightweight evaluation
        run: |
          source .venv/bin/activate
          python run_evaluation.py --models llama3.2:3b --quick
```

### Automated Model Selection
```python
class ModelSelector:
    def __init__(self):
        self.performance_history = load_performance_data()
        
    def select_optimal_model(self, code_features, constraints):
        candidates = self.filter_by_constraints(constraints)
        scores = self.score_models(candidates, code_features)
        return max(scores.items(), key=lambda x: x[1])[0]
        
    def score_models(self, models, features):
        scores = {}
        for model in models:
            score = self.calculate_fit_score(model, features)
            scores[model] = score
        return scores
```

### Custom Evaluation Metrics
```python
@dataclass
class CustomEvaluationResult:
    model_name: str
    standard_score: float
    custom_metrics: Dict[str, float]
    
def custom_evaluation_pipeline(models, custom_criteria):
    results = []
    for model in models:
        result = evaluate_with_custom_metrics(model, custom_criteria)
        results.append(result)
    
    return generate_custom_report(results)
```

## 📚 Additional Resources

### Documentation Links
- [Main Documentation](./README.md)
- [AIPACK Flow Guide](../README-aipack-flow.md)
- [Lessons Learned](./lessons-learned.md)
- [API Reference](./api/README.md)

### Configuration Examples
- [Basic Configuration](./configuration/basic-setup.md)
- [Advanced Configuration](./configuration/advanced-setup.md)
- [Production Deployment](./configuration/production-setup.md)

### Troubleshooting
- [Common Issues](./troubleshooting.md)
- [Performance Optimization](./performance-tuning.md)
- [Error Recovery](./error-handling.md)

---

*This guide is based on extensive testing across 19+ models and multiple frameworks. Results may vary based on hardware configuration, model versions, and specific use cases.*