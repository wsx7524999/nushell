# Nushell API Documentation

## Overview

This document provides comprehensive documentation for the Nushell REST API. The API allows developers to programmatically interact with Nushell instances, execute commands, and manage shell sessions.

**Base URL:** `https://api.nushell.sh/v1`

**Authentication:** Bearer token (JWT)

---

## Table of Contents

1. [Authentication](#authentication)
2. [API Endpoints](#api-endpoints)
3. [API Playground](#api-playground)
4. [Integration Examples](#integration-examples)
5. [Analytics and Monitoring](#analytics-and-monitoring)

---

## Authentication

All API requests require authentication using a Bearer token in the Authorization header.

### Request Headers

```http
Authorization: Bearer YOUR_API_TOKEN
Content-Type: application/json
```

### Example Authentication Request

```bash
curl -H "Authorization: Bearer YOUR_API_TOKEN" \
     -H "Content-Type: application/json" \
     https://api.nushell.sh/v1/commands
```

---

## API Endpoints

### 1. Commands API

#### GET /api/v1/commands

Retrieve a list of available Nushell commands.

**Request:**

```http
GET /api/v1/commands HTTP/1.1
Host: api.nushell.sh
Authorization: Bearer YOUR_API_TOKEN
```

**Response:**

```json
{
  "status": "success",
  "data": {
    "commands": [
      {
        "name": "ls",
        "category": "filesystem",
        "description": "List directory contents",
        "usage": "ls [path]"
      },
      {
        "name": "cd",
        "category": "filesystem",
        "description": "Change directory",
        "usage": "cd <path>"
      },
      {
        "name": "ps",
        "category": "system",
        "description": "List running processes",
        "usage": "ps"
      }
    ],
    "total": 3
  }
}
```

**Status Codes:**
- `200 OK` - Success
- `401 Unauthorized` - Invalid or missing authentication token
- `500 Internal Server Error` - Server error

---

#### POST /api/v1/commands/execute

Execute a Nushell command and return the result.

**Request:**

```http
POST /api/v1/commands/execute HTTP/1.1
Host: api.nushell.sh
Authorization: Bearer YOUR_API_TOKEN
Content-Type: application/json

{
  "command": "ls | where type == \"dir\"",
  "session_id": "session-123",
  "timeout": 5000
}
```

**Parameters:**
- `command` (string, required) - The Nushell command to execute
- `session_id` (string, optional) - Session identifier for stateful operations
- `timeout` (integer, optional) - Execution timeout in milliseconds (default: 30000)

**Response:**

```json
{
  "status": "success",
  "data": {
    "output": [
      {
        "name": "crates",
        "type": "dir",
        "size": "4.0 KiB",
        "modified": "2024-01-15T10:30:00Z"
      },
      {
        "name": "src",
        "type": "dir",
        "size": "4.0 KiB",
        "modified": "2024-01-15T09:45:00Z"
      }
    ],
    "execution_time_ms": 125,
    "session_id": "session-123"
  }
}
```

**Status Codes:**
- `200 OK` - Command executed successfully
- `400 Bad Request` - Invalid command syntax
- `401 Unauthorized` - Invalid authentication
- `408 Request Timeout` - Command execution exceeded timeout
- `500 Internal Server Error` - Execution error

---

### 2. Sessions API

#### POST /api/v1/sessions

Create a new shell session.

**Request:**

```http
POST /api/v1/sessions HTTP/1.1
Host: api.nushell.sh
Authorization: Bearer YOUR_API_TOKEN
Content-Type: application/json

{
  "environment": {
    "PATH": "/usr/local/bin:/usr/bin:/bin",
    "HOME": "/home/user"
  },
  "working_directory": "/home/user/projects"
}
```

**Response:**

```json
{
  "status": "success",
  "data": {
    "session_id": "session-abc123",
    "created_at": "2024-01-15T10:00:00Z",
    "expires_at": "2024-01-15T11:00:00Z"
  }
}
```

**Status Codes:**
- `201 Created` - Session created successfully
- `401 Unauthorized` - Invalid authentication
- `429 Too Many Requests` - Session limit exceeded

---

#### GET /api/v1/sessions/{session_id}

Retrieve session information.

**Request:**

```http
GET /api/v1/sessions/session-abc123 HTTP/1.1
Host: api.nushell.sh
Authorization: Bearer YOUR_API_TOKEN
```

**Response:**

```json
{
  "status": "success",
  "data": {
    "session_id": "session-abc123",
    "created_at": "2024-01-15T10:00:00Z",
    "last_activity": "2024-01-15T10:15:00Z",
    "expires_at": "2024-01-15T11:00:00Z",
    "working_directory": "/home/user/projects",
    "command_history": 15
  }
}
```

---

#### DELETE /api/v1/sessions/{session_id}

Terminate a shell session.

**Request:**

```http
DELETE /api/v1/sessions/session-abc123 HTTP/1.1
Host: api.nushell.sh
Authorization: Bearer YOUR_API_TOKEN
```

**Response:**

```json
{
  "status": "success",
  "message": "Session terminated successfully"
}
```

**Status Codes:**
- `200 OK` - Session terminated
- `404 Not Found` - Session not found

---

### 3. Plugins API

#### GET /api/v1/plugins

List available Nushell plugins.

**Request:**

```http
GET /api/v1/plugins HTTP/1.1
Host: api.nushell.sh
Authorization: Bearer YOUR_API_TOKEN
```

**Response:**

```json
{
  "status": "success",
  "data": {
    "plugins": [
      {
        "name": "nu_plugin_query",
        "version": "0.108.0",
        "description": "Query data using xpath/css selectors",
        "enabled": true
      },
      {
        "name": "nu_plugin_gstat",
        "version": "0.108.0",
        "description": "Git status plugin",
        "enabled": true
      }
    ],
    "total": 2
  }
}
```

---

#### POST /api/v1/plugins/{plugin_name}/execute

Execute a plugin command.

**Request:**

```http
POST /api/v1/plugins/nu_plugin_query/execute HTTP/1.1
Host: api.nushell.sh
Authorization: Bearer YOUR_API_TOKEN
Content-Type: application/json

{
  "input": "<html><body><h1>Title</h1></body></html>",
  "selector": "h1",
  "query_type": "css"
}
```

**Response:**

```json
{
  "status": "success",
  "data": {
    "results": ["Title"],
    "execution_time_ms": 45
  }
}
```

---

### 4. Files API

#### GET /api/v1/files

List files in a directory.

**Request:**

```http
GET /api/v1/files?path=/home/user&session_id=session-abc123 HTTP/1.1
Host: api.nushell.sh
Authorization: Bearer YOUR_API_TOKEN
```

**Query Parameters:**
- `path` (string, required) - Directory path
- `session_id` (string, required) - Session identifier

**Response:**

```json
{
  "status": "success",
  "data": {
    "path": "/home/user",
    "files": [
      {
        "name": "document.txt",
        "type": "file",
        "size": 1024,
        "modified": "2024-01-15T09:30:00Z"
      },
      {
        "name": "projects",
        "type": "dir",
        "size": 4096,
        "modified": "2024-01-15T10:00:00Z"
      }
    ],
    "total": 2
  }
}
```

---

#### POST /api/v1/files/read

Read file contents.

**Request:**

```http
POST /api/v1/files/read HTTP/1.1
Host: api.nushell.sh
Authorization: Bearer YOUR_API_TOKEN
Content-Type: application/json

{
  "path": "/home/user/config.toml",
  "session_id": "session-abc123",
  "encoding": "utf-8"
}
```

**Response:**

```json
{
  "status": "success",
  "data": {
    "content": "[package]\nname = \"example\"\nversion = \"1.0.0\"",
    "size": 45,
    "encoding": "utf-8",
    "mime_type": "text/toml"
  }
}
```

---

## API Playground

### Interactive API Testing

You can test the Nushell API endpoints directly using our integrated API playground tools.

#### Swagger UI Integration

Embed Swagger UI in your documentation or application:

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Nushell API - Swagger UI</title>
    <link rel="stylesheet" href="https://unpkg.com/swagger-ui-dist@5.9.0/swagger-ui.css">
</head>
<body>
    <div id="swagger-ui"></div>
    <script src="https://unpkg.com/swagger-ui-dist@5.9.0/swagger-ui-bundle.js"></script>
    <script>
        window.onload = function() {
            SwaggerUIBundle({
                url: "https://api.nushell.sh/v1/openapi.json",
                dom_id: '#swagger-ui',
                deepLinking: true,
                presets: [
                    SwaggerUIBundle.presets.apis,
                    SwaggerUIBundle.SwaggerUIStandalonePreset
                ]
            })
        }
    </script>
</body>
</html>
```

#### Hoppscotch (Postwoman) Integration

Use the following iframe to embed Hoppscotch:

```html
<iframe 
    src="https://hoppscotch.io/embed?url=https://api.nushell.sh/v1/commands"
    width="100%" 
    height="600px" 
    frameborder="0" 
    style="border: 1px solid #ccc; border-radius: 4px;">
</iframe>
```

#### Quick Test Example

Try this simple cURL command to test the API:

```bash
# List available commands
curl -X GET "https://api.nushell.sh/v1/commands" \
     -H "Authorization: Bearer YOUR_API_TOKEN" \
     -H "Content-Type: application/json"

# Execute a command
curl -X POST "https://api.nushell.sh/v1/commands/execute" \
     -H "Authorization: Bearer YOUR_API_TOKEN" \
     -H "Content-Type: application/json" \
     -d '{
       "command": "ls | length",
       "session_id": "test-session"
     }'
```

---

## Integration Examples

### AWS Integration

#### Using AWS Lambda with Nushell API

```javascript
// AWS Lambda function to execute Nushell commands
const https = require('https');

exports.handler = async (event) => {
    const options = {
        hostname: 'api.nushell.sh',
        port: 443,
        path: '/v1/commands/execute',
        method: 'POST',
        headers: {
            'Authorization': `Bearer ${process.env.NUSHELL_API_TOKEN}`,
            'Content-Type': 'application/json'
        }
    };

    const requestBody = JSON.stringify({
        command: event.command || 'ls',
        session_id: event.session_id || `lambda-${Date.now()}`,
        timeout: 5000
    });

    return new Promise((resolve, reject) => {
        const req = https.request(options, (res) => {
            let data = '';
            
            res.on('data', (chunk) => {
                data += chunk;
            });
            
            res.on('end', () => {
                resolve({
                    statusCode: res.statusCode,
                    body: data
                });
            });
        });

        req.on('error', (error) => {
            reject(error);
        });

        req.write(requestBody);
        req.end();
    });
};
```

#### AWS SDK v3 Integration

```javascript
// Using AWS SDK v3 with Nushell API
import { SecretsManagerClient, GetSecretValueCommand } from "@aws-sdk/client-secrets-manager";
import axios from 'axios';

class NushellAWSIntegration {
    constructor(region = 'us-east-1') {
        this.secretsClient = new SecretsManagerClient({ region });
        this.apiToken = null;
    }

    async getApiToken() {
        if (this.apiToken) return this.apiToken;
        
        const command = new GetSecretValueCommand({
            SecretId: "nushell-api-token"
        });
        
        const response = await this.secretsClient.send(command);
        this.apiToken = JSON.parse(response.SecretString).token;
        return this.apiToken;
    }

    async executeCommand(command, sessionId = null) {
        const token = await this.getApiToken();
        
        const response = await axios.post(
            'https://api.nushell.sh/v1/commands/execute',
            {
                command,
                session_id: sessionId || `aws-${Date.now()}`,
                timeout: 10000
            },
            {
                headers: {
                    'Authorization': `Bearer ${token}`,
                    'Content-Type': 'application/json'
                }
            }
        );
        
        return response.data;
    }

    async listFiles(path, sessionId) {
        const token = await this.getApiToken();
        
        const response = await axios.get(
            `https://api.nushell.sh/v1/files`,
            {
                params: { path, session_id: sessionId },
                headers: {
                    'Authorization': `Bearer ${token}`
                }
            }
        );
        
        return response.data;
    }
}

// Usage example
const integration = new NushellAWSIntegration();
const result = await integration.executeCommand('ps | where cpu > 0');
console.log(result);
```

---

### Google Cloud Integration

#### Using Google Cloud Functions

```javascript
// Google Cloud Function with Nushell API
const { SecretManagerServiceClient } = require('@google-cloud/secret-manager');
const axios = require('axios');

const secretClient = new SecretManagerServiceClient();

async function getNushellToken() {
    const [version] = await secretClient.accessSecretVersion({
        name: 'projects/YOUR_PROJECT_ID/secrets/nushell-api-token/versions/latest',
    });
    
    return version.payload.data.toString();
}

exports.executeNushellCommand = async (req, res) => {
    try {
        const { command, sessionId } = req.body;
        const apiToken = await getNushellToken();
        
        const response = await axios.post(
            'https://api.nushell.sh/v1/commands/execute',
            {
                command,
                session_id: sessionId || `gcp-${Date.now()}`,
                timeout: 8000
            },
            {
                headers: {
                    'Authorization': `Bearer ${apiToken}`,
                    'Content-Type': 'application/json'
                }
            }
        );
        
        res.status(200).json(response.data);
    } catch (error) {
        console.error('Error executing Nushell command:', error);
        res.status(500).json({ 
            error: 'Failed to execute command',
            message: error.message 
        });
    }
};
```

#### Google Cloud Pub/Sub Integration

```javascript
// Process Nushell commands from Pub/Sub messages
const { PubSub } = require('@google-cloud/pubsub');
const axios = require('axios');

const pubsub = new PubSub();
const subscriptionName = 'nushell-commands-sub';

async function processMessage(message) {
    const data = JSON.parse(message.data.toString());
    const apiToken = process.env.NUSHELL_API_TOKEN;
    
    try {
        const response = await axios.post(
            'https://api.nushell.sh/v1/commands/execute',
            {
                command: data.command,
                session_id: data.sessionId,
                timeout: 5000
            },
            {
                headers: {
                    'Authorization': `Bearer ${apiToken}`,
                    'Content-Type': 'application/json'
                }
            }
        );
        
        console.log('Command executed:', response.data);
        message.ack();
    } catch (error) {
        console.error('Error:', error);
        message.nack();
    }
}

// Subscribe to messages
const subscription = pubsub.subscription(subscriptionName);
subscription.on('message', processMessage);
```

---

### Facebook API Integration

#### Facebook Webhook Integration with Nushell

```javascript
// Express.js server integrating Facebook webhooks with Nushell API
const express = require('express');
const axios = require('axios');
const crypto = require('crypto');

const app = express();
app.use(express.json());

const FACEBOOK_VERIFY_TOKEN = process.env.FACEBOOK_VERIFY_TOKEN;
const FACEBOOK_APP_SECRET = process.env.FACEBOOK_APP_SECRET;
const NUSHELL_API_TOKEN = process.env.NUSHELL_API_TOKEN;

// Verify Facebook webhook
app.get('/webhook', (req, res) => {
    const mode = req.query['hub.mode'];
    const token = req.query['hub.verify_token'];
    const challenge = req.query['hub.challenge'];
    
    if (mode === 'subscribe' && token === FACEBOOK_VERIFY_TOKEN) {
        console.log('Webhook verified');
        res.status(200).send(challenge);
    } else {
        res.sendStatus(403);
    }
});

// Handle Facebook webhook events
app.post('/webhook', async (req, res) => {
    const signature = req.headers['x-hub-signature-256'];
    
    // Verify signature
    const expectedSignature = crypto
        .createHmac('sha256', FACEBOOK_APP_SECRET)
        .update(JSON.stringify(req.body))
        .digest('hex');
    
    if (signature !== `sha256=${expectedSignature}`) {
        return res.sendStatus(403);
    }
    
    // Process webhook event
    const body = req.body;
    
    if (body.object === 'page') {
        for (const entry of body.entry) {
            for (const event of entry.messaging) {
                await handleMessage(event);
            }
        }
    }
    
    res.sendStatus(200);
});

async function handleMessage(event) {
    const senderId = event.sender.id;
    const message = event.message.text;
    
    // Execute Nushell command based on message
    if (message.startsWith('/nu ')) {
        const command = message.substring(4);
        
        try {
            const response = await axios.post(
                'https://api.nushell.sh/v1/commands/execute',
                {
                    command,
                    session_id: `fb-${senderId}`,
                    timeout: 5000
                },
                {
                    headers: {
                        'Authorization': `Bearer ${NUSHELL_API_TOKEN}`,
                        'Content-Type': 'application/json'
                    }
                }
            );
            
            // Send result back to Facebook user
            await sendFacebookMessage(senderId, JSON.stringify(response.data.output, null, 2));
        } catch (error) {
            await sendFacebookMessage(senderId, `Error: ${error.message}`);
        }
    }
}

async function sendFacebookMessage(recipientId, text) {
    const PAGE_ACCESS_TOKEN = process.env.FACEBOOK_PAGE_ACCESS_TOKEN;
    
    await axios.post(
        `https://graph.facebook.com/v18.0/me/messages?access_token=${PAGE_ACCESS_TOKEN}`,
        {
            recipient: { id: recipientId },
            message: { text }
        }
    );
}

app.listen(3000, () => {
    console.log('Facebook webhook server running on port 3000');
});
```

---

### JavaScript SDK Example

#### Browser-based Integration

```javascript
// Nushell API JavaScript SDK
class NushellAPI {
    constructor(apiToken, baseURL = 'https://api.nushell.sh/v1') {
        this.apiToken = apiToken;
        this.baseURL = baseURL;
    }

    async request(endpoint, options = {}) {
        const url = `${this.baseURL}${endpoint}`;
        const headers = {
            'Authorization': `Bearer ${this.apiToken}`,
            'Content-Type': 'application/json',
            ...options.headers
        };

        const response = await fetch(url, {
            ...options,
            headers
        });

        if (!response.ok) {
            throw new Error(`API Error: ${response.status} ${response.statusText}`);
        }

        return response.json();
    }

    // Commands
    async listCommands() {
        return this.request('/commands');
    }

    async executeCommand(command, sessionId = null, timeout = 30000) {
        return this.request('/commands/execute', {
            method: 'POST',
            body: JSON.stringify({
                command,
                session_id: sessionId,
                timeout
            })
        });
    }

    // Sessions
    async createSession(environment = {}, workingDirectory = null) {
        return this.request('/sessions', {
            method: 'POST',
            body: JSON.stringify({
                environment,
                working_directory: workingDirectory
            })
        });
    }

    async getSession(sessionId) {
        return this.request(`/sessions/${sessionId}`);
    }

    async deleteSession(sessionId) {
        return this.request(`/sessions/${sessionId}`, {
            method: 'DELETE'
        });
    }

    // Files
    async listFiles(path, sessionId) {
        const params = new URLSearchParams({ path, session_id: sessionId });
        return this.request(`/files?${params}`);
    }

    async readFile(path, sessionId, encoding = 'utf-8') {
        return this.request('/files/read', {
            method: 'POST',
            body: JSON.stringify({
                path,
                session_id: sessionId,
                encoding
            })
        });
    }

    // Plugins
    async listPlugins() {
        return this.request('/plugins');
    }

    async executePlugin(pluginName, input, options = {}) {
        return this.request(`/plugins/${pluginName}/execute`, {
            method: 'POST',
            body: JSON.stringify({
                input,
                ...options
            })
        });
    }
}

// Usage example
const api = new NushellAPI('YOUR_API_TOKEN');

// Create a session
const session = await api.createSession({
    PATH: '/usr/local/bin:/usr/bin',
    HOME: '/home/user'
});

console.log('Session created:', session.data.session_id);

// Execute commands
const result = await api.executeCommand(
    'ls | where type == "dir"',
    session.data.session_id
);

console.log('Command result:', result.data.output);

// List files
const files = await api.listFiles('/home/user', session.data.session_id);
console.log('Files:', files.data.files);

// Clean up
await api.deleteSession(session.data.session_id);
```

#### Node.js Integration with TypeScript

```typescript
// Nushell API TypeScript SDK
import axios, { AxiosInstance, AxiosRequestConfig } from 'axios';

interface NushellConfig {
    apiToken: string;
    baseURL?: string;
}

interface CommandExecuteRequest {
    command: string;
    session_id?: string;
    timeout?: number;
}

interface SessionCreateRequest {
    environment?: Record<string, string>;
    working_directory?: string;
}

interface ApiResponse<T> {
    status: string;
    data: T;
}

class NushellClient {
    private client: AxiosInstance;

    constructor(config: NushellConfig) {
        this.client = axios.create({
            baseURL: config.baseURL || 'https://api.nushell.sh/v1',
            headers: {
                'Authorization': `Bearer ${config.apiToken}`,
                'Content-Type': 'application/json'
            }
        });
    }

    async listCommands(): Promise<ApiResponse<any>> {
        const response = await this.client.get('/commands');
        return response.data;
    }

    async executeCommand(request: CommandExecuteRequest): Promise<ApiResponse<any>> {
        const response = await this.client.post('/commands/execute', request);
        return response.data;
    }

    async createSession(request: SessionCreateRequest): Promise<ApiResponse<any>> {
        const response = await this.client.post('/sessions', request);
        return response.data;
    }

    async getSession(sessionId: string): Promise<ApiResponse<any>> {
        const response = await this.client.get(`/sessions/${sessionId}`);
        return response.data;
    }

    async deleteSession(sessionId: string): Promise<ApiResponse<any>> {
        const response = await this.client.delete(`/sessions/${sessionId}`);
        return response.data;
    }

    async listFiles(path: string, sessionId: string): Promise<ApiResponse<any>> {
        const response = await this.client.get('/files', {
            params: { path, session_id: sessionId }
        });
        return response.data;
    }

    async readFile(path: string, sessionId: string, encoding: string = 'utf-8'): Promise<ApiResponse<any>> {
        const response = await this.client.post('/files/read', {
            path,
            session_id: sessionId,
            encoding
        });
        return response.data;
    }
}

// Usage
const client = new NushellClient({
    apiToken: process.env.NUSHELL_API_TOKEN!
});

async function example() {
    // Create session
    const session = await client.createSession({
        environment: {
            PATH: '/usr/local/bin:/usr/bin'
        }
    });

    // Execute command
    const result = await client.executeCommand({
        command: 'ps | length',
        session_id: session.data.session_id
    });

    console.log(result.data);

    // Cleanup
    await client.deleteSession(session.data.session_id);
}

example().catch(console.error);
```

---

## Analytics and Monitoring

### Best Practices for API Usage Tracking

#### 1. Request Logging

Log all API requests with relevant metadata:

```javascript
// Express middleware for API request logging
const morgan = require('morgan');
const winston = require('winston');

const logger = winston.createLogger({
    level: 'info',
    format: winston.format.json(),
    transports: [
        new winston.transports.File({ filename: 'api-requests.log' })
    ]
});

// Custom token for user ID
morgan.token('user-id', (req) => req.user?.id || 'anonymous');

// Morgan format
const format = ':method :url :status :response-time ms - :user-id';

app.use(morgan(format, {
    stream: {
        write: (message) => logger.info(message.trim())
    }
}));
```

#### 2. Performance Monitoring

Track API performance metrics:

```javascript
// Performance monitoring middleware
function performanceMonitor(req, res, next) {
    const start = Date.now();
    
    res.on('finish', () => {
        const duration = Date.now() - start;
        const metric = {
            method: req.method,
            path: req.path,
            statusCode: res.statusCode,
            duration,
            timestamp: new Date().toISOString()
        };
        
        // Log slow requests
        if (duration > 1000) {
            console.warn('Slow API request detected:', metric);
        }
        
        // Send to monitoring service
        sendMetric(metric);
    });
    
    next();
}

app.use(performanceMonitor);
```

---

### Google Analytics Integration

```javascript
// Google Analytics 4 integration for API events
const { BetaAnalyticsDataClient } = require('@google-analytics/data');

class APIAnalytics {
    constructor(propertyId) {
        this.analyticsDataClient = new BetaAnalyticsDataClient();
        this.propertyId = propertyId;
    }

    async trackApiCall(endpoint, method, statusCode, duration) {
        // Send event to Google Analytics
        const event = {
            name: 'api_call',
            params: {
                endpoint,
                method,
                status_code: statusCode,
                duration_ms: duration,
                timestamp: Date.now()
            }
        };
        
        // Use Measurement Protocol for server-side tracking
        await this.sendEvent(event);
    }

    async sendEvent(event) {
        const fetch = require('node-fetch');
        const measurementId = process.env.GA_MEASUREMENT_ID;
        const apiSecret = process.env.GA_API_SECRET;
        
        const url = `https://www.google-analytics.com/mp/collect?measurement_id=${measurementId}&api_secret=${apiSecret}`;
        
        await fetch(url, {
            method: 'POST',
            body: JSON.stringify({
                client_id: 'nushell-api-server',
                events: [event]
            })
        });
    }

    async getApiMetrics(startDate, endDate) {
        const [response] = await this.analyticsDataClient.runReport({
            property: `properties/${this.propertyId}`,
            dateRanges: [{ startDate, endDate }],
            dimensions: [
                { name: 'customEvent:endpoint' },
                { name: 'customEvent:method' }
            ],
            metrics: [
                { name: 'eventCount' },
                { name: 'averageSessionDuration' }
            ]
        });
        
        return response;
    }
}

// Usage
const analytics = new APIAnalytics('YOUR_PROPERTY_ID');

app.use(async (req, res, next) => {
    const start = Date.now();
    
    res.on('finish', async () => {
        const duration = Date.now() - start;
        await analytics.trackApiCall(
            req.path,
            req.method,
            res.statusCode,
            duration
        );
    });
    
    next();
});
```

---

### Datadog Integration

```javascript
// Datadog APM integration
const tracer = require('dd-trace').init({
    service: 'nushell-api',
    env: process.env.NODE_ENV || 'development',
    version: '1.0.0'
});

const { StatsD } = require('hot-shots');
const dogstatsd = new StatsD({
    host: process.env.DD_AGENT_HOST || 'localhost',
    port: 8125,
    prefix: 'nushell.api.'
});

class DatadogMonitoring {
    trackApiCall(endpoint, method, statusCode, duration) {
        // Increment request counter
        dogstatsd.increment('requests.count', 1, [`endpoint:${endpoint}`, `method:${method}`]);
        
        // Track response time
        dogstatsd.histogram('requests.duration', duration, [`endpoint:${endpoint}`]);
        
        // Track status codes
        dogstatsd.increment(`requests.status.${statusCode}`, 1, [`endpoint:${endpoint}`]);
        
        // Track error rate
        if (statusCode >= 500) {
            dogstatsd.increment('requests.errors', 1, [`endpoint:${endpoint}`]);
        }
    }

    trackCommandExecution(command, success, duration) {
        dogstatsd.increment('commands.executed', 1, [`success:${success}`]);
        dogstatsd.histogram('commands.duration', duration);
        
        if (!success) {
            dogstatsd.increment('commands.failed', 1);
        }
    }

    trackSessionCreation(sessionId) {
        dogstatsd.increment('sessions.created', 1);
        dogstatsd.gauge('sessions.active', 1, [`session:${sessionId}`]);
    }

    trackSessionDestruction(sessionId, duration) {
        dogstatsd.decrement('sessions.active', 1, [`session:${sessionId}`]);
        dogstatsd.histogram('sessions.lifetime', duration);
    }
}

// Express middleware
const monitoring = new DatadogMonitoring();

app.use((req, res, next) => {
    const start = Date.now();
    
    res.on('finish', () => {
        const duration = Date.now() - start;
        monitoring.trackApiCall(
            req.path,
            req.method,
            res.statusCode,
            duration
        );
    });
    
    next();
});

// Custom metrics
app.post('/api/v1/commands/execute', async (req, res) => {
    const start = Date.now();
    let success = false;
    
    try {
        // Execute command
        const result = await executeCommand(req.body.command);
        success = true;
        res.json(result);
    } catch (error) {
        res.status(500).json({ error: error.message });
    } finally {
        const duration = Date.now() - start;
        monitoring.trackCommandExecution(req.body.command, success, duration);
    }
});
```

---

### AWS CloudWatch Integration

```javascript
// AWS CloudWatch monitoring
const { CloudWatchClient, PutMetricDataCommand } = require('@aws-sdk/client-cloudwatch');
const { CloudWatchLogsClient, PutLogEventsCommand, CreateLogStreamCommand } = require('@aws-sdk/client-cloudwatch-logs');

class CloudWatchMonitoring {
    constructor(region = 'us-east-1') {
        this.cloudwatch = new CloudWatchClient({ region });
        this.logs = new CloudWatchLogsClient({ region });
        this.namespace = 'NushellAPI';
        this.logGroupName = '/aws/nushell-api';
    }

    async putMetric(metricName, value, unit = 'Count', dimensions = []) {
        const params = {
            Namespace: this.namespace,
            MetricData: [
                {
                    MetricName: metricName,
                    Value: value,
                    Unit: unit,
                    Timestamp: new Date(),
                    Dimensions: dimensions
                }
            ]
        };

        try {
            await this.cloudwatch.send(new PutMetricDataCommand(params));
        } catch (error) {
            console.error('Error sending metric to CloudWatch:', error);
        }
    }

    async trackApiCall(endpoint, method, statusCode, duration) {
        const dimensions = [
            { Name: 'Endpoint', Value: endpoint },
            { Name: 'Method', Value: method }
        ];

        // Track request count
        await this.putMetric('RequestCount', 1, 'Count', dimensions);

        // Track response time
        await this.putMetric('ResponseTime', duration, 'Milliseconds', dimensions);

        // Track errors
        if (statusCode >= 500) {
            await this.putMetric('ErrorCount', 1, 'Count', dimensions);
        }

        // Track status codes
        await this.putMetric(`StatusCode${statusCode}`, 1, 'Count', dimensions);
    }

    async logApiRequest(requestData) {
        const logStreamName = `api-requests-${new Date().toISOString().split('T')[0]}`;
        
        try {
            // Create log stream if it doesn't exist
            try {
                await this.logs.send(new CreateLogStreamCommand({
                    logGroupName: this.logGroupName,
                    logStreamName
                }));
            } catch (error) {
                // Stream might already exist
            }

            // Put log event
            await this.logs.send(new PutLogEventsCommand({
                logGroupName: this.logGroupName,
                logStreamName,
                logEvents: [
                    {
                        message: JSON.stringify(requestData),
                        timestamp: Date.now()
                    }
                ]
            }));
        } catch (error) {
            console.error('Error logging to CloudWatch:', error);
        }
    }

    async trackCommandExecution(command, success, duration, output) {
        const dimensions = [
            { Name: 'Success', Value: success.toString() }
        ];

        await this.putMetric('CommandExecutions', 1, 'Count', dimensions);
        await this.putMetric('CommandDuration', duration, 'Milliseconds', dimensions);

        // Log detailed command execution
        await this.logApiRequest({
            type: 'command_execution',
            command,
            success,
            duration,
            output: success ? output : undefined,
            timestamp: new Date().toISOString()
        });
    }

    async createDashboard() {
        // Create CloudWatch dashboard configuration
        const dashboardBody = {
            widgets: [
                {
                    type: 'metric',
                    properties: {
                        metrics: [
                            ['NushellAPI', 'RequestCount', { stat: 'Sum' }]
                        ],
                        period: 300,
                        stat: 'Sum',
                        region: 'us-east-1',
                        title: 'Total API Requests'
                    }
                },
                {
                    type: 'metric',
                    properties: {
                        metrics: [
                            ['NushellAPI', 'ResponseTime', { stat: 'Average' }]
                        ],
                        period: 300,
                        stat: 'Average',
                        region: 'us-east-1',
                        title: 'Average Response Time'
                    }
                },
                {
                    type: 'metric',
                    properties: {
                        metrics: [
                            ['NushellAPI', 'ErrorCount', { stat: 'Sum' }]
                        ],
                        period: 300,
                        stat: 'Sum',
                        region: 'us-east-1',
                        title: 'Error Count'
                    }
                }
            ]
        };

        console.log('Dashboard configuration:', JSON.stringify(dashboardBody, null, 2));
        return dashboardBody;
    }
}

// Express middleware
const monitoring = new CloudWatchMonitoring();

app.use(async (req, res, next) => {
    const start = Date.now();
    
    res.on('finish', async () => {
        const duration = Date.now() - start;
        
        await monitoring.trackApiCall(
            req.path,
            req.method,
            res.statusCode,
            duration
        );

        await monitoring.logApiRequest({
            method: req.method,
            path: req.path,
            statusCode: res.statusCode,
            duration,
            userAgent: req.headers['user-agent'],
            timestamp: new Date().toISOString()
        });
    });
    
    next();
});
```

---

### Monitoring Dashboard Template

```javascript
// Combined monitoring dashboard using multiple services
class MonitoringDashboard {
    constructor(config) {
        this.googleAnalytics = config.googleAnalytics;
        this.datadog = config.datadog;
        this.cloudwatch = config.cloudwatch;
    }

    async getApiMetrics(startDate, endDate) {
        const metrics = {
            totalRequests: 0,
            averageResponseTime: 0,
            errorRate: 0,
            topEndpoints: [],
            statusCodeDistribution: {}
        };

        // Aggregate from different sources
        if (this.googleAnalytics) {
            const gaMetrics = await this.googleAnalytics.getApiMetrics(startDate, endDate);
            metrics.totalRequests += gaMetrics.totalEvents;
        }

        if (this.datadog) {
            // Fetch from Datadog API
            const ddMetrics = await this.fetchDatadogMetrics(startDate, endDate);
            metrics.averageResponseTime = ddMetrics.avgResponseTime;
        }

        if (this.cloudwatch) {
            // Fetch from CloudWatch
            const cwMetrics = await this.fetchCloudWatchMetrics(startDate, endDate);
            metrics.errorRate = cwMetrics.errorRate;
        }

        return metrics;
    }

    generateReport(metrics) {
        return {
            summary: {
                totalRequests: metrics.totalRequests,
                averageResponseTime: `${metrics.averageResponseTime}ms`,
                errorRate: `${(metrics.errorRate * 100).toFixed(2)}%`,
                uptime: '99.9%'
            },
            topEndpoints: metrics.topEndpoints,
            recommendations: this.generateRecommendations(metrics)
        };
    }

    generateRecommendations(metrics) {
        const recommendations = [];

        if (metrics.averageResponseTime > 1000) {
            recommendations.push({
                type: 'performance',
                message: 'Average response time exceeds 1 second. Consider optimization.',
                priority: 'high'
            });
        }

        if (metrics.errorRate > 0.05) {
            recommendations.push({
                type: 'reliability',
                message: 'Error rate exceeds 5%. Investigate error causes.',
                priority: 'critical'
            });
        }

        return recommendations;
    }
}
```

---

## Rate Limiting and Quotas

All API endpoints are subject to rate limiting:

- **Free Tier**: 1,000 requests per hour
- **Pro Tier**: 10,000 requests per hour
- **Enterprise Tier**: Custom limits

Rate limit headers are included in all responses:

```http
X-RateLimit-Limit: 1000
X-RateLimit-Remaining: 995
X-RateLimit-Reset: 1705334400
```

---

## Error Handling

All errors follow a consistent format:

```json
{
  "status": "error",
  "error": {
    "code": "INVALID_COMMAND",
    "message": "The command syntax is invalid",
    "details": "Unexpected token at position 5"
  }
}
```

**Common Error Codes:**
- `INVALID_TOKEN` - Authentication token is invalid
- `INVALID_COMMAND` - Command syntax error
- `SESSION_NOT_FOUND` - Session does not exist
- `TIMEOUT` - Command execution timed out
- `RATE_LIMIT_EXCEEDED` - Too many requests

---

## Support

For API support and questions:
- Documentation: https://www.nushell.sh/book/
- Discord: https://discord.gg/NtAbbGn
- GitHub Issues: https://github.com/nushell/nushell/issues

---

**Last Updated:** January 15, 2025
**API Version:** 1.0.0
