# ChatGPT Integration for Nushell

This directory contains a complete ChatGPT integration that allows users to interact with OpenAI's GPT models in two ways:
1. **Native Nushell Commands** - Use chatbot commands directly in your shell
2. **Web Interface** - Interact via a browser-based frontend

## 📚 Documentation

- **[Quick Reference](QUICK_REFERENCE.md)** - Essential commands and quick tips
- **[User Guide](NUSHELL_COMMANDS.md)** - Comprehensive guide with examples
- **[FAQ](FAQ.md)** - Frequently asked questions and troubleshooting
- **[Deployment Guide](DEPLOYMENT.md)** - Web interface deployment options

## 🚀 Quick Start (Nushell Commands)

### Prerequisites

- Nushell installed
- OpenAI API key ([Get one here](https://platform.openai.com/api-keys))

### Step 1: Configure API Key

Set your OpenAI API key in Nushell:

```nushell
# For current session only
$env.OPENAI_API_KEY = "sk-your-actual-api-key-here"

# OR add to your env.nu file for persistence
# Edit $nu.env-path and add:
$env.OPENAI_API_KEY = "sk-your-actual-api-key-here"
```

### Step 2: Verify Setup

```nushell
chatbot config --status
```

### Step 3: Start Using the Chatbot

```nushell
# Ask a general question
chatbot "What is Nushell?"

# Get help with shell commands
chatbot --shell-help "How do I list files recursively?"

# Explain an error message
chatbot --explain-error "command not found: xyz"

# Use a specific model
chatbot --model gpt-3.5-turbo "What is the capital of France?"
```

## 📋 Nushell Command Reference

### `chatbot` - Main Command

Interact with an AI chatbot assistant for shell command help and general queries.

**Syntax:**
```nushell
chatbot [query] [--model <model>] [--shell-help] [--explain-error]
```

**Flags:**
- `--model, -m <model>`: The AI model to use (gpt-4, gpt-4-turbo, gpt-3.5-turbo)
- `--shell-help, -s`: Get help with shell commands and Nushell syntax
- `--explain-error, -e`: Explain a shell error message

**Examples:**
```nushell
# General question
chatbot "What is the capital of France?"

# Shell help
chatbot --shell-help "How do I filter a table?"

# Error explanation
chatbot --explain-error "type mismatch during operation"

# Use GPT-4 for more complex queries
chatbot --model gpt-4 "Explain closures in Nushell"
```

### `chatbot config` - Configuration Management

Configure and check the status of the chatbot integration.

**Syntax:**
```nushell
chatbot config [--status] [--setup]
```

**Flags:**
- `--status, -s`: Show the current chatbot configuration status
- `--setup`: Show detailed setup instructions

**Examples:**
```nushell
# Check if chatbot is configured
chatbot config --status

# Show setup instructions
chatbot config --setup
```

## 🌐 Web Interface (Alternative Method)

For users who prefer a web interface, we also provide a browser-based chatbot.

### Web Interface Directory Structure

```
chatgpt-integration/
├── backend/              # Python Flask backend server
│   ├── server.py        # Main API server
│   └── requirements.txt # Python dependencies
├── frontend/            # Web-based user interface
│   ├── index.html      # Main HTML page
│   └── app.js          # JavaScript frontend logic
├── tests/              # Integration tests
│   └── test_integration.py
└── .env.example        # Environment variables template
```

## 🚀 Quick Start

### Prerequisites

- Python 3.8 or higher
- pip (Python package manager)
- OpenAI API key ([Get one here](https://platform.openai.com/api-keys))

### Step 1: Install Dependencies

```bash
cd chatgpt-integration/backend
pip install -r requirements.txt
```

### Step 2: Configure API Key

1. Copy the example environment file:
   ```bash
   cd ..
   cp .env.example .env
   ```

2. Edit the `.env` file and add your OpenAI API key:
   ```
   OPENAI_API_KEY=sk-your-actual-api-key-here
   ```

   **Important:** Never commit the `.env` file to version control!

### Step 3: Start the Backend Server

```bash
cd backend
python server.py
```

The server will start on `http://localhost:5000`

### Step 4: Open the Frontend

Open `frontend/index.html` in your web browser. You can:
- Double-click the file, or
- Use a local web server:
  ```bash
  cd frontend
  python -m http.server 8000
  # Then open http://localhost:8000 in your browser
  ```

## 🧪 Running Tests

The integration includes a test suite to verify functionality:

```bash
# Make sure the backend server is running first
cd tests
python test_integration.py
```

The test suite checks:
- ✓ Backend health endpoint
- ✓ Available models endpoint
- ✓ Error handling without API key
- ✓ Input validation
- ✓ Chat functionality with sample query (requires API key)

## 🔧 Configuration

### Environment Variables

Edit the `.env` file to configure:

| Variable | Description | Default |
|----------|-------------|---------|
| `OPENAI_API_KEY` | Your OpenAI API key | (required) |
| `PORT` | Backend server port | 5000 |
| `DEBUG` | Enable debug mode | false |

### Available Models

The integration supports multiple GPT models:
- **GPT-4**: Most capable model, best quality
- **GPT-4 Turbo**: Faster GPT-4 variant
- **GPT-3.5 Turbo**: Fast and cost-effective

Select the model from the dropdown in the frontend interface.

## 📡 API Endpoints

### Health Check
```
GET /api/health
```
Returns the server status.

### Get Models
```
GET /api/models
```
Returns available ChatGPT models.

### Chat
```
POST /api/chat
Content-Type: application/json

{
  "message": "Your prompt here",
  "model": "gpt-4"  // optional, defaults to gpt-4
}
```

Response:
```json
{
  "response": "ChatGPT's response",
  "model": "gpt-4",
  "usage": {
    "prompt_tokens": 10,
    "completion_tokens": 50,
    "total_tokens": 60
  }
}
```

## 🛡️ Security Best Practices

1. **Never commit `.env` files** - They contain sensitive API keys
2. **Use environment variables** - Don't hardcode API keys in source code
3. **Rotate API keys regularly** - Generate new keys periodically
4. **Monitor API usage** - Check your OpenAI dashboard for usage
5. **Set usage limits** - Configure limits in your OpenAI account

## 🌐 Deployment Options

### Option 1: Local Development
Best for testing and development. Follow the Quick Start guide above.

### Option 2: Deploy Backend to Heroku

1. Install Heroku CLI
2. Create a new Heroku app:
   ```bash
   heroku create your-app-name
   ```
3. Set environment variables:
   ```bash
   heroku config:set OPENAI_API_KEY=your-key-here
   ```
4. Create a `Procfile`:
   ```
   web: cd backend && python server.py
   ```
5. Deploy:
   ```bash
   git push heroku main
   ```

### Option 3: Deploy Frontend to GitHub Pages

1. Push the frontend directory to a GitHub repository
2. Enable GitHub Pages in repository settings
3. Update `API_BASE_URL` in `app.js` to your backend URL

### Option 4: Docker Deployment

Create a `Dockerfile` in the backend directory:
```dockerfile
FROM python:3.9-slim
WORKDIR /app
COPY requirements.txt .
RUN pip install -r requirements.txt
COPY . .
CMD ["python", "server.py"]
```

Build and run:
```bash
docker build -t chatgpt-backend .
docker run -p 5000:5000 --env-file ../.env chatgpt-backend
```

## 🐛 Troubleshooting

### Backend won't start
- Check if Python 3.8+ is installed: `python --version`
- Verify all dependencies are installed: `pip install -r requirements.txt`
- Check if port 5000 is available: `lsof -i :5000`

### "Backend not running" error in frontend
- Ensure the backend server is running
- Check the console for CORS errors
- Verify `API_BASE_URL` in `app.js` matches your backend URL

### API key errors
- Verify your API key is correct in `.env`
- Check if your OpenAI account has sufficient credits
- Ensure the API key has proper permissions

### Rate limit errors
- Wait a few minutes and try again
- Consider upgrading your OpenAI plan
- Implement request throttling in your application

## 📊 Usage Examples

### Example 1: Simple Question
```
User: What is the capital of France?
ChatGPT: The capital of France is Paris.
```

### Example 2: Code Generation
```
User: Write a Python function to calculate factorial
ChatGPT: Here's a Python function to calculate factorial:
[code response]
```

### Example 3: Creative Writing
```
User: Write a haiku about coding
ChatGPT: [creative haiku response]
```

## 🔗 Resources

- [OpenAI API Documentation](https://platform.openai.com/docs)
- [Flask Documentation](https://flask.palletsprojects.com/)
- [OpenAI Pricing](https://openai.com/pricing)
- [Rate Limits](https://platform.openai.com/docs/guides/rate-limits)

## 📝 License

This integration is part of the Nushell project and follows the same MIT license.

## 🤝 Contributing

Contributions are welcome! Please follow the main Nushell contributing guidelines.

## ⚠️ Disclaimer

This integration uses OpenAI's API which is a paid service. Monitor your usage to avoid unexpected charges.
