# SketchDDD

**Domain-Driven Design with Category Theory foundations**

SketchDDD is a domain-specific language and toolchain for modeling complex domains using the principles of Domain-Driven Design (DDD), backed by rigorous category theory semantics.

## Why SketchDDD?

Traditional DDD tools focus on diagrams and documentation. SketchDDD goes further by providing:

- **Mathematical Rigor**: Your domain model is a *sketch* in category theory, ensuring consistency
- **Multi-Language Code Generation**: Generate idiomatic code in 7 languages from a single model
- **Real-time Validation**: Catch modeling errors before they become code bugs
- **Visual & Textual**: Work with `.sddd` files or a visual editor (coming soon)

## Quick Example

```sddd
context Commerce {
  // Define your domain objects
  entity Customer {
    id: UUID
    name: String
    email: Email
  }

  entity Order {
    id: UUID
    status: OrderStatus
    total: Money
  }

  // Define relationships as morphisms
  morphisms {
    placedBy: Order -> Customer
    items: Order -> List<LineItem>
  }

  // Aggregates define consistency boundaries
  aggregate Order {
    root: Order
    contains: [LineItem, Payment]
    invariant: total = sum(items.price)
  }

  // Value objects with structural equality
  value Money {
    amount: Decimal
    currency: Currency
  }

  // Enums as sum types
  enum OrderStatus = Pending | Confirmed | Shipped | Delivered | Cancelled
}
```

## Features

### Code Generation

Generate production-ready code in multiple languages:

=== "Rust"
    ```bash
    sketchddd codegen model.sddd --target rust
    ```

=== "TypeScript"
    ```bash
    sketchddd codegen model.sddd --target typescript
    ```

=== "Kotlin"
    ```bash
    sketchddd codegen model.sddd --target kotlin
    ```

=== "Python"
    ```bash
    sketchddd codegen model.sddd --target python
    ```

### Visualization

Generate diagrams from your model:

```bash
# Mermaid diagram
sketchddd viz model.sddd --format mermaid

# Graphviz DOT
sketchddd viz model.sddd --format graphviz
```

### Validation

Real-time validation with helpful error messages:

```bash
sketchddd check model.sddd
```

## Installation

```bash
# Using Cargo
cargo install sketchddd

# Or build from source
git clone https://github.com/ibrahimcesar/SketchDDD
cd SketchDDD
cargo install --path crates/sketchddd-cli
```

## Getting Started

1. [Install SketchDDD](getting-started/installation.md)
2. [Quick Start Guide](getting-started/quick-start.md)
3. [Your First Model](getting-started/first-model.md)

## Category Theory Foundation

SketchDDD models domains as *sketches* - a concept from category theory. This provides:

| DDD Concept | Category Theory |
|-------------|-----------------|
| Bounded Context | Sketch |
| Entity | Object with identity morphism |
| Value Object | Limit (product type) |
| Aggregate | Limit cone with root |
| Enum | Colimit (sum type) |
| Relationship | Morphism |
| Invariant | Equalizer |
| Context Map | Sketch morphism (functor) |

Learn more in the [Category Theory section](theory/introduction.md).

## License

MIT License - see [LICENSE](https://github.com/ibrahimcesar/SketchDDD/blob/main/LICENSE) for details.
