# ADR-0008: Code Generation Architecture

## Status
Accepted

## Context
SketchDDD needs to generate idiomatic code from domain models for multiple target languages (Rust, TypeScript, Kotlin). Each language has different idioms, type systems, and best practices for representing DDD patterns.

Key challenges:
1. **Consistency**: Same model should generate semantically equivalent code across languages
2. **Idiomaticity**: Generated code should follow each language's conventions
3. **Extensibility**: Easy to add new target languages
4. **Maintainability**: Changes to generation logic should be localized
5. **Validation**: Generated code should compile and enforce model invariants

We considered two main approaches:
- **Template-based**: Use string templates (Handlebars, Tera) with placeholders
- **AST-based**: Build language-specific AST nodes and render to code

## Decision

### Approach: Hybrid AST-Based Generation
We adopt an AST-based approach with language-specific renderers, using a common intermediate representation.

```
SketchDDD Model → Common IR → Language-Specific AST → Rendered Code
```

### Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                    sketchddd-codegen crate                      │
├─────────────────────────────────────────────────────────────────┤
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────────────┐ │
│  │   Model     │    │  Common IR  │    │  Language Backends  │ │
│  │  (Input)    │ → │  (CodeUnit) │ → │  ├── rust.rs         │ │
│  │             │    │             │    │  ├── typescript.rs  │ │
│  │             │    │             │    │  └── kotlin.rs      │ │
│  └─────────────┘    └─────────────┘    └─────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
```

### Common Intermediate Representation (IR)

```rust
/// A unit of generated code (file or module)
pub struct CodeUnit {
    pub name: String,
    pub items: Vec<CodeItem>,
    pub imports: Vec<Import>,
}

pub enum CodeItem {
    Struct(StructDef),
    Enum(EnumDef),
    Function(FunctionDef),
    Trait(TraitDef),      // Rust
    Interface(InterfaceDef), // TS/Kotlin
    Module(ModuleDef),
}

pub struct StructDef {
    pub name: String,
    pub fields: Vec<FieldDef>,
    pub derives: Vec<String>,
    pub doc: Option<String>,
    pub kind: StructKind, // Entity, ValueObject, Aggregate
}

pub enum StructKind {
    Entity { id_field: String },
    ValueObject,
    AggregateRoot { members: Vec<String> },
}
```

### DDD Pattern Mappings

| DDD Concept | Rust | TypeScript | Kotlin |
|-------------|------|------------|--------|
| Entity | `struct` with ID + `impl` | `interface` + `class` | `data class` with ID |
| Value Object | `struct` with `Eq`, `Hash` | `readonly interface` | `data class` |
| Aggregate | Module with root + members | Namespace/module | Package + classes |
| Enum | `enum` | Union type / `enum` | `sealed class` |
| Invariant | `impl` method with `Result` | Zod schema `.refine()` | `init { require() }` |
| Morphism | Field + getter | Property | Property |
| Optional | `Option<T>` | `T \| null` or `T?` | `T?` |
| Collection | `Vec<T>` | `readonly T[]` | `List<T>` |

### Output Structure

#### Single-File Mode (default)
```
generated/
└── commerce.rs        # All types in one file
```

#### Module Mode (--modules)
```
generated/
└── commerce/
    ├── mod.rs         # Re-exports
    ├── entities.rs    # Customer, Order
    ├── values.rs      # Money, Address
    ├── aggregates.rs  # OrderAggregate
    └── enums.rs       # OrderStatus
```

### Invariant Generation

Invariants from the model translate to runtime validation:

**Model:**
```sketchddd
aggregate OrderAggregate {
    root: Order
    contains: [LineItem]
    invariant: totalPrice = sum(items.price)
}
```

**Rust:**
```rust
impl Order {
    pub fn validate(&self) -> Result<(), ValidationError> {
        let sum: Money = self.items.iter()
            .map(|i| i.price)
            .sum();
        if self.total_price != sum {
            return Err(ValidationError::invariant(
                "totalPrice must equal sum of item prices"
            ));
        }
        Ok(())
    }
}
```

**TypeScript:**
```typescript
export const OrderSchema = z.object({
    id: OrderIdSchema,
    items: z.array(LineItemSchema),
    totalPrice: MoneySchema,
}).refine(
    (o) => o.totalPrice.equals(sumPrices(o.items)),
    { message: "totalPrice must equal sum of item prices" }
);
```

**Kotlin:**
```kotlin
data class Order(
    val id: OrderId,
    val items: List<LineItem>,
    val totalPrice: Money
) {
    init {
        require(totalPrice == items.sumOf { it.price }) {
            "totalPrice must equal sum of item prices"
        }
    }
}
```

### CLI Interface

```bash
# Basic generation
sketchddd codegen commerce.sddd --target rust

# Specify output directory
sketchddd codegen commerce.sddd --target typescript --output ./src/domain

# Module mode
sketchddd codegen commerce.sddd --target rust --modules

# Dry-run preview
sketchddd codegen commerce.sddd --target kotlin --dry-run

# Multiple targets
sketchddd codegen commerce.sddd --target rust --target typescript
```

### Configuration File (optional)
`.sketchddd.toml`:
```toml
[codegen]
default_target = "rust"
output_dir = "./generated"
modules = true

[codegen.rust]
derives = ["Debug", "Clone", "Serialize", "Deserialize"]
use_builder_pattern = true

[codegen.typescript]
use_zod = true
export_style = "named"  # or "default"

[codegen.kotlin]
package = "com.example.domain"
use_kotlinx_serialization = true
```

### Extensibility

Adding a new language requires:
1. Implement `CodeGenerator` trait
2. Map Common IR to language constructs
3. Implement rendering to string
4. Add CLI option

```rust
pub trait CodeGenerator {
    fn generate(&self, model: &BoundedContext) -> Result<Vec<CodeUnit>, CodegenError>;
    fn render(&self, unit: &CodeUnit) -> String;
    fn file_extension(&self) -> &'static str;
}
```

## Consequences

### Positive
- Clean separation between model semantics and language syntax
- Easy to add new target languages
- Generated code is idiomatic per language
- Invariants become runtime validation
- Dry-run allows preview before generation
- Configuration allows per-project customization

### Negative
- More complex than simple templates
- Each language needs significant implementation
- Keeping languages in sync requires discipline
- Generated code may need manual tweaks for edge cases

### Neutral
- Generated code includes "do not edit" warnings
- Round-trip from generated code back to model is not supported
- Repository/persistence patterns are optional extensions

## References
- [Issue #23: Implement Rust code generation](https://github.com/ibrahimcesar/SketchDDD/issues/23)
- [Issue #24: Implement TypeScript code generation](https://github.com/ibrahimcesar/SketchDDD/issues/24)
- [Issue #25: Implement Kotlin code generation](https://github.com/ibrahimcesar/SketchDDD/issues/25)
- [ADR-0006: DSL Syntax Design](0006-dsl-syntax-design.md)
- [ADR-0005: Validation Error Code Conventions](0005-validation-error-code-conventions.md)
- [Zod](https://zod.dev/) - TypeScript schema validation
- [kotlinx.serialization](https://github.com/Kotlin/kotlinx.serialization) - Kotlin serialization
