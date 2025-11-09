# Chatbot Integration - Quick Reference

## Setup (One-Time)

```nushell
# Set API key for current session
$env.OPENAI_API_KEY = "sk-your-api-key-here"

# OR add to env.nu for persistence
# Edit: config env
# Add: $env.OPENAI_API_KEY = "sk-your-api-key-here"
```

## Basic Commands

### Check Configuration
```nushell
chatbot config --status    # Check if configured
chatbot config --setup     # Show setup instructions
```

### Ask Questions
```nushell
# General question
chatbot "What is Nushell?"

# Shell command help
chatbot --shell-help "How do I list files?"

# Error explanation
chatbot --explain-error "command not found: xyz"

# Choose model
chatbot --model gpt-4 "Complex question"
```

## Available Models

| Model | Speed | Cost | Best For |
|-------|-------|------|----------|
| gpt-3.5-turbo | Fast | Low | Quick questions, simple tasks |
| gpt-4 | Slower | Higher | Complex explanations, detailed analysis |
| gpt-4-turbo | Fast | Medium | Balance of speed and capability |

## Common Use Cases

### Learning Nushell
```nushell
chatbot --shell-help "What is a pipeline?"
chatbot --shell-help "How do I filter data?"
chatbot --shell-help "What are closures?"
```

### Debugging
```nushell
chatbot --explain-error "type mismatch"
chatbot --explain-error "file not found"
chatbot --explain-error "cannot convert string to int"
```

### Getting Command Examples
```nushell
chatbot --shell-help "How do I sort a list?"
chatbot --shell-help "How do I convert CSV to JSON?"
chatbot --shell-help "How do I group data?"
```

### Script Writing Help
```nushell
chatbot "How do I iterate over files?"
chatbot "How do I handle errors in Nushell?"
chatbot "How do I write a custom command?"
```

## Tips

1. **Be specific** - "How do I filter rows where age > 30?" is better than "filtering"
2. **Use appropriate flags** - `--shell-help` for Nushell questions, `--explain-error` for errors
3. **Choose the right model** - Use gpt-3.5-turbo for most questions to save costs
4. **Monitor usage** - Check your OpenAI dashboard regularly

## Troubleshooting

### API Key Not Set
```nushell
# Error: OpenAI API key not configured
# Fix:
$env.OPENAI_API_KEY = "sk-your-key-here"
```

### Connection Failed
- Check internet connection
- Verify API key is correct
- Check OpenAI service status

### Rate Limit Exceeded
- Wait a few minutes
- Consider upgrading your OpenAI plan
- Use less frequently

## Security Reminders

- ✅ Store API key in environment variables
- ✅ Add .env to .gitignore
- ✅ Set usage limits in OpenAI dashboard
- ✅ Rotate keys periodically
- ❌ Never commit API keys to git
- ❌ Never share API keys publicly

## Getting Help

```nushell
help chatbot              # Show command help
help chatbot config       # Show config help
chatbot config --setup    # Detailed setup guide
```

## Resources

- OpenAI API Keys: https://platform.openai.com/api-keys
- OpenAI Dashboard: https://platform.openai.com/usage
- OpenAI Documentation: https://platform.openai.com/docs
- Nushell Book: https://www.nushell.sh/book/
