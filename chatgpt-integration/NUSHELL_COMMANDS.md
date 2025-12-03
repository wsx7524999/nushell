# Nushell Chatbot Commands - User Guide

## Overview

The Nushell chatbot integration provides AI-powered assistance directly within your shell. It can help you with:
- Shell command suggestions and explanations
- Nushell syntax and feature questions
- Error message interpretation
- General programming and system administration queries

## Installation & Setup

### 1. Prerequisites

- Nushell 0.108.1 or later
- OpenAI API key

### 2. Get an API Key

1. Visit [OpenAI Platform](https://platform.openai.com/api-keys)
2. Sign in or create an account
3. Create a new API key
4. Copy the key (it starts with "sk-")

### 3. Configure the API Key

**Option A: Current Session Only**
```nushell
$env.OPENAI_API_KEY = "sk-your-actual-api-key-here"
```

**Option B: Persistent Configuration**

Edit your Nushell environment configuration:
```nushell
# Open your env.nu file
config env

# Add this line:
$env.OPENAI_API_KEY = "sk-your-actual-api-key-here"
```

The location of your env.nu file:
```nushell
$nu.env-path
```

### 4. Verify Setup

```nushell
chatbot config --status
```

You should see a "CONFIGURED" status with a masked version of your API key.

## Usage Examples

### Basic Usage

**Simple Question:**
```nushell
chatbot "What is Nushell?"
```

**Programming Question:**
```nushell
chatbot "How do I sort a list in reverse order?"
```

### Shell Command Help

Use the `--shell-help` flag for Nushell-specific assistance:

```nushell
# Learn about commands
chatbot --shell-help "How do I list files recursively?"

# Understand syntax
chatbot --shell-help "What does the pipe operator do?"

# Get command examples
chatbot --shell-help "How do I filter data in a table?"

# Pipeline questions
chatbot --shell-help "How can I chain multiple commands together?"
```

### Error Explanation

Use the `--explain-error` flag to understand error messages:

```nushell
# Explain a command not found error
chatbot --explain-error "command not found: xyz"

# Understand type mismatches
chatbot --explain-error "type mismatch during operation"

# Get help with syntax errors
chatbot --explain-error "unexpected token in pipeline"
```

### Model Selection

Choose different models based on your needs:

```nushell
# Fast and cost-effective (default)
chatbot --model gpt-3.5-turbo "Quick question"

# More capable for complex queries
chatbot --model gpt-4 "Explain closures and how they work in Nushell"

# Latest GPT-4 variant
chatbot --model gpt-4-turbo "Complex multi-step task explanation"
```

## Real-World Scenarios

### Scenario 1: Learning Nushell

```nushell
# Understanding concepts
chatbot --shell-help "What are records in Nushell?"

# Learning commands
chatbot --shell-help "How do I work with JSON files?"

# Pipeline operations
chatbot --shell-help "How do I select specific columns from a table?"
```

### Scenario 2: Debugging

```nushell
# When you get an error
chatbot --explain-error "cannot convert string to int"

# Understanding what went wrong
chatbot --explain-error "access beyond end of stream"

# Getting suggestions
chatbot --explain-error "file not found"
```

### Scenario 3: Task Automation

```nushell
# Get script ideas
chatbot --shell-help "How can I process all CSV files in a directory?"

# Learn best practices
chatbot --shell-help "What's the best way to handle errors in scripts?"

# Optimization tips
chatbot --shell-help "How can I make my data processing faster?"
```

### Scenario 4: System Administration

```nushell
# File operations
chatbot "How do I find the largest files in a directory?"

# Process management
chatbot "How can I monitor system resources?"

# Network tasks
chatbot "How do I parse JSON from an HTTP response?"
```

## Configuration Management

### Check Status

```nushell
chatbot config --status
```

**Output when configured:**
```
╭─────────────────────────────────────────────╮
│   Chatbot Configuration Status             │
╰─────────────────────────────────────────────╯

Status: ✓ CONFIGURED

API Key: sk-1...xyz9 (masked)
Ready to use: Yes

Try it out:
  chatbot "Hello, can you help me?"
  chatbot --shell-help "How do I use ls?"
```

**Output when not configured:**
```
╭─────────────────────────────────────────────╮
│   Chatbot Configuration Status             │
╰─────────────────────────────────────────────╯

Status: ✗ NOT CONFIGURED

API Key: Not set
Ready to use: No
```

### Get Setup Instructions

```nushell
chatbot config --setup
```

This displays detailed setup instructions including:
- How to get an API key
- Configuration options
- Security best practices
- Usage examples

## Tips & Best Practices

### 1. Be Specific

**Good:**
```nushell
chatbot --shell-help "How do I filter rows where age is greater than 30?"
```

**Less Good:**
```nushell
chatbot "filtering"
```

### 2. Use Appropriate Flags

- Use `--shell-help` for Nushell-specific questions
- Use `--explain-error` when you have an error message
- Use no flags for general questions

### 3. Choose the Right Model

- **gpt-3.5-turbo**: Fast, cost-effective, great for simple questions
- **gpt-4**: More capable, better for complex explanations
- **gpt-4-turbo**: Latest features, faster than standard GPT-4

### 4. Monitor Your Usage

- Check your OpenAI dashboard regularly: https://platform.openai.com/usage
- Set usage limits to avoid unexpected charges
- Use gpt-3.5-turbo for routine questions to save costs

### 5. Security

- **Never share your API key** with others
- Don't commit your API key to version control
- Rotate your API keys periodically
- Set usage limits in your OpenAI account

## Troubleshooting

### "OpenAI API key not configured"

**Solution:**
```nushell
# Set the API key
$env.OPENAI_API_KEY = "sk-your-key-here"

# Verify
chatbot config --status
```

### "Failed to communicate with OpenAI API"

**Possible causes:**
1. **No internet connection** - Check your network
2. **Invalid API key** - Verify your key at platform.openai.com
3. **Insufficient credits** - Check your OpenAI account balance
4. **Rate limit exceeded** - Wait a few minutes and try again

**Check your configuration:**
```nushell
chatbot config --status
```

### "Request timed out"

**Solution:**
- Check your internet connection
- Try again in a few moments
- The OpenAI API might be experiencing issues

### Slow responses

**Expected behavior:**
- Responses typically take 2-10 seconds
- GPT-4 is slower than GPT-3.5-turbo
- Complex queries take longer

**Tips:**
- Use gpt-3.5-turbo for faster responses
- Break complex queries into smaller ones

## Cost Information

The chatbot uses OpenAI's API which is a paid service. Costs depend on:
- **Model used**: GPT-4 costs more than GPT-3.5-turbo
- **Token usage**: Both input (your query) and output (response) count
- **Frequency**: More queries = higher cost

**Typical costs (as of 2024):**
- GPT-3.5-turbo: ~$0.001 per query
- GPT-4: ~$0.03-0.06 per query

**Cost management:**
- Use `--model gpt-3.5-turbo` for most queries
- Set usage limits in your OpenAI dashboard
- Monitor usage regularly

## Advanced Usage

### Combining with Nushell Pipelines

You can use chatbot output in pipelines:

```nushell
# Get a command suggestion and evaluate it
chatbot --shell-help "command to list files" | print

# Store chatbot response
let $response = (chatbot "What is Nushell?")
echo $response
```

### Using in Scripts

```nushell
# Script that uses chatbot for help
def explain-error [error: string] {
    chatbot --explain-error $error
}

# Usage
explain-error "command not found: xyz"
```

### Environment-Specific Configuration

```nushell
# Different keys for different environments
if ($env.ENVIRONMENT == "production") {
    $env.OPENAI_API_KEY = (open ~/.openai-prod-key | str trim)
} else {
    $env.OPENAI_API_KEY = (open ~/.openai-dev-key | str trim)
}
```

## Getting Help

### Within Nushell

```nushell
# View command help
help chatbot

# View configuration help
help chatbot config

# Show examples
chatbot --help
```

### External Resources

- [OpenAI API Documentation](https://platform.openai.com/docs)
- [Nushell Book](https://www.nushell.sh/book/)
- [OpenAI Pricing](https://openai.com/pricing)
- [API Usage Dashboard](https://platform.openai.com/usage)

## Examples Library

### Data Processing

```nushell
chatbot --shell-help "How do I convert CSV to JSON?"
chatbot --shell-help "How do I merge two tables?"
chatbot --shell-help "How do I calculate average of a column?"
```

### File Management

```nushell
chatbot "How do I find files modified in the last 24 hours?"
chatbot "How do I rename multiple files at once?"
chatbot "How do I compare two directories?"
```

### String Operations

```nushell
chatbot --shell-help "How do I extract text between two delimiters?"
chatbot --shell-help "How do I replace text in multiple files?"
chatbot --shell-help "How do I format dates in Nushell?"
```

### System Tasks

```nushell
chatbot "How do I check disk space in Nushell?"
chatbot "How do I monitor CPU usage?"
chatbot "How do I list running processes?"
```

## Feedback & Contributions

Found a bug or have a suggestion? Please open an issue in the Nushell repository.

## License

This integration is part of the Nushell project and follows the same MIT license.
