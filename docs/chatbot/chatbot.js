// Simulated AI responses for demo purposes
const simulatedResponses = {
    'filter files larger than 1mb': {
        response: `To filter files larger than 1MB in Nushell, you can use the \`where\` command with a size comparison:

\`\`\`
ls | where size > 1mb
\`\`\`

This command:
1. Lists all files in the current directory with \`ls\`
2. Filters the results using \`where\` to show only files where \`size > 1mb\`

You can also combine this with other filters:

\`\`\`
# Show only files (not directories) larger than 1MB
ls | where type == "file" and size > 1mb

# Sort by size and show top 10
ls | where size > 1mb | sort-by size --reverse | first 10
\`\`\`

The size comparison supports units like \`kb\`, \`mb\`, \`gb\`, etc.`
    },
    'where and select': {
        response: `Great question! \`where\` and \`select\` serve different purposes in Nushell:

**where** - Filters rows (records)
- Keeps rows that match a condition
- Works like a SQL WHERE clause
- Example: \`ls | where size > 1mb\` (keeps only large files)

**select** - Selects columns (fields)
- Chooses which columns to display
- Works like a SQL SELECT clause
- Example: \`ls | select name size\` (shows only name and size columns)

**Common Usage Pattern:**
\`\`\`
ls | where size > 1mb | select name size modified
\`\`\`

This filters to large files (where), then shows only specific columns (select).

**Key Difference:**
- \`where\` reduces the number of rows
- \`select\` reduces the number of columns

Think of \`where\` as asking "which items?" and \`select\` as asking "which information about those items?"`
    },
    'json to csv': {
        response: `Converting JSON to CSV in Nushell is straightforward using the \`to csv\` command:

\`\`\`
# Basic conversion
open data.json | to csv | save output.csv

# With custom delimiter (e.g., tab-separated)
open data.json | to csv --separator "\\t" | save output.tsv

# Without headers
open data.json | to csv --noheaders | save output.csv
\`\`\`

**Example with real data:**
\`\`\`
# Convert user data
open users.json | to csv | save users.csv

# Select specific fields before converting
open users.json | select name email age | to csv | save contacts.csv
\`\`\`

**For nested JSON:**
If your JSON has nested objects, you might need to flatten it first:
\`\`\`
open data.json | flatten | to csv | save output.csv
\`\`\`

The conversion preserves the structure and automatically handles the CSV formatting for you!`
    },
    'group data by column': {
        response: `To group data by a column in Nushell, use the \`group-by\` command:

\`\`\`
# Basic grouping by a single column
open data.csv | group-by category

# Group and count items in each group
open data.csv | group-by status | each { |key, values| 
    { 
        status: $key, 
        count: ($values | length) 
    } 
}

# Group and calculate sum
open sales.csv | group-by region | each { |key, values|
    {
        region: $key,
        total_sales: ($values | get amount | math sum),
        avg_sales: ($values | get amount | math avg)
    }
}
\`\`\`

**Real Example - Sales by Region:**
\`\`\`
open sales.csv 
  | group-by region 
  | each { |key, values| 
      {
        region: $key,
        total: ($values | get amount | math sum),
        count: ($values | length),
        average: ($values | get amount | math avg)
      }
    }
  | sort-by total --reverse
\`\`\`

This groups sales data by region and calculates totals, counts, and averages for each region.`
    },
    'parse analyze log files': {
        response: `Here's how to parse and analyze log files with Nushell:

**1. Read and Filter Logs:**
\`\`\`
# Read log file and filter by keyword
open app.log | lines | where $it =~ "ERROR"

# Count errors by hour
open app.log | lines | where $it =~ "ERROR" | length
\`\`\`

**2. Parse Structured Logs:**
\`\`\`
# Parse common log format
open access.log 
  | lines 
  | parse "{ip} - - [{timestamp}] \"{method} {path} {protocol}\" {status} {size}"
  | where status >= 400

# For JSON logs
open app.json | lines | each { |line| $line | from json } | where level == "error"
\`\`\`

**3. Aggregate Statistics:**
\`\`\`
# Count status codes
open access.log 
  | lines 
  | parse "{ip} - - [{timestamp}] \"{method} {path} {protocol}\" {status} {size}"
  | group-by status 
  | each { |key, values| { status: $key, count: ($values | length) } }
  | sort-by count --reverse

# Find most accessed paths
open access.log 
  | lines 
  | parse "{ip} - - [{timestamp}] \"{method} {path} {protocol}\" {status} {size}"
  | group-by path 
  | each { |key, values| { path: $key, hits: ($values | length) } }
  | sort-by hits --reverse 
  | first 10
\`\`\`

**4. Time-based Analysis:**
\`\`\`
# Errors in the last hour
open app.log 
  | lines 
  | where $it =~ "ERROR" 
  | where modified > ((date now) - 1hr)
\`\`\`

Nushell makes log analysis powerful and intuitive!`
    },
    'default': {
        response: `I'm a simulated chatbot demonstrating Nushell AI capabilities. Here are some things I can help with:

**Common Topics:**
- Basic Nushell commands and syntax
- Data filtering and transformation
- Working with structured data (JSON, CSV, TOML)
- Pipeline operations
- File and directory operations
- Data aggregation and analysis

**For Real AI Assistance:**
Follow our [AI Integration Setup Tutorial](../tutorials/ai-integration-setup.html) to connect to ChatGPT for comprehensive, personalized help.

**Try asking:**
- "How do I filter files larger than 1MB?"
- "What's the difference between where and select?"
- "How can I convert JSON to CSV?"

What would you like to know about Nushell?`
    }
};

// DOM elements
const messagesContainer = document.getElementById('messages');
const userInput = document.getElementById('user-input');
const sendBtn = document.getElementById('send-btn');
const modelSelect = document.getElementById('model-select');
const questionBtns = document.querySelectorAll('.question-btn');

// Initialize
let messageCount = 0;

// Add event listeners
sendBtn.addEventListener('click', sendMessage);
userInput.addEventListener('keypress', (e) => {
    if (e.key === 'Enter') {
        sendMessage();
    }
});

questionBtns.forEach(btn => {
    btn.addEventListener('click', () => {
        userInput.value = btn.textContent;
        sendMessage();
    });
});

function sendMessage() {
    const message = userInput.value.trim();
    if (!message) return;

    // Clear empty state on first message
    if (messageCount === 0) {
        messagesContainer.innerHTML = '';
    }

    // Add user message
    addMessage(message, 'user');
    userInput.value = '';
    userInput.disabled = true;
    sendBtn.disabled = true;

    // Show typing indicator
    const typingIndicator = addTypingIndicator();

    // Simulate AI response delay
    setTimeout(() => {
        removeTypingIndicator(typingIndicator);
        const response = getSimulatedResponse(message);
        addMessage(response, 'assistant');
        userInput.disabled = false;
        sendBtn.disabled = false;
        userInput.focus();
    }, 1500 + Math.random() * 1000);

    messageCount++;
}

function addMessage(text, type) {
    const messageDiv = document.createElement('div');
    messageDiv.className = `message ${type}-message`;
    
    if (type === 'assistant') {
        // Parse markdown-style code blocks
        text = text.replace(/```(\w*)\n?([\s\S]*?)```/g, (match, lang, code) => {
            return `<pre><code>${escapeHtml(code.trim())}</code></pre>`;
        });
        // Parse inline code
        text = text.replace(/`([^`]+)`/g, '<code>$1</code>');
        // Parse bold
        text = text.replace(/\*\*([^*]+)\*\*/g, '<strong>$1</strong>');
        // Preserve line breaks
        text = text.replace(/\n/g, '<br>');
        
        messageDiv.innerHTML = text;
    } else {
        messageDiv.textContent = text;
    }
    
    messagesContainer.appendChild(messageDiv);
    messagesContainer.scrollTop = messagesContainer.scrollHeight;
    
    return messageDiv;
}

function addTypingIndicator() {
    const indicator = document.createElement('div');
    indicator.className = 'typing-indicator active';
    indicator.innerHTML = '<span></span><span></span><span></span>';
    messagesContainer.appendChild(indicator);
    messagesContainer.scrollTop = messagesContainer.scrollHeight;
    return indicator;
}

function removeTypingIndicator(indicator) {
    if (indicator && indicator.parentNode) {
        indicator.parentNode.removeChild(indicator);
    }
}

function getSimulatedResponse(question) {
    const lowerQuestion = question.toLowerCase();
    
    // Match keywords to responses
    for (const [key, data] of Object.entries(simulatedResponses)) {
        if (key !== 'default' && lowerQuestion.includes(key)) {
            return data.response;
        }
    }
    
    // Default response
    return simulatedResponses.default.response;
}

function escapeHtml(text) {
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
}

// Show welcome message after a short delay
setTimeout(() => {
    if (messageCount === 0) {
        messagesContainer.innerHTML = '';
        addMessage(
            `Hello! I'm your Nushell AI Assistant. 👋

I can help you with:
- Understanding Nushell commands
- Writing pipelines and filters
- Working with structured data
- Converting between file formats
- And much more!

**Note:** This is a demo with simulated responses. For real ChatGPT assistance, check out our [setup tutorial](../tutorials/ai-integration-setup.html).

Try clicking a suggested question below or ask me anything about Nushell!`,
            'assistant'
        );
        messageCount++;
    }
}, 500);
