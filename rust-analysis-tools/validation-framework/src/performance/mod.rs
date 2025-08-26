/// # Performance Optimization Module
/// 
/// This module provides comprehensive performance optimizations for the evaluation process:
/// - Parallel evaluation support for multiple models with resource limits
/// - Model warm-up functionality to improve first-run performance
/// - Response caching system to enable re-analysis without re-evaluation
/// - Memory optimization with resource cleanup between evaluations
/// - Progress indicators with ETA calculations for better user experience
/// 
/// ## Key Features
/// 
/// ### Parallel Evaluation
/// - Configurable concurrency limits to prevent system overload
/// - Resource-aware scheduling based on model size and system capabilities
/// - Smart batching to optimize throughput while maintaining stability
/// 
/// ### Model Warm-up
/// - Pre-loads models before evaluation to reduce cold start latency
/// - Configurable warm-up prompts to ensure models are ready
/// - Parallel warm-up with resource management
/// 
/// ### Response Caching
/// - Persistent cache for model responses to enable quick re-analysis
/// - Content-based cache keys to ensure accuracy
/// - Configurable cache expiration and size limits
/// - Cache invalidation strategies
/// 
/// ### Memory Optimization
/// - Automatic resource cleanup between evaluations
/// - Memory pool management for large evaluations
/// - Garbage collection hints for optimal memory usage
/// 
/// ### Progress Tracking
/// - Real-time progress indicators with completion percentages
/// - ETA calculations based on current throughput
/// - Detailed status reporting for individual model evaluations

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{Semaphore, RwLock, Mutex};
use tokio::time::sleep;
use serde::{Deserialize, Serialize};
use anyhow::{Result, Context};
use reqwest::Client;
use crate::evaluation::{EvaluationMetrics, EvaluationParser, EvaluationStatus};

pub mod cache;
pub mod warmup;
pub mod progress;
pub mod parallel;
pub mod memory;

use cache::ResponseCache;
use warmup::ModelWarmup;
use progress::{ProgressTracker, EvaluationProgress};
use parallel::ParallelEvaluator;
use memory::MemoryManager;

/// Configuration for performance optimization features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    /// Maximum number of concurrent model evaluations
    pub max_concurrent_evaluations: usize,
    /// Maximum number of concurrent warm-up operations
    pub max_concurrent_warmups: usize,
    /// Enable response caching
    pub enable_cache: bool,
    /// Cache expiration time in hours
    pub cache_expiration_hours: u64,
    /// Maximum cache size in MB
    pub max_cache_size_mb: u64,
    /// Enable model warm-up
    pub enable_warmup: bool,
    /// Warm-up timeout in seconds
    pub warmup_timeout_seconds: u64,
    /// Enable memory optimization
    pub enable_memory_optimization: bool,
    /// Memory cleanup interval in seconds
    pub memory_cleanup_interval_seconds: u64,
    /// Enable progress tracking
    pub enable_progress_tracking: bool,
    /// Progress update interval in milliseconds
    pub progress_update_interval_ms: u64,
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            max_concurrent_evaluations: num_cpus::get().min(4), // Don't overload system
            max_concurrent_warmups: 2,
            enable_cache: true,
            cache_expiration_hours: 24,
            max_cache_size_mb: 512,
            enable_warmup: true,
            warmup_timeout_seconds: 30,
            enable_memory_optimization: true,
            memory_cleanup_interval_seconds: 300, // 5 minutes
            enable_progress_tracking: true,
            progress_update_interval_ms: 500,
        }
    }
}

/// Performance-optimized evaluation coordinator
pub struct PerformanceEvaluationCoordinator {
    config: PerformanceConfig,
    http_client: Client,
    evaluation_parser: EvaluationParser,
    response_cache: Arc<ResponseCache>,
    model_warmup: Arc<ModelWarmup>,
    progress_tracker: Arc<ProgressTracker>,
    parallel_evaluator: Arc<ParallelEvaluator>,
    memory_manager: Arc<MemoryManager>,
    evaluation_semaphore: Arc<Semaphore>,
}

impl PerformanceEvaluationCoordinator {
    /// Create a new performance-optimized evaluation coordinator
    pub fn new() -> Self {
        Self::with_config(PerformanceConfig::default())
    }

    /// Create a new coordinator with custom performance configuration
    pub fn with_config(config: PerformanceConfig) -> Self {
        let http_client = Client::builder()
            .timeout(Duration::from_secs(300)) // 5 minute timeout for evaluations
            .build()
            .expect("Failed to create HTTP client");

        let evaluation_parser = EvaluationParser::new();
        let response_cache = Arc::new(ResponseCache::new(&config));
        let model_warmup = Arc::new(ModelWarmup::new(&config, http_client.clone()));
        let progress_tracker = Arc::new(ProgressTracker::new(&config));
        let parallel_evaluator = Arc::new(ParallelEvaluator::new(&config, http_client.clone()));
        let memory_manager = Arc::new(MemoryManager::new(&config));
        
        let evaluation_semaphore = Arc::new(Semaphore::new(config.max_concurrent_evaluations));

        Self {
            config,
            http_client,
            evaluation_parser,
            response_cache,
            model_warmup,
            progress_tracker,
            parallel_evaluator,
            memory_manager,
            evaluation_semaphore,
        }
    }

    /// Run optimized evaluation with all performance features enabled
    /// 
    /// This is the main entry point for performance-optimized evaluations that:
    /// 1. Warms up models if enabled
    /// 2. Checks cache for existing results
    /// 3. Runs parallel evaluations with resource limits
    /// 4. Tracks progress with ETA calculations
    /// 5. Manages memory throughout the process
    /// 6. Returns comprehensive results with performance metrics
    pub async fn evaluate_models_optimized(
        &self,
        models: Vec<String>,
        prompts: Vec<String>,
        ollama_url: &str,
    ) -> Result<OptimizedEvaluationResult> {
        let start_time = Instant::now();
        
        log::info!("🚀 Starting optimized evaluation for {} models with {} prompts", 
                   models.len(), prompts.len());

        // Initialize progress tracking
        let total_evaluations = models.len() * prompts.len();
        let progress = self.progress_tracker.start_evaluation(total_evaluations).await;

        // Phase 1: Model warm-up (if enabled)
        if self.config.enable_warmup {
            log::info!("🔥 Warming up {} models...", models.len());
            let warmup_start = Instant::now();
            
            let warmup_result = self.model_warmup.warmup_models(&models, ollama_url).await
                .context("Failed to warm up models")?;
            
            let warmup_duration = warmup_start.elapsed();
            log::info!("✅ Model warm-up completed in {:.2}s", warmup_duration.as_secs_f64());
            
            // Update progress
            self.progress_tracker.update_phase(&progress, "Warm-up completed").await;
        }

        // Phase 2: Check cache and prepare evaluation tasks
        let mut evaluation_tasks = Vec::new();
        let mut cached_results = Vec::new();
        
        for model in &models {
            for prompt in &prompts {
                let cache_key = self.response_cache.generate_cache_key(model, prompt);
                
                if self.config.enable_cache {
                    if let Some(cached_result) = self.response_cache.get(&cache_key).await {
                        log::debug!("📦 Using cached result for model: {}", model);
                        cached_results.push(cached_result);
                        self.progress_tracker.increment_progress(&progress).await;
                        continue;
                    }
                }
                
                evaluation_tasks.push(EvaluationTask {
                    model: model.clone(),
                    prompt: prompt.clone(),
                    cache_key,
                });
            }
        }

        log::info!("📊 Evaluation plan: {} cached results, {} new evaluations", 
                   cached_results.len(), evaluation_tasks.len());

        // Phase 3: Parallel evaluation execution
        let mut all_results = cached_results;
        
        if !evaluation_tasks.is_empty() {
            let parallel_results = self.parallel_evaluator
                .evaluate_parallel(evaluation_tasks, ollama_url, &progress)
                .await
                .context("Failed to execute parallel evaluations")?;
            
            // Cache new results
            if self.config.enable_cache {
                for result in &parallel_results {
                    let cache_key = self.response_cache.generate_cache_key(&result.model_name, &result.prompt_used);
                    self.response_cache.put(cache_key, result.clone()).await;
                }
            }
            
            all_results.extend(parallel_results);
        }

        // Phase 4: Memory cleanup
        if self.config.enable_memory_optimization {
            self.memory_manager.cleanup_resources().await
                .context("Failed to cleanup memory resources")?;
        }

        // Phase 5: Generate comprehensive results
        let total_duration = start_time.elapsed();
        let final_progress = self.progress_tracker.complete_evaluation(&progress).await;

        let result = OptimizedEvaluationResult {
            metrics: all_results,
            performance_stats: PerformanceStats {
                total_duration_secs: total_duration.as_secs_f64(),
                cache_hit_rate: if total_evaluations > 0 {
                    cached_results.len() as f64 / total_evaluations as f64
                } else { 0.0 },
                average_evaluation_time_secs: if evaluation_tasks.len() > 0 {
                    total_duration.as_secs_f64() / evaluation_tasks.len() as f64
                } else { 0.0 },
                parallel_efficiency: final_progress.parallel_efficiency,
                memory_peak_usage_mb: self.memory_manager.get_peak_usage_mb().await,
                warmup_enabled: self.config.enable_warmup,
                cache_enabled: self.config.enable_cache,
            },
            progress_summary: final_progress,
        };

        log::info!("🎉 Optimized evaluation completed in {:.2}s with {:.1}% cache hit rate", 
                   total_duration.as_secs_f64(), result.performance_stats.cache_hit_rate * 100.0);

        Ok(result)
    }

    /// Get current performance statistics
    pub async fn get_performance_stats(&self) -> PerformanceStats {
        PerformanceStats {
            total_duration_secs: 0.0,
            cache_hit_rate: 0.0,
            average_evaluation_time_secs: 0.0,
            parallel_efficiency: 0.0,
            memory_peak_usage_mb: self.memory_manager.get_peak_usage_mb().await,
            warmup_enabled: self.config.enable_warmup,
            cache_enabled: self.config.enable_cache,
        }
    }

    /// Clear all caches and reset performance counters
    pub async fn reset_performance_state(&self) -> Result<()> {
        if self.config.enable_cache {
            self.response_cache.clear().await;
        }
        
        self.memory_manager.cleanup_resources().await?;
        self.progress_tracker.reset().await;
        
        log::info!("🔄 Performance state reset completed");
        Ok(())
    }
}

/// Task for individual model evaluation
#[derive(Debug, Clone)]
struct EvaluationTask {
    model: String,
    prompt: String,
    cache_key: String,
}

/// Comprehensive result of optimized evaluation
#[derive(Debug, Clone, Serialize)]
pub struct OptimizedEvaluationResult {
    /// Individual evaluation metrics for each model/prompt combination
    pub metrics: Vec<EvaluationMetrics>,
    /// Performance statistics for the evaluation run
    pub performance_stats: PerformanceStats,
    /// Progress tracking summary
    pub progress_summary: EvaluationProgress,
}

/// Performance statistics for evaluation runs
#[derive(Debug, Clone, Serialize)]
pub struct PerformanceStats {
    /// Total duration of the evaluation in seconds
    pub total_duration_secs: f64,
    /// Cache hit rate (0.0 to 1.0)
    pub cache_hit_rate: f64,
    /// Average time per evaluation in seconds
    pub average_evaluation_time_secs: f64,
    /// Parallel processing efficiency (0.0 to 1.0)
    pub parallel_efficiency: f64,
    /// Peak memory usage in MB during evaluation
    pub memory_peak_usage_mb: f64,
    /// Whether model warm-up was enabled
    pub warmup_enabled: bool,
    /// Whether response caching was enabled
    pub cache_enabled: bool,
}

// Add methods to EvaluationMetrics for extended functionality
impl EvaluationMetrics {
    /// Helper field for tracking which prompt was used (for caching)
    pub fn with_prompt_used(mut self, prompt: String) -> Self {
        // We'll store this in a new field or use existing metadata
        self.prompt_used = prompt;
        self
    }
    
    /// The prompt that was used for this evaluation (for caching purposes)
    pub prompt_used: String,
}

// Extend EvaluationMetrics with additional fields needed for performance tracking
impl EvaluationMetrics {
    pub fn new_with_prompt(model_name: String, prompt: String) -> Self {
        Self {
            model_name,
            score: None,
            duration_seconds: None,
            prompt_tokens: None,
            completion_tokens: None,
            model_size: "Unknown".to_string(),
            strengths: Vec::new(),
            issues: Vec::new(),
            primary_recommendation: None,
            status: EvaluationStatus::Success,
            timestamp: chrono::Utc::now().to_rfc3339(),
            prompt_used: prompt,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_performance_coordinator_creation() {
        let coordinator = PerformanceEvaluationCoordinator::new();
        let stats = coordinator.get_performance_stats().await;
        
        assert_eq!(stats.cache_hit_rate, 0.0);
        assert_eq!(stats.total_duration_secs, 0.0);
    }

    #[tokio::test]
    async fn test_performance_config_defaults() {
        let config = PerformanceConfig::default();
        
        assert!(config.enable_cache);
        assert!(config.enable_warmup);
        assert!(config.enable_memory_optimization);
        assert!(config.enable_progress_tracking);
        assert!(config.max_concurrent_evaluations > 0);
    }

    #[test]
    fn test_evaluation_task_creation() {
        let task = EvaluationTask {
            model: "llama3.2:3b".to_string(),
            prompt: "Test prompt".to_string(),
            cache_key: "test_key".to_string(),
        };
        
        assert_eq!(task.model, "llama3.2:3b");
        assert_eq!(task.prompt, "Test prompt");
    }
}
