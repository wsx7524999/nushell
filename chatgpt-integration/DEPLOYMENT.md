# Deployment Guide for ChatGPT Integration

This guide provides step-by-step instructions for deploying the ChatGPT integration in various environments.

## Table of Contents
- [Local Development](#local-development)
- [Docker Deployment](#docker-deployment)
- [Heroku Deployment](#heroku-deployment)
- [GitHub Pages + Backend](#github-pages--backend)
- [Production Considerations](#production-considerations)

---

## Local Development

### Quick Start (Recommended)

```bash
cd chatgpt-integration
./start.sh
```

### Manual Setup

1. **Install Python Dependencies**
   ```bash
   cd chatgpt-integration/backend
   pip install -r requirements.txt
   ```

2. **Configure Environment**
   ```bash
   cd ..
   cp .env.example .env
   ```
   
   Edit `.env` and add your OpenAI API key:
   ```
   OPENAI_API_KEY=sk-your-actual-api-key-here
   ```

3. **Start Backend Server**
   ```bash
   cd backend
   python server.py
   ```
   Backend will be available at `http://localhost:5000`

4. **Open Frontend**
   - Option A: Double-click `frontend/index.html`
   - Option B: Use a local server:
     ```bash
     cd frontend
     python -m http.server 8000
     ```
     Then open `http://localhost:8000` in your browser

---

## Docker Deployment

### Using Docker Compose (Recommended)

1. **Create docker-compose.yml**

   Create this file in the `chatgpt-integration` directory:

   ```yaml
   version: '3.8'
   
   services:
     backend:
       build: ./backend
       ports:
         - "5000:5000"
       environment:
         - OPENAI_API_KEY=${OPENAI_API_KEY}
         - PORT=5000
       restart: unless-stopped
     
     frontend:
       image: nginx:alpine
       ports:
         - "8080:80"
       volumes:
         - ./frontend:/usr/share/nginx/html:ro
       depends_on:
         - backend
       restart: unless-stopped
   ```

2. **Create Dockerfile for Backend**

   Create `backend/Dockerfile`:

   ```dockerfile
   FROM python:3.9-slim

   WORKDIR /app

   # Install dependencies
   COPY requirements.txt .
   RUN pip install --no-cache-dir -r requirements.txt

   # Copy application
   COPY server.py .

   # Expose port
   EXPOSE 5000

   # Run the application
   CMD ["python", "server.py"]
   ```

3. **Deploy**
   ```bash
   # Create .env file with your API key
   echo "OPENAI_API_KEY=sk-your-api-key" > .env
   
   # Start containers
   docker-compose up -d
   
   # View logs
   docker-compose logs -f
   
   # Stop containers
   docker-compose down
   ```

4. **Access the Application**
   - Frontend: `http://localhost:8080`
   - Backend API: `http://localhost:5000/api/health`

---

## Heroku Deployment

### Backend Deployment

1. **Install Heroku CLI**
   ```bash
   # macOS
   brew tap heroku/brew && brew install heroku
   
   # Linux
   curl https://cli-assets.heroku.com/install.sh | sh
   ```

2. **Login to Heroku**
   ```bash
   heroku login
   ```

3. **Prepare Backend for Heroku**

   Create `backend/Procfile`:
   ```
   web: python server.py
   ```

   Create `backend/runtime.txt`:
   ```
   python-3.9.16
   ```

4. **Deploy Backend**
   ```bash
   cd chatgpt-integration/backend
   
   # Initialize git (if not already)
   git init
   
   # Create Heroku app
   heroku create your-chatgpt-backend
   
   # Set environment variables
   heroku config:set OPENAI_API_KEY=sk-your-api-key
   
   # Deploy
   git add .
   git commit -m "Deploy ChatGPT backend"
   git push heroku main
   
   # Open the app
   heroku open
   ```

5. **Get Backend URL**
   ```bash
   heroku info
   # Note the "Web URL" - this is your backend URL
   ```

### Frontend Deployment

1. **Update API URL**

   Edit `frontend/app.js` and change:
   ```javascript
   const API_BASE_URL = 'https://your-chatgpt-backend.herokuapp.com/api';
   ```

2. **Deploy to GitHub Pages**
   ```bash
   cd frontend
   
   # Create a new branch
   git checkout -b gh-pages
   
   # Commit frontend files
   git add .
   git commit -m "Deploy frontend"
   
   # Push to GitHub
   git push origin gh-pages
   ```

3. **Enable GitHub Pages**
   - Go to repository Settings > Pages
   - Select `gh-pages` branch
   - Your frontend will be available at: `https://username.github.io/repository-name/`

---

## GitHub Pages + Backend

### Option 1: Backend on Heroku

1. Deploy backend to Heroku (see above)
2. Update `API_BASE_URL` in `app.js`
3. Deploy frontend to GitHub Pages

### Option 2: Backend on Any Cloud Provider

Popular options:
- **Railway**: Easy deployment, good free tier
- **Render**: Similar to Heroku, good free tier
- **Fly.io**: Edge deployment, generous free tier
- **AWS Lambda**: Serverless option with API Gateway

Example for Railway:

```bash
# Install Railway CLI
npm install -g @railway/cli

# Login
railway login

# Initialize project
cd chatgpt-integration/backend
railway init

# Deploy
railway up

# Set environment variables
railway variables set OPENAI_API_KEY=sk-your-api-key
```

---

## Production Considerations

### Security

1. **HTTPS Only**
   - Use HTTPS for both frontend and backend
   - Many platforms (Heroku, Netlify, Vercel) provide free SSL

2. **Environment Variables**
   - Never commit `.env` files
   - Use platform-specific secrets management
   - Rotate API keys regularly

3. **CORS Configuration**
   ```python
   # In server.py, update CORS to specific origins
   CORS(app, resources={
       r"/api/*": {
           "origins": ["https://yourdomain.com"]
       }
   })
   ```

4. **Rate Limiting**
   ```python
   from flask_limiter import Limiter
   
   limiter = Limiter(
       app,
       key_func=lambda: request.remote_addr,
       default_limits=["100 per day", "10 per minute"]
   )
   ```

### Performance

1. **Caching**
   - Consider caching common responses
   - Use Redis for session management

2. **Load Balancing**
   - Deploy multiple backend instances
   - Use a load balancer (nginx, HAProxy, or cloud provider)

3. **Monitoring**
   ```python
   # Add logging
   import logging
   logging.basicConfig(level=logging.INFO)
   
   # Track metrics
   from prometheus_flask_exporter import PrometheusMetrics
   metrics = PrometheusMetrics(app)
   ```

### Cost Management

1. **OpenAI API Costs**
   - Set usage limits in OpenAI dashboard
   - Monitor token usage
   - Use GPT-3.5-turbo for cost-effective responses
   - Implement user rate limiting

2. **Backend Hosting**
   - Free tiers:
     - Heroku: 550-1000 hours/month
     - Railway: $5 credit/month
     - Render: 750 hours/month
   - Consider serverless for low traffic

3. **Implementation**
   ```python
   # Add usage tracking
   @app.route('/api/usage', methods=['GET'])
   def get_usage():
       # Track and return usage statistics
       pass
   ```

### Backup and Reliability

1. **Database for Chat History** (Optional)
   ```python
   # Add SQLite or PostgreSQL
   from flask_sqlalchemy import SQLAlchemy
   
   app.config['SQLALCHEMY_DATABASE_URI'] = os.getenv('DATABASE_URL')
   db = SQLAlchemy(app)
   ```

2. **Error Handling**
   - Implement retry logic for API calls
   - Graceful degradation when API is unavailable
   - User-friendly error messages

3. **Health Checks**
   - Already implemented at `/api/health`
   - Configure uptime monitoring (UptimeRobot, Pingdom)

---

## Testing Production Deployment

1. **Backend Health Check**
   ```bash
   curl https://your-backend-url.com/api/health
   ```

2. **Test Chat Endpoint**
   ```bash
   curl -X POST https://your-backend-url.com/api/chat \
     -H "Content-Type: application/json" \
     -d '{"message": "Hello", "model": "gpt-3.5-turbo"}'
   ```

3. **Frontend Testing**
   - Open browser console
   - Check for network errors
   - Verify API calls succeed

4. **Load Testing**
   ```bash
   # Using Apache Bench
   ab -n 100 -c 10 https://your-backend-url.com/api/health
   ```

---

## Troubleshooting

### Common Issues

1. **CORS Errors**
   - Update CORS settings in `server.py`
   - Ensure frontend URL is whitelisted

2. **API Key Not Found**
   - Verify environment variable is set
   - Check variable name matches exactly

3. **502 Bad Gateway**
   - Backend server may be down
   - Check logs for errors
   - Verify port configuration

4. **Slow Response Times**
   - OpenAI API can take 2-10 seconds
   - Consider adding loading indicators
   - Implement timeout handling

---

## Next Steps

- [ ] Set up monitoring and alerting
- [ ] Implement user authentication
- [ ] Add chat history persistence
- [ ] Create mobile-responsive design improvements
- [ ] Add multi-language support
- [ ] Implement conversation context management
- [ ] Add file upload capabilities
- [ ] Create admin dashboard for usage tracking

---

## Support

For issues specific to this integration, please open an issue in the repository.

For OpenAI API issues, consult the [OpenAI Documentation](https://platform.openai.com/docs).
