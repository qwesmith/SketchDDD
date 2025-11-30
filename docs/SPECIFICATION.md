# SketchDDD Language Specification

**Version**: 0.1.0
**Status**: Draft
**Last Updated**: 2025

## 1. Introduction

SketchDDD is a domain-specific language for expressing Domain-Driven Design (DDD) models with formal categorical semantics. This specification defines the syntax, semantics, and validation rules for SketchDDD.

### 1.1 Design Goals

1. **Accessibility** - Readable by domain experts and developers alike
2. **Precision** - Formal semantics based on category theory
3. **Validation** - Compile-time verification of model consistency
4. **Bidirectional** - Round-trip between textual DSL and visual builder

### 1.2 Mathematical Foundation

SketchDDD models are formalized as **sketches** from category theory:

```
S = (G, E, L, C)
```

Where:
- **G**: Directed graph (objects and morphisms)
- **E**: Path equations (business rules)
- **L**: Limit cones (aggregates, value objects)
- **C**: Colimit cocones (sum types, enumerations)

## 2. Lexical Structure

### 2.1 Character Set

SketchDDD source files use UTF-8 encoding.

### 2.2 Identifiers

```ebnf
identifier = letter { letter | digit | "_" }
letter     = "A".."Z" | "a".."z"
digit      = "0".."9"
```

Conventions:
- `PascalCase` for objects, contexts, and type names
- `camelCase` for morphisms and fields
- `SCREAMING_SNAKE_CASE` for enum variants (optional)

### 2.3 Keywords

Reserved keywords:
```
context     map         objects     entity      value
aggregate   enum        morphisms   invariant   equation
root        contains    pattern
```

### 2.4 Comments

```sketchddd
// Single-line comment
/* Multi-line
   comment */
/// Documentation comment (reserved for future use)
```

### 2.5 Whitespace

Whitespace (spaces, tabs, newlines) is insignificant except as separator. The grammar is whitespace-insensitive.

## 3. Grammar

### 3.1 Top-Level Structure

```ebnf
file = { context_def | map_def }

context_def = "context" identifier "{" { block } "}"
map_def     = "map" identifier ":" identifier "->" identifier "{" map_body "}"
```

### 3.2 Blocks

```ebnf
block = objects_block
      | entity_block
      | value_block
      | aggregate_block
      | enum_block
      | morphisms_block
      | equation_block
```

### 3.3 Objects Block

```ebnf
objects_block = "objects" "{" identifier { "," identifier } "}"
```

Example:
```sketchddd
objects { Customer, Order, Product, Money }
```

### 3.4 Entity Block

```ebnf
entity_block = "entity" identifier "{" { field } "}"
field = identifier ":" type
```

Example:
```sketchddd
entity Customer {
    id: UUID
    name: String
    email: Email
}
```

### 3.5 Value Block

```ebnf
value_block = "value" identifier "{" { field } "}"
```

Example:
```sketchddd
value Money {
    amount: Decimal
    currency: Currency
}
```

### 3.6 Aggregate Block

```ebnf
aggregate_block = "aggregate" identifier "{"
    "root" ":" identifier
    [ "contains" ":" "[" identifier { "," identifier } "]" ]
    [ "invariant" ":" expression ]
"}"
```

Example:
```sketchddd
aggregate OrderAggregate {
    root: Order
    contains: [LineItem, Payment]
    invariant: totalPrice = sum(items.price)
}
```

### 3.7 Enum Block

```ebnf
enum_block = "enum" identifier "=" variant { "|" variant }
variant = identifier [ "(" type { "," type } ")" ]
```

Examples:
```sketchddd
enum OrderStatus = Pending | Confirmed | Shipped | Cancelled

enum PaymentResult = Success(TransactionId) | Failed(ErrorCode, String)
```

### 3.8 Morphisms Block

```ebnf
morphisms_block = "morphisms" "{" { morphism_def } "}"
morphism_def = identifier ":" type "->" type [ annotations ]
annotations = "[" identifier { "," identifier } "]"
```

Example:
```sketchddd
morphisms {
    placedBy: Order -> Customer
    items: Order -> List<LineItem>
    price: LineItem -> Money?
    status: Order -> OrderStatus [lazy]
}
```

### 3.9 Equation Block

```ebnf
equation_block = "equation" identifier ":" path "=" path
path = identifier { "." identifier }
```

Example:
```sketchddd
equation priceConsistency: Order.total = sum(Order.items.price)
```

### 3.10 Context Map

```ebnf
map_body = [ pattern_clause ] [ mappings_block ] [ morphism_mappings_block ]
pattern_clause = "pattern" ":" relationship_pattern
mappings_block = "mappings" "{" { object_mapping } "}"
object_mapping = identifier "->" identifier
morphism_mappings_block = "morphism_mappings" "{" { morphism_mapping } "}"
morphism_mapping = identifier "->" identifier
```

Example:
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

### 3.11 Relationship Patterns

Supported patterns from DDD strategic design:

| Pattern | Alias | Description |
|---------|-------|-------------|
| `Partnership` | - | Mutual cooperation between contexts |
| `CustomerSupplier` | - | Upstream/downstream relationship |
| `Conformist` | - | Downstream conforms to upstream model |
| `AntiCorruptionLayer` | `ACL` | Translation layer protects downstream |
| `SeparateWays` | - | No integration between contexts |
| `PublishedLanguage` | - | Shared schema/protocol |
| `OpenHostService` | `OHS` | Public API for integration |
| `SharedKernel` | - | Shared model subset |

### 3.12 Type System

```ebnf
type = simple_type | generic_type | optional_type | user_type
simple_type = "String" | "Integer" | "Decimal" | "Boolean" | "UUID" | "DateTime" | "Date" | "Time"
generic_type = ("List" | "Set" | "Map") "<" type [ "," type ] ">"
optional_type = type "?"
user_type = identifier
```

## 4. Semantic Model

### 4.1 Objects

Objects are nodes in the categorical graph. They represent domain concepts.

**Categorical interpretation**: Objects in the sketch category.

### 4.2 Morphisms

Morphisms are directed edges between objects, representing relationships or transformations.

**Categorical interpretation**: Arrows in the sketch category.

Properties:
- Each morphism has exactly one source and one target object
- Morphisms compose: if `f: A -> B` and `g: B -> C`, then `g ∘ f: A -> C`

### 4.3 Entities

Entities are objects with identity. They have:
- An identity morphism `id: E -> E`
- Identity survives state changes

**Categorical interpretation**: Objects with explicit identity morphisms.

### 4.4 Value Objects

Value objects are defined by their structure, not identity. Two value objects with identical fields are equal.

**Categorical interpretation**: Limit cones (products) with structural equality.

### 4.5 Aggregates

Aggregates are consistency boundaries. They have:
- A root entity (the aggregate root)
- Contained entities and value objects
- Transactional boundary for invariants

**Categorical interpretation**: Limit cones with a designated root object.

### 4.6 Enumerations (Sum Types)

Enumerations are disjoint unions of variants.

**Categorical interpretation**: Colimit cocones (coproducts).

### 4.7 Path Equations

Path equations assert that two paths between objects are equivalent.

**Categorical interpretation**: Commutative diagrams in the sketch.

### 4.8 Context Maps

Context maps define relationships between bounded contexts as sketch morphisms.

**Categorical interpretation**: Functors between sketch categories.

Requirements:
- Object mappings preserve object references
- Morphism mappings preserve source/target consistency
- Pattern annotations indicate integration strategy

## 5. Validation Rules

### 5.1 Error Codes

Errors (prevent compilation):
| Code | Description |
|------|-------------|
| E0001 | Duplicate object name in context |
| E0002 | Duplicate morphism name in context |
| E0010 | Duplicate context name |
| E0020 | Duplicate context map name |
| E0030-E0032 | Invalid morphism definition |
| E0060-E0067 | Context map validation errors |
| E0070-E0071 | Context reference errors |
| E0100-E0108 | Path validation errors |
| E0110-E0117 | Limit cone (aggregate/value object) errors |
| E0120-E0123 | Colimit cocone (enum) errors |

Warnings (non-fatal):
| Code | Description |
|------|-------------|
| W0001 | Duplicate equation name |
| W0010 | Long path (>10 morphisms) |
| W0100-W0102 | Trivial equation warnings |
| W0110-W0112 | Limit cone structure warnings |
| W0120-W0122 | Colimit structure warnings |
| W0130-W0136 | Context map completeness warnings |

### 5.2 Object Validation

- Names must be unique within a context (E0001)
- Objects must be referenced by valid morphisms

### 5.3 Morphism Validation

- Names must be unique within a context (E0002)
- Source object must exist (E0030)
- Target object must exist (E0031)
- Self-referential morphisms are allowed (identity)

### 5.4 Path Validation

- Source object must exist (E0100)
- Target object must exist (E0101)
- All morphisms must exist (E0102)
- Morphisms must compose correctly (E0103-E0105)
- Empty paths require source = target (E0106)

### 5.5 Equation Validation

- Both paths must have same source object (E0107)
- Both paths must have same target object (E0108)
- All morphisms in paths must exist and compose

### 5.6 Limit Cone Validation (Aggregates, Value Objects)

- Apex object must exist (E0110)
- Root object must exist for aggregates (E0111)
- Root must be reachable from apex (E0112)
- Projection morphisms must exist (E0113)
- Projection targets must exist (E0114)
- Projection sources must equal apex (E0115)
- Projection targets must match declarations (E0116)
- No duplicate projection targets (E0117)

### 5.7 Colimit Cocone Validation (Enums)

- Apex object must exist (E0120)
- Injection source objects must exist (E0121)
- Variant names cannot be empty (E0122)
- Variant names must be unique (E0123)

### 5.8 Context Map Validation

- Source context must exist (E0060)
- Target context must exist (E0061)
- Mapped source objects must exist in source context (E0062)
- Mapped target objects must exist in target context (E0063)
- Mapped morphisms must exist (E0064, E0065)
- Morphism mappings must preserve endpoints (E0066, E0067)

## 6. DDD to Category Theory Mapping

| DDD Concept | Categorical Structure | SketchDDD Syntax |
|-------------|----------------------|------------------|
| Bounded Context | Sketch | `context Name { }` |
| Ubiquitous Language | Graph + Equations | objects, morphisms |
| Entity | Object with identity morphism | `entity Name { }` |
| Value Object | Limit with structural equality | `value Name { }` |
| Aggregate | Limit cone with root | `aggregate Name { }` |
| Aggregate Root | Apex of limit cone | `root: ObjectName` |
| Enumeration | Colimit cocone | `enum Name = ...` |
| Business Rule | Path equation | `equation Name: ...` |
| Invariant | Equalizer | `invariant: ...` |
| Context Map | Sketch morphism (functor) | `map Name: A -> B { }` |
| Relationship | Morphism | `name: A -> B` |

## 7. File Format

### 7.1 Extension

SketchDDD files use the `.sddd` extension.

### 7.2 Encoding

Files must be UTF-8 encoded.

### 7.3 Structure

A single file may contain multiple contexts and maps. Recommended organization:
```
my-domain/
├── contexts/
│   ├── commerce.sddd
│   └── shipping.sddd
└── maps/
    └── commerce-to-shipping.sddd
```

## 8. Expression Language

For invariants and equations, a simple expression language is supported:

### 8.1 Operators

| Category | Operators |
|----------|-----------|
| Arithmetic | `+`, `-`, `*`, `/`, `%` |
| Comparison | `==`, `!=`, `<`, `<=`, `>`, `>=` |
| Logical | `&&`, `||`, `!` |

### 8.2 Path Navigation

```
Object.field.subfield
```

### 8.3 Built-in Functions

| Function | Description |
|----------|-------------|
| `sum(path)` | Sum of numeric values |
| `count(path)` | Count of elements |
| `all(path, predicate)` | All elements satisfy predicate |
| `any(path, predicate)` | Any element satisfies predicate |
| `min(path)` | Minimum value |
| `max(path)` | Maximum value |

## 9. Appendices

### A. Complete Example

```sketchddd
context Commerce {

  objects { Customer, Order, LineItem, Product, Money }

  entity Customer {
    id: UUID
    name: String
    email: Email
  }

  entity Order {
    id: UUID
    createdAt: DateTime
  }

  entity Product {
    id: UUID
    name: String
    sku: String
  }

  value Money {
    amount: Decimal
    currency: Currency
  }

  value LineItem {
    quantity: Integer
    unitPrice: Money
  }

  enum OrderStatus = Pending | Confirmed | Shipped | Delivered | Cancelled

  morphisms {
    placedBy: Order -> Customer
    items: Order -> List<LineItem>
    product: LineItem -> Product
    price: LineItem -> Money
    status: Order -> OrderStatus
    totalPrice: Order -> Money
  }

  aggregate OrderAggregate {
    root: Order
    contains: [LineItem]
    invariant: totalPrice = sum(items.price)
  }

  equation priceConsistency: Order.totalPrice = sum(Order.items.price)
}

context Shipping {
  objects { Shipment, Recipient, Address }

  morphisms {
    assignedTo: Shipment -> Recipient
    destination: Shipment -> Address
  }
}

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

### B. Grammar (EBNF)

See Section 3 for complete grammar rules.

### C. Error Code Reference

See Section 5.1 for complete error code listing.

## 10. References

1. Barr, M. & Wells, C. (1990). *Category Theory for Computing Science*
2. Evans, E. (2003). *Domain-Driven Design: Tackling Complexity in the Heart of Software*
3. Vernon, V. (2013). *Implementing Domain-Driven Design*
4. [Pest Parser](https://pest.rs/) - Grammar implementation
5. [Ariadne](https://docs.rs/ariadne) - Diagnostic rendering
