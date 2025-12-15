use handlebars::Handlebars;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TemplateError {
    #[error("Template not found: {0}")]
    TemplateNotFound(String),
    #[error("Template rendering failed: {0}")]
    RenderingError(#[from] handlebars::RenderError),
    #[error("File I/O error: {0}")]
    IoError(#[from] std::io::Error),
}

pub struct TemplateEngine {
    handlebars: Handlebars<'static>,
}

#[derive(Debug, Clone)]
pub struct Template {
    pub name: String,
    pub framework: String,
    pub files: HashMap<String, String>,
    pub dependencies: HashMap<String, String>,
}

impl TemplateEngine {
    pub fn new() -> Self {
        let mut handlebars = Handlebars::new();
        handlebars.register_helper("upper", Box::new(upper_helper));
        handlebars.register_helper("lower", Box::new(lower_helper));
        
        Self { handlebars }
    }

    pub fn load_templates(&self) -> Result<HashMap<String, Template>, TemplateError> {
        let mut templates = HashMap::new();
        
        // Load React + Vite template
        templates.insert("react".to_string(), self.load_react_template()?);
        
        // Load Next.js template
        templates.insert("nextjs".to_string(), self.load_nextjs_template()?);
        
        // Load Flask template
        templates.insert("flask".to_string(), self.load_flask_template()?);
        
        // Load Express template
        templates.insert("express".to_string(), self.load_express_template()?);
        
        // Load CLI template
        templates.insert("cli".to_string(), self.load_cli_template()?);
        
        Ok(templates)
    }

    fn load_react_template(&self) -> Result<Template, TemplateError> {
        let mut files = HashMap::new();
        
        files.insert("package.json".to_string(), r#"{
  "name": "{{project_name}}",
  "private": true,
  "version": "0.0.0",
  "type": "module",
  "scripts": {
    "dev": "vite",
    "build": "tsc && vite build",
    "lint": "eslint . --ext ts,tsx --report-unused-disable-directives --max-warnings 0",
    "preview": "vite preview"
  },
  "dependencies": {
    "react": "^18.2.0",
    "react-dom": "^18.2.0"
  },
  "devDependencies": {
    "@types/react": "^18.2.66",
    "@types/react-dom": "^18.2.22",
    "@typescript-eslint/eslint-plugin": "^7.2.0",
    "@typescript-eslint/parser": "^7.2.0",
    "@vitejs/plugin-react": "^4.2.1",
    "autoprefixer": "^10.4.19",
    "eslint": "^8.57.0",
    "eslint-plugin-react-hooks": "^4.6.0",
    "eslint-plugin-react-refresh": "^0.4.6",
    "postcss": "^8.4.38",
    "tailwindcss": "^3.4.1",
    "typescript": "^5.2.2",
    "vite": "^5.2.0"
  }
}"#.to_string());

        files.insert("vite.config.ts".to_string(), r#"import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

export default defineConfig({
  plugins: [react()],
})"#.to_string());

        files.insert("tsconfig.json".to_string(), r#"{
  "compilerOptions": {
    "target": "ES2020",
    "useDefineForClassFields": true,
    "lib": ["ES2020", "DOM", "DOM.Iterable"],
    "module": "ESNext",
    "skipLibCheck": true,
    "moduleResolution": "bundler",
    "allowImportingTsExtensions": true,
    "resolveJsonModule": true,
    "isolatedModules": true,
    "noEmit": true,
    "jsx": "react-jsx",
    "strict": true,
    "noUnusedLocals": true,
    "noUnusedParameters": true,
    "noFallthroughCasesInSwitch": true
  },
  "include": ["src"],
  "references": [{ "path": "./tsconfig.node.json" }]
}"#.to_string());

        files.insert("tsconfig.node.json".to_string(), r#"{
  "compilerOptions": {
    "composite": true,
    "skipLibCheck": true,
    "module": "ESNext",
    "moduleResolution": "bundler",
    "allowSyntheticDefaultImports": true
  },
  "include": ["vite.config.ts"]
}"#.to_string());

        files.insert("index.html".to_string(), r#"<!doctype html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <link rel="icon" type="image/svg+xml" href="/vite.svg" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>{{project_name}}</title>
  </head>
  <body>
    <div id="root"></div>
    <script type="module" src="/src/main.tsx"></script>
  </body>
</html>"#.to_string());

        files.insert("postcss.config.js".to_string(), r#"export default {
  plugins: {
    tailwindcss: {},
    autoprefixer: {},
  },
}"#.to_string());

        files.insert("tailwind.config.js".to_string(), r#"/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {},
  },
  plugins: [],
}"#.to_string());

        files.insert("src/main.tsx".to_string(), r#"import React from 'react'
import ReactDOM from 'react-dom/client'
import App from './App.tsx'
import './index.css'

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
)"#.to_string());

        files.insert("src/App.tsx".to_string(), r#"import { useState } from 'react'
import './App.css'

function App() {
  const [count, setCount] = useState(0)

  return (
    <div className="text-center">
      <h1 className="text-4xl font-bold mb-8">{{project_name}}</h1>
      <div className="card">
        <button onClick={() => setCount((count) => count + 1)}>
          count is {count}
        </button>
        <p>
          Edit <code>src/App.tsx</code> and save to test HMR
        </p>
      </div>
      <p className="read-the-docs">
        Click on the Vite and React logos to learn more
      </p>
    </div>
  )
}

export default App"#.to_string());

        files.insert("src/App.css".to_string(), r#"#root {
  max-width: 1280px;
  margin: 0 auto;
  padding: 2rem;
  text-align: center;
}

.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: filter 300ms;
}
.logo:hover {
  filter: drop-shadow(0 0 2em #646cffaa);
}
.logo.react:hover {
  filter: drop-shadow(0 0 2em #61dafbaa);
}

@keyframes logo-spin {
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(360deg);
  }
}

@media (prefers-reduced-motion: no-preference) {
  a:nth-of-type(2) .logo {
    animation: logo-spin infinite 20s linear;
  }
}

.card {
  padding: 2em;
}

.read-the-docs {
  color: #888;
}"#.to_string());

        files.insert("src/index.css".to_string(), r#"@tailwind base;
@tailwind components;
@tailwind utilities;

:root {
  font-family: Inter, system-ui, Avenir, Helvetica, Arial, sans-serif;
  line-height: 1.5;
  font-weight: 400;
  color-scheme: dark;
  color: rgba(255, 255, 255, 0.87);
  background-color: #242424;
  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

body {
  margin: 0;
  min-width: 320px;
  min-height: 100vh;
}"#.to_string());

        files.insert(".gitignore".to_string(), r#"# Logs
logs
*.log
npm-debug.log*
yarn-debug.log*
yarn-error.log*
pnpm-debug.log*
lerna-debug.log*

node_modules
dist
dist-ssr
*.local

# Editor directories and files
.vscode/*
!.vscode/extensions.json
.idea
.DS_Store
*.suo
*.ntvs*
*.njsproj
*.sln
*.sw?"#.to_string());

        files.insert("README.md".to_string(), r#"# {{project_name}}

A React application built with Vite and Tailwind CSS.

## Getting Started

```bash
# Install dependencies
npm install

# Start development server
npm run dev

# Build for production
npm run build
```

## Features

- React 18 with TypeScript
- Vite for fast development
- Tailwind CSS for styling
- ESLint for code quality

## Learn More

- [React Documentation](https://react.dev)
- [Vite Documentation](https://vitejs.dev)
- [Tailwind CSS Documentation](https://tailwindcss.com)"#.to_string());

        let mut dependencies = HashMap::new();
        dependencies.insert("react".to_string(), "^18.2.0".to_string());
        dependencies.insert("react-dom".to_string(), "^18.2.0".to_string());
        dependencies.insert("vite".to_string(), "^5.2.0".to_string());
        dependencies.insert("typescript".to_string(), "^5.2.2".to_string());
        dependencies.insert("tailwindcss".to_string(), "^3.4.1".to_string());

        Ok(Template {
            name: "React + Vite".to_string(),
            framework: "react".to_string(),
            files,
            dependencies,
        })
    }

    fn load_nextjs_template(&self) -> Result<Template, TemplateError> {
        // Similar structure for Next.js template
        // For brevity, I'll include a simplified version
        let mut files = HashMap::new();
        
        files.insert("package.json".to_string(), r#"{
  "name": "{{project_name}}",
  "version": "0.1.0",
  "private": true,
  "scripts": {
    "dev": "next dev",
    "build": "next build",
    "start": "next start",
    "lint": "next lint"
  },
  "dependencies": {
    "react": "^18",
    "react-dom": "^18",
    "next": "^14.0.0"
  },
  "devDependencies": {
    "typescript": "^5",
    "@types/node": "^20",
    "@types/react": "^18",
    "@types/react-dom": "^18",
    "autoprefixer": "^10.0.1",
    "postcss": "^8",
    "tailwindcss": "^3.3.0",
    "eslint": "^8",
    "eslint-config-next": "14.0.0"
  }
}"#.to_string());

        files.insert("next.config.js".to_string(), r#"/** @type {import('next').NextConfig} */
const nextConfig = {}

module.exports = nextConfig"#.to_string());

        files.insert("tailwind.config.js".to_string(), r#"/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    './pages/**/*.{js,ts,jsx,tsx,mdx}',
    './components/**/*.{js,ts,jsx,tsx,mdx}',
    './app/**/*.{js,ts,jsx,tsx,mdx}',
  ],
  theme: {
    extend: {
      backgroundImage: {
        'gradient-radial': 'radial-gradient(var(--tw-gradient-stops))',
        'gradient-conic':
          'conic-gradient(from 180deg at 50% 50%, var(--tw-gradient-stops))',
      },
    },
  },
  plugins: [],
}"#.to_string());

        files.insert("app/layout.tsx".to_string(), r#"import type { Metadata } from 'next'
import './globals.css'

export const metadata: Metadata = {
  title: '{{project_name}}',
  description: 'A Next.js application',
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="en">
      <body className="antialiased">{children}</body>
    </html>
  )
}"#.to_string());

        files.insert("app/page.tsx".to_string(), r#"export default function Home() {
  return (
    <main className="flex min-h-screen flex-col items-center justify-center p-24">
      <div className="z-10 max-w-5xl w-full items-center justify-between font-mono text-sm">
        <h1 className="text-4xl font-bold mb-8">{{project_name}}</h1>
        <p className="text-lg">Welcome to your Next.js application!</p>
      </div>
    </main>
  )
}"#.to_string());

        files.insert("app/globals.css".to_string(), r#"@tailwind base;
@tailwind components;
@tailwind utilities;

:root {
  --foreground-rgb: 0, 0, 0;
  --background-start-rgb: 214, 219, 220;
  --background-end-rgb: 255, 255, 255;
}

@media (prefers-color-scheme: dark) {
  :root {
    --foreground-rgb: 255, 255, 255;
    --background-start-rgb: 0, 0, 0;
    --background-end-rgb: 0, 0, 0;
  }
}

body {
  color: rgb(var(--foreground-rgb));
  background: linear-gradient(
      to bottom,
      transparent,
      rgb(var(--background-end-rgb))
    )
    rgb(var(--background-start-rgb));
}"#.to_string());

        files.insert(".gitignore".to_string(), r#"# Dependencies
/node_modules
/.pnp
.pnp.js

# Testing
/coverage

# Next.js
/.next/
/out/

# Production
/build

# Misc
.DS_Store
*.tsbuildinfo
next-env.d.ts

# Debug
npm-debug.log*
yarn-debug.log*
yarn-error.log*

# Local env files
.env*.local

# Vercel
.vercel

# Typescript
*.tsbuildinfo"#.to_string());

        files.insert("README.md".to_string(), r#"# {{project_name}}

A Next.js application built with TypeScript and Tailwind CSS.

## Getting Started

```bash
# Install dependencies
npm install

# Start development server
npm run dev

# Build for production
npm run build

# Start production server
npm start
```

## Features

- Next.js 14 with App Router
- TypeScript support
- Tailwind CSS for styling
- ESLint for code quality

## Learn More

- [Next.js Documentation](https://nextjs.org/docs)
- [Tailwind CSS Documentation](https://tailwindcss.com)"#.to_string());

        let mut dependencies = HashMap::new();
        dependencies.insert("next".to_string(), "^14.0.0".to_string());
        dependencies.insert("react".to_string(), "^18".to_string());
        dependencies.insert("react-dom".to_string(), "^18".to_string());
        dependencies.insert("tailwindcss".to_string(), "^3.3.0".to_string());

        Ok(Template {
            name: "Next.js".to_string(),
            framework: "nextjs".to_string(),
            files,
            dependencies,
        })
    }

    fn load_flask_template(&self) -> Result<Template, TemplateError> {
        let mut files = HashMap::new();
        
        files.insert("app.py".to_string(), r#"from flask import Flask, render_template

app = Flask(__name__)

@app.route('/')
def home():
    return render_template('index.html')

@app.route('/api/health')
def health():
    return {'status': 'healthy', 'app': '{{project_name}}'}

if __name__ == '__main__':
    app.run(debug=True, host='0.0.0.0', port=5000)"#.to_string());

        files.insert("requirements.txt".to_string(), r#"Flask==3.0.0
Werkzeug==3.0.1"#.to_string());

        files.insert("templates/index.html".to_string(), r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{{project_name}}</title>
    <style>
        body {
            font-family: Arial, sans-serif;
            max-width: 800px;
            margin: 0 auto;
            padding: 20px;
        }
        .container {
            text-align: center;
            padding: 50px 0;
        }
        h1 {
            color: #333;
        }
    </style>
</head>
<body>
    <div class="container">
        <h1>{{project_name}}</h1>
        <p>Welcome to your Flask application!</p>
        <p><a href="/api/health">Health Check</a></p>
    </div>
</body>
</html>"#.to_string());

        files.insert(".gitignore".to_string(), r#"# Python
__pycache__/
*.pyc
*.pyo
*.pyd
.Python
env/
venv/
.venv/
pip-log.txt
pip-delete-this-directory.txt

# Flask
instance/
.webassets-cache

# Environment variables
.env
.env.local
.env.*.local

# IDE
.vscode/
.idea/

# OS
.DS_Store
Thumbs.db"#.to_string());

        files.insert("README.md".to_string(), r#"# {{project_name}}

A Flask web application.

## Getting Started

```bash
# Create virtual environment
python -m venv venv

# Activate virtual environment
source venv/bin/activate  # On Windows: venv\Scripts\activate

# Install dependencies
pip install -r requirements.txt

# Run the application
python app.py
```

## Features

- Flask web framework
- HTML templates
- REST API endpoints
- Health check endpoint

## API Endpoints

- `GET /` - Home page
- `GET /api/health` - Health check

## Learn More

- [Flask Documentation](https://flask.palletsprojects.com)"#.to_string());

        let mut dependencies = HashMap::new();
        dependencies.insert("Flask".to_string(), "3.0.0".to_string());

        Ok(Template {
            name: "Flask".to_string(),
            framework: "flask".to_string(),
            files,
            dependencies,
        })
    }

    fn load_express_template(&self) -> Result<Template, TemplateError> {
        let mut files = HashMap::new();
        
        files.insert("package.json".to_string(), r#"{
  "name": "{{project_name}}",
  "version": "1.0.0",
  "description": "{{project_description}}",
  "main": "server.js",
  "scripts": {
    "start": "node server.js",
    "dev": "nodemon server.js",
    "test": "echo \"Error: no test specified\" && exit 1"
  },
  "dependencies": {
    "express": "^4.18.2",
    "cors": "^2.8.5",
    "helmet": "^7.1.0",
    "dotenv": "^16.3.1"
  },
  "devDependencies": {
    "nodemon": "^3.0.2"
  },
  "keywords": ["express", "node", "api"],
  "author": "",
  "license": "ISC"
}"#.to_string());

        files.insert("server.js".to_string(), r#"const express = require('express');
const cors = require('cors');
const helmet = require('helmet');
require('dotenv').config();

const app = express();
const PORT = process.env.PORT || 3000;

// Middleware
app.use(helmet());
app.use(cors());
app.use(express.json());
app.use(express.urlencoded({ extended: true }));

// Routes
app.get('/', (req, res) => {
  res.json({
    message: '{{project_name}} API',
    status: 'running',
    timestamp: new Date().toISOString()
  });
});

app.get('/api/health', (req, res) => {
  res.json({
    status: 'healthy',
    uptime: process.uptime(),
    memory: process.memoryUsage(),
    timestamp: new Date().toISOString()
  });
});

// 404 handler
app.use('*', (req, res) => {
  res.status(404).json({
    error: 'Route not found',
    path: req.originalUrl
  });
});

// Error handler
app.use((err, req, res, next) => {
  console.error(err.stack);
  res.status(500).json({
    error: 'Something went wrong!',
    message: err.message
  });
});

app.listen(PORT, () => {
  console.log(`Server running on port ${PORT}`);
});

module.exports = app;"#.to_string());

        files.insert(".env.example".to_string(), r#"# Server Configuration
PORT=3000
NODE_ENV=development

# Database Configuration (if needed)
# DB_HOST=localhost
# DB_PORT=5432
# DB_NAME={{project_name}}
# DB_USER=username
# DB_PASSWORD=password"#.to_string());

        files.insert(".gitignore".to_string(), r#"# Dependencies
node_modules/
npm-debug.log*
yarn-debug.log*
yarn-error.log*

# Runtime data
pids
*.pid
*.seed
*.pid.lock

# Environment variables
.env
.env.local
.env.development.local
.env.test.local
.env.production.local

# Logs
logs
*.log

# Coverage directory used by tools like istanbul
coverage/

# Dependency directories
jspm_packages/

# Optional npm cache directory
.npm

# Optional eslint cache
.eslintcache

# IDE
.vscode/
.idea/

# OS
.DS_Store
Thumbs.db"#.to_string());

        files.insert("README.md".to_string(), r#"# {{project_name}}

A Node.js Express API server.

## Getting Started

```bash
# Install dependencies
npm install

# Start development server with nodemon
npm run dev

# Start production server
npm start
```

## Features

- Express.js web framework
- CORS support
- Helmet security headers
- Environment configuration
- Health check endpoint

## API Endpoints

- `GET /` - API information
- `GET /api/health` - Health check

## Configuration

Copy `.env.example` to `.env` and configure your environment variables.

## Learn More

- [Express.js Documentation](https://expressjs.com)
- [Node.js Documentation](https://nodejs.org)"#.to_string());

        let mut dependencies = HashMap::new();
        dependencies.insert("express".to_string(), "^4.18.2".to_string());
        dependencies.insert("cors".to_string(), "^2.8.5".to_string());
        dependencies.insert("helmet".to_string(), "^7.1.0".to_string());
        dependencies.insert("dotenv".to_string(), "^16.3.1".to_string());

        Ok(Template {
            name: "Express".to_string(),
            framework: "express".to_string(),
            files,
            dependencies,
        })
    }

    fn load_cli_template(&self) -> Result<Template, TemplateError> {
        let mut files = HashMap::new();
        
        files.insert("package.json".to_string(), r#"{
  "name": "{{project_name}}",
  "version": "1.0.0",
  "description": "{{project_description}}",
  "main": "index.js",
  "bin": {
    "{{project_name}}": "./bin/{{project_name}}.js"
  },
  "scripts": {
    "start": "node bin/{{project_name}}.js",
    "test": "echo \"Error: no test specified\" && exit 1"
  },
  "dependencies": {
    "commander": "^11.1.0",
    "chalk": "^4.1.2"
  },
  "keywords": ["cli", "command-line", "tool"],
  "author": "",
  "license": "ISC"
}"#.to_string());

        files.insert("bin/{{project_name}}.js".to_string(), r#"#!/usr/bin/env node

const { Command } = require('commander');
const chalk = require('chalk');

const program = new Command();

program
  .name('{{project_name}}')
  .description('{{project_description}}')
  .version('1.0.0');

program
  .command('hello')
  .description('Say hello')
  .option('-n, --name <name>', 'Name to greet')
  .action((options) => {
    const name = options.name || 'World';
    console.log(chalk.blue(`Hello, ${name}!`));
  });

program
  .command('greet <name>')
  .description('Greet someone')
  .action((name) => {
    console.log(chalk.green(`Hello, ${name}!`));
  });

program.parse();"#.to_string());

        files.insert(".gitignore".to_string(), r#"# Dependencies
node_modules/
npm-debug.log*
yarn-debug.log*
yarn-error.log*

# Runtime data
pids
*.pid
*.seed
*.pid.lock

# Logs
logs
*.log

# Coverage directory used by tools like istanbul
coverage/

# Environment variables
.env
.env.local

# IDE
.vscode/
.idea/

# OS
.DS_Store
Thumbs.db"#.to_string());

        files.insert("README.md".to_string(), r#"# {{project_name}}

A Node.js command-line tool.

## Installation

```bash
# Install globally
npm install -g .

# Or use npx
npx {{project_name}}
```

## Usage

```bash
# Basic usage
{{project_name}} hello

# With options
{{project_name}} hello --name "Your Name"

# Positional arguments
{{project_name}} greet "John Doe"
```

## Commands

- `hello` - Say hello with optional name
- `greet <name>` - Greet someone

## Features

- Commander.js for CLI parsing
- Chalk for colored output
- Clean command structure

## Learn More

- [Commander.js Documentation](https://github.com/tj/commander.js)
- [Chalk Documentation](https://github.com/chalk/chalk)"#.to_string());

        let mut dependencies = HashMap::new();
        dependencies.insert("commander".to_string(), "^11.1.0".to_string());
        dependencies.insert("chalk".to_string(), "^4.1.2".to_string());

        Ok(Template {
            name: "CLI".to_string(),
            framework: "cli".to_string(),
            files,
            dependencies,
        })
    }

    pub fn render_template(&self, template: &Template, variables: &HashMap<String, String>) -> Result<Template, TemplateError> {
        let mut rendered_files = HashMap::new();
        
        for (file_path, file_content) in &template.files {
            let rendered_content = self.handlebars.render_template(file_content, variables)?;
            rendered_files.insert(file_path.clone(), rendered_content);
        }

        Ok(Template {
            name: template.name.clone(),
            framework: template.framework.clone(),
            files: rendered_files,
            dependencies: template.dependencies.clone(),
        })
    }
}

fn upper_helper(
    h: &handlebars::Helper,
    _: &handlebars::Handlebars,
    _: &handlebars::Context,
    _: &mut handlebars::RenderContext,
    out: &mut dyn handlebars::Output,
) -> handlebars::HelperResult {
    if let Some(value) = h.param(0).and_then(|v| v.as_str()) {
        write!(out, "{}", value.to_uppercase())?;
    }
    Ok(())
}

fn lower_helper(
    h: &handlebars::Helper,
    _: &handlebars::Handlebars,
    _: &handlebars::Context,
    _: &mut handlebars::RenderContext,
    out: &mut dyn handlebars::Output,
) -> handlebars::HelperResult {
    if let Some(value) = h.param(0).and_then(|v| v.as_str()) {
        write!(out, "{}", value.to_lowercase())?;
    }
    Ok(())
}

impl Default for TemplateEngine {
    fn default() -> Self {
        Self::new()
    }
}