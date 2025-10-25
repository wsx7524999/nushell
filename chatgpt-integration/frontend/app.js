// ChatGPT Frontend JavaScript
// API Configuration
const API_BASE_URL = 'http://localhost:5000/api';

// DOM Elements
const messagesContainer = document.getElementById('messages');
const userInput = document.getElementById('user-input');
const sendBtn = document.getElementById('send-btn');
const modelSelect = document.getElementById('model-select');
const statusDiv = document.getElementById('status');
const errorDiv = document.getElementById('error');

// State
let isLoading = false;

// Initialize
document.addEventListener('DOMContentLoaded', () => {
    checkBackendHealth();
    setupEventListeners();
});

// Setup Event Listeners
function setupEventListeners() {
    sendBtn.addEventListener('click', sendMessage);
    userInput.addEventListener('keypress', (e) => {
        if (e.key === 'Enter' && !isLoading) {
            sendMessage();
        }
    });
}

// Check Backend Health
async function checkBackendHealth() {
    try {
        const response = await fetch(`${API_BASE_URL}/health`);
        const data = await response.json();
        
        if (data.status === 'ok') {
            showStatus('Backend connected successfully', 'success');
            setTimeout(() => hideStatus(), 3000);
        }
    } catch (error) {
        showError('Backend server not running. Please start the backend server first.');
    }
}

// Send Message
async function sendMessage() {
    const message = userInput.value.trim();
    
    if (!message || isLoading) {
        return;
    }

    // Clear input
    userInput.value = '';
    hideError();
    
    // Remove empty state
    const emptyState = messagesContainer.querySelector('.empty-state');
    if (emptyState) {
        emptyState.remove();
    }

    // Add user message
    addMessage(message, 'user');

    // Set loading state
    setLoading(true);

    try {
        const selectedModel = modelSelect.value;
        
        const response = await fetch(`${API_BASE_URL}/chat`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({
                message: message,
                model: selectedModel
            })
        });

        const data = await response.json();

        if (!response.ok) {
            throw new Error(data.error || 'Failed to get response from ChatGPT');
        }

        // Add assistant response
        addMessage(data.response, 'assistant');

        // Show usage info (optional)
        if (data.usage) {
            console.log('Token usage:', data.usage);
        }

    } catch (error) {
        console.error('Error:', error);
        showError(error.message || 'Failed to communicate with the backend');
        addMessage('Sorry, I encountered an error. Please try again.', 'assistant');
    } finally {
        setLoading(false);
    }
}

// Add Message to Chat
function addMessage(text, type) {
    const messageDiv = document.createElement('div');
    messageDiv.className = `message ${type}-message`;
    messageDiv.textContent = text;
    messagesContainer.appendChild(messageDiv);
    
    // Scroll to bottom
    messagesContainer.scrollTop = messagesContainer.scrollHeight;
}

// Set Loading State
function setLoading(loading) {
    isLoading = loading;
    sendBtn.disabled = loading;
    userInput.disabled = loading;
    
    if (loading) {
        sendBtn.innerHTML = '<span class="loading"></span>';
    } else {
        sendBtn.textContent = 'Send';
    }
}

// Show Status Message
function showStatus(message, type = 'info') {
    statusDiv.textContent = message;
    statusDiv.className = `status ${type}`;
    statusDiv.style.display = 'block';
}

// Hide Status Message
function hideStatus() {
    statusDiv.style.display = 'none';
}

// Show Error Message
function showError(message) {
    errorDiv.textContent = message;
    errorDiv.style.display = 'block';
}

// Hide Error Message
function hideError() {
    errorDiv.style.display = 'none';
}

// Clear Chat
function clearChat() {
    messagesContainer.innerHTML = `
        <div class="empty-state">
            <svg fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z"></path>
            </svg>
            <p>Start a conversation with ChatGPT!</p>
        </div>
    `;
}
