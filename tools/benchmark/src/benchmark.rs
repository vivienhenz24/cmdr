//! Performance benchmarking tool for cmdr
//! 
//! This tool provides comprehensive benchmarking capabilities for
//! measuring performance of different components.

use std::time::{Duration, Instant};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResult {
    pub name: String,
    pub duration: Duration,
    pub iterations: usize,
    pub throughput: f64, // operations per second
    pub memory_usage: Option<usize>, // bytes
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BenchmarkSuite {
    pub name: String,
    pub results: Vec<BenchmarkResult>,
    pub total_duration: Duration,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

pub struct BenchmarkRunner {
    suite_name: String,
    results: Vec<BenchmarkResult>,
    start_time: Instant,
}

impl BenchmarkRunner {
    pub fn new(suite_name: &str) -> Self {
        Self {
            suite_name: suite_name.to_string(),
            results: Vec::new(),
            start_time: Instant::now(),
        }
    }

    pub fn benchmark<F>(&mut self, name: &str, iterations: usize, f: F) -> BenchmarkResult
    where
        F: Fn() + Send + Sync,
    {
        println!("Running benchmark: {} ({} iterations)", name, iterations);
        
        let start = Instant::now();
        for _ in 0..iterations {
            f();
        }
        let duration = start.elapsed();
        
        let throughput = iterations as f64 / duration.as_secs_f64();
        
        let result = BenchmarkResult {
            name: name.to_string(),
            duration,
            iterations,
            throughput,
            memory_usage: None, // TODO: Add memory measurement
            metadata: HashMap::new(),
        };
        
        self.results.push(result.clone());
        result
    }

    pub fn finish(self) -> BenchmarkSuite {
        let total_duration = self.start_time.elapsed();
        
        BenchmarkSuite {
            name: self.suite_name,
            results: self.results,
            total_duration,
            timestamp: chrono::Utc::now(),
        }
    }
}

pub fn save_results(suite: &BenchmarkSuite, path: &str) -> anyhow::Result<()> {
    let json = serde_json::to_string_pretty(suite)?;
    std::fs::write(path, json)?;
    println!("Benchmark results saved to: {}", path);
    Ok(())
}

pub fn compare_results(path1: &str, path2: &str) -> anyhow::Result<()> {
    let suite1: BenchmarkSuite = serde_json::from_str(&std::fs::read_to_string(path1)?)?;
    let suite2: BenchmarkSuite = serde_json::from_str(&std::fs::read_to_string(path2)?)?;
    
    println!("Comparing benchmarks:");
    println!("  {} vs {}", suite1.name, suite2.name);
    println!();
    
    for result1 in &suite1.results {
        if let Some(result2) = suite2.results.iter().find(|r| r.name == result1.name) {
            let speedup = result2.throughput / result1.throughput;
            let change = if speedup > 1.0 {
                format!("+{:.1}%", (speedup - 1.0) * 100.0)
            } else {
                format!("-{:.1}%", (1.0 - speedup) * 100.0)
            };
            
            println!("  {}: {} ({} vs {} ops/sec)", 
                result1.name, change, result1.throughput, result2.throughput);
        }
    }
    
    Ok(())
} 