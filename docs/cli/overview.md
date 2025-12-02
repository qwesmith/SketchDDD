# CLI Overview

The `sketchddd` command-line interface provides tools for working with SketchDDD domain models.

## Installation

```bash
cargo install sketchddd
```

## Basic Usage

```bash
sketchddd [OPTIONS] [COMMAND]
```

## Global Options

| Option | Description |
|--------|-------------|
| `-v`, `--verbose` | Increase output verbosity |
| `-q`, `--quiet` | Suppress non-essential output |
| `--version` | Show version information |
| `--help` | Show help message |

## Commands

| Command | Description |
|---------|-------------|
| `check` | Validate a SketchDDD file |
| `codegen` | Generate code from a model |
| `viz` | Generate visualizations |
| `init` | Create a new project |
| `export` | Export to JSON |
| `import` | Import from JSON |
| `template` | Manage templates |
| `update` | Check for updates |

## Auto-Detection

When you're in a directory with a `.sddd` file, you can omit the filename:

```bash
# These are equivalent (if there's a single .sddd file)
sketchddd check
sketchddd check ./myproject.sddd
```

If there are multiple `.sddd` files, SketchDDD will:
1. Look for a file matching the directory name (e.g., `myproject/myproject.sddd`)
2. If not found, ask you to specify which file

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | Error (validation, codegen, etc.) |
| 2 | Invalid arguments |

## Quick Examples

```bash
# Validate a model
sketchddd check domain.sddd

# Generate Rust code
sketchddd codegen domain.sddd --target rust

# Generate TypeScript with output file
sketchddd codegen domain.sddd --target typescript --output src/types.ts

# Generate Mermaid diagram
sketchddd viz domain.sddd --format mermaid

# Create new project from template
sketchddd init my-project --template ecommerce

# List available templates
sketchddd template list
```

## Configuration

SketchDDD can be configured via:

1. Command-line arguments (highest priority)
2. Project configuration (`.sketchddd/config.json`)
3. User configuration (`~/.sketchddd/config.json`)

### Project Configuration

Create `.sketchddd/config.json`:

```json
{
  "defaultTarget": "typescript",
  "outputDir": "src/generated",
  "validation": {
    "strict": true
  }
}
```

## Shell Completion

Generate shell completions:

```bash
# Bash
sketchddd completions bash > ~/.local/share/bash-completion/completions/sketchddd

# Zsh
sketchddd completions zsh > ~/.zfunc/_sketchddd

# Fish
sketchddd completions fish > ~/.config/fish/completions/sketchddd.fish
```

## Next Steps

- [Commands Reference](commands.md) - Detailed command documentation
- [Code Generation](codegen.md) - Generate code in multiple languages
- [Visualization](visualization.md) - Create diagrams
- [Templates](templates.md) - Work with templates
