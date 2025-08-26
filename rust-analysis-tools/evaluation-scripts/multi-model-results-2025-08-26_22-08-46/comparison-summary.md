# Multi-Model Rust Code Evaluation Comparison

## Model Performance Summary

| Model | Score | Duration | Primary Strength | Main Issue |
|-------|-------|----------|------------------|------------|
| llama3.2:3b | **Score: 5/10** | 15s |  **Use of async programming**: The code u... |  **Lack of error handling in `init_app`**... |
| codellama:7b |  | 30s | ... | ... |
| deepseek-coder:latest |  | 7s | ... | ... |
| victornitu/rust-coder:latest |  | 58s | ... | ... |
| cogito:latest | **Score: 7/10** | 41s |  **Proper Architecture Design**: Separate... |  **Critical Memory Leaks**: The global Ap... |
| qwen3-coder:latest | **Score: 5/10** | 41s |  Use of `lazy_static` for thread-safe glo... |  Unsafe global mutable state access in as... |
| qwen2.5-coder:latest | **Score: 6/10** | 23s |  **Basic Structured Design**: The `AppSta... |  **Lack of Asynchronous Data Fetching**: ... |
| gemma3:27b | **Score: 6/10** | 124s |  **Global State Management with `lazy_sta... |  **Unnecessary `unwrap()`:** - The `Store... |
| qwen3:latest | **Score: 6/10** | 51s |  Async initialization pattern - [Using `s... |  Unused async function - [The `init_app()... |
| qwen3:8b | **Score: 5/10** | 32s |  **State Management with Mutex** - [The u... |  **Unimplemented Async Initialization** -... |

## Detailed Analysis
See individual model result files for complete evaluations.
