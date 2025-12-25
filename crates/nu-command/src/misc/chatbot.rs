use nu_engine::command_prelude::*;
use std::env;
use std::io::{BufReader, Read};

#[derive(Clone)]
pub struct Chatbot;

impl Command for Chatbot {
    fn name(&self) -> &str {
        "chatbot"
    }

    fn signature(&self) -> Signature {
        Signature::build("chatbot")
            .input_output_types(vec![(Type::Nothing, Type::String)])
            .allow_variants_without_examples(true)
            .required(
                "query",
                SyntaxShape::String,
                "The question or command to ask the chatbot",
            )
            .named(
                "model",
                SyntaxShape::String,
                "The AI model to use (gpt-4, gpt-4-turbo, gpt-3.5-turbo)",
                Some('m'),
            )
            .switch(
                "shell-help",
                "Get help with shell commands and Nushell syntax",
                Some('s'),
            )
            .switch("explain-error", "Explain a shell error message", Some('e'))
            .category(Category::Misc)
    }

    fn description(&self) -> &str {
        "Interact with an AI chatbot assistant for shell command help and general queries"
    }

    fn extra_description(&self) -> &str {
        r#"The chatbot command provides an AI-powered assistant directly within Nushell.
It can help with:
- Shell command suggestions and explanations
- Nushell syntax and feature questions
- Error message interpretation
- General programming and system administration queries

To use this command, you must set the OPENAI_API_KEY environment variable.
Get your API key from: https://platform.openai.com/api-keys

Example: $env.OPENAI_API_KEY = "sk-your-api-key-here"

Note: Using this command will consume OpenAI API credits."#
    }

    fn search_terms(&self) -> Vec<&str> {
        vec!["ai", "help", "assistant", "gpt", "chatgpt", "openai", "ask"]
    }

    fn run(
        &self,
        engine_state: &EngineState,
        stack: &mut Stack,
        call: &Call,
        _input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        chatbot_command(engine_state, stack, call)
    }

    fn examples(&self) -> Vec<Example<'_>> {
        vec![
            Example {
                description: "Ask the chatbot a general question",
                example: "chatbot 'What is the capital of France?'",
                result: None,
            },
            Example {
                description: "Get help with a shell command",
                example: "chatbot --shell-help 'How do I list files recursively?'",
                result: None,
            },
            Example {
                description: "Explain an error message",
                example: "chatbot --explain-error 'command not found: xyz'",
                result: None,
            },
            Example {
                description: "Use a specific model",
                example: "chatbot --model gpt-3.5-turbo 'What is Nushell?'",
                result: None,
            },
        ]
    }
}

fn chatbot_command(
    engine_state: &EngineState,
    stack: &mut Stack,
    call: &Call,
) -> Result<PipelineData, ShellError> {
    let span = call.head;

    // Get the query from the user
    let query: String = call.req(engine_state, stack, 0)?;

    // Get optional flags
    let model: Option<String> = call.get_flag(engine_state, stack, "model")?;
    let shell_help: bool = call.has_flag(engine_state, stack, "shell-help")?;
    let explain_error: bool = call.has_flag(engine_state, stack, "explain-error")?;

    // Check if API key is set
    let api_key = env::var("OPENAI_API_KEY").or_else(|_| {
        // Also check in Nushell's environment
        stack
            .get_env_var(engine_state, "OPENAI_API_KEY")
            .and_then(|v| v.clone().coerce_into_string().ok())
            .ok_or(env::VarError::NotPresent)
    });

    let api_key = match api_key {
        Ok(key) if !key.is_empty() => key,
        _ => {
            return Err(ShellError::GenericError {
                error: "OpenAI API key not configured".to_string(),
                msg: "Please set the OPENAI_API_KEY environment variable".to_string(),
                span: Some(span),
                help: Some("Get your API key from: https://platform.openai.com/api-keys\nThen set it with: $env.OPENAI_API_KEY = \"sk-your-key-here\"".to_string()),
                inner: vec![],
            });
        }
    };

    // Determine the model to use
    let selected_model = model.unwrap_or_else(|| "gpt-3.5-turbo".to_string());

    // Build the prompt based on flags
    let system_prompt = if shell_help {
        "You are a helpful Nushell shell expert. Provide clear, concise answers about shell commands, Nushell syntax, and best practices. Include example commands when appropriate."
    } else if explain_error {
        "You are a helpful assistant that explains error messages. Analyze the error, explain what it means, why it might have occurred, and suggest solutions."
    } else {
        "You are a helpful assistant integrated into the Nushell shell. Provide clear, accurate, and concise answers to user questions."
    };

    // Make the API call
    match call_openai_api(&api_key, &selected_model, system_prompt, &query) {
        Ok(response) => {
            Ok(Value::string(response, span).into_pipeline_data())
        }
        Err(e) => Err(ShellError::GenericError {
            error: "Failed to communicate with OpenAI API".to_string(),
            msg: e.to_string(),
            span: Some(span),
            help: Some("Check your API key and internet connection. Ensure you have sufficient API credits.".to_string()),
            inner: vec![],
        }),
    }
}

fn call_openai_api(
    api_key: &str,
    model: &str,
    system_prompt: &str,
    user_message: &str,
) -> Result<String, String> {
    // Prepare the request payload
    let payload = serde_json::json!({
        "model": model,
        "messages": [
            {
                "role": "system",
                "content": system_prompt
            },
            {
                "role": "user",
                "content": user_message
            }
        ],
        "max_tokens": 500,
        "temperature": 0.7
    });

    // Make HTTP request using ureq
    let response = ureq::post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", &format!("Bearer {api_key}"))
        .header("Content-Type", "application/json")
        .send_json(payload);

    match response {
        Ok(resp) => {
            let mut buf_reader = BufReader::new(resp.into_body().into_reader());
            let mut body = Vec::new();
            buf_reader
                .read_to_end(&mut body)
                .map_err(|e| format!("Failed to read response: {e}"))?;

            let body_str =
                String::from_utf8(body).map_err(|e| format!("Failed to decode response: {e}"))?;

            // Parse the JSON response
            let json: serde_json::Value = serde_json::from_str(&body_str)
                .map_err(|e| format!("Failed to parse response: {e}"))?;

            // Extract the message content with proper error handling
            let choices = json["choices"]
                .as_array()
                .ok_or_else(|| "No choices in response".to_string())?;

            if choices.is_empty() {
                return Err("Empty response from API".to_string());
            }

            let content = choices[0]["message"]["content"]
                .as_str()
                .ok_or_else(|| "No response content found".to_string())?;

            Ok(content.to_string())
        }
        Err(e) => {
            // Handle different error types
            match e {
                ureq::Error::Timeout(_) => {
                    Err("Request timed out. The API took too long to respond.".to_string())
                }
                ureq::Error::ConnectionFailed => {
                    Err("Connection failed. Check your internet connection.".to_string())
                }
                ureq::Error::Io(io_err) => Err(format!("I/O error: {io_err}")),
                _ => Err(format!("API error: {e}")),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        use crate::test_examples;
        test_examples(Chatbot)
    }
}
