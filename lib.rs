use std::time::{Duration, Instant};
use rayon;


/// Structure to store sorting results and metrics
#[derive(Debug, Clone)]
pub struct SortResult<T> {
    pub sorted_data: Vec<T>,
    pub duration: Duration,
    pub algorithm: String,
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
