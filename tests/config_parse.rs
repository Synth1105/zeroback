use zeroback::parse_config;

#[test]
fn test_parse_config_valid() {
    let toml = r#"
        [target]
        url = "http://example.com"
        method = "GET"
        port = "8080"
        timeout = 30

        [request.header]
        User-Agent = "TestAgent"
        Accept = "application/json"
    "#;

    let result = parse_config(&toml.to_string());
    assert!(result.is_ok());
}

#[test]
fn test_parse_config_with_body() {
    let toml = r#"
        [target]
        url = "http://example.com"
        method = "POST"
        port = "8080"
        timeout = 30

        [request.header]
        Content-Type = "application/json"

        [request.body]
        test = "data"
    "#;

    let result = parse_config(&toml.to_string());
    assert!(result.is_ok());
}

#[test]
fn test_parse_config_invalid_toml() {
    let toml = "invalid toml = [";
    let result = parse_config(&toml.to_string());
    assert!(result.is_err());
}
