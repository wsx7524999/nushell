# Chatbot Integration - Frequently Asked Questions

## General Questions

### What is the chatbot integration?

The chatbot integration provides AI-powered assistance directly within Nushell. It can help with:
- Shell command suggestions and explanations
- Nushell syntax and feature questions
- Error message interpretation
- General programming queries

### How does it work?

The chatbot uses OpenAI's GPT models (GPT-3.5-turbo, GPT-4, GPT-4-turbo) to generate responses to your queries. It sends your question to the OpenAI API and returns the AI-generated response directly in your shell.

### Is it free?

No, the chatbot uses OpenAI's API which is a paid service. You need:
1. An OpenAI account
2. An API key
3. Sufficient credits in your account

However, costs are typically very low for casual use (pennies per query).

## Setup & Configuration

### How do I get an API key?

1. Visit https://platform.openai.com/api-keys
2. Sign in or create an account
3. Click "Create new secret key"
4. Copy the key (starts with "sk-")
5. Set it in Nushell: `$env.OPENAI_API_KEY = "sk-your-key"`

### Where should I store my API key?

**For current session:**
```nushell
$env.OPENAI_API_KEY = "sk-your-key-here"
```

**For persistent storage (recommended):**
Edit your env.nu file (find location with `$nu.env-path`):
```nushell
$env.OPENAI_API_KEY = "sk-your-key-here"
```

**IMPORTANT:** Never commit your API key to version control!

### How do I verify my setup?

```nushell
chatbot config --status
```

This will show whether your API key is configured and ready to use.

### Can I use multiple API keys?

Yes, you can switch between API keys by changing the environment variable:
```nushell
# Use key 1
$env.OPENAI_API_KEY = "sk-key-one"

# Use key 2
$env.OPENAI_API_KEY = "sk-key-two"
```

## Usage Questions

### What's the difference between the flags?

| Flag | Purpose | Example |
|------|---------|---------|
| (none) | General questions | `chatbot "What is Nushell?"` |
| `--shell-help` | Nushell-specific help | `chatbot --shell-help "How do I filter data?"` |
| `--explain-error` | Error explanations | `chatbot --explain-error "type mismatch"` |
| `--model` | Choose AI model | `chatbot --model gpt-4 "question"` |

### Which model should I use?

**gpt-3.5-turbo (default):**
- Fastest responses
- Lowest cost (~$0.001 per query)
- Great for most questions

**gpt-4:**
- More capable
- Better reasoning
- Higher cost (~$0.03-0.06 per query)
- Use for complex questions

**gpt-4-turbo:**
- Faster than standard GPT-4
- Latest features
- Medium cost

### Can I use the chatbot without internet?

No, the chatbot requires an internet connection to communicate with OpenAI's API.

### How long do responses take?

Typical response times:
- GPT-3.5-turbo: 2-5 seconds
- GPT-4: 5-10 seconds
- GPT-4-turbo: 3-7 seconds

Complex queries may take longer.

### Can I use the chatbot in scripts?

Yes! Example:
```nushell
# Define a helper function
def explain [error: string] {
    chatbot --explain-error $error
}

# Use it
explain "command not found: xyz"
```

### Can I pipe output to the chatbot?

The chatbot doesn't currently accept piped input, but you can use command substitution:
```nushell
let $error = "some error message"
chatbot --explain-error $error
```

## Troubleshooting

### "OpenAI API key not configured"

**Cause:** The environment variable is not set.

**Solution:**
```nushell
$env.OPENAI_API_KEY = "sk-your-key-here"
chatbot config --status  # Verify
```

### "Failed to communicate with OpenAI API"

**Possible causes:**
1. No internet connection
2. Invalid API key
3. Insufficient credits
4. Rate limit exceeded

**Solutions:**
- Check internet connection
- Verify API key at https://platform.openai.com/api-keys
- Check account balance at https://platform.openai.com/usage
- Wait a few minutes and retry

### "Authentication error"

**Cause:** Invalid or expired API key.

**Solution:**
1. Generate a new API key at https://platform.openai.com/api-keys
2. Update your environment variable
3. Verify with `chatbot config --status`

### "Rate limit exceeded"

**Cause:** You've made too many requests in a short time.

**Solutions:**
- Wait 1-5 minutes before trying again
- Upgrade your OpenAI plan for higher limits
- Space out your requests

### Responses are slow

**This is normal** - AI models take time to generate responses.

**Tips to improve speed:**
- Use gpt-3.5-turbo instead of gpt-4
- Ask more specific questions
- Check your internet connection speed

### Getting incorrect or unhelpful responses

**Tips for better results:**
1. Be more specific in your questions
2. Provide context
3. Use the appropriate flag (--shell-help, --explain-error)
4. Try rephrasing your question
5. For complex topics, use gpt-4 instead of gpt-3.5-turbo

## Cost & Billing

### How much does it cost?

Approximate costs per query:
- GPT-3.5-turbo: $0.001 - $0.003
- GPT-4: $0.03 - $0.06
- GPT-4-turbo: $0.01 - $0.03

Actual costs depend on:
- Length of your query
- Length of the response
- Model used

### How can I track my usage?

Visit your OpenAI dashboard:
https://platform.openai.com/usage

### How can I control costs?

1. **Use gpt-3.5-turbo by default** (add `--model gpt-3.5-turbo`)
2. **Set usage limits** in your OpenAI account settings
3. **Ask concise questions** - shorter queries = lower cost
4. **Monitor usage regularly** in the OpenAI dashboard
5. **Use built-in help first** (`help <command>`) for simple questions

### Will I be charged for failed requests?

No, you're only charged for successful API calls.

## Security & Privacy

### Is my data sent to OpenAI?

Yes, your queries are sent to OpenAI's servers to generate responses. See OpenAI's privacy policy for details.

### Should I send sensitive information?

**No!** Don't send:
- Passwords
- API keys
- Personal information
- Proprietary code
- Confidential data

### How do I keep my API key secure?

✅ **Do:**
- Store in environment variables
- Add .env files to .gitignore
- Rotate keys periodically
- Set usage limits

❌ **Don't:**
- Commit keys to git
- Share keys publicly
- Hardcode keys in scripts
- Share keys with others

### Can I revoke an API key?

Yes, you can delete API keys anytime at:
https://platform.openai.com/api-keys

If you suspect your key is compromised, delete it immediately and create a new one.

## Advanced Usage

### Can I change the default model?

Currently, gpt-3.5-turbo is the default. To use a different model, always specify `--model`:
```nushell
chatbot --model gpt-4 "your question"
```

### Can I adjust response length?

The current implementation limits responses to 500 tokens (~375 words). This is hardcoded to control costs.

### Can I customize the system prompt?

Not currently. The system prompts are:
- General: "You are a helpful assistant integrated into the Nushell shell."
- Shell help: "You are a helpful Nushell shell expert."
- Error explanation: "You are a helpful assistant that explains error messages."

### Can I use it with other AI providers?

Not currently. The implementation is specifically for OpenAI's API. Support for other providers (Anthropic, etc.) would require code changes.

### Can I see the API request/response?

Enable debug output in your Nushell config if you need to debug API calls. The implementation uses standard ureq HTTP client.

## Feature Requests & Bugs

### How do I report bugs?

Open an issue in the Nushell GitHub repository with:
- Description of the problem
- Steps to reproduce
- Expected vs actual behavior
- Error messages (if any)

### How do I request features?

Open a feature request issue in the Nushell repository with:
- Clear description of the feature
- Use cases
- Why it would be valuable

### What features are planned?

Possible future enhancements:
- Support for conversation history
- Ability to customize system prompts
- Support for other AI providers
- Integration with local LLMs
- Streaming responses
- Cost tracking

## Comparison with Web Interface

### What's the difference between the native commands and web interface?

**Native Commands (`chatbot`):**
- ✅ Fast and integrated into your workflow
- ✅ Can be used in scripts
- ✅ Lightweight (no browser needed)
- ❌ No conversation history
- ❌ Text-only output

**Web Interface:**
- ✅ Better for longer conversations
- ✅ Nicer visual interface
- ✅ Better for complex interactions
- ❌ Requires running backend server
- ❌ Can't be used in scripts

Both use the same OpenAI API and have similar capabilities.

## Additional Resources

### Where can I learn more?

- **OpenAI Documentation:** https://platform.openai.com/docs
- **Nushell Book:** https://www.nushell.sh/book/
- **OpenAI Pricing:** https://openai.com/pricing
- **Usage Dashboard:** https://platform.openai.com/usage

### Where is the full documentation?

- Quick reference: `chatgpt-integration/QUICK_REFERENCE.md`
- User guide: `chatgpt-integration/NUSHELL_COMMANDS.md`
- Setup: `chatgpt-integration/README.md`

### How do I get help?

```nushell
# Built-in help
help chatbot
help chatbot config

# Setup instructions
chatbot config --setup

# Status check
chatbot config --status
```

## Still have questions?

Open an issue in the Nushell repository or check the documentation files in the `chatgpt-integration` directory.
