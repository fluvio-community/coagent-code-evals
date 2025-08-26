# Getting Started Tutorial

## Prerequisites

Before starting, ensure you have:

1. **Python 3.8+** installed
2. **uv** package manager installed
3. **Ollama** installed and running
4. At least one Ollama model pulled

### Installing Prerequisites

```bash
# Install uv (if not already installed)
curl -LsSf https://astral.sh/uv/install.sh | sh

# Install Ollama
curl -fsSL https://ollama.ai/install.sh | sh

# Start Ollama service
ollama serve

# Pull a model (in another terminal)
ollama pull llama3.2:3b
```

## Step 1: Setting Up the Environment

### Clone the Repository

```bash
git clone https://github.com/terraphim/evaluation_pipeline_llms.git
cd evaluation_pipeline_llms
```

### Create Virtual Environment

```bash
# uv automatically creates and manages the virtual environment
uv venv
```

### Install Dependencies

```bash
# Install required packages
uv pip install ollama requests

# Optional: Install framework dependencies
uv pip install crewai aipack smolagents
```

## Step 2: Your First Credit Assessment

### Run a Simple Assessment

```bash
# Run credit assessment with one model
uv run python credit_assessment_pipeline/run_credit_assessment.py \
  --models llama3.2:3b
```

This will:
1. Load local company and DRS data
2. Generate a prompt from the template
3. Send it to the LLM
4. Save the assessment to `credit-assessment-results/`

### View the Results

```bash
# List generated reports
ls credit-assessment-results/

# View a specific report
cat credit-assessment-results/llama3.2_3b-json-credit-assessment.md
```

## Step 3: Using Different Data Formats

### JSON Format (Default)

```bash
uv run python credit_assessment_pipeline/run_credit_assessment.py \
  --models llama3.2:3b \
  --format json
```

### JSON-AD Format

```bash
uv run python credit_assessment_pipeline/run_credit_assessment.py \
  --models llama3.2:3b \
  --format json-ad
```

### Turtle/RDF Format

```bash
uv run python credit_assessment_pipeline/run_credit_assessment.py \
  --models llama3.2:3b \
  --format turtle
```

## Step 4: Fetching Data from Atomic Server

### Enable Atomic Server Integration

```bash
uv run python credit_assessment_pipeline/run_credit_assessment.py \
  --models llama3.2:3b \
  --format json \
  --use-atomic
```

### Try Different Formats with Atomic Server

```bash
# JSON-AD format from Atomic Server
uv run python credit_assessment_pipeline/run_credit_assessment.py \
  --models llama3.2:3b \
  --format json-ad \
  --use-atomic

# Turtle format from Atomic Server
uv run python credit_assessment_pipeline/run_credit_assessment.py \
  --models llama3.2:3b \
  --format turtle \
  --use-atomic
```

## Step 5: Running Multiple Models

### Evaluate Specific Models

```bash
uv run python credit_assessment_pipeline/run_credit_assessment.py \
  --models llama3.2:3b codellama:7b qwen:1.8b \
  --format json \
  --use-atomic
```

### Evaluate All Available Models

```bash
# This will automatically discover and run all Ollama models
uv run python credit_assessment_pipeline/run_credit_assessment.py \
  --use-atomic
```

## Step 6: Generating Summary Reports

### Create a Summary

After running multiple models, generate a summary:

```bash
uv run python credit_assessment_pipeline/create_summary.py
```

This creates a consolidated report comparing all model performances.

## Step 7: Custom Output Directory

### Specify Output Location

```bash
uv run python credit_assessment_pipeline/run_credit_assessment.py \
  --models llama3.2:3b \
  --output-dir my-assessments \
  --use-atomic
```

## Common Workflows

### Daily Assessment Run

```bash
#!/bin/bash
# daily_assessment.sh

DATE=$(date +%Y%m%d)
OUTPUT_DIR="assessments_${DATE}"

# Run assessments
uv run python credit_assessment_pipeline/run_credit_assessment.py \
  --models llama3.2:3b codellama:7b \
  --output-dir ${OUTPUT_DIR} \
  --format json-ad \
  --use-atomic

# Generate summary
uv run python credit_assessment_pipeline/create_summary.py \
  --input-dir ${OUTPUT_DIR}

echo "Assessment complete. Results in ${OUTPUT_DIR}/"
```

### Comparing Data Formats

```bash
#!/bin/bash
# compare_formats.sh

MODEL="llama3.2:3b"

for FORMAT in json json-ad turtle; do
  echo "Running with format: ${FORMAT}"
  uv run python credit_assessment_pipeline/run_credit_assessment.py \
    --models ${MODEL} \
    --format ${FORMAT} \
    --use-atomic \
    --output-dir "format_comparison"
done

echo "Format comparison complete"
```

## Troubleshooting

### Issue: "Model not found"

```bash
# Check available models
ollama list

# Pull the required model
ollama pull llama3.2:3b
```

### Issue: "Connection refused"

```bash
# Check if Ollama is running
ps aux | grep ollama

# Start Ollama if not running
ollama serve
```

### Issue: "Module not found"

```bash
# Ensure you're using uv run
uv run python credit_assessment_pipeline/run_credit_assessment.py

# Or install missing dependencies
uv pip install ollama requests
```

### Issue: "Atomic Server unreachable"

The pipeline will automatically fall back to local files. To use only local files:

```bash
uv run python credit_assessment_pipeline/run_credit_assessment.py \
  --models llama3.2:3b
  # Note: no --use-atomic flag
```

## Next Steps

1. **Explore Advanced Features**: See [Advanced Usage](./advanced-usage.md)
2. **Learn About Frameworks**: Read [Multi-Framework Guide](./multi-framework-guide.md)
3. **Customize Prompts**: Check [Prompt Engineering](./prompt-engineering.md)
4. **API Integration**: Review [API Usage](./api-integration.md)

## Tips for Success

1. **Start Small**: Begin with one model before running all
2. **Monitor Resources**: Large models need more RAM
3. **Use Appropriate Timeouts**: Adjust based on model size
4. **Save Results**: Use meaningful output directory names
5. **Document Findings**: Keep notes on model performance

## Example Output

A successful run produces output like:

```
--- Running assessment for model: llama3.2:3b with format: json ---
Credit assessment for llama3.2:3b (format: json) saved to: credit-assessment-results/llama3.2_3b-json-credit-assessment.md
```

The generated report includes:
- Executive Summary
- Quantitative Analysis
- Qualitative Assessment
- Risk Evaluation
- Sector Analysis
- Credit Recommendation
- Monitoring Requirements

## Getting Help

If you encounter issues:

1. Check the [Troubleshooting Guide](../configuration/troubleshooting.md)
2. Review [Common Issues](../configuration/common-issues.md)
3. Consult the [API Documentation](../api/README.md)
4. Check project [memories.md](../../memories.md) for historical context