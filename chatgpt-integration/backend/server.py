#!/usr/bin/env python3
"""
ChatGPT Backend Server
A simple Flask server to interact with OpenAI's ChatGPT API
"""

import os
import sys
from flask import Flask, request, jsonify
from flask_cors import CORS
import openai
from dotenv import load_dotenv

# Load environment variables from .env file
load_dotenv()

app = Flask(__name__)
CORS(app)  # Enable CORS for frontend access

# Configure OpenAI API
openai.api_key = os.getenv('OPENAI_API_KEY')

@app.route('/api/health', methods=['GET'])
def health_check():
    """Health check endpoint"""
    return jsonify({
        'status': 'ok',
        'message': 'ChatGPT backend is running'
    })

@app.route('/api/chat', methods=['POST'])
def chat():
    """
    Main chat endpoint
    Expects JSON: {"message": "user prompt", "model": "gpt-4"}
    """
    try:
        data = request.get_json()
        
        if not data or 'message' not in data:
            return jsonify({
                'error': 'Missing required field: message'
            }), 400
        
        user_message = data['message']
        model = data.get('model', 'gpt-4')
        
        # Check if API key is configured
        if not openai.api_key:
            return jsonify({
                'error': 'OpenAI API key not configured. Please set OPENAI_API_KEY in .env file'
            }), 500
        
        # Call OpenAI API
        response = openai.ChatCompletion.create(
            model=model,
            messages=[
                {"role": "system", "content": "You are a helpful assistant."},
                {"role": "user", "content": user_message}
            ],
            max_tokens=500,
            temperature=0.7
        )
        
        assistant_message = response.choices[0].message.content
        
        return jsonify({
            'response': assistant_message,
            'model': model,
            'usage': {
                'prompt_tokens': response.usage.prompt_tokens,
                'completion_tokens': response.usage.completion_tokens,
                'total_tokens': response.usage.total_tokens
            }
        })
        
    except openai.error.AuthenticationError:
        return jsonify({
            'error': 'Invalid API key. Please check your OPENAI_API_KEY in .env file'
        }), 401
    except openai.error.RateLimitError:
        return jsonify({
            'error': 'Rate limit exceeded. Please try again later'
        }), 429
    except openai.error.APIError as e:
        return jsonify({
            'error': f'OpenAI API error: {str(e)}'
        }), 500
    except Exception as e:
        return jsonify({
            'error': f'Server error: {str(e)}'
        }), 500

@app.route('/api/models', methods=['GET'])
def get_models():
    """Get available ChatGPT models"""
    return jsonify({
        'models': [
            {'id': 'gpt-4', 'name': 'GPT-4'},
            {'id': 'gpt-4-turbo-preview', 'name': 'GPT-4 Turbo'},
            {'id': 'gpt-3.5-turbo', 'name': 'GPT-3.5 Turbo'}
        ]
    })

if __name__ == '__main__':
    # Check if API key is set
    if not os.getenv('OPENAI_API_KEY'):
        print("WARNING: OPENAI_API_KEY not found in environment variables")
        print("Please create a .env file with your OpenAI API key")
        print("Example: OPENAI_API_KEY=sk-your-api-key-here")
    
    port = int(os.getenv('PORT', 5000))
    debug = os.getenv('DEBUG', 'false').lower() == 'true'
    
    print(f"Starting ChatGPT Backend Server on port {port}...")
    app.run(host='0.0.0.0', port=port, debug=debug)
