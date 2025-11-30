# ADR-0006: DSL Syntax Design

## Status
Accepted

## Context
SketchDDD needs a domain-specific language (DSL) that allows users to define domain models textually. The DSL must:

1. Be accessible to non-programmers (domain experts, business analysts)
2. Map cleanly to Domain-Driven Design (DDD) tactical patterns
3. Support the underlying category theory semantics (objects, morphisms, limits, colimits)
4. Be parseable and transformable to the semantic model
5. Support visual round-tripping (DSL ↔ Visual Builder)

We needed to design a syntax that balances expressiveness with simplicity, while maintaining mathematical rigor.

## Decision

### Overall Structure
The DSL uses a block-based syntax inspired by configuration languages (TOML, HCL) rather than programming languages. This makes it approachable for non-programmers.

```sketchddd
context ContextName {
    // Building blocks go here
}

map MapName: SourceContext -> TargetContext {
    // Mappings go here
}
```

### Building Blocks

#### 1. Objects Block
Simple declaration of domain concepts (categorical objects):

```sketchddd
objects { Customer, Order, Product, Money }
```

**Rationale**: Minimal syntax for basic types. No fields required - they're just nodes in the graph.

#### 2. Entity Block
Entities are objects with identity (have identity morphism):

```sketchddd
entity Customer {
    id: UUID
    name: String
    email: Email
}
```

**Rationale**: Fields are optional but useful for code generation. The `entity` keyword signals DDD semantics.

#### 3. Value Block
Value objects are defined by their structure (categorical limit/product):

```sketchddd
value Money {
    amount: Decimal
    currency: Currency
}
```

**Rationale**: `value` keyword distinguishes from entities. Structural equality is implied.

#### 4. Morphisms Block
Relationships between objects (categorical morphisms):

```sketchddd
morphisms {
    placedBy: Order -> Customer
    items: Order -> List<LineItem>
    price: LineItem -> Money?
}
```

**Rationale**:
- Arrow syntax (`->`) is intuitive and matches category theory notation
- Generic types (`List<T>`) supported for collections
- Optional types (`?`) supported for nullable relationships
- Annotations supported: `[cascade, lazy]`

#### 5. Aggregate Block
Aggregates define consistency boundaries (categorical limit cone):

```sketchddd
aggregate OrderAggregate {
    root: Order
    contains: [LineItem, Payment]
    invariant: totalPrice = sum(items.price)
}
```

**Rationale**:
- `root` specifies the aggregate root entity
- `contains` lists member entities/value objects
- `invariant` captures business rules as equations

#### 6. Enum Block
Sum types / enumerations (categorical colimit/coproduct):

```sketchddd
enum OrderStatus = Pending | Confirmed | Shipped | Cancelled
```

**Rationale**: ML/Haskell-style sum type syntax is concise. Pipe (`|`) clearly shows alternatives.

Variants can have payloads:
```sketchddd
enum PaymentResult = Success(TransactionId) | Failed(ErrorCode, String)
```

#### 7. Equation Block

Path equations express business rules (categorical commutative diagrams):

```sketchddd
equation priceConsistency: Order.total = sum(Order.items.price)
```

**Rationale**: Named equations allow documentation and error reporting.

### Context Maps
Context maps define relationships between bounded contexts (sketch morphisms/functors):

```sketchddd
map CommerceToShipping: Commerce -> Shipping {
    pattern: CustomerSupplier
    mappings {
        Order -> Shipment
        Customer -> Recipient
    }
    morphism_mappings {
        placedBy -> assignedTo
    }
}
```

**Supported Patterns** (from DDD):
- `Partnership` - Mutual cooperation
- `CustomerSupplier` - Upstream/downstream relationship
- `Conformist` - Downstream conforms to upstream
- `AntiCorruptionLayer` (alias: `ACL`) - Translation layer
- `SeparateWays` - No integration
- `PublishedLanguage` - Shared schema
- `OpenHostService` (alias: `OHS`) - Public API
- `SharedKernel` - Shared model subset

### Type System
Types support:
- Simple types: `String`, `Integer`, `UUID`, `DateTime`
- Generic types: `List<T>`, `Set<T>`, `Map<K,V>`
- Optional types: `T?` (suffix notation)
- User-defined types: Any `PascalCase` identifier

### Comments
Standard comment syntax:
```sketchddd
// Single-line comment
/* Multi-line
   comment */
/// Documentation comment (for future use)
```

### Expression Language
For invariants and equations, a simple expression language:
- Arithmetic: `+`, `-`, `*`, `/`, `%`
- Comparison: `==`, `!=`, `<`, `<=`, `>`, `>=`
- Paths: `Order.items.price`
- Functions: `sum(...)`, `count(...)`, `all(...)`, `any(...)`

## Consequences

### Positive
- Non-programmers can read and understand models
- Direct mapping to DDD tactical patterns
- Clean mapping to category theory (objects, morphisms, limits, colimits)
- Extensible - new blocks can be added without breaking changes
- Supports round-trip with visual builder
- Familiar syntax for developers (block-based, typed)

### Negative
- Learning curve for those unfamiliar with DDD terminology
- Expression language is limited (no conditionals, loops)
- Cannot express all possible categorical structures
- Parser must handle whitespace-insensitive syntax

### Neutral
- File extension is `.sddd` (see ADR-0007)
- Grammar implemented with pest parser (see ADR-0002)
- Semantic validation happens after parsing (separate phase)

## References
- [Issue #5: Implement DSL parser](https://github.com/ibrahimcesar/SketchDDD/issues/5)
- [Issue #6: AST to Semantic Model transformation](https://github.com/ibrahimcesar/SketchDDD/issues/6)
- [ADR-0001: Use Category Theory as Foundation](0001-use-category-theory-as-foundation.md)
- [ADR-0003: Dual Interface - Visual and Text](0003-dual-interface-visual-and-text.md)
- [Domain-Driven Design Reference](https://www.domainlanguage.com/ddd/reference/)
- [Pest Parser](https://pest.rs/) - Grammar implementation
