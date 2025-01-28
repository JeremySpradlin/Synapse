# Synapse

A modern application launcher and command palette built with Tauri, Rust, and TypeScript.

## Features

- 🚀 Lightning-fast application launcher
- ⌨️ Global hotkey activation (Cmd/Ctrl + Shift + Space)
- 🎯 Smooth animations and transitions
- 🎨 Clean, minimal interface
- ⚡ Native performance with Rust backend
- 🔍 Keyboard-first navigation
- 🎯 Focus management and accessibility

## Prerequisites

- [Rust](https://rustup.rs/) (1.70 or later)
- [Node.js](https://nodejs.org/) (18 or later)
- [pnpm](https://pnpm.io/) (8 or later)
- [Tauri Prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites)

## Development Setup

1. Clone the repository:
   ```bash
   git clone https://github.com/erbun/synapse.git
   cd synapse
   ```

2. Install dependencies:
   ```bash
   pnpm install
   ```

3. Start the development server:
   ```bash
   pnpm tauri dev
   ```

## Building

To create a production build:

```bash
pnpm tauri build
```

The built application will be available in `src-tauri/target/release`.

## Project Structure

```
synapse/
├── src/                    # Frontend TypeScript code
│   ├── main.ts            # Main window management
│   └── styles.css         # Global styles
├── src-tauri/             # Rust backend code
│   ├── src/
│   │   └── main.rs        # Main application logic
│   └── Cargo.toml         # Rust dependencies
└── package.json           # Node.js dependencies
```

## Development Guidelines

### Code Style

- **Rust**: Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- **TypeScript**: Use Prettier for formatting
- Run `pnpm format` before committing

### Testing

- Write unit tests for all new Rust functions
- Test keyboard navigation and focus management
- Ensure cross-platform compatibility

### Performance

- Profile any performance-critical code paths
- Keep animations smooth (60fps)
- Minimize system resource usage

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [Tauri](https://tauri.app/) for the framework
- [window-shadows](https://github.com/tauri-apps/window-shadows) for native window shadows
