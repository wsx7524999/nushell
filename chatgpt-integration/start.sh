#!/bin/bash
# Startup script for ChatGPT Integration

echo "=========================================="
echo "  ChatGPT Integration Startup Script"
echo "=========================================="
echo ""

# Check if we're in the right directory
if [ ! -f "backend/server.py" ]; then
    echo "Error: Please run this script from the chatgpt-integration directory"
    echo "Usage: cd chatgpt-integration && ./start.sh"
    exit 1
fi

# Check if Python is installed
if ! command -v python3 &> /dev/null; then
    echo "Error: Python 3 is not installed"
    echo "Please install Python 3.8 or higher"
    exit 1
fi

# Check if .env file exists
if [ ! -f ".env" ]; then
    echo "Warning: .env file not found"
    echo "Creating .env from .env.example..."
    cp .env.example .env
    echo ""
    echo "⚠️  IMPORTANT: Edit the .env file and add your OpenAI API key!"
    echo "   Open .env and replace 'your-openai-api-key-here' with your actual key"
    echo ""
    read -p "Press Enter after you've added your API key to continue..."
fi

# Check if dependencies are installed
echo "Checking Python dependencies..."
if ! python3 -c "import flask" &> /dev/null; then
    echo "Installing Python dependencies..."
    pip3 install -r backend/requirements.txt
    if [ $? -ne 0 ]; then
        echo "Error: Failed to install dependencies"
        exit 1
    fi
else
    echo "✓ Dependencies are installed"
fi

echo ""
echo "=========================================="
echo "Starting Backend Server..."
echo "=========================================="
echo ""

# Start the backend server
cd backend
python3 server.py &
BACKEND_PID=$!

# Wait a moment for the server to start
sleep 2

# Check if server is running
if ps -p $BACKEND_PID > /dev/null; then
    echo ""
    echo "✓ Backend server started successfully!"
    echo ""
    echo "=========================================="
    echo "  How to Use:"
    echo "=========================================="
    echo ""
    echo "1. Backend API is running at: http://localhost:5000"
    echo "2. Open the frontend:"
    echo "   - Option A: Open frontend/index.html in your browser"
    echo "   - Option B: Run a local web server:"
    echo "     cd ../frontend && python3 -m http.server 8000"
    echo "     Then open: http://localhost:8000"
    echo ""
    echo "3. To run tests:"
    echo "   cd ../tests && python3 test_integration.py"
    echo ""
    echo "4. To stop the server:"
    echo "   kill $BACKEND_PID"
    echo ""
    echo "=========================================="
    echo ""
    echo "Press Ctrl+C to stop the backend server"
    
    # Wait for the server process
    wait $BACKEND_PID
else
    echo "Error: Failed to start backend server"
    exit 1
fi
