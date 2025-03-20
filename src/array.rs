use wasm_bindgen::prelude::*;

/// Sorts a vector of integers using the quicksort algorithm.
///
/// The `quick_sort` function implements the classic quicksort algorithm to sort a vector of integers
/// in ascending order. It recursively partitions the array around a pivot element, then sorts the
/// sub-arrays to the left and right of the pivot. Finally, the function concatenates the results
/// to produce the sorted array.
///
/// The pivot element is chosen as the last element of the vector for simplicity. This function
/// has a time complexity of O(n log n) on average, but can degrade to O(n^2) in the worst case if the
/// pivot selection consistently results in unbalanced partitions.
///
/// # Arguments
///
/// * `arr` - A vector of integers (`Vec<i32>`) to be sorted.
///
/// # Returns
///
/// * `Vec<i32>` - A new vector containing the sorted integers from the input vector in ascending order.
///
/// # Example
/// ```rust
/// let unsorted_array = vec![5, 2, 9, 1, 5, 6];
/// let sorted_array = quick_sort(unsorted_array);
/// println!("{:?}", sorted_array); // Outputs: [1, 2, 5, 5, 6, 9]
/// ```
///
/// # Performance Considerations
///
/// - **Average Time Complexity**: The average time complexity of quicksort is O(n log n), where `n` is the
///   number of elements in the vector. The algorithm works by partitioning the array into sub-arrays around a pivot,
///   and then recursively sorting those sub-arrays.
///
/// - **Worst-Case Time Complexity**: The worst-case time complexity occurs when the pivot chosen results in
///   unbalanced partitions (e.g., the pivot is always the smallest or largest element). In such cases, the algorithm
///   can degrade to O(n^2).
///
/// - **Memory Usage**: The function is recursive and creates new vectors during each partitioning step. Thus, the
///   memory usage grows with the size of the input array, and the function has a space complexity of O(n) in the worst case,
///   due to the stack depth from recursion.
#[wasm_bindgen]
pub fn quick_sort(mut arr: Vec<i32>) -> Vec<i32> {
    // Base case: an array of length 0 or 1 is already sorted
    if arr.len() <= 1 {
        return arr;
    }

    // Choose a pivot (for simplicity, choose the last element)
    let pivot = arr.pop().unwrap(); // Removes and returns the last element as pivot

    // Partition into two arrays: one for elements < pivot, and one for elements >= pivot
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for x in arr {
        if x < pivot {
            left.push(x);
        } else {
            right.push(x);
        }
    }

    // Recursively sort the left and right arrays
    let mut sorted_left = quick_sort(left);
    let sorted_right = quick_sort(right);

    // Combine sorted left, pivot, and sorted right into a single sorted array
    sorted_left.push(pivot); // Insert the pivot back in
    sorted_left.extend(sorted_right); // Append sorted right

    sorted_left // Return the fully sorted array
}

/// Sorts a vector of integers using the MergeSort algorithm.
///
/// This function implements the MergeSort algorithm, a comparison-based sorting algorithm
/// that follows the divide-and-conquer paradigm. The input array is recursively divided into
/// smaller subarrays, each of which is sorted and merged back together to produce the final sorted array.
///
/// MergeSort is known for its stable sorting behavior and predictable O(n log n) time complexity.
///
/// # Arguments
///
/// * `arr` - A mutable vector `Vec<i32>` containing the list of integers to be sorted.
///
/// # Returns
///
/// * `Vec<i32>` - A sorted vector of integers in ascending order.
///
/// # Example
/// ```rust
/// let arr = vec![3, 1, 4, 1, 5, 9, 2, 6, 5];
/// let sorted_arr = merge_sort(arr);
/// println!("{:?}", sorted_arr); // Outputs: [1, 1, 2, 3, 4, 5, 5, 6, 9]
/// ```
///
/// # Performance Considerations
///
/// MergeSort has a time complexity of O(n log n), where `n` is the number of elements in the input vector.
/// It is considered efficient for large datasets as it guarantees worst-case O(n log n) performance.
/// However, MergeSort requires O(n) extra space for the auxiliary array, making it less memory efficient than some
/// other algorithms (e.g., QuickSort), which can operate in-place.
///
/// - **Time Complexity**: O(n log n) in all cases (best, worst, average).
/// - **Space Complexity**: O(n) due to the extra space used for the auxiliary array.
#[wasm_bindgen]
pub fn merge_sort(mut arr: Vec<i32>) -> Vec<i32> {
    let len = arr.len();
    if len <= 1 {
        return arr;
    }

    let mut aux = arr.clone(); // Temporary auxiliary array for merging
    merge_sort_recursive(&mut arr, &mut aux, 0, len);
    arr
}
/// Recursively splits and sorts the array using MergeSort.
///
/// This function divides the array into smaller subarrays and then merges them back together.
///
/// # Arguments
///
/// * `arr` - The array to be sorted.
/// * `aux` - The auxiliary array used for temporary storage during merging.
/// * `start` - The starting index of the subarray being processed.
/// * `end` - The ending index of the subarray being processed.
///
/// # Returns
///
/// This function does not return a value, as it operates in-place on the array.
fn merge_sort_recursive(arr: &mut [i32], aux: &mut [i32], start: usize, end: usize) {
    if end - start <= 1 {
        return;
    }

    let mid = (start + end) / 2;

    // Recursively sort both halves
    merge_sort_recursive(aux, arr, start, mid);
    merge_sort_recursive(aux, arr, mid, end);

    // Merge sorted halves
    merge(arr, aux, start, mid, end);
}

/// Merges two sorted subarrays back into a single sorted array.
///
/// This helper function takes two sorted subarrays and merges them into a single sorted subarray.
///
/// # Arguments
///
/// * `arr` - The array being sorted, which will hold the final merged result.
/// * `aux` - The auxiliary array containing the sorted subarrays.
/// * `start` - The starting index of the left subarray.
/// * `mid` - The ending index of the left subarray, which is the starting index of the right subarray.
/// * `end` - The ending index of the right subarray.
///
/// # Returns
///
/// This function does not return a value, as it operates in-place on the array.
fn merge(arr: &mut [i32], aux: &[i32], start: usize, mid: usize, end: usize) {
    let (mut left, mut right) = (start, mid);
    let mut idx = start;

    // Merge elements from aux (sorted) back into arr
    while left < mid && right < end {
        if aux[left] <= aux[right] {
            arr[idx] = aux[left];
            left += 1;
        } else {
            arr[idx] = aux[right];
            right += 1;
        }
        idx += 1;
    }

    // Copy any remaining elements from the left half
    if left < mid {
        arr[idx..end].copy_from_slice(&aux[left..mid]);
    }
    // If there are remaining elements in the right half, they are already in place
}