# Merge Sort Algorithm Details

## Sequential Merge Sort

### Pseudocode

```
MERGE-SORT(A, p, r)
    if p < r
        q = ⌊(p + r)/2⌋
        MERGE-SORT(A, p, q)
        MERGE-SORT(A, q + 1, r)
        MERGE(A, p, q, r)

MERGE(A, p, q, r)
    n₁ = q - p + 1
    n₂ = r - q
    let L[1..n₁] and R[1..n₂] be new arrays
    for i = 1 to n₁
        L[i] = A[p + i - 1]
    for j = 1 to n₂
        R[j] = A[q + j]
    i = 1
    j = 1
    for k = p to r
        if i > n₁
            A[k] = R[j]
            j = j + 1
        else if j > n₂
            A[k] = L[i]
            i = i + 1
        else if L[i] ≤ R[j]
            A[k] = L[i]
            i = i + 1
        else
            A[k] = R[j]
            j = j + 1
```

### Time Complexity Analysis

The time complexity of merge sort is O(n log n) for all cases:

1. **Dividing**: Dividing the array takes constant time O(1)
2. **Recursive Sorting**: We have 2 recursive calls on subarrays of size n/2
3. **Merging**: Merging takes O(n) time

Using the master theorem, we get T(n) = 2T(n/2) + O(n), which gives us O(n log n).

### Space Complexity Analysis

The space complexity is O(n) due to:

1. **Recursive Call Stack**: O(log n) - the maximum depth of recursion
2. **Temporary Arrays**: O(n) - the auxiliary arrays used during merging

## Parallel Merge Sort

### Parallelization Strategy

The parallel implementation follows the same divide-and-conquer strategy, but takes advantage of multiple cores by:

1. Dividing the problem into subproblems
2. Solving subproblems in parallel on different threads
3. Merging the results sequentially

### Pseudocode

```
PARALLEL-MERGE-SORT(A)
    if length(A) ≤ 1
        return A
    if length(A) ≤ THRESHOLD
        return SEQUENTIAL-MERGE-SORT(A)
    
    mid = length(A) / 2
    left = A[0:mid]
    right = A[mid:length(A)]
    
    in parallel:
        left_sorted = PARALLEL-MERGE-SORT(left)
        right_sorted = PARALLEL-MERGE-SORT(right)
    
    return MERGE(left_sorted, right_sorted)
```

### Threshold Determination

The threshold for switching to sequential sorting is critical for performance. If set too low, the overhead of creating and managing threads exceeds the benefits of parallelism. If set too high, we underutilize available cores.

Our implementation uses a threshold of 4096 elements based on empirical testing, but the optimal value depends on:

1. Hardware characteristics (number of cores, CPU cache size)
2. Memory bandwidth
3. Thread creation and management overhead

### Performance Analysis

#### Amdahl's Law

Parallel speedup is limited by the sequential portion of the algorithm. According to Amdahl's law:

```
Speedup = 1 / (S + (1-S)/P)
```

Where:
- S is the proportion of execution time spent on the sequential part
- P is the number of processors

For merge sort:
- The merge phase is inherently sequential
- The recursive sorting of sub-arrays can be parallelized

#### Work and Span Analysis

- **Work (W)**: Total operations performed = O(n log n)
- **Span (S)**: Length of the critical path = O(log² n)

Theoretical maximum speedup = W/S = O(n / log n)

#### Memory Bandwidth Limitations

For large arrays, performance becomes memory-bound rather than compute-bound. The speedup may plateau when:

1. All CPU cores are saturated
2. Memory bandwidth becomes the bottleneck

## Optimization Techniques

### 1. Cache Efficiency

Our implementation employs these cache optimization techniques:

- **Locality of Reference**: Processing array elements in sequence
- **Sequential Threshold**: Switching to sequential algorithm for cache-friendly sizes

### 2. Work-Stealing Scheduler

Rayon's work-stealing scheduler dynamically balances the workload:

- Each thread maintains its own work queue
- When a thread runs out of work, it "steals" work from other threads' queues
- This minimizes thread idle time and adapts to varying subtask complexity

### 3. Memory Allocation Optimization

- **Pre-allocation**: Allocate merge buffers once and reuse
- **Capacity Hints**: Provide size hints to the allocator

## Memory Access Patterns

Merge sort has predictable memory access patterns:

1. **Division Phase**: Accesses memory in small, localized regions
2. **Merge Phase**: Sequential access to two separate regions, writing to a third

The parallel version maintains these patterns within each thread but introduces potential cache contention between threads. 