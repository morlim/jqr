use jsonpath_rust::{JsonPath, JsonPathValue};
use serde_json::Value;
use serde_yaml;
use colored::*;


/// Pretty prints a JSON string with optional JSONPath querying.
/// 
/// # Arguments
/// 
/// * `content` - A string slice containing the JSON content to format.
/// * `query` - An optional JSONPath query string to filter the JSON data.
/// 
/// # Returns
/// 
/// * `Ok(String)` - The formatted JSON string.
/// * `Err(String)` - An error message if parsing or formatting fails.
///
/// # Dependencies
/// 
/// This function relies on `serde_json` for JSON parsing and serialization.
/// It also assumes the existence of an `extract_jsonpath()` function that
/// applies a JSONPath query to filter the JSON data.
/// 
/// # Errors
/// 
/// * Returns an error if the input JSON is invalid.
/// * Returns an error if serialization to pretty-printed JSON fails.
/// 
/// # Example
/// 
/// ```
/// use jqr::pretty_print_json; // Ensure this is correctly importing from your crate
/// let json_str = r#"{"name": "Alice", "age": 25}"#;
/// let formatted = pretty_print_json(json_str, None);
/// println!("{}", formatted.unwrap());
/// ```
pub fn pretty_print_json(content: &str, query: Option<&String>) -> Result<String, String> {
    // Attempt to parse the input string into a JSON `Value`
    match serde_json::from_str::<serde_json::Value>(content) {
        Ok(json) => {
            // If a query is provided, extract the relevant JSON data
            let result = if let Some(q) = query {
                extract_jsonpath(&json, q) // Assuming `extract_jsonpath()` processes JSONPath queries
            } else {
                json
            };

            // Serialize the JSON value to a pretty-printed string
            serde_json::to_string_pretty(&result)
                .map_err(|e| format!("Serialization error: {}", e))
        }
        Err(e) => Err(format!("{}", format!("Invalid JSON: {}", e.to_string()).red())),
    }
}


/// Extracts data from a JSON structure based on a JSONPath query.
///
/// This function takes a `serde_json::Value` (parsed JSON) and a JSONPath query,
/// then attempts to extract matching values from the JSON structure.
///
/// - If the JSONPath query is **valid**, it searches for matching values:
///   - If no matches are found, it returns `"No results found"`.
///   - If exactly **one** match is found, it returns the single extracted value.
///   - If **multiple** matches are found, it returns an array of extracted values.
/// - If the JSONPath query is **invalid**, it returns `"Invalid JSONPath query"`.
///
/// # Parameters
///
/// - `json`: A reference to a `serde_json::Value` containing the JSON data.
/// - `query`: A string slice representing the JSONPath query.
///
/// # Returns
///
/// Returns a `serde_json::Value`:
/// - A **single value**, if one match is found.
/// - An **array of values**, if multiple matches are found.
/// - A **string message**, if no matches are found or if the query is invalid.
///
/// # Examples
///
/// ```
/// use jqr::extract_jsonpath;
/// use serde_json::json;
///
/// let json_data = json!({
///     "name": "Alice",
///     "age": 25,
///     "pets": [
///         { "type": "dog", "name": "Buddy" },
///         { "type": "cat", "name": "Whiskers" }
///     ]
/// });
///
/// let query = "$.pets[*].name"; // JSONPath query to get all pet names
/// let result = extract_jsonpath(&json_data, query);
///
/// assert_eq!(result, json!(["Buddy", "Whiskers"])); // Expected output
/// ```
///
/// ```
/// use jqr::extract_jsonpath;
/// use serde_json::json;
///
/// let json_data = json!({ "name": "Alice", "age": 25 });
///
/// let invalid_query = "$..[?(@.missing)]"; // Invalid JSONPath query
/// let result = extract_jsonpath(&json_data, invalid_query);
///
/// assert_eq!(result, json!("Invalid JSONPath query"));
/// ```
pub fn extract_jsonpath(json: &Value, query: &str) -> Value {
    // Attempt to parse the JSONPath query
    match JsonPath::try_from(query) {
        Ok(path) => {
            // Execute the query and collect results
            let results: Vec<JsonPathValue<Value>> = path.find_slice(json);
            
            if results.is_empty() {
                // No matches found, return a string message
                Value::String("No results found".to_string())
            } else if results.len() == 1 {
                // Single result: convert and return the value
                let converted: Value = json_path_value_to_json(results[0].clone());
                converted.clone()
            } else {
                // Multiple results: convert each and return as an array
                Value::Array(
                    results.into_iter()
                        .map(|jp_value| json_path_value_to_json(jp_value))
                        .collect()
                )
            }
        }
        Err(_) => {
            // Invalid JSONPath query, return an error message
            Value::String("Invalid JSONPath query".to_string())
        }
    }
}

/// Converts a `JsonPathValue<Value>` into a `serde_json::Value`.
///
/// This function takes a `JsonPathValue<Value>` and returns a `serde_json::Value`,
/// handling different cases based on the input variant:
///
/// - `JsonPathValue::Slice(v, _)`: Clones the provided value.
/// - `JsonPathValue::NewValue(v)`: Takes ownership of the new value.
/// - `JsonPathValue::NoValue`: Returns `Value::Null` when no value exists.
///
/// # Examples
///
/// ```
/// use jqr::json_path_value_to_json;
/// use jsonpath_rust::JsonPathValue;
/// use serde_json::Value;
///
/// let value = JsonPathValue::NewValue(Value::String("example".to_string()));
/// let result = json_path_value_to_json(value);
///
/// assert_eq!(result, Value::String("example".to_string()));
/// ```
pub fn json_path_value_to_json(value: JsonPathValue<Value>) -> Value {
    match value {
        JsonPathValue::Slice(v, _) => v.clone(), // Clone the reference, ignore path
        JsonPathValue::NewValue(v) => v, // Take ownership and return
        JsonPathValue::NoValue => Value::Null, // No value found, return JSON null
    }
}

/// Converts a JSON string into a YAML-formatted string.
///
/// This function takes a JSON string as input and attempts to convert it into YAML.
/// If the input is valid JSON, it will be serialized to YAML.
/// If the input is not valid JSON, an error message is returned.
///
/// # Errors
///
/// - Returns `Err(String)` if the input is not valid JSON.
/// - Returns `Err(String)` if the YAML conversion fails.
///
/// # Examples
///
/// ```
/// use jqr::convert_to_yaml;
///
/// let json_str = r#"{"name": "Alice", "age": 25}"#;
/// let yaml_result = convert_to_yaml(json_str);
///
/// assert!(yaml_result.is_ok());
/// assert!(yaml_result.unwrap().contains("name: Alice"));
/// ```
///
/// ```
/// use jqr::convert_to_yaml;
///
/// let invalid_json = r#"{name: Alice, age:}"#;
/// let result = convert_to_yaml(invalid_json);
///
/// assert!(result.is_err());
/// ```
pub fn convert_to_yaml(content: &str) -> Result<String, String> {
    match serde_json::from_str::<Value>(content) {
        Ok(json) => {
            // Convert JSON to YAML, return error if conversion fails
            serde_yaml::to_string(&json).map_err(|e| e.to_string())
        }
        Err(e) => {
            // Return an error with colored output indicating invalid JSON
            Err(format!("Invalid JSON: {}", e.to_string().red()))
        }
    }
}

/// Converts a YAML string into a formatted JSON string and prints the result.
///
/// This function takes a YAML-formatted string as input and converts it into a
/// pretty-printed JSON format. If the input is valid YAML, the converted JSON
/// is printed to `stdout`. If the input is not valid YAML, an error message is
/// printed to `stderr`.
///
/// # Errors
///
/// - Prints an error message if the input is not valid YAML.
/// - Panics if JSON serialization unexpectedly fails (though highly unlikely).
///
/// # Examples
///
/// ```
/// use jqr::convert_to_json;
///
/// let yaml_str = r#"
/// name: Alice
/// age: 25
/// "#;
///
/// convert_to_json(yaml_str); // Prints: { "name": "Alice", "age": 25 }
/// ```
///
/// ```
/// use jqr::convert_to_json;
///
/// let invalid_yaml = r#"
/// name: Alice
/// age:
/// "#;
///
/// convert_to_json(invalid_yaml); // Prints: "Invalid YAML: ..."
/// ```
pub fn convert_to_json(content: &str) {
    match serde_yaml::from_str::<Value>(content) {
        Ok(yaml) => {
            // Convert YAML to pretty-printed JSON and print the result
            println!("{}", serde_json::to_string_pretty(&yaml).unwrap());
        }
        Err(e) => {
            // Print an error message with colored output for invalid YAML
            eprintln!("Invalid YAML: {}", e.to_string().red());
        }
    }
}
