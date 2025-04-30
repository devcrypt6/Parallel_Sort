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

/// Utility function to generate a random vector of integers
pub fn generate_random_data(size: usize, max_value: i32) -> Vec<i32> {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    
    (0..size).map(|_| rng.gen_range(0..max_value)).collect()
}
