use wasm_bindgen::prelude::*;


use std::arch::wasm32::*;
/// Inverts the color values of a given array of bytes.
///
/// This function performs an in-place inversion of each byte in the input array `bytes`.
/// Each byte is assumed to represent a color component (e.g., RGB or grayscale values) where 0 is black
/// and 255 is white. Inversion is achieved by subtracting each byte from 255.
///
/// # Parameters
/// - `bytes`: A mutable slice of bytes representing color values. After this function completes,
///   each byte in `bytes` will be replaced with its inverted value (255 - original value).
///
/// # Performance
/// This function is optimized for high performance, utilizing SIMD (Single Instruction, Multiple Data)
/// instructions where available to invert colors in chunks of 16 bytes at a time. For inputs not divisible
/// by 16, any remaining bytes are inverted in a secondary loop. The SIMD optimization is made possible
/// through WebAssembly SIMD (128-bit wide) instructions, which must be supported by the target environment
/// (e.g., modern web browsers with WebAssembly SIMD support).
///
/// # Safety
/// The function uses `unsafe` blocks to bypass bounds checks in two cases:
/// - When processing data in 16-byte chunks with SIMD, where it is ensured that `i + 16 <= len`.
/// - When handling any remaining bytes individually after the main SIMD loop, using `get_unchecked_mut`
///   to maximize speed by eliminating bounds checks.
///
/// Although `unsafe` is used, this function is safe to call as it performs internal bounds management.
///
/// # Examples
/// ```rust
/// use your_module::invert_colors;
///
/// let mut colors = vec![0, 128, 255, 100, 50];
/// invert_colors(&mut colors);
/// assert_eq!(colors, vec![255, 127, 0, 155, 205]);
/// ```
///
/// # Requirements
/// To use this function in a WebAssembly context with SIMD, make sure to compile with
/// `-C target-feature=+simd128`.
///
/// # Panics
/// This function does not panic, as it manages empty input by exiting early.
///
/// # Compatibility
/// This function requires a WebAssembly environment with SIMD enabled.
/// Older environments or those without SIMD support may experience reduced performance.
#[wasm_bindgen]
pub fn invert_colors(bytes: &mut [u8]) {
    let len = bytes.len();
    let mut i = 0;

    // Invert colors in chunks of 16 bytes using SIMD for better performance
    unsafe {
        while i + 16 <= len {
            // Load 16 bytes into a SIMD vector
            let vec = v128_load(bytes.as_ptr().add(i) as *const v128);
            // Subtract each byte from 255 in parallel
            let inverted_vec = u8x16_sub(u8x16_splat(255), vec);
            // Store the inverted result back into the byte array
            v128_store(bytes.as_mut_ptr().add(i) as *mut v128, inverted_vec);
            i += 16;
        }
    }

    // Invert remaining bytes individually without bounds checks for extra performance
    unsafe {
        for j in i..len {
            *bytes.get_unchecked_mut(j) = 255 - *bytes.get_unchecked(j);
        }
    }
}

/// Converts an image represented as a flat array of RGBA pixels into grayscale while
/// preserving the alpha (transparency) channel. Each pixel is converted to a single
/// grayscale value based on the red, green, and blue channels using a weighted average
/// formula. The alpha channel remains unchanged.
///
/// # Arguments
///
/// * `input` - A slice of bytes representing an image where each set of 4 consecutive
///   bytes corresponds to a single pixel in the format `[R, G, B, A]`. `R` is the red 
///   channel, `G` is the green channel, `B` is the blue channel, and `A` is the alpha channel.
///
///   The input data should be a flat array where the number of elements is a multiple of 4,
///   and each pixel is represented by 4 bytes: `[R, G, B, A]`.
///
/// # Returns
///
/// * A `Vec<u8>` representing the grayscale image data in the same RGBA format. The grayscale 
///   value for the red, green, and blue channels is calculated using the luminance formula:
///   ```
///   Gray = (0.299 * R) + (0.587 * G) + (0.114 * B)
///   ```
///   The alpha channel remains unchanged.
///
/// # Example
/// ```rust
/// let input_image: Vec<u8> = vec![
///     255, 0, 0, 255,  // Red pixel (opaque)
///     0, 255, 0, 255,  // Green pixel (opaque)
///     0, 0, 255, 255,  // Blue pixel (opaque)
///     255, 255, 255, 255, // White pixel (opaque)
/// ];
/// let output_image = grayscale(&input_image);
/// ```
///
/// After calling `grayscale`, the `output_image` will contain the grayscale equivalents of
/// the input pixels, with their alpha channel intact.
///
#[wasm_bindgen]
pub fn grayscale(input: &[u8]) -> Vec<u8> {
    input.chunks(4).flat_map(|pixel| {
        let gray = (0.299 * pixel[0] as f32 + 0.587 * pixel[1] as f32 + 0.114 * pixel[2] as f32) as u8;
        vec![gray, gray, gray, pixel[3]]
    }).collect()
}

