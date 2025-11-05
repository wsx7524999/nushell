# Nushell GitHub Pages Site

This directory contains the GitHub Pages website for the Nushell project.

## 🌐 Live Site

Once deployed, the site will be available at: `https://wsx7524999.github.io/nushell/`

## 📁 Structure

- **`index.html`** - Homepage with features overview
- **`blog/`** - Blog section with posts about Nushell and AI integration
- **`tutorials/`** - Step-by-step tutorials from beginner to advanced
- **`chatbot/`** - Interactive AI chatbot demonstration
- **`assets/`** - CSS, JavaScript, and images
- **`_config.yml`** - Jekyll configuration for GitHub Pages

## 🚀 Deployment

To enable GitHub Pages for this site:

1. Go to repository **Settings** → **Pages**
2. Set **Source** to "Deploy from a branch"
3. Select branch and folder: `main` (or your branch) → `/docs`
4. Click **Save**

GitHub will automatically build and deploy the site.

## 🛠️ Local Development

To test the site locally:

```bash
# Navigate to the docs directory
cd docs

# Start a simple HTTP server
python3 -m http.server 8080

# Open http://localhost:8080 in your browser
```

## 📝 Content

### Blog Posts
1. **Integrating ChatGPT with Nushell** - About AI integration benefits
2. **The Power of Structured Data** - How Nushell handles structured data
3. **Building a True Cross-Platform Shell** - Cross-platform development journey

### Tutorials
1. **Getting Started with Nushell** (Beginner, 15 min) - Installation and basics
2. **Mastering Data Pipelines** (Intermediate, 25 min) - Advanced data manipulation
3. **Setting Up AI Integration** (Intermediate, 20 min) - ChatGPT setup guide

### Chatbot
Interactive demo with simulated AI responses. For real ChatGPT integration, follow the setup tutorial.

## 🎨 Customization

### Colors
Edit `assets/css/main.css` to change the color scheme. Main colors are defined in CSS variables:

```css
:root {
    --primary-color: #667eea;
    --secondary-color: #764ba2;
    --accent-color: #f093fb;
}
```

### Navigation
Edit the navigation menu in each HTML file or update `_config.yml` for site-wide changes.

### Content
- Blog posts: Edit HTML files in `blog/`
- Tutorials: Edit HTML files in `tutorials/`
- Chatbot responses: Edit `chatbot/chatbot.js`

## 📱 Responsive Design

The site is fully responsive with breakpoints at:
- **Mobile**: < 768px (hamburger menu)
- **Tablet**: 768px - 1024px
- **Desktop**: > 1024px

## 🔧 Technologies

- **HTML5** - Semantic markup
- **CSS3** - Modern styling with flexbox and grid
- **JavaScript (ES6)** - Interactive features
- **Jekyll** - Static site generation (via GitHub Pages)

## 📄 License

This site content is part of the Nushell project and follows the same MIT license.
