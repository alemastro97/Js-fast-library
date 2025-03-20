


use wasm_bindgen::prelude::*;
use csv::{ReaderBuilder, StringRecord};

use serde_json;
/// Parses a CSV string into a vector of JSON objects (JsValue).
///
/// This function takes a string containing CSV data and converts each record into a JSON object. It processes
/// the CSV content by reading it line by line, converting each record into a `JsValue` object, and then returning
/// the collection of these objects as a `Result<Vec<JsValue>, JsValue>`. If any errors occur during the CSV parsing
/// or JSON conversion, they are propagated as `JsValue` errors.
///
/// The CSV format is expected to have rows of data separated by newlines, and each field in a row is separated
/// by commas. Each row is converted into a JSON array, with each field of the row becoming an element in the array.
///
/// # Arguments
///
/// * `content` - A string containing the CSV data to be parsed. Each line of the string should represent a row
///   in the CSV, and each value in the row should be separated by commas.
///
/// # Returns
///
/// * `Result<Vec<JsValue>, JsValue>` - A result containing either:
///     - `Ok(Vec<JsValue>)`: A vector of `JsValue` objects representing the parsed CSV records.
///     - `Err(JsValue)`: A `JsValue` error, which contains a string representation of the error message.
///
/// # Example
/// ```rust
/// let csv_content = "name,age,city\nJohn,30,New York\nAlice,25,Los Angeles";
/// let json_result = parse_csv_to_json(csv_content.to_string());
/// match json_result {
///     Ok(json_records) => {
///         for record in json_records {
///             println!("{}", record.as_string().unwrap());
///         }
///     }
///     Err(e) => {
///         console_error!("{}", e.as_string().unwrap());
///     }
/// }
/// ```
///
/// # Performance Considerations
///
/// The time complexity of this function depends on the size of the CSV content. Specifically, it processes each row
/// and each field in the CSV content, resulting in a time complexity of O(n), where `n` is the total number of fields
/// in the CSV. The function also performs serialization into JSON format, which adds some overhead.
///
/// - **Memory Usage**: The memory usage grows with the size of the CSV content and the number of rows and fields.
///   Each row is converted into a JSON object, and the resulting `JsValue` objects are stored in a vector, which may
///   be memory-intensive for large CSV files.
#[wasm_bindgen]
pub fn parse_csv_to_json(content: String) -> Result<Vec<JsValue>, JsValue> {
    // Create a CSV reader from the content string
    let mut rdr = ReaderBuilder::new().from_reader(content.as_bytes());

    // Create a vector to store the JSON objects
    let mut records: Vec<JsValue> = Vec::new();

    // Process the records
    for result in rdr.records() {
        let record = result.map_err(|e| JsValue::from_str(&e.to_string()))?;
        
        // Convert the record to a JSON object (JsValue)
        let json = record_to_json(&record)?;
        
        // Add the JSON object to the results
        records.push(json);
    }

    // Return the vector of records as JsValue
    Ok(records)
}

/// Converts a single CSV record to a JSON object (JsValue).
///
/// This helper function takes a CSV record represented as a `StringRecord`, converts each field in the record
/// into a string, and then serializes the record as a JSON array. The resulting JSON array is returned as a `JsValue`.
/// The function returns an error as `JsValue` if the conversion or serialization fails.
///
/// # Arguments
///
/// * `record` - A reference to a `StringRecord` representing a single row from the CSV data. This record contains
///   the fields (as strings) of the CSV row.
///
/// # Returns
///
/// * `Result<JsValue, JsValue>` - A result containing either:
///     - `Ok(JsValue)`: The serialized JSON representation of the CSV record as a `JsValue`.
///     - `Err(JsValue)`: A `JsValue` error if serialization fails.
///
/// # Example
/// ```rust
/// let record = StringRecord::from(vec!["John", "30", "New York"]);
/// let json_result = record_to_json(&record);
/// match json_result {
///     Ok(json) => println!("{}", json.as_string().unwrap()),
///     Err(e) => console_error!("{}", e.as_string().unwrap()),
/// }
/// ```
///
/// # Performance Considerations
///
/// The time complexity of this function is O(m), where `m` is the number of fields in the CSV record. Each field is
/// processed individually, and the function performs serialization into JSON format for the entire record. The memory
/// usage is proportional to the number of fields in the record since each field is copied into a vector and serialized.
/// 
/// The function may incur additional overhead due to serialization, particularly for large records or complex data.
fn record_to_json(record: &StringRecord) -> Result<JsValue, JsValue> {
    // Convert the record to a vector of strings
    let values: Vec<String> = record.iter().map(|s| s.to_string()).collect();
    
    // Convert the vector to a JSON value
    let json = serde_json::to_string(&values)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    
    // Return the JSON object as JsValue
    Ok(JsValue::from_str(&json))
}
