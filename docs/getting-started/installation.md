# Installation

## Requirements

- Rust 1.70 or later (for building from source)
- Git (for cloning the repository)

## Installing via Cargo

The easiest way to install SketchDDD is using Cargo:

```bash
cargo install sketchddd
```

This installs the `sketchddd` CLI tool globally.

## Building from Source

Clone the repository and build:

```bash
git clone https://github.com/ibrahimcesar/SketchDDD
cd SketchDDD
cargo install --path crates/sketchddd-cli
```

## Verifying Installation

Check that SketchDDD is installed correctly:

```bash
sketchddd --version
```

You should see output like:

```
sketchddd 0.1.0
```

## Editor Support

### VS Code

A VS Code extension for `.sddd` files is coming soon, providing:

- Syntax highlighting
- Real-time validation
- Code completion
- Go to definition

For now, you can associate `.sddd` files with a generic syntax highlighter or use the built-in text mode.

### Vim/Neovim

Add to your configuration:

```vim
" Associate .sddd files with a similar syntax
autocmd BufNewFile,BufRead *.sddd set filetype=rust
```

### JetBrains IDEs

Associate `.sddd` files with a text file type or await the upcoming plugin.

## WASM for Browser

If you want to use SketchDDD in the browser:

```bash
# Install wasm-pack
cargo install wasm-pack

# Build the WASM package
cd crates/sketchddd-wasm
wasm-pack build --target web
```

This creates a `pkg/` directory with JavaScript bindings.

## Next Steps

- [Quick Start Guide](quick-start.md) - Get started with your first model
- [CLI Reference](../cli/overview.md) - Learn all CLI commands
