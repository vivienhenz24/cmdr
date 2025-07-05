use clap::Parser;
use cmdr_benchmark::{BenchmarkRunner, save_results, compare_results};

#[derive(Parser)]
#[command(name = "cmdr-benchmark")]
#[command(about = "Performance benchmarking tool for cmdr")]
struct Args {
    /// Benchmark suite name
    #[arg(short, long, default_value = "cmdr-benchmark")]
    name: String,
    
    /// Number of iterations per benchmark
    #[arg(short, long, default_value = "1000")]
    iterations: usize,
    
    /// Output file for results
    #[arg(short, long, default_value = "benchmark-results.json")]
    output: String,
    
    /// Compare with previous results
    #[arg(long)]
    compare: Option<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    
    if let Some(compare_path) = args.compare {
        compare_results(&args.output, &compare_path)?;
        return Ok(());
    }
    
    let mut runner = BenchmarkRunner::new(&args.name);
    
    // Benchmark translation engine
    runner.benchmark("translation_engine_init", args.iterations, || {
        let _engine = cmdr_core::TranslationEngine::new(
            cmdr_core::MockInferenceEngine::new()
        );
    });
    
    // Benchmark shell executor
    runner.benchmark("shell_executor_init", args.iterations, || {
        let _executor = cmdr_core::ShellExecutor::default();
    });
    
    // Benchmark command parsing
    runner.benchmark("command_parsing", args.iterations, || {
        let _parts = cmdr_core::shell::CommandParser::parse("ls -la").unwrap();
    });
    
    let suite = runner.finish();
    save_results(&suite, &args.output)?;
    
    println!("Benchmark suite completed in {:?}", suite.total_duration);
    Ok(())
} 