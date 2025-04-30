mod benchmark;

use parallel_sort::{
    compare_sort_results, generate_random_data, parallel_merge_sort, sequential_merge_sort,
};
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    
    // Default to benchmarking mode if no special arguments provided
    if args.len() == 1 || (args.len() > 1 && !args[1].starts_with("--")) {
        run_single_benchmark(args)?;
    } else {
        match args[1].as_str() {
            "--benchmark" => run_full_benchmark()?,
            "--help" | "-h" => print_usage(),
            _ => {
                eprintln!("Unknown option: {}", args[1]);
                print_usage();
            }
        }
    }
    
    Ok(())
}

/// Runs a single benchmark with a specific array size
fn run_single_benchmark(args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    // Default values
    let mut array_size = 1_000_000;
    let mut max_value = 1_000_000;
    
    // Parse command line arguments
    if args.len() > 1 {
        if let Ok(size) = args[1].parse::<usize>() {
            array_size = size;
        }
    }
    if args.len() > 2 {
        if let Ok(max) = args[2].parse::<i32>() {
            max_value = max;
        }
    }
    
    println!("Benchmarking Merge Sort with array size: {}", array_size);
    println!("Generating random data...");
    
    // Generate random data
    let data = generate_random_data(array_size, max_value);
    
    // Clone data for each algorithm to ensure fair comparison
    let data_for_sequential = data.clone();
    let data_for_parallel = data;
    
    // Run sequential merge sort
    println!("Running sequential merge sort...");
    let sequential_result = sequential_merge_sort(data_for_sequential);
    
    // Run parallel merge sort
    println!("Running parallel merge sort...");
    let parallel_result = parallel_merge_sort(data_for_parallel);
    
    // Compare and print results
    println!("\n{}", compare_sort_results(&sequential_result, &parallel_result));
    
    // Print the first few elements to verify sorting
    println!("\nFirst 10 elements of sorted array:");
    for i in 0..std::cmp::min(10, parallel_result.sorted_data.len()) {
        print!("{} ", parallel_result.sorted_data[i]);
    }
    println!();
    
    Ok(())
}

/// Runs a comprehensive benchmark with various array sizes
fn run_full_benchmark() -> Result<(), Box<dyn std::error::Error>> {
    println!("Running comprehensive benchmark suite...");
    
    // Define sizes to benchmark (logarithmic scale)
    let sizes = vec![
        1_000,
        10_000,
        100_000,
        1_000_000,
        5_000_000,
        10_000_000,
    ];
    
    // Number of runs to average for each size
    let runs_per_size = 3;
    
    // Maximum random value
    let max_value = 1_000_000;
    
    println!("Testing array sizes: {:?}", sizes);
    println!("Runs per size: {}", runs_per_size);
    println!("\nSize        | Sequential   | Parallel     | Speedup");
    println!("----------------------------------------------------------");
    
    // Run benchmarks
    let results = benchmark::run_benchmarks(&sizes, max_value, runs_per_size);
    
    // Plot results
    println!("\nGenerating performance graphs...");
    benchmark::plot_results(&results, "merge_sort_benchmark.png")?;
    println!("Benchmark graphs saved to results/merge_sort_benchmark.png");
    
    Ok(())
}

/// Prints usage information
fn print_usage() {
    println!("Parallel Merge Sort - Performance Benchmark");
    println!("\nUsage:");
    println!("  cargo run [OPTIONS] [ARRAY_SIZE] [MAX_VALUE]");
    println!("\nOptions:");
    println!("  --benchmark    Run comprehensive benchmark with multiple array sizes");
    println!("  --help, -h     Show this help message");
    println!("\nArguments:");
    println!("  ARRAY_SIZE     Size of the array to sort (default: 1,000,000)");
    println!("  MAX_VALUE      Maximum random value (default: 1,000,000)");
}
