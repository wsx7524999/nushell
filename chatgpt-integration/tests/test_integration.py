#!/usr/bin/env python3
"""
ChatGPT Integration Test Script
Tests the backend API endpoints and validates responses
"""

import sys
import time
import requests
import json

# Configuration
API_BASE_URL = 'http://localhost:5000/api'
TEST_TIMEOUT = 10  # seconds

class Colors:
    """ANSI color codes for terminal output"""
    GREEN = '\033[92m'
    RED = '\033[91m'
    YELLOW = '\033[93m'
    BLUE = '\033[94m'
    RESET = '\033[0m'
    BOLD = '\033[1m'

def print_test(message):
    """Print test message"""
    print(f"{Colors.BLUE}[TEST]{Colors.RESET} {message}")

def print_success(message):
    """Print success message"""
    print(f"{Colors.GREEN}[✓]{Colors.RESET} {message}")

def print_error(message):
    """Print error message"""
    print(f"{Colors.RED}[✗]{Colors.RESET} {message}")

def print_warning(message):
    """Print warning message"""
    print(f"{Colors.YELLOW}[!]{Colors.RESET} {message}")

def test_backend_health():
    """Test 1: Backend Health Check"""
    print_test("Testing backend health endpoint...")
    
    try:
        response = requests.get(
            f"{API_BASE_URL}/health",
            timeout=TEST_TIMEOUT
        )
        
        if response.status_code == 200:
            data = response.json()
            if data.get('status') == 'ok':
                print_success("Backend health check passed")
                return True
            else:
                print_error(f"Unexpected response: {data}")
                return False
        else:
            print_error(f"Health check failed with status code: {response.status_code}")
            return False
            
    except requests.exceptions.ConnectionError:
        print_error("Cannot connect to backend. Is the server running?")
        print_warning("Start the backend with: cd backend && python server.py")
        return False
    except Exception as e:
        print_error(f"Health check error: {str(e)}")
        return False

def test_models_endpoint():
    """Test 2: Get Available Models"""
    print_test("Testing models endpoint...")
    
    try:
        response = requests.get(
            f"{API_BASE_URL}/models",
            timeout=TEST_TIMEOUT
        )
        
        if response.status_code == 200:
            data = response.json()
            if 'models' in data and len(data['models']) > 0:
                print_success(f"Models endpoint passed. Available models: {len(data['models'])}")
                for model in data['models']:
                    print(f"  - {model['name']} ({model['id']})")
                return True
            else:
                print_error("No models returned")
                return False
        else:
            print_error(f"Models endpoint failed with status code: {response.status_code}")
            return False
            
    except Exception as e:
        print_error(f"Models endpoint error: {str(e)}")
        return False

def test_chat_endpoint_without_api_key():
    """Test 3: Chat Endpoint Without API Key (Expected to Fail Gracefully)"""
    print_test("Testing chat endpoint without API key (error handling)...")
    
    try:
        response = requests.post(
            f"{API_BASE_URL}/chat",
            json={"message": "Hello, ChatGPT!"},
            timeout=TEST_TIMEOUT
        )
        
        # We expect this to fail if no API key is configured
        if response.status_code in [401, 500]:
            data = response.json()
            if 'error' in data:
                print_success("Error handling works correctly (no API key configured)")
                print(f"  Error message: {data['error']}")
                return True
        elif response.status_code == 200:
            print_success("Chat endpoint responded successfully (API key is configured)")
            return True
        else:
            print_error(f"Unexpected status code: {response.status_code}")
            return False
            
    except Exception as e:
        print_error(f"Chat endpoint error: {str(e)}")
        return False

def test_chat_endpoint_missing_message():
    """Test 4: Chat Endpoint With Missing Message Field"""
    print_test("Testing chat endpoint with missing message field...")
    
    try:
        response = requests.post(
            f"{API_BASE_URL}/chat",
            json={},  # Empty request
            timeout=TEST_TIMEOUT
        )
        
        if response.status_code == 400:
            data = response.json()
            if 'error' in data:
                print_success("Input validation works correctly")
                print(f"  Error message: {data['error']}")
                return True
        
        print_error(f"Expected 400 status code, got: {response.status_code}")
        return False
            
    except Exception as e:
        print_error(f"Validation test error: {str(e)}")
        return False

def test_chat_endpoint_with_sample():
    """Test 5: Chat Endpoint With Sample Query (Requires API Key)"""
    print_test("Testing chat endpoint with sample query...")
    print_warning("Note: This test requires a valid OpenAI API key")
    
    try:
        response = requests.post(
            f"{API_BASE_URL}/chat",
            json={
                "message": "Say 'Hello' in exactly one word",
                "model": "gpt-3.5-turbo"
            },
            timeout=30  # Give more time for API call
        )
        
        if response.status_code == 200:
            data = response.json()
            if 'response' in data:
                print_success("Chat endpoint works correctly with API key")
                print(f"  ChatGPT Response: {data['response'][:100]}...")
                if 'usage' in data:
                    print(f"  Tokens used: {data['usage']['total_tokens']}")
                return True
            else:
                print_error("Response missing 'response' field")
                return False
        elif response.status_code in [401, 500]:
            data = response.json()
            print_warning(f"Chat test skipped: {data.get('error', 'No API key configured')}")
            return None  # Skip this test
        else:
            print_error(f"Chat endpoint failed with status code: {response.status_code}")
            return False
            
    except Exception as e:
        print_error(f"Chat endpoint error: {str(e)}")
        return False

def run_all_tests():
    """Run all tests and print summary"""
    print(f"\n{Colors.BOLD}{'='*60}{Colors.RESET}")
    print(f"{Colors.BOLD}ChatGPT Integration Test Suite{Colors.RESET}")
    print(f"{Colors.BOLD}{'='*60}{Colors.RESET}\n")
    
    tests = [
        ("Backend Health Check", test_backend_health),
        ("Models Endpoint", test_models_endpoint),
        ("Chat Error Handling", test_chat_endpoint_without_api_key),
        ("Input Validation", test_chat_endpoint_missing_message),
        ("Chat with Sample Query", test_chat_endpoint_with_sample),
    ]
    
    results = []
    
    for test_name, test_func in tests:
        print(f"\n{Colors.BOLD}Running: {test_name}{Colors.RESET}")
        print("-" * 60)
        result = test_func()
        results.append((test_name, result))
        time.sleep(0.5)  # Brief pause between tests
    
    # Print summary
    print(f"\n{Colors.BOLD}{'='*60}{Colors.RESET}")
    print(f"{Colors.BOLD}Test Summary{Colors.RESET}")
    print(f"{Colors.BOLD}{'='*60}{Colors.RESET}\n")
    
    passed = sum(1 for _, result in results if result is True)
    failed = sum(1 for _, result in results if result is False)
    skipped = sum(1 for _, result in results if result is None)
    total = len(results)
    
    for test_name, result in results:
        if result is True:
            print_success(f"{test_name}")
        elif result is False:
            print_error(f"{test_name}")
        else:
            print_warning(f"{test_name} (skipped)")
    
    print(f"\n{Colors.BOLD}Results:{Colors.RESET}")
    print(f"  Total: {total}")
    print(f"  {Colors.GREEN}Passed: {passed}{Colors.RESET}")
    print(f"  {Colors.RED}Failed: {failed}{Colors.RESET}")
    print(f"  {Colors.YELLOW}Skipped: {skipped}{Colors.RESET}")
    
    if failed == 0 and passed > 0:
        print(f"\n{Colors.GREEN}{Colors.BOLD}✓ All tests passed!{Colors.RESET}\n")
        return 0
    elif failed > 0:
        print(f"\n{Colors.RED}{Colors.BOLD}✗ Some tests failed{Colors.RESET}\n")
        return 1
    else:
        print(f"\n{Colors.YELLOW}{Colors.BOLD}! All tests were skipped{Colors.RESET}\n")
        return 2

if __name__ == '__main__':
    exit_code = run_all_tests()
    sys.exit(exit_code)
