use nu_engine::command_prelude::*;

#[derive(Clone)]
pub struct ChatbotConfig;

impl Command for ChatbotConfig {
    fn name(&self) -> &str {
        "chatbot config"
    }

    fn signature(&self) -> Signature {
        Signature::build("chatbot config")
            .input_output_types(vec![(Type::Nothing, Type::String)])
            .allow_variants_without_examples(true)
            .switch(
                "status",
                "Show the current chatbot configuration status",
                Some('s'),
            )
            .switch(
                "setup",
                "Show setup instructions for the chatbot",
                None, // No short flag to avoid conflict with help (-h)
            )
            .category(Category::Misc)
    }

    fn description(&self) -> &str {
        "Configure and check the status of the chatbot integration"
    }

    fn extra_description(&self) -> &str {
        r#"This command helps you configure and verify the chatbot setup.

Use --status to check if the API key is configured.
Use --setup to get detailed setup instructions.

The chatbot uses the OpenAI API and requires an API key to function."#
    }

    fn search_terms(&self) -> Vec<&str> {
        vec!["config", "setup", "configure", "chatbot", "ai"]
    }

    fn run(
        &self,
        engine_state: &EngineState,
        stack: &mut Stack,
        call: &Call,
        _input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        chatbot_config_command(engine_state, stack, call)
    }

    fn examples(&self) -> Vec<Example<'_>> {
        vec![
            Example {
                description: "Check chatbot configuration status",
                example: "chatbot config --status",
                result: None,
            },
            Example {
                description: "Show setup instructions",
                example: "chatbot config --setup",
                result: None,
            },
        ]
    }
}

fn chatbot_config_command(
    engine_state: &EngineState,
    stack: &mut Stack,
    call: &Call,
) -> Result<PipelineData, ShellError> {
    let span = call.head;

    let status: bool = call.has_flag(engine_state, stack, "status")?;
    let setup: bool = call.has_flag(engine_state, stack, "setup")?;

    if setup {
        let setup_text = r#"
╭─────────────────────────────────────────────╮
│   Chatbot Setup Instructions               │
╰─────────────────────────────────────────────╯

The chatbot feature requires an OpenAI API key to function.

Step 1: Get an API Key
  • Visit: https://platform.openai.com/api-keys
  • Sign in or create an account
  • Create a new API key
  • Copy the key (starts with "sk-")

Step 2: Set the API Key in Nushell
  
  Option A: Set for the current session
  $env.OPENAI_API_KEY = "sk-your-api-key-here"

  Option B: Add to your config file (persistent)
  Edit your env.nu file (found at: $nu.env-path):
  
  $env.OPENAI_API_KEY = "sk-your-api-key-here"

Step 3: Verify the Setup
  chatbot config --status

Step 4: Start Using the Chatbot
  chatbot "What is Nushell?"
  chatbot --shell-help "How do I list files?"
  chatbot --explain-error "command not found"

Security Notes:
  • Never commit your API key to version control
  • Keep your API key secret
  • Set usage limits in your OpenAI account
  • Monitor your API usage regularly

For more information, visit:
  https://platform.openai.com/docs
"#;
        return Ok(Value::string(setup_text, span).into_pipeline_data());
    }

    if status {
        let api_key = std::env::var("OPENAI_API_KEY").or_else(|_| {
            stack
                .get_env_var(engine_state, "OPENAI_API_KEY")
                .and_then(|v| v.clone().coerce_into_string().ok())
                .ok_or(std::env::VarError::NotPresent)
        });

        let status_text = match api_key {
            Ok(key) if !key.is_empty() => {
                let masked_key = if key.len() > 8 {
                    format!("{}...{}", &key[0..4], &key[key.len() - 4..])
                } else {
                    "****".to_string()
                };

                format!(
                    r#"
╭─────────────────────────────────────────────╮
│   Chatbot Configuration Status             │
╰─────────────────────────────────────────────╯

Status: ✓ CONFIGURED

API Key: {masked_key} (masked)
Ready to use: Yes

Try it out:
  chatbot "Hello, can you help me?"
  chatbot --shell-help "How do I use ls?"

To see setup instructions:
  chatbot config --setup
"#
                )
            }
            _ => r#"
╭─────────────────────────────────────────────╮
│   Chatbot Configuration Status             │
╰─────────────────────────────────────────────╯

Status: ✗ NOT CONFIGURED

API Key: Not set
Ready to use: No

The chatbot requires an OpenAI API key to function.

To configure:
  chatbot config --setup

Quick setup:
  $env.OPENAI_API_KEY = "sk-your-api-key-here"

Get your API key from:
  https://platform.openai.com/api-keys
"#
            .to_string(),
        };

        return Ok(Value::string(status_text, span).into_pipeline_data());
    }

    // Default: show brief help
    let default_text = r#"
╭─────────────────────────────────────────────╮
│   Chatbot Configuration                    │
╰─────────────────────────────────────────────╯

Use the following flags to configure the chatbot:

  --status    Check if the chatbot is configured
  --setup     Show detailed setup instructions

Examples:
  chatbot config --status
  chatbot config --setup
"#;

    Ok(Value::string(default_text, span).into_pipeline_data())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        use crate::test_examples;
        test_examples(ChatbotConfig)
    }
}
