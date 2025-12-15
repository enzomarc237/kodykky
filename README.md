# Scaffold AI

[![MacOS](https://img.shields.io/badge/macOS-11.0+-blue.svg)](https://www.apple.com/macos/)
[![Tauri](https://img.shields.io/badge/Tauri-2.x-FFC131?logo=Tauri)](https://tauri.app/)
[![React](https://img.shields.io/badge/React-18-61DAFB?logo=React)](https://reactjs.org/)
[![TypeScript](https://img.shields.io/badge/TypeScript-5-3178C6?logo=TypeScript)](https://www.typescriptlang.org/)
[![Rust](https://img.shields.io/badge/Rust-1.77+-000000?logo=Rust)](https://www.rust-lang.org/)

> AI-Powered Project Scaffolding Assistant for macOS

Scaffold AI transforms natural language descriptions into complete, working project scaffolds across multiple frameworks. Built with Tauri for native macOS performance, OpenAI GPT-4 for intelligent code generation, and a curated template system for reliability.

![Scaffold AI Screenshot](docs/screenshot.png)

## ✨ Features

### 🤖 AI-Powered Generation
- Natural language to complete project scaffolds
- OpenAI GPT-4 integration with secure API key storage
- Intelligent framework detection and customization
- Context-aware code generation and best practices

### 🏗️ Multi-Framework Support
- **React + Vite**: Modern React with TypeScript and Tailwind CSS
- **Next.js**: Full-stack React framework with App Router
- **Flask**: Python web framework with templates
- **Express**: Node.js REST API server
- **CLI Tools**: Node.js command-line applications

### 🔒 Security First
- macOS Keychain integration for API key storage
- Secure file system operations with path validation
- Sandboxed execution environment
- No cloud data storage - everything stays local

### 📁 Project Management
- Interactive file tree preview
- Syntax-highlighted code preview
- Project history with search and filtering
- One-click VSCode integration
- Export to any directory

### 🎨 Developer Experience
- Dark modern professional UI
- Responsive design optimized for long work sessions
- Keyboard shortcuts and accessibility features
- Real-time loading states and error handling

## 🚀 Quick Start

### Prerequisites
- macOS 11.0 or later
- Node.js 18+ 
- Rust 1.77+
- OpenAI API key

### Installation

1. **Clone the repository**
   ```bash
   git clone https://github.com/yourusername/scaffold-ai.git
   cd scaffold-ai
   ```

2. **Install dependencies**
   ```bash
   npm install
   ```

3. **Configure OpenAI API Key**
   - Launch the app: `npm run tauri dev`
   - Navigate to Settings
   - Enter your OpenAI API key
   - Click "Validate" to test the connection

4. **Run in development**
   ```bash
   npm run tauri dev
   ```

5. **Build for production**
   ```bash
   npm run tauri build
   ```

## 📖 Usage

### Basic Project Generation

1. **Describe Your Project**: Enter a natural language description
   ```
   "Create a React todo app with Tailwind CSS and local storage"
   "Build a Flask REST API for managing books with CRUD endpoints"
   "Make a Node.js Express server with authentication"
   ```

2. **Choose Framework** (Optional): Select a specific framework or let AI auto-detect

3. **Review & Export**: Preview the generated structure, review code, and export to your desired location

### Example Prompts

**React Application:**
```
Create a modern React todo app with:
- Tailwind CSS styling
- Local storage persistence
- TypeScript support
- Dark/light theme toggle
- Responsive design
```

**Flask API:**
```
Build a Flask REST API for a bookstore with:
- CRUD operations for books
- SQLite database
- API documentation
- Error handling
- CORS support
```

**Express Server:**
```
Create a Node.js Express server with:
- JWT authentication
- User management endpoints
- Database integration
- Input validation
- Logging middleware
```

## 🛠️ Architecture

### Frontend (React + TypeScript)
```
src/
├── components/          # Reusable UI components
│   ├── ui/             # Base UI components (Layout, Navbar, Sidebar)
│   └── features/       # Feature-specific components
├── pages/              # Route-based page components
├── stores/             # Zustand state management
├── services/           # Tauri API integration
├── types/              # TypeScript type definitions
└── hooks/              # Custom React hooks
```

### Backend (Rust + Tauri)
```
src-tauri/src/
├── ai_service.rs       # OpenAI API integration
├── template_engine.rs  # Project template system
├── fs_service.rs       # File system operations
├── storage_service.rs  # Project history and settings
├── editor_service.rs   # VSCode integration
├── commands.rs         # Tauri IPC commands
└── types.rs            # Shared data structures
```

### Template System
- **Curated Templates**: Hand-crafted templates for each framework
- **AI Customization**: AI enhances templates based on user requirements
- **Handlebars Rendering**: Variable substitution and customization
- **Version Management**: Templates stay up-to-date with framework releases

## 🎯 Supported Frameworks

| Framework | Template | Dependencies | Documentation |
|-----------|----------|--------------|---------------|
| **React + Vite** | ✅ Complete | React 18, Vite, TypeScript, Tailwind | [React Docs](https://react.dev) |
| **Next.js** | ✅ Complete | Next.js 14, TypeScript, Tailwind | [Next.js Docs](https://nextjs.org/docs) |
| **Flask** | ✅ Complete | Flask 3.0, Jinja2 | [Flask Docs](https://flask.palletsprojects.com) |
| **Express** | ✅ Complete | Express, CORS, Helmet | [Express Docs](https://expressjs.com) |
| **CLI Tool** | ✅ Complete | Commander.js, Chalk | [Commander.js Docs](https://github.com/tj/commander.js) |

## ⚙️ Configuration

### Environment Variables
Create a `.env` file (optional):
```env
OPENAI_API_KEY=your_api_key_here
TAURI_DEV_URL=http://localhost:1420
```

### Tauri Configuration
Edit `src-tauri/tauri.conf.json`:
```json
{
  "bundle": {
    "identifier": "com.yourcompany.scaffold-ai",
    "publisher": "Your Company"
  }
}
```

## 🔧 Development

### Available Scripts
```bash
# Development
npm run dev              # Start Vite dev server
npm run tauri dev        # Start Tauri development build

# Building
npm run build            # Build for production
npm run tauri build      # Build Tauri application
npm run preview          # Preview production build

# Linting & Formatting
npm run lint             # Run ESLint
npm run type-check       # Run TypeScript compiler check
```

### Project Structure
```
scaffold-ai/
├── src/                 # Frontend source code
├── src-tauri/           # Rust backend source code
├── public/              # Static assets
├── dist/                # Built frontend assets
├── docs/                # Documentation and screenshots
└── package.json         # Dependencies and scripts
```

## 🧪 Testing

### Manual Testing Checklist
- [ ] Generate React project from natural language prompt
- [ ] Generate Next.js project with specific features
- [ ] Generate Flask API with database integration
- [ ] Generate Express server with authentication
- [ ] Generate CLI tool with custom commands
- [ ] Preview files before export
- [ ] Export to custom location
- [ ] Open project in VSCode
- [ ] View and manage project history
- [ ] Configure and validate API key
- [ ] Test error handling for invalid inputs
- [ ] Test offline scenarios

### Performance Benchmarks
- App startup time: < 2 seconds
- AI response time: < 10 seconds (typical)
- File generation: < 5 seconds
- Memory usage: < 200MB idle
- Bundle size: < 10MB

## 🔒 Security

### API Key Management
- Stored securely in macOS Keychain
- Never logged or exposed in UI
- Validated on startup and settings changes
- Revocable through system preferences

### File System Security
- Path traversal prevention
- User-confirmed write operations
- Sandboxed project locations
- No arbitrary code execution

### Data Privacy
- No telemetry or analytics by default
- Project history stored locally only
- No cloud synchronization
- User data remains on device

## 📦 Distribution

### Current Status
- ✅ Development build ready
- 🔄 Production build testing
- ⏳ App Store submission planned

### Distribution Methods
1. **GitHub Releases** (V1.0)
   - Direct download of DMG installer
   - Manual updates

2. **Code Signing** (V1.1)
   - Apple Developer ID signing
   - No security warnings
   - Professional distribution

3. **Mac App Store** (V2.0)
   - Automated updates
   - Broader reach
   - 30% revenue share

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

### Development Setup
1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Make your changes and test thoroughly
4. Commit with conventional commits: `git commit -m "feat: add amazing feature"`
5. Push to your branch: `git push origin feature/amazing-feature`
6. Open a Pull Request

### Contribution Areas
- 🐛 Bug fixes and error handling
- ✨ New framework templates
- 🎨 UI/UX improvements
- 📚 Documentation updates
- 🧪 Test coverage improvements
- ⚡ Performance optimizations

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [Tauri](https://tauri.app/) for the amazing desktop framework
- [OpenAI](https://openai.com/) for GPT-4 API
- [React](https://reactjs.org/) team for the frontend framework
- [Tailwind CSS](https://tailwindcss.com/) for the styling system
- [Lucide](https://lucide.dev/) for the beautiful icons

## 📞 Support

- 📧 **Email**: support@scaffold-ai.app
- 🐛 **Issues**: [GitHub Issues](https://github.com/yourusername/scaffold-ai/issues)
- 💬 **Discussions**: [GitHub Discussions](https://github.com/yourusername/scaffold-ai/discussions)
- 📖 **Documentation**: [Project Wiki](https://github.com/yourusername/scaffold-ai/wiki)

---

<div align="center">

**[Website](https://scaffold-ai.app)** • 
**[Download](https://github.com/yourusername/scaffold-ai/releases)** • 
**[Documentation](https://github.com/yourusername/scaffold-ai/wiki)** • 
**[Support](mailto:support@scaffold-ai.app)**

Made with ❤️ for developers, by developers

</div>
