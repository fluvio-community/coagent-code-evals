use std::collections::HashMap;
use regex::Regex;
use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};

/// Comprehensive evaluation metrics extracted from model output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationMetrics {
    pub model_name: String,
    pub score: Option<u8>,
    pub duration_seconds: Option<u64>,
    pub prompt_tokens: Option<u32>,
    pub completion_tokens: Option<u32>,
    pub model_size: String,
    pub strengths: Vec<String>,
    pub issues: Vec<String>,
    pub primary_recommendation: Option<String>,
    pub status: EvaluationStatus,
    pub timestamp: String,
    /// Helper field for tracking which prompt was used (for caching)
    pub prompt_used: String,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EvaluationStatus {
    Success,
    Failed,
    Timeout,
    ParseError,
}

/// Enhanced parser for evaluation results with multiple fallback strategies
pub struct EvaluationParser {
    score_patterns: Vec<Regex>,
    strength_patterns: Vec<Regex>,
    issue_patterns: Vec<Regex>,
    token_patterns: Vec<Regex>,
    duration_patterns: Vec<Regex>,
}

impl Default for EvaluationParser {
    fn default() -> Self {
        Self::new()
    }
}

impl EvaluationParser {
    /// Create a new parser with comprehensive regex patterns
    pub fn new() -> Self {
        let score_patterns = vec![
            // Pattern 1: **Score: X/10**
            Regex::new(r"\*\*Score[:\s]*(\d+)/10\*\*").unwrap(),
            // Pattern 2: Score: X/10 (without asterisks)
            Regex::new(r"Score[:\s]*(\d+)/10").unwrap(),
            // Pattern 3: X/10 at beginning of line
            Regex::new(r"^(\d+)/10").unwrap(),
            // Pattern 4: Any X/10 pattern
            Regex::new(r"(\d+)/10").unwrap(),
            // Pattern 5: Rating X out of 10
            Regex::new(r"[Rr]ating[:\s]*(\d+)\s*out of 10").unwrap(),
        ];

        let strength_patterns = vec![
            // Pattern 1: Numbered list after "Strengths:"
            Regex::new(r"(?i)\*\*strengths:\*\*\s*\n((?:\d+\.\s*.+\n?)+)").unwrap(),
            // Pattern 2: Bullet points after "Strengths:"
            Regex::new(r"(?i)\*\*strengths:\*\*\s*\n((?:[-*]\s*.+\n?)+)").unwrap(),
            // Pattern 3: Simple "Strengths:" header
            Regex::new(r"(?i)strengths:\s*\n((?:.+\n?)+?)(?=\n\*\*|\n[A-Z]|$)").unwrap(),
        ];

        let issue_patterns = vec![
            // Pattern 1: Numbered list after "Issues:"
            Regex::new(r"(?i)\*\*issues:\*\*\s*\n((?:\d+\.\s*.+\n?)+)").unwrap(),
            // Pattern 2: Bullet points after "Issues:"
            Regex::new(r"(?i)\*\*issues:\*\*\s*\n((?:[-*]\s*.+\n?)+)").unwrap(),
            // Pattern 3: Simple "Issues:" header
            Regex::new(r"(?i)issues:\s*\n((?:.+\n?)+?)(?=\n\*\*|\n[A-Z]|$)").unwrap(),
        ];

        let token_patterns = vec![
            Regex::new(r"Prompt Tokens:\s*(\d+)").unwrap(),
            Regex::new(r"Completion Tokens:\s*(\d+)").unwrap(),
            Regex::new(r"Input tokens:\s*(\d+)").unwrap(),
            Regex::new(r"Output tokens:\s*(\d+)").unwrap(),
        ];

        let duration_patterns = vec![
            Regex::new(r"Duration:\s*(\d+)s").unwrap(),
            Regex::new(r"Duration:\s*(\d+)m\s*(\d+)s").unwrap(),
            Regex::new(r"(\d+)s\s*(\d+)ms").unwrap(),
            Regex::new(r"\*\*Duration:\*\*\s*(\d+)s").unwrap(),
        ];

        Self {
            score_patterns,
            strength_patterns,
            issue_patterns,
            token_patterns,
            duration_patterns,
        }
    }

    /// Parse evaluation result with comprehensive fallback strategies
    pub fn parse_evaluation(&self, content: &str, model_name: &str) -> Result<EvaluationMetrics> {
        self.parse_evaluation_with_prompt(content, model_name, "")
    }

    /// Parse evaluation result with prompt information for caching
    pub fn parse_evaluation_with_prompt(&self, content: &str, model_name: &str, prompt: &str) -> Result<EvaluationMetrics> {
        let mut metrics = EvaluationMetrics {
            model_name: model_name.to_string(),
            score: None,
            duration_seconds: None,
            prompt_tokens: None,
            completion_tokens: None,
            model_size: self.estimate_model_size(model_name),
            strengths: Vec::new(),
            issues: Vec::new(),
            primary_recommendation: None,
            status: EvaluationStatus::Success,
            timestamp: chrono::Utc::now().to_rfc3339(),
            prompt_used: prompt.to_string(),
        };

        // Extract score with multiple fallback patterns
        metrics.score = self.extract_score(content);

        // Extract duration
        metrics.duration_seconds = self.extract_duration(content);

        // Extract token counts
        let (prompt_tokens, completion_tokens) = self.extract_tokens(content);
        metrics.prompt_tokens = prompt_tokens;
        metrics.completion_tokens = completion_tokens;

        // Extract strengths and issues
        metrics.strengths = self.extract_strengths(content);
        metrics.issues = self.extract_issues(content);

        // Extract primary recommendation
        metrics.primary_recommendation = self.extract_recommendation(content);

        // Determine status based on content
        metrics.status = self.determine_status(content);

        Ok(metrics)
    }

    /// Extract score using multiple patterns with fallbacks
    fn extract_score(&self, content: &str) -> Option<u8> {
        for pattern in &self.score_patterns {
            if let Some(captures) = pattern.captures(content) {
                if let Some(score_str) = captures.get(1) {
                    if let Ok(score) = score_str.as_str().parse::<u8>() {
                        if score <= 10 {
                            return Some(score);
                        }
                    }
                }
            }
        }
        None
    }

    /// Extract duration in seconds with multiple format support
    fn extract_duration(&self, content: &str) -> Option<u64> {
        for pattern in &self.duration_patterns {
            if let Some(captures) = pattern.captures(content) {
                match captures.len() {
                    2 => {
                        // Simple seconds format
                        if let Ok(seconds) = captures[1].parse::<u64>() {
                            return Some(seconds);
                        }
                    }
                    3 => {
                        // Minutes and seconds format
                        if let (Ok(minutes), Ok(seconds)) = (
                            captures[1].parse::<u64>(),
                            captures[2].parse::<u64>()
                        ) {
                            return Some(minutes * 60 + seconds);
                        }
                    }
                    _ => continue,
                }
            }
        }
        None
    }

    /// Extract token counts for both prompt and completion
    fn extract_tokens(&self, content: &str) -> (Option<u32>, Option<u32>) {
        let mut prompt_tokens = None;
        let mut completion_tokens = None;

        for pattern in &self.token_patterns {
            for captures in pattern.captures_iter(content) {
                if let Some(token_str) = captures.get(1) {
                    if let Ok(tokens) = token_str.as_str().parse::<u32>() {
                        let pattern_str = pattern.as_str();
                        if pattern_str.contains("Prompt") || pattern_str.contains("Input") {
                            prompt_tokens = Some(tokens);
                        } else if pattern_str.contains("Completion") || pattern_str.contains("Output") {
                            completion_tokens = Some(tokens);
                        }
                    }
                }
            }
        }

        (prompt_tokens, completion_tokens)
    }

    /// Extract strengths with robust parsing
    fn extract_strengths(&self, content: &str) -> Vec<String> {
        for pattern in &self.strength_patterns {
            if let Some(captures) = pattern.captures(content) {
                if let Some(strengths_block) = captures.get(1) {
                    return self.parse_list_items(strengths_block.as_str());
                }
            }
        }

        // Fallback: look for lines containing positive keywords after "Strengths"
        self.extract_by_keywords(content, "strengths", &[
            "good", "excellent", "proper", "well", "effective", "strong", "robust"
        ])
    }

    /// Extract issues with robust parsing
    fn extract_issues(&self, content: &str) -> Vec<String> {
        for pattern in &self.issue_patterns {
            if let Some(captures) = pattern.captures(content) {
                if let Some(issues_block) = captures.get(1) {
                    return self.parse_list_items(issues_block.as_str());
                }
            }
        }

        // Fallback: look for lines containing negative keywords after "Issues"
        self.extract_by_keywords(content, "issues", &[
            "problem", "issue", "error", "missing", "poor", "weak", "bad", "incorrect"
        ])
    }

    /// Parse numbered or bulleted list items
    fn parse_list_items(&self, text: &str) -> Vec<String> {
        let list_regex = Regex::new(r"(?m)^(?:\d+\.|\*|-)\s*(.+)$").unwrap();
        list_regex
            .captures_iter(text)
            .filter_map(|cap| cap.get(1))
            .map(|m| m.as_str().trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    }

    /// Extract items by searching for keywords near section headers
    fn extract_by_keywords(&self, content: &str, section: &str, keywords: &[&str]) -> Vec<String> {
        let lines: Vec<&str> = content.lines().collect();
        let mut results = Vec::new();
        let mut in_section = false;
        let mut lines_since_header = 0;

        for line in lines {
            if line.to_lowercase().contains(section) {
                in_section = true;
                lines_since_header = 0;
                continue;
            }

            if in_section {
                lines_since_header += 1;
                
                // Stop if we've gone too far past the header
                if lines_since_header > 10 {
                    break;
                }

                // Check if line contains any of the keywords
                let line_lower = line.to_lowercase();
                if keywords.iter().any(|&keyword| line_lower.contains(keyword)) {
                    results.push(line.trim().to_string());
                }

                // Stop if we hit another section header
                if line.starts_with("**") && line.ends_with("**") {
                    break;
                }
            }
        }

        results
    }

    /// Extract primary recommendation
    fn extract_recommendation(&self, content: &str) -> Option<String> {
        let recommendation_regex = 
            Regex::new(r"(?i)\*\*Primary Recommendation:\*\*\s*(.+?)(?=\n\*\*|\n##|$)").unwrap();
        
        if let Some(captures) = recommendation_regex.captures(content) {
            if let Some(rec) = captures.get(1) {
                return Some(rec.as_str().trim().to_string());
            }
        }

        // Fallback: look for "recommendation" keyword
        let fallback_regex = Regex::new(r"(?i)recommendation[:\s]*(.+?)(?=\n|$)").unwrap();
        if let Some(captures) = fallback_regex.captures(content) {
            if let Some(rec) = captures.get(1) {
                return Some(rec.as_str().trim().to_string());
            }
        }

        None
    }

    /// Determine evaluation status from content
    fn determine_status(&self, content: &str) -> EvaluationStatus {
        let content_lower = content.to_lowercase();
        
        if content_lower.contains("error") || content_lower.contains("failed") {
            EvaluationStatus::Failed
        } else if content_lower.contains("timeout") || content_lower.contains("timed out") {
            EvaluationStatus::Timeout
        } else if content_lower.contains("**status:** success") {
            EvaluationStatus::Success
        } else if self.extract_score(content).is_none() {
            EvaluationStatus::ParseError
        } else {
            EvaluationStatus::Success
        }
    }

    /// Estimate model size based on model name
    fn estimate_model_size(&self, model_name: &str) -> String {
        let name_lower = model_name.to_lowercase();
        
        if name_lower.contains("3b") {
            "3B".to_string()
        } else if name_lower.contains("7b") {
            "7B".to_string()
        } else if name_lower.contains("8b") {
            "8B".to_string()
        } else if name_lower.contains("27b") {
            "27B".to_string()
        } else if name_lower.contains("70b") {
            "70B".to_string()
        } else if name_lower.contains("latest") {
            "Variable".to_string()
        } else {
            "Unknown".to_string()
        }
    }
}

/// Generate terminal-friendly visualization charts
pub struct VisualizationGenerator;

impl VisualizationGenerator {
    /// Generate ASCII bar chart for scores
    pub fn generate_score_chart(metrics: &[EvaluationMetrics]) -> String {
        let mut chart = String::new();
        chart.push_str("Model Performance Chart (Score out of 10)\n");
        chart.push_str("0    2    4    6    8    10\n");
        chart.push_str("|    |    |    |    |    |\n");
        
        for metric in metrics {
            let score = metric.score.unwrap_or(0);
            let filled_bars = "█".repeat(score as usize);
            let empty_bars = "░".repeat((10 - score) as usize);
            let model_name = if metric.model_name.len() > 20 {
                format!("{}...", &metric.model_name[..17])
            } else {
                format!("{:<20}", metric.model_name)
            };
            
            chart.push_str(&format!("{} {} {}/10\n", model_name, filled_bars + &empty_bars, score));
        }
        
        chart
    }

    /// Generate comparison table with visual elements
    pub fn generate_comparison_table(metrics: &[EvaluationMetrics]) -> String {
        let mut table = String::new();
        table.push_str("| Model | Visual Score | Score | Duration | Tokens (P/C) | Size | Status |\n");
        table.push_str("|-------|-------------|-------|----------|--------------|------|--------|\n");
        
        for metric in metrics {
            let score = metric.score.unwrap_or(0);
            let visual_bar = "█".repeat(score as usize) + &"░".repeat((10 - score) as usize);
            let duration = metric.duration_seconds
                .map(|d| format!("{}s", d))
                .unwrap_or_else(|| "N/A".to_string());
            let tokens = format!("{}/{}", 
                metric.prompt_tokens.unwrap_or(0),
                metric.completion_tokens.unwrap_or(0)
            );
            let status = match metric.status {
                EvaluationStatus::Success => "✅",
                EvaluationStatus::Failed => "❌",
                EvaluationStatus::Timeout => "⏰",
                EvaluationStatus::ParseError => "⚠️",
            };
            
            table.push_str(&format!(
                "| {} | {} | {}/10 | {} | {} | {} | {} |\n",
                metric.model_name,
                visual_bar,
                score,
                duration,
                tokens,
                metric.model_size,
                status
            ));
        }
        
        table
    }

    /// Generate performance statistics
    pub fn generate_statistics(metrics: &[EvaluationMetrics]) -> String {
        let mut stats = String::new();
        
        let valid_scores: Vec<u8> = metrics
            .iter()
            .filter_map(|m| m.score)
            .collect();
        
        if !valid_scores.is_empty() {
            let avg_score = valid_scores.iter().sum::<u8>() as f32 / valid_scores.len() as f32;
            let best_score = *valid_scores.iter().max().unwrap();
            let worst_score = *valid_scores.iter().min().unwrap();
            
            let unknown = "Unknown".to_string();
            let best_model = metrics
                .iter()
                .find(|m| m.score == Some(best_score))
                .map(|m| &m.model_name)
                .unwrap_or(&unknown);
            
            let worst_model = metrics
                .iter()
                .find(|m| m.score == Some(worst_score))
                .map(|m| &m.model_name)
                .unwrap_or(&unknown);
            
            stats.push_str(&format!("- **Average Score:** {:.1}/10\n", avg_score));
            stats.push_str(&format!("- **Best Performer:** {} ({}/10)\n", best_model, best_score));
            stats.push_str(&format!("- **Needs Improvement:** {} ({}/10)\n", worst_model, worst_score));
        }
        
        let successful_evals = metrics
            .iter()
            .filter(|m| matches!(m.status, EvaluationStatus::Success))
            .count();
        
        stats.push_str(&format!("- **Successful Evaluations:** {}/{}\n", successful_evals, metrics.len()));
        stats.push_str(&format!("- **Total Models Evaluated:** {}\n", metrics.len()));
        
        stats
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score_extraction() {
        let parser = EvaluationParser::new();
        
        // Test various score formats
        assert_eq!(parser.extract_score("**Score: 8/10**"), Some(8));
        assert_eq!(parser.extract_score("Score: 7/10"), Some(7));
        assert_eq!(parser.extract_score("Rating 9 out of 10"), Some(9));
        assert_eq!(parser.extract_score("No score here"), None);
    }

    #[test]
    fn test_duration_extraction() {
        let parser = EvaluationParser::new();
        
        assert_eq!(parser.extract_duration("Duration: 45s"), Some(45));
        assert_eq!(parser.extract_duration("Duration: 2m 30s"), Some(150));
        assert_eq!(parser.extract_duration("**Duration:** 120s"), Some(120));
    }

    #[test]
    fn test_token_extraction() {
        let parser = EvaluationParser::new();
        let content = "Prompt Tokens: 617 | Completion Tokens: 377";
        
        let (prompt, completion) = parser.extract_tokens(content);
        assert_eq!(prompt, Some(617));
        assert_eq!(completion, Some(377));
    }

    #[test]
    fn test_model_size_estimation() {
        let parser = EvaluationParser::new();
        
        assert_eq!(parser.estimate_model_size("llama3.2:3b"), "3B");
        assert_eq!(parser.estimate_model_size("codellama:7b"), "7B");
        assert_eq!(parser.estimate_model_size("gemma3:27b"), "27B");
        assert_eq!(parser.estimate_model_size("unknown-model"), "Unknown");
    }
}
