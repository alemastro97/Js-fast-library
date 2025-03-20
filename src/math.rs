use wasm_bindgen::prelude::*;

use std::arch::wasm32::*;

/// Computes the mean of an array of f64 values.
///
/// # Parameters
/// - `data`: A slice of f64 numbers for which the mean will be computed.
///
/// # Returns
/// - `f64`: The mean of the values in `data`. If the slice is empty, returns 0.0.
///
/// # Safety
/// Uses `unsafe` for bounds-less access in SIMD operations. Assumes the caller
/// guarantees that the input is a valid slice.
///
/// # Requirements
/// SIMD support for WebAssembly (`target-feature=+simd128`).
#[wasm_bindgen]
pub fn calculate_mean(data: &[f64]) -> f64 {
    let len = data.len();
    if len == 0 {
        return 0.0;
    }

    let mut sum = 0.0;
    let mut i = 0;

    // Sum elements in chunks of 2 using SIMD
    unsafe {
        while i + 2 <= len {
            // Load two f64 values into SIMD register
            let vec = v128_load(data.as_ptr().add(i) as *const v128);
            // Sum the two values in parallel and add to total sum
            let partial_sum = f64x2_extract_lane::<0>(vec) + f64x2_extract_lane::<1>(vec);
            sum += partial_sum;
            i += 2;
        }
    }

    // Sum remaining elements if any
    for j in i..len {
        sum += data[j];
    }

    // Calculate the mean
    sum / len as f64
}


/// Computes the sum of a vector of integers.
///
/// This function takes a vector of `i32` integers and returns their sum. It utilizes
/// Rust's iterator methods to sum the elements efficiently.
///
/// # Arguments
///
/// * `numbers` - A `Vec<i32>` containing the integers to be summed.
///
/// # Returns
///
/// * `i32` - The sum of the integers in the input vector.
///
/// # Performance Considerations
///
/// The `iter()` method creates an iterator over the elements of the vector. The `sum()` method
/// then consumes the iterator to accumulate the total. The time complexity of this operation is
/// O(n), where n is the number of elements in the vector. This is optimal for summing all elements
/// in a collection, as each element must be processed exactly once.
///
/// The function is efficient for relatively small to medium-sized vectors. However, if you need to sum
/// very large vectors (millions of elements), you may want to consider parallel processing for 
/// improved performance, as this implementation does not take advantage of multiple cores or threads.
/// Rust provides tools like `par_iter()` from the `rayon` crate for parallel iteration if needed.
///
/// # Example
/// ```rust
/// let numbers = vec![1, 2, 3, 4, 5];
/// let result = sum(numbers);
/// assert_eq!(result, 15);
/// ```
#[wasm_bindgen]
pub fn sum(numbers: Vec<i32>) -> i32 {
    numbers.iter().sum()
}

/// Computes the factorial of a non-negative integer `n`.
///
/// This function calculates the factorial of `n` using Rust's range and iterator
/// methods. It multiplies all the integers from 1 to `n` (inclusive) to compute the result.
///
/// # Arguments
///
/// * `n` - A non-negative integer (`u32`) for which the factorial is to be computed.
///   The factorial of `n` is the product of all integers from 1 to `n`.
///
/// # Returns
///
/// * `u32` - The factorial of the input integer `n`. The factorial of `n` is the product:
///   ```
///   n! = n * (n-1) * (n-2) * ... * 1
///   ```
///   For example, `5! = 5 * 4 * 3 * 2 * 1 = 120`.
///
/// # Performance Considerations
///
/// The time complexity of this function is O(n), where `n` is the input value. This is because the
/// `product()` method iterates over the range `1..=n`, multiplying the values together.
///
/// The function uses the iterator pattern to compute the product efficiently. The range `1..=n` generates
/// the sequence of integers, and `product()` consumes the iterator to compute the factorial. This is an
/// optimal approach for calculating factorials as it iterates exactly `n` times, performing one multiplication
/// per iteration.
///
/// For very large values of `n`, the result can exceed the maximum value that a `u32` can hold (4,294,967,295),
/// which may lead to an overflow. If you need to compute factorials for larger values, consider using `u64` or
/// `BigInt` (from the `num-bigint` crate).
///
/// # Example
/// ```rust
/// let result = factorial(5);
/// assert_eq!(result, 120);  // 5! = 120
/// ```
#[wasm_bindgen]
pub fn factorial(n: u32) -> u32 {
    (1..=n).product()
}


#[wasm_bindgen]
pub fn max(arr: &[i32]) -> i32 {
    arr.iter().copied().max().unwrap_or(-1) // Return -1 if the array is empty
}

#[wasm_bindgen]
pub fn min(arr: &[i32]) -> i32 {
    arr.iter().copied().min().unwrap_or(-1) // Return -1 if the array is empty
}