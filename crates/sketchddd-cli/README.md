# sketchddd-cli

Command-line interface for [SketchDDD](https://github.com/ibrahimcesar/SketchDDD).

## Installation

```bash
cargo install sketchddd-cli
```

## Usage

```bash
# Validate a model
sketchddd check my-domain.sketch

# Generate code
sketchddd codegen my-domain.sketch --target rust

# Generate diagrams
sketchddd viz my-domain.sketch --format mermaid

# Initialize a new project
sketchddd init my-project

# Start visual builder
sketchddd serve
```

## License

Licensed under either of [MIT](../../LICENSE-MIT) or [Apache-2.0](../../LICENSE-APACHE) at your option.
