use crate::{
    generate_random_data, parallel_merge_sort, sequential_merge_sort,
};
use plotters::prelude::*;
use std::error::Error;
use std::time::Duration;

pub struct BenchmarkResult {
    pub sizes: Vec<usize>,
    pub sequential_times: Vec<Duration>,
    pub parallel_times: Vec<Duration>,
    pub speedups: Vec<f64>,
}

/// Run benchmarks on various array sizes
pub fn run_benchmarks(sizes: &[usize], max_value: i32, runs_per_size: usize) -> BenchmarkResult {
    let mut sequential_times = Vec::with_capacity(sizes.len());
    let mut parallel_times = Vec::with_capacity(sizes.len());
    let mut speedups = Vec::with_capacity(sizes.len());

    for &size in sizes {
        let mut seq_total = Duration::new(0, 0);
        let mut par_total = Duration::new(0, 0);

        for _ in 0..runs_per_size {
            // Generate a new random array for each run
            let data = generate_random_data(size, max_value);
            
            // Run sequential sort
            let seq_result = sequential_merge_sort(data.clone());
            seq_total += seq_result.duration;
            
            // Run parallel sort
            let par_result = parallel_merge_sort(data);
            par_total += par_result.duration;
            
            // Verify results match
            assert_eq!(seq_result.sorted_data, par_result.sorted_data);
        }
        
        // Calculate average durations
        let seq_avg = seq_total / runs_per_size as u32;
        let par_avg = par_total / runs_per_size as u32;
        
        // Calculate speedup
        let speedup = seq_avg.as_secs_f64() / par_avg.as_secs_f64();
        
        sequential_times.push(seq_avg);
        parallel_times.push(par_avg);
        speedups.push(speedup);
        
        println!(
            "Size: {:9} | Sequential: {:7.3} s | Parallel: {:7.3} s | Speedup: {:.2}x",
            size,
            seq_avg.as_secs_f64(),
            par_avg.as_secs_f64(),
            speedup
        );
    }
    
    BenchmarkResult {
        sizes: sizes.to_vec(),
        sequential_times,
        parallel_times,
        speedups,
    }
}

/// Plot benchmark results to a file
pub fn plot_results(results: &BenchmarkResult, output_file: &str) -> Result<(), Box<dyn Error>> {
    // Create the output directory if it doesn't exist
    std::fs::create_dir_all("./results")?;
    let output_path = format!("./results/{}", output_file);
    
    // Setup the plotting area
    let root = BitMapBackend::new(&output_path, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;
    
    // Create two chart areas
    let root = root.split_vertically(300);
    
    // Plot the timing comparison
    {
        let mut chart = ChartBuilder::on(&root.0)
            .caption("Sort Duration vs Array Size", ("sans-serif", 20).into_font())
            .margin(10)
            .x_label_area_size(40)
            .y_label_area_size(60)
            .build_cartesian_2d(
                (results.sizes[0] as f64..*results.sizes.last().unwrap() as f64).log_scale(),
                0f64..results.sequential_times.iter().map(|d| d.as_secs_f64()).fold(0f64, f64::max) * 1.1,
            )?;
            
        chart
            .configure_mesh()
            .x_desc("Array Size (log scale)")
            .y_desc("Time (seconds)")
            .draw()?;
            
        // Plot sequential times
        chart.draw_series(LineSeries::new(
            results.sizes.iter().zip(results.sequential_times.iter())
                .map(|(&size, &time)| (size as f64, time.as_secs_f64())),
            &RED,
        ))?
        .label("Sequential")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
        
        // Plot parallel times
        chart.draw_series(LineSeries::new(
            results.sizes.iter().zip(results.parallel_times.iter())
                .map(|(&size, &time)| (size as f64, time.as_secs_f64())),
            &BLUE,
        ))?
        .label("Parallel")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));
        
        chart
            .configure_series_labels()
            .background_style(&WHITE.mix(0.8))
            .border_style(&BLACK)
            .draw()?;
    }
    
    // Plot the speedup
    {
        let mut chart = ChartBuilder::on(&root.1)
            .caption("Speedup vs Array Size", ("sans-serif", 20).into_font())
            .margin(10)
            .x_label_area_size(40)
            .y_label_area_size(60)
            .build_cartesian_2d(
                (results.sizes[0] as f64..*results.sizes.last().unwrap() as f64).log_scale(),
                0f64..results.speedups.iter().cloned().fold(0f64, f64::max) * 1.1,
            )?;
            
        chart
            .configure_mesh()
            .x_desc("Array Size (log scale)")
            .y_desc("Speedup Factor (x times)")
            .draw()?;
            
        chart.draw_series(LineSeries::new(
            results.sizes.iter().zip(results.speedups.iter())
                .map(|(&size, &speedup)| (size as f64, speedup)),
            &GREEN,
        ))?;
        
        // Draw a reference line at speedup = 1.0
        chart.draw_series(LineSeries::new(
            vec![(results.sizes[0] as f64, 1.0), (*results.sizes.last().unwrap() as f64, 1.0)],
            &BLACK.mix(0.5),
        ))?;
    }
    
    Ok(())
} 