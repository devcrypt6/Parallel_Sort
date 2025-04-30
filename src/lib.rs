pub mod benchmark;

// Only import used functions from rayon
use rayon;
use std::time::{Duration, Instant};

/// Structure to store sorting results and metrics
#[derive(Debug, Clone)]
pub struct SortResult<T> {
    pub sorted_data: Vec<T>,
    pub duration: Duration,
    pub algorithm: String,
}
 
/// Sequential merge sort implementation
pub fn sequential_merge_sort<T: Ord + Copy>(mut arr: Vec<T>) -> SortResult<T> {
    let start = Instant::now();
    
    if arr.len() > 1 {
        let len = arr.len();
        _sequential_merge_sort(&mut arr, 0, len - 1);
    }
    
    let duration = start.elapsed();
    
    SortResult {
        sorted_data: arr,
        duration,
        algorithm: "Sequential Merge Sort".to_string(),
    }
}

fn _sequential_merge_sort<T: Ord + Copy>(arr: &mut [T], low: usize, high: usize) {
    if low < high {
        let mid = low + (high - low) / 2;
        
        // Sort first and second halves
        _sequential_merge_sort(arr, low, mid);
        _sequential_merge_sort(arr, mid + 1, high);
        
        // Merge the sorted halves
        merge(arr, low, mid, high);
    }
}

fn merge<T: Ord + Copy>(arr: &mut [T], low: usize, mid: usize, high: usize) {
    let n1 = mid - low + 1;
    let n2 = high - mid;
    
    // Create temporary arrays
    let mut left = Vec::with_capacity(n1);
    let mut right = Vec::with_capacity(n2);
    
    // Copy data to temporary arrays
    for i in 0..n1 {
        left.push(arr[low + i]);
    }
    
    for i in 0..n2 {
        right.push(arr[mid + 1 + i]);
    }
    
    // Merge the temporary arrays back
    let mut i = 0;
    let mut j = 0;
    let mut k = low;
    
    while i < n1 && j < n2 {
        if left[i] <= right[j] {
            arr[k] = left[i];
            i += 1;
        } else {
            arr[k] = right[j];
            j += 1;
        }
        k += 1;
    }
    
    // Copy remaining elements of left array if any
    while i < n1 {
        arr[k] = left[i];
        i += 1;
        k += 1;
    }
    
    // Copy remaining elements of right array if any
    while j < n2 {
        arr[k] = right[j];
        j += 1;
        k += 1;
    }
}

/// Parallel merge sort implementation using Rayon
pub fn parallel_merge_sort<T: Ord + Copy + Send + Sync>(mut arr: Vec<T>) -> SortResult<T> {
    let start = Instant::now();
    
    if arr.len() > 1 {
        _parallel_merge_sort(&mut arr);
    }
    
    let duration = start.elapsed();
    
    SortResult {
        sorted_data: arr,
        duration,
        algorithm: "Parallel Merge Sort".to_string(),
    }
}

fn _parallel_merge_sort<T: Ord + Copy + Send + Sync>(arr: &mut [T]) {
    const SEQUENTIAL_THRESHOLD: usize = 4096;
    
    if arr.len() <= 1 {
        return;
    }
    
    // Use sequential algorithm for small arrays to avoid overhead
    let arr_len = arr.len();
    if arr_len <= SEQUENTIAL_THRESHOLD {
        _sequential_merge_sort(arr, 0, arr_len - 1);
        return;
    }
    
    let mid = arr_len / 2;
    
    // Split the array and sort both halves in parallel
    let (left, right) = arr.split_at_mut(mid);
    
    rayon::join(
        || _parallel_merge_sort(left),
        || _parallel_merge_sort(right)
    );
    
    // Create a temporary array for merging
    let mut merged = Vec::with_capacity(arr_len);
    
    // Need to clone the slices to avoid borrowing issues
    let left_clone = left.to_vec();
    let right_clone = right.to_vec();
    
    let mut left_index = 0;
    let mut right_index = 0;
    
    // Merge the sorted halves
    while left_index < left_clone.len() && right_index < right_clone.len() {
        if left_clone[left_index] <= right_clone[right_index] {
            merged.push(left_clone[left_index]);
            left_index += 1;
        } else {
            merged.push(right_clone[right_index]);
            right_index += 1;
        }
    }
    
    // Copy any remaining elements
    merged.extend_from_slice(&left_clone[left_index..]);
    merged.extend_from_slice(&right_clone[right_index..]);
    
    // Copy the merged result back to the original array
    arr.copy_from_slice(&merged);
}

/// Utility function to generate a random vector of integers
pub fn generate_random_data(size: usize, max_value: i32) -> Vec<i32> {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    
    (0..size).map(|_| rng.gen_range(0..max_value)).collect()
}

/// Compares results of two sorting algorithms and returns a string with the comparison
pub fn compare_sort_results<T: Ord + PartialEq>(result1: &SortResult<T>, result2: &SortResult<T>) -> String {
    let is_equal = result1.sorted_data == result2.sorted_data;
    let speedup = result1.duration.as_secs_f64() / result2.duration.as_secs_f64();
    
    let validation = if is_equal {
        "Results are identical ✓"
    } else {
        "ERROR: Results are different ✗"
    };
    
    format!(
        "Performance Comparison:\n\
        {}: {:.6} seconds\n\
        {}: {:.6} seconds\n\
        Speedup: {:.2}x\n\
        {}",
        result1.algorithm, result1.duration.as_secs_f64(),
        result2.algorithm, result2.duration.as_secs_f64(),
        speedup,
        validation
    )
} 
