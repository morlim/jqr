// tests/lib_tests.rs

#[cfg(test)]
mod tests {
    use jqr::*;
    use serde_json::json;

    #[test]
    fn test_pretty_print_json() {
        let input = r#"{"age": 30, "name": "Alice"}"#;
        let expected = "{\n  \"age\": 30,\n  \"name\": \"Alice\"\n}";
        assert_eq!(pretty_print_json(input, None).unwrap(), expected);
    }

    #[test]
    fn test_jsonpath_query() {
        let input = json!({"user": {"name": "Alice"}});
        let query = "$.user.name".to_string();
        let result = extract_jsonpath(&input, &query);
        assert_eq!(result, json!("Alice"));
    }

    #[test]
    fn test_invalid_json() {
        let input = "invalid json";
        let result = serde_json::from_str::<serde_json::Value>(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_convert_to_yaml() {
        let input = r#"{"name": "Alice", "age": 30}"#;
        let yaml_output = convert_to_yaml(input).unwrap();
        assert!(yaml_output.contains("name: Alice"));
        assert!(yaml_output.contains("age: 30"));
    }

    #[test]
    fn test_empty_json() {
        let input = "{}";
        let expected = "{}";
        assert_eq!(pretty_print_json(input, None).unwrap(), expected);
    }

    #[test]
    fn test_nested_json_query() {
        let input = json!({"user": {"profile": {"name": "Bob"}}});
        let query = "$.user.profile.name".to_string();
        let result = extract_jsonpath(&input, &query);
        assert_eq!(result, json!("Bob"));
    }

    #[test]
    fn test_array_json_query() {
        let input = json!({"users": [{"name": "Alice"}, {"name": "Bob"}]});
        let query = "$.users[*].name".to_string();
        let result = extract_jsonpath(&input, &query);
        assert_eq!(result, json!(vec!["Alice", "Bob"]));
    }

    #[test]
    fn test_convert_to_yaml_invalid_json() {
        let input = "invalid json";
        let _result = convert_to_yaml(input);
        assert!(_result.is_err());

    }

    #[test]
    fn test_jsonpath_query_non_existent_field() {
        let input = json!({"user": {"name": "Alice"}});
        let query = "$.user.age".to_string();
        let result = extract_jsonpath(&input, &query);
        assert_eq!(result, json!(null));
    }

    #[test]
    fn test_json_with_special_characters() {
        let input = r#"{"text": "Hello \"World\"!"}"#;
        let expected = "{\n  \"text\": \"Hello \\\"World\\\"!\"\n}";
        assert_eq!(pretty_print_json(input, None).unwrap(), expected);
    }

    #[test]
    fn test_large_json_handling() {
        let data: Vec<_> = (0..1000).map(|i| json!({"id": i, "value": i * 2})).collect();
        let input = json!({"data": data});
        let query = "$.data[999].value".to_string();
        let result = extract_jsonpath(&input, &query);
        assert_eq!(result, json!(1998));
    }
}
