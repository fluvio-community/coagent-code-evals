# Mode: PLAN 🎯

Current Task: Evaluate AIPACK alternatives (rust-genai, rig crate) for Rust-based evaluation framework
Understanding: Need to assess current AIPACK usage, analyze rust-genai and rig capabilities, determine migration feasibility
Questions: 
1. What specific AIPACK features are we currently using?
2. What are the capabilities and limitations of rust-genai?
3. What are the capabilities and limitations of rig crate?
4. What would be the migration effort and benefits?
5. Which solution best fits our current architecture and future needs?
Confidence: 60% - Need to analyze current usage patterns and explore alternatives
Next Steps: 
- Analyze current AIPACK configuration and usage
- Research rust-genai framework capabilities
- Research rig crate capabilities  
- Compare features, performance, and ecosystem
- Recommend best path forward

# AIPACK vs rust-genai vs rig Crate Analysis

## Current State Analysis

### AIPACK Usage in Current Project
Based on the codebase analysis, our current AIPACK usage includes:

#### Current AIPACK Configuration
- Located in `.aipack/config.toml`
- Provides model configuration and prompt management
- Integrates with Ollama for local model execution
- Used for multi-model evaluation workflows

#### AIPACK Integration Points
1. **Validation Module**: `AipackValidator` validates TOML configuration
2. **Shell Scripts**: Multiple bash scripts use AIPACK for orchestration
3. **Model Management**: Handles model selection and prompt routing
4. **Evaluation Workflows**: Coordinates multi-model evaluations

#### Current AIPACK Benefits
- ✅ Simple TOML-based configuration
- ✅ Works well with existing shell script workflows
- ✅ Proven integration with Ollama
- ✅ Minimal learning curve
- ✅ Good for prompt templating

#### Current AIPACK Limitations
- ❌ Limited Rust ecosystem integration
- ❌ No native async support
- ❌ Basic error handling
- ❌ Limited observability features
- ❌ No built-in caching or optimization
- ❌ Shell-script dependent for complex workflows

## Alternative 1: rust-genai

### Overview
rust-genai appears to be a Rust-native AI/ML framework focused on generative AI applications.

### Potential Benefits
- ✅ Native Rust implementation
- ✅ Better type safety and error handling
- ✅ Async/await support
- ✅ More sophisticated API design
- ✅ Better integration with Rust ecosystem

### Potential Limitations
- ❌ Learning curve for team
- ❌ Migration effort required
- ❌ Potential breaking changes in early ecosystem
- ❌ Need to rewrite existing AIPACK configurations

### Research Findings

**API Design**: 
- ✅ Clean, ergonomic Rust-native API with async/await support
- ✅ Multi-provider abstraction (OpenAI, Anthropic, Gemini, Ollama, Groq, etc.)
- ✅ Strong type safety with comprehensive error handling
- ✅ Streaming support for real-time responses

**Ollama Integration**: 
- ✅ **Native Ollama support** - First-class integration with local models
- ✅ Same API as other providers - easy to switch between local/remote
- ✅ Supports tool use and streaming with Ollama models

**Performance Characteristics**:
- ✅ Async-first design for high concurrency
- ✅ Streaming responses to reduce latency
- ✅ Built-in request batching capabilities
- ✅ Memory efficient with proper resource management

**Documentation & Community**:
- ✅ Excellent documentation with comprehensive examples
- ✅ Active development by Jeremy Chone (experienced Rust developer)
- ✅ Regular updates and new provider additions
- ✅ Growing community adoption

**Key Strengths for Our Use Case**:
- ✅ **Direct Ollama compatibility** - no adaptation needed
- ✅ **Async/await native** - perfect for our performance requirements
- ✅ **Multi-model evaluation** - can test different providers easily
- ✅ **Streaming support** - for real-time progress tracking
- ✅ **Tool support** - extensible for future evaluation features

## Alternative 2: rig Crate

### Overview
rig is a Rust crate for building AI applications with LLMs.

### Potential Benefits
- ✅ Purpose-built for LLM applications
- ✅ Native Rust implementation
- ✅ Modern async design
- ✅ Built-in prompt management
- ✅ Better error handling

### Potential Limitations
- ❌ Different API paradigm
- ❌ Migration complexity
- ❌ Potential vendor lock-in
- ❌ Learning investment required

### Research Findings

**API Design**:
- ✅ Modern, ergonomic Rust API with strong typing
- ✅ Agent-based architecture for building AI applications
- ✅ Built-in RAG (Retrieval Augmented Generation) support
- ✅ Comprehensive provider ecosystem (OpenAI, Anthropic, Gemini, xAI, Perplexity)
- ⚠️ **Limited Ollama support** - no first-class Ollama integration found

**LLM Application Features**:
- ✅ **Agent framework** - sophisticated agent building capabilities
- ✅ **Vector store integrations** - MongoDB, LanceDB, Neo4j, SQLite, etc.
- ✅ **Built-in embeddings** - multiple embedding model support
- ✅ **Tool calling** - function calling and tool use
- ✅ **RAG workflows** - document processing and retrieval

**Performance Characteristics**:
- ✅ Async-first design with tokio
- ✅ Modular architecture for scalability
- ✅ Vector database optimizations
- ✅ Streaming support for responses

**Documentation & Community**:
- ✅ Comprehensive documentation and examples
- ✅ Active development by 0xPlaygrounds team
- ✅ Growing production usage (Dria, MCP Rust SDK, etc.)
- ✅ Regular feature additions and improvements

**Limitations for Our Use Case**:
- ❌ **No native Ollama support** - would require custom implementation
- ❌ **Agent-focused** - might be overkill for simple evaluation tasks
- ❌ **Complex architecture** - steeper learning curve
- ❌ **Breaking changes** - explicitly warns about API instability

## Evaluation Criteria

### Technical Criteria
1. **Ollama Integration**: How well does it work with our Ollama setup?
2. **Performance**: Evaluation speed and resource usage
3. **Type Safety**: Rust-native error handling and type safety
4. **Async Support**: Native async/await compatibility
5. **Extensibility**: Ability to extend and customize
6. **Testing**: Built-in testing and validation capabilities

### Operational Criteria
1. **Migration Effort**: Time and complexity to migrate
2. **Learning Curve**: Team adoption difficulty
3. **Maintenance**: Long-term maintenance burden
4. **Documentation**: Quality of documentation and examples
5. **Community**: Ecosystem support and community size
6. **Stability**: API stability and breaking change frequency

### Strategic Criteria
1. **Future-Proofing**: Alignment with Rust ecosystem trends
2. **Vendor Lock-in**: Risk of dependency on specific solutions
3. **Customization**: Ability to customize for our specific needs
4. **Performance Optimization**: Built-in performance features
5. **Observability**: Monitoring and debugging capabilities

## Preliminary Recommendation Framework

### Keep AIPACK If:
- Current functionality meets 90%+ of needs
- Migration effort exceeds business value
- Team prefers stability over features
- Shell script integration is critical
- Simple configuration is preferred

### Migrate to rust-genai If:
- Native Rust benefits outweigh migration costs
- Better async performance is critical
- Type safety improvements are valuable
- Framework provides unique capabilities
- Long-term Rust ecosystem alignment is priority

### Migrate to rig If:
- LLM-specific features provide significant value
- Built-in prompt management improves workflow
- Performance improvements are substantial
- API design fits our use cases better
- Community/ecosystem is strong

## Next Steps for Analysis

1. **Deep Dive Research**: Examine rust-genai and rig documentation, examples, and APIs
2. **Prototype Development**: Create small prototypes with each solution
3. **Performance Benchmarking**: Compare evaluation performance across solutions
4. **Migration Estimation**: Estimate effort to migrate current AIPACK workflows
5. **Team Assessment**: Evaluate team preferences and learning investment
6. **Decision Matrix**: Create weighted scoring matrix for final recommendation

## Questions for Further Investigation

1. How do rust-genai and rig handle model lifecycle management?
2. What are the performance characteristics under load?
3. How do they integrate with existing Rust async ecosystems?
4. What observability and debugging tools do they provide?
5. How stable are their APIs and what's their release cadence?
6. Do they support the same model providers we use (Ollama, etc.)?
7. What's the migration path from AIPACK configurations?
8. How do they handle prompt templating and management?
9. What testing and validation capabilities do they provide?
10. How do they compare in terms of memory usage and performance optimization?

This analysis will form the basis for a detailed evaluation and recommendation.
