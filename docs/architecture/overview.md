# Architecture Overview

SketchDDD is built as a modular Rust workspace with clearly separated concerns.

## Project Structure

```
sketchddd/
├── crates/
│   ├── sketchddd-core/     # Core data structures and sketch theory
│   ├── sketchddd-parser/   # Language parser (Pest grammar)
│   ├── sketchddd-codegen/  # Code generation for multiple targets
│   ├── sketchddd-viz/      # Diagram generation (Mermaid, Graphviz)
│   ├── sketchddd-cli/      # Command-line interface
│   └── sketchddd-wasm/     # WebAssembly bindings
├── docs/                    # Documentation (MkDocs)
├── examples/                # Example domain models
└── tests/                   # Integration tests
```

## Crate Responsibilities

### sketchddd-core

The foundation crate containing:

- **Sketch Theory**: Category-theoretic data structures
  - `Sketch`: The main sketch representation
  - `Graph`: Objects and morphisms
  - `LimitCone`: Product types (entities, value objects)
  - `ColimitCocone`: Sum types (enums)
  - `Aggregate`: Aggregate roots with invariants

- **Domain Model**: DDD-specific structures
  - `BoundedContext`: Context with entities, values, enums
  - `Entity`: Domain entity with identity
  - `ValueObject`: Immutable value type
  - `Morphism`: Relationships between objects
  - `ContextMap`: Cross-context mappings

- **Validation**: Model validation and error reporting
  - Type checking
  - Reference resolution
  - Invariant verification

### sketchddd-parser

The language parser:

- **Grammar**: Pest PEG grammar for `.sddd` files
- **AST**: Abstract syntax tree representation
- **Parser**: Source code to AST transformation
- **Error Recovery**: Helpful error messages with source locations

### sketchddd-codegen

Code generation for multiple languages:

- **Generators**: Language-specific code generators
  - `RustGenerator`
  - `TypeScriptGenerator`
  - `KotlinGenerator`
  - `PythonGenerator`
  - `JavaGenerator`
  - `ClojureGenerator`
  - `HaskellGenerator`

- **Templates**: Code templates for each language
- **Type Mapping**: SketchDDD types to target language types

### sketchddd-viz

Visualization generation:

- **Mermaid**: Class and ER diagrams
- **Graphviz**: DOT format graphs
- **Layout**: Automatic diagram layout

### sketchddd-cli

Command-line interface:

- **Commands**: check, codegen, viz, init, format
- **Config**: Project configuration
- **Watch**: File watching for development

### sketchddd-wasm

WebAssembly bindings:

- **JavaScript API**: Browser-compatible API
- **Serialization**: wasm-bindgen for JS interop
- **Bundle**: Optimized WASM build

## Data Flow

```
Source Code (.sddd)
       │
       ▼
   ┌───────┐
   │ Parser │  (sketchddd-parser)
   └───┬───┘
       │ AST
       ▼
   ┌───────┐
   │ Core  │  (sketchddd-core)
   └───┬───┘
       │ Sketch/BoundedContext
       ▼
   ┌───────────────────────────┐
   │                           │
   ▼           ▼               ▼
┌─────┐    ┌──────┐      ┌──────────┐
│Codegen│  │ Viz  │      │Validation│
└───┬──┘  └───┬──┘       └────┬────┘
    │         │               │
    ▼         ▼               ▼
  Code    Diagrams         Errors
```

## Key Design Decisions

### 1. Workspace Organization

Using a Cargo workspace provides:
- Independent crate versioning
- Parallel compilation
- Clear dependency boundaries
- Easy testing

### 2. Core as Foundation

`sketchddd-core` has no dependencies on other SketchDDD crates, ensuring:
- Stable API surface
- Reusability in different contexts
- Minimal compile times for core changes

### 3. Parser Separation

The parser is separate to:
- Allow alternative frontends
- Enable grammar evolution
- Support tooling (syntax highlighting, formatting)

### 4. Generator Abstraction

Code generators share a common trait:

```rust
pub trait CodeGenerator {
    fn generate(&self, context: &BoundedContext) -> Result<String, Error>;
    fn file_extension(&self) -> &str;
}
```

This enables:
- Easy addition of new languages
- Consistent code generation API
- Testing with mock generators

### 5. WASM as Separate Crate

WASM bindings are isolated to:
- Keep CLI free of WASM dependencies
- Optimize for browser use case
- Separate build configuration

## Error Handling

SketchDDD uses rich error types:

```rust
pub struct ValidationError {
    pub message: String,
    pub location: SourceLocation,
    pub severity: Severity,
    pub suggestions: Vec<String>,
}
```

Errors include:
- Source location (file, line, column)
- Severity (error, warning, info)
- Actionable suggestions
- Related information

## Configuration

Project configuration via `sketchddd.toml`:

```toml
[project]
name = "my-domain"
version = "0.1.0"

[codegen.rust]
output = "src/domain"
derives = ["Debug", "Clone", "PartialEq"]

[codegen.typescript]
output = "src/types"
zod = true
```

## Extension Points

### Custom Code Generators

Implement `CodeGenerator` trait:

```rust
pub struct MyGenerator;

impl CodeGenerator for MyGenerator {
    fn generate(&self, context: &BoundedContext) -> Result<String, Error> {
        // Generate code
    }

    fn file_extension(&self) -> &str {
        "my"
    }
}
```

### Custom Validation Rules

Add validation rules to the core:

```rust
pub fn validate_custom(context: &BoundedContext) -> Vec<ValidationError> {
    // Custom validation logic
}
```

### Custom Visualization

Implement `VizGenerator` trait:

```rust
pub struct MyVizGenerator;

impl VizGenerator for MyVizGenerator {
    fn generate(&self, context: &BoundedContext) -> Result<String, Error> {
        // Generate visualization
    }
}
```

## Testing Strategy

### Unit Tests

Each crate has unit tests:

```bash
cargo test -p sketchddd-core
cargo test -p sketchddd-parser
```

### Integration Tests

End-to-end tests in `/tests`:

```bash
cargo test --test integration
```

### Golden Tests

Snapshot testing for code generation:

```bash
cargo test --test golden
```

### WASM Tests

Browser-based tests:

```bash
cd crates/sketchddd-wasm
wasm-pack test --headless --chrome
```

## Performance Considerations

### Parsing

- Pest parser is efficient
- Lazy evaluation where possible
- Incremental parsing planned

### Code Generation

- Templates pre-compiled
- String building optimized
- Parallel generation per context

### WASM

- Tree shaking for minimal bundle
- Lazy initialization
- Memory-efficient serialization

## Future Architecture

Planned improvements:

1. **Language Server Protocol (LSP)**
   - Editor integration
   - Real-time diagnostics
   - Code completion

2. **Incremental Compilation**
   - Cache intermediate results
   - Rebuild only changed contexts

3. **Plugin System**
   - Dynamic code generators
   - Custom validations
   - User-defined transforms
