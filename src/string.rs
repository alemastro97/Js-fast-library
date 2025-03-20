use wasm_bindgen::prelude::*;
use std::collections::HashMap;

use serde_wasm_bindgen::to_value;
/// Calculates the frequency of each word in a given text and returns the result as a JavaScript object.
///
/// This function takes a string of text and counts the occurrences of each unique word. It splits the input
/// text into words by whitespace and then counts how many times each word appears. The result is returned as a
/// JavaScript object (a `JsValue`), which contains the word frequencies in the form of key-value pairs where the
/// key is the word (a string) and the value is the frequency (an integer).
///
/// The function utilizes `serde_wasm_bindgen::to_value` to serialize the Rust `HashMap` of word frequencies into
/// a format that can be used in JavaScript.
///
/// # Arguments
///
/// * `text` - A string slice (`&str`) containing the input text for which word frequencies are to be calculated.
///
/// # Returns
///
/// * `JsValue` - A JavaScript object representing the word frequencies. Each key in the object is a word (string),
///   and each value is the frequency (integer) of that word in the text.
///
/// # Example
/// ```rust
/// let input_text = "hello world hello";
/// let word_freq = word_frequency(input_text);
/// console.log(word_freq); // Outputs: {hello: 2, world: 1}
/// ```
///
/// # Performance Considerations
///
/// The time complexity of this function is O(n), where `n` is the number of words in the input text. The function
/// processes each word exactly once and stores the frequency in a hash map. Therefore, the time taken is linear
/// with respect to the number of words.
///
/// - **Word Splitting**: The `split_whitespace` method is used to break the text into words. This operation is
///   O(n) with respect to the length of the text, where `n` is the number of characters in the text.
///
/// - **HashMap Operations**: Inserting or updating a word in the `HashMap` takes O(1) on average, making the
///   word frequency counting efficient.
#[wasm_bindgen]
pub fn word_frequency(text: &str) -> JsValue {
    // Count word frequencies
    let mut freq = HashMap::new();
    for word in text.split_whitespace() {
        *freq.entry(word).or_insert(0) += 1;
    }

    // Serialize the HashMap to a JsValue using serde_wasm_bindgen::to_value
    to_value(&freq).unwrap()
}

