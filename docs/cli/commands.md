# CLI Commands

Complete reference for all SketchDDD CLI commands.

## check

Validate a SketchDDD file for errors and warnings.

```bash
sketchddd check [FILE] [OPTIONS]
```

### Arguments

| Argument | Description |
|----------|-------------|
| `FILE` | Path to `.sddd` file (optional, auto-detected) |

### Options

| Option | Description |
|--------|-------------|
| `--format <FORMAT>` | Output format: `pretty` (default), `json` |
| `-v`, `--verbose` | Show detailed output |
| `-q`, `--quiet` | Only show errors |

### Examples

```bash
# Basic validation
sketchddd check domain.sddd

# JSON output (for CI/CD)
sketchddd check domain.sddd --format json

# Verbose output
sketchddd check domain.sddd --verbose

# Check multiple files
sketchddd check models/*.sddd
```

### Output

```
✓ Checking domain.sddd...
✓ No issues found in 3 contexts

Contexts: Commerce, Shipping, Billing
Objects: 15
Morphisms: 23
```

---

## codegen

Generate code from a SketchDDD model.

```bash
sketchddd codegen [FILE] [OPTIONS]
```

### Arguments

| Argument | Description |
|----------|-------------|
| `FILE` | Path to `.sddd` file (optional, auto-detected) |

### Options

| Option | Description | Default |
|--------|-------------|---------|
| `--target <TARGET>` | Target language | `rust` |
| `--output <PATH>` | Output file path | stdout |

### Supported Targets

| Target | Aliases |
|--------|---------|
| `rust` | `rs` |
| `typescript` | `ts` |
| `kotlin` | `kt` |
| `python` | `py` |
| `java` | - |
| `clojure` | `clj` |
| `haskell` | `hs` |

### Examples

```bash
# Generate Rust to stdout
sketchddd codegen domain.sddd --target rust

# Generate TypeScript to file
sketchddd codegen domain.sddd --target typescript --output src/domain.ts

# Generate Python
sketchddd codegen domain.sddd --target py --output domain.py

# Generate Kotlin
sketchddd codegen domain.sddd -t kotlin -o Domain.kt
```

---

## viz

Generate visualizations from a SketchDDD model.

```bash
sketchddd viz [FILE] [OPTIONS]
```

### Arguments

| Argument | Description |
|----------|-------------|
| `FILE` | Path to `.sddd` file (optional, auto-detected) |

### Options

| Option | Description | Default |
|--------|-------------|---------|
| `--format <FORMAT>` | Output format | `mermaid` |
| `--output <PATH>` | Output file path | stdout |

### Supported Formats

| Format | Description |
|--------|-------------|
| `mermaid` | Mermaid diagram syntax |
| `graphviz` / `dot` | Graphviz DOT syntax |

### Examples

```bash
# Generate Mermaid diagram
sketchddd viz domain.sddd --format mermaid

# Generate Graphviz DOT
sketchddd viz domain.sddd --format graphviz --output domain.dot

# Render to PNG (requires Graphviz)
sketchddd viz domain.sddd -f dot | dot -Tpng -o domain.png
```

---

## init

Create a new SketchDDD project.

```bash
sketchddd init <NAME> [OPTIONS]
```

### Arguments

| Argument | Description |
|----------|-------------|
| `NAME` | Project name (creates directory) |

### Options

| Option | Description | Default |
|--------|-------------|---------|
| `--template <TEMPLATE>` | Project template | `minimal` |

### Built-in Templates

| Template | Description |
|----------|-------------|
| `minimal` | Empty project structure |
| `ecommerce` | E-commerce domain |
| `microservices` | Microservices architecture |

### Examples

```bash
# Create minimal project
sketchddd init my-domain

# Create from template
sketchddd init my-shop --template ecommerce

# Create in current directory
sketchddd init .
```

---

## export

Export a model to JSON.

```bash
sketchddd export <FILE> [OPTIONS]
```

### Options

| Option | Description | Default |
|--------|-------------|---------|
| `--output <PATH>` | Output file path | stdout |

### Examples

```bash
# Export to stdout
sketchddd export domain.sddd

# Export to file
sketchddd export domain.sddd --output domain.json
```

---

## import

Import a model from JSON.

```bash
sketchddd import <FILE> [OPTIONS]
```

### Options

| Option | Description | Default |
|--------|-------------|---------|
| `--output <PATH>` | Output `.sddd` file | stdout |

### Examples

```bash
# Import and print
sketchddd import domain.json

# Import to file
sketchddd import domain.json --output domain.sddd
```

---

## template

Manage project templates.

```bash
sketchddd template <SUBCOMMAND>
```

### Subcommands

#### list

List available templates:

```bash
sketchddd template list
```

#### info

Show template details:

```bash
sketchddd template info <NAME>
```

#### validate

Validate a template:

```bash
sketchddd template validate <PATH>
```

#### create

Create a new template:

```bash
sketchddd template create <NAME>
```

#### install

Install a template from path or URL:

```bash
sketchddd template install <SOURCE>
```

#### remove

Remove an installed template:

```bash
sketchddd template remove <NAME> [--force]
```

### Examples

```bash
# List all templates
sketchddd template list

# Get info about ecommerce template
sketchddd template info ecommerce

# Create a new template
sketchddd template create my-template

# Install from path
sketchddd template install ./my-template

# Remove a template
sketchddd template remove my-template --force
```

---

## update

Check for SketchDDD updates.

```bash
sketchddd update [OPTIONS]
```

### Options

| Option | Description |
|--------|-------------|
| `--check` | Only check, don't update |

### Examples

```bash
# Check for updates
sketchddd update --check

# Update to latest version
sketchddd update
```

---

## serve

Start a local development server (coming soon).

```bash
sketchddd serve [OPTIONS]
```

### Options

| Option | Description | Default |
|--------|-------------|---------|
| `--port <PORT>` | Server port | `3000` |

---

## diff

Compare two SketchDDD files (coming soon).

```bash
sketchddd diff <OLD> <NEW>
```

### Examples

```bash
sketchddd diff domain-v1.sddd domain-v2.sddd
```
