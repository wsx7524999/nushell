use nu_engine::command_prelude::*;
use ureq::Agent;

/// Helper to get Azure access token from environment variables or explicit token
pub fn get_azure_token(
    engine_state: &EngineState,
    call: &Call,
    stack: &mut Stack,
) -> Result<String, ShellError> {
    // Try to get token from --token flag first
    if let Some(token_val) = call.get_flag::<String>(engine_state, stack, "token")? {
        return Ok(token_val);
    }
    
    // Try environment variable AZURE_ACCESS_TOKEN
    if let Ok(token) = std::env::var("AZURE_ACCESS_TOKEN") {
        return Ok(token);
    }
    
    Err(ShellError::GenericError {
        error: "Azure authentication required".into(),
        msg: "No Azure access token provided".into(),
        span: Some(call.head),
        help: Some("Set AZURE_ACCESS_TOKEN environment variable or use --token flag".into()),
        inner: vec![],
    })
}

/// Helper to get Azure subscription ID
pub fn get_azure_subscription(
    engine_state: &EngineState,
    call: &Call,
    stack: &mut Stack,
) -> Result<String, ShellError> {
    // Try to get subscription from --subscription flag first
    if let Some(sub_val) = call.get_flag::<String>(engine_state, stack, "subscription")? {
        return Ok(sub_val);
    }
    
    // Try environment variable AZURE_SUBSCRIPTION_ID
    if let Ok(subscription) = std::env::var("AZURE_SUBSCRIPTION_ID") {
        return Ok(subscription);
    }
    
    Err(ShellError::GenericError {
        error: "Azure subscription ID required".into(),
        msg: "No Azure subscription ID provided".into(),
        span: Some(call.head),
        help: Some("Set AZURE_SUBSCRIPTION_ID environment variable or use --subscription flag".into()),
        inner: vec![],
    })
}

/// Create HTTP client for Azure REST API calls
pub fn create_azure_client() -> Result<Agent, ShellError> {
    let config = ureq::config::Config::builder()
        .user_agent("nushell-azure-client")
        .build();
    
    Ok(ureq::Agent::new_with_config(config))
}

/// Make an authenticated GET request to Azure REST API
pub fn azure_get_request(
    client: &Agent,
    url: &str,
    token: &str,
    span: Span,
) -> Result<serde_json::Value, ShellError> {
    let mut response = client
        .get(url)
        .header("Authorization", &format!("Bearer {}", token))
        .header("Content-Type", "application/json")
        .call()
        .map_err(|e| ShellError::NetworkFailure {
            msg: format!("Azure API request failed: {}", e),
            span,
        })?;
    
    let status = response.status();
    if status != 200 {
        let body = response.body_mut().read_to_string()
            .unwrap_or_else(|_| "Failed to read error response body from Azure API".to_string());
        return Err(ShellError::NetworkFailure {
            msg: format!("Azure API returned status {}: {}", status, body),
            span,
        });
    }
    
    response.body_mut().read_json()
        .map_err(|e| ShellError::GenericError {
            error: "Failed to parse Azure API response".into(),
            msg: format!("JSON parsing error: {}", e),
            span: Some(span),
            help: Some("The response from Azure API was not valid JSON".into()),
            inner: vec![],
        })
}

/// Convert Azure API response to nushell Value
pub fn azure_response_to_value(
    json_value: serde_json::Value,
    span: Span,
) -> Result<Value, ShellError> {
    match json_value {
        serde_json::Value::Object(map) => {
            let mut record = Record::new();
            for (key, value) in map {
                record.push(key, json_to_value(value, span)?);
            }
            Ok(Value::record(record, span))
        }
        serde_json::Value::Array(arr) => {
            let values: Result<Vec<_>, _> = arr
                .into_iter()
                .map(|v| json_to_value(v, span))
                .collect();
            Ok(Value::list(values?, span))
        }
        _ => json_to_value(json_value, span),
    }
}

/// Convert JSON value to nushell Value
fn json_to_value(json: serde_json::Value, span: Span) -> Result<Value, ShellError> {
    match json {
        serde_json::Value::Null => Ok(Value::nothing(span)),
        serde_json::Value::Bool(b) => Ok(Value::bool(b, span)),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(Value::int(i, span))
            } else if let Some(f) = n.as_f64() {
                Ok(Value::float(f, span))
            } else {
                Ok(Value::string(n.to_string(), span))
            }
        }
        serde_json::Value::String(s) => Ok(Value::string(s, span)),
        serde_json::Value::Array(arr) => {
            let values: Result<Vec<_>, _> = arr
                .into_iter()
                .map(|v| json_to_value(v, span))
                .collect();
            Ok(Value::list(values?, span))
        }
        serde_json::Value::Object(map) => {
            let mut record = Record::new();
            for (key, value) in map {
                record.push(key, json_to_value(value, span)?);
            }
            Ok(Value::record(record, span))
        }
    }
}
