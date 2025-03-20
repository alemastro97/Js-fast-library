
use wasm_bindgen::prelude::*;


/// Counts the total number of zero bits in the provided data.
///
/// This function takes a slice of bytes (`&[u8]`) and calculates the total number of zero bits (0s)
/// in all the bytes. It uses the `count_zeros` method on each byte to determine how many bits in
/// that byte are zero, then sums these counts for all the bytes in the slice.
///
/// The function operates by iterating over each byte in the slice, using the `count_zeros` method to
/// determine the number of zero bits in each byte, and returns the total count as a `u32`.
///
/// # Arguments
///
/// * `data` - A byte slice (`&[u8]`) containing the data to analyze. Each byte in the slice will be checked for zero bits.
///
/// # Returns
///
/// * `u32` - The total number of zero bits across all the bytes in the `data` slice. The result is a non-negative integer.
///
/// # Example
/// ```rust
/// let input_data = [0b11001100, 0b10101010, 0b11110000];
/// let zero_bits_count = count_zero_bits(&input_data);
/// println!("Zero bits count: {}", zero_bits_count); // Outputs the total zero bits in the byte slice
/// ```
///
/// # Performance Considerations
///
/// The time complexity of the `count_zero_bits` function is O(n), where `n` is the number of bytes
/// in the input data. The function processes each byte in the slice exactly once and uses the `count_zeros`
/// method, which is an O(1) operation for each byte since the byte is always 8 bits long. Therefore, the
/// time taken grows linearly with the size of the input data.
///
/// - **Iteration and Counting**: Each byte is iterated once, and the number of zero bits is computed
///   using the efficient `count_zeros` method, which runs in constant time (O(1)) for each byte.
///   
/// - **Memory Usage**: The memory usage is minimal and proportional to the size of the input slice. No
///   additional significant memory is allocated beyond the input data.
#[wasm_bindgen]
pub fn count_zero_bits(data: &[u8]) -> u32 {
    data.iter().map(|&byte| byte.count_zeros()).sum()
}

/// Performs a bitwise XOR operation on two byte slices and returns the result as a vector of bytes.
///
/// This function takes two byte slices (`&[u8]`), applies a bitwise XOR operation on corresponding bytes
/// from each slice, and returns the resulting bytes as a `Vec<u8>`. The XOR operation is performed pairwise
/// on each byte of the two input slices. If the slices are of unequal lengths, the operation will be limited
/// to the length of the shorter slice.
///
/// XOR (exclusive OR) is a bitwise operation that compares corresponding bits of two values:
/// - If the bits are the same, the result is `0`.
/// - If the bits are different, the result is `1`.
///
/// # Arguments
///
/// * `data1` - The first byte slice (`&[u8]`) to apply the XOR operation on.
/// * `data2` - The second byte slice (`&[u8]`) to apply the XOR operation on.
///
/// # Returns
///
/// * `Vec<u8>` - A vector of bytes representing the result of applying the XOR operation to corresponding bytes
///   from `data1` and `data2`. The length of the resulting vector is the same as the length of the shorter input slice.
///
/// # Example
/// ```rust
/// let data1 = [0b11010101, 0b10101010];
/// let data2 = [0b01100011, 0b11110000];
/// let result = xor_bytes(&data1, &data2);
/// println!("{:?}", result); // Outputs: [0b10110110, 0b11011010]
/// ```
///
/// # Performance Considerations
///
/// The time complexity of the `xor_bytes` function is O(n), where `n` is the length of the shorter input slice
/// (`data1` or `data2`). The function iterates through each byte in the two slices and performs the XOR operation
/// on the corresponding bytes. Since the XOR operation itself is O(1) for each byte, the overall time complexity
/// is linear in relation to the length of the shorter input slice.
///
/// - **Iteration and XOR**: The `iter()` and `zip()` methods are used to pair corresponding bytes from the two
///   slices, and `map()` applies the XOR operation on each pair. Both operations are O(n), where `n` is the length
///   of the shorter slice.
#[wasm_bindgen]
pub fn xor_bytes(data1: &[u8], data2: &[u8]) -> Vec<u8> {
    data1.iter().zip(data2.iter()).map(|(&x1, &x2)| x1 ^ x2).collect()
}


