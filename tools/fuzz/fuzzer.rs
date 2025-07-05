//! Fuzzing tool for cmdr
//! 
//! This tool provides fuzzing capabilities to find edge cases and bugs
//! in the cmdr codebase.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FuzzResult {
    pub test_name: String,
    pub success: bool,
    pub duration: std::time::Duration,
    pub iterations: usize,
    pub crashes: usize,
    pub errors: Vec<String>,
    pub coverage: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FuzzReport {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub total_tests: usize,
    pub total_iterations: usize,
    pub total_crashes: usize,
    pub results: Vec<FuzzResult>,
}

pub struct Fuzzer {
    max_iterations: usize,
    timeout: std::time::Duration,
    results: Vec<FuzzResult>,
}

impl Fuzzer {
    pub fn new(max_iterations: usize, timeout: std::time::Duration) -> Self {
        Self {
            max_iterations,
            timeout,
            results: Vec::new(),
        }
    }

    pub fn fuzz<F>(&mut self, test_name: &str, mut f: F) -> FuzzResult
    where
        F: FnMut(&[u8]) -> Result<(), Box<dyn std::error::Error>>,
    {
        println!("Fuzzing: {}", test_name);
        
        let start = std::time::Instant::now();
        let mut crashes = 0;
        let mut errors = Vec::new();
        
        for i in 0..self.max_iterations {
            if start.elapsed() > self.timeout {
                println!("  Timeout reached after {} iterations", i);
                break;
            }
            
            // Generate random input
            let input = self.generate_random_input();
            
            // Run the test
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                f(&input)
            })) {
                Ok(Ok(())) => {
                    // Success
                }
                Ok(Err(e)) => {
                    errors.push(format!("Error at iteration {}: {}", i, e));
                }
                Err(_) => {
                    crashes += 1;
                    errors.push(format!("Panic at iteration {} with input: {:?}", i, input));
                }
            }
        }
        
        let duration = start.elapsed();
        let success = crashes == 0 && errors.is_empty();
        
        let result = FuzzResult {
            test_name: test_name.to_string(),
            success,
            duration,
            iterations: self.max_iterations,
            crashes,
            errors,
            coverage: None, // TODO: Add coverage tracking
        };
        
        self.results.push(result.clone());
        result
    }

    fn generate_random_input(&self) -> Vec<u8> {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        // Generate random length input
        let len = rng.gen_range(0..1024);
        let mut input = Vec::with_capacity(len);
        
        for _ in 0..len {
            input.push(rng.gen());
        }
        
        input
    }

    pub fn generate_report(self) -> FuzzReport {
        let total_tests = self.results.len();
        let total_iterations: usize = self.results.iter().map(|r| r.iterations).sum();
        let total_crashes: usize = self.results.iter().map(|r| r.crashes).sum();
        
        FuzzReport {
            timestamp: chrono::Utc::now(),
            total_tests,
            total_iterations,
            total_crashes,
            results: self.results,
        }
    }
}

pub fn save_report(report: &FuzzReport, path: &str) -> anyhow::Result<()> {
    let json = serde_json::to_string_pretty(report)?;
    std::fs::write(path, json)?;
    println!("Fuzz report saved to: {}", path);
    Ok(())
} 