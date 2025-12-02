# Language Overview

The SketchDDD language (`.sddd` files) provides a concise, readable syntax for defining domain models.

## Structure

A SketchDDD file contains:

1. **Bounded Contexts** - Define domain boundaries
2. **Context Maps** - Define relationships between contexts

```sddd
// Context definitions
context ContextName {
  // Domain elements
}

// Context map definitions
map MapName: SourceContext -> TargetContext {
  // Mappings
}
```

## Comments

```sddd
// Single-line comment

/* Multi-line
   comment */

context Example {
  // Comments can appear anywhere
  entity Thing {
    /* Including inside blocks */
    id: UUID
  }
}
```

## Bounded Contexts

A bounded context is a linguistic boundary containing:

```sddd
context Commerce {
  // Objects (simple declarations)
  objects { Customer, Order, Product }

  // Entities (with fields)
  entity Customer {
    id: UUID
    name: String
  }

  // Value Objects
  value Money {
    amount: Decimal
    currency: Currency
  }

  // Enums
  enum OrderStatus = Pending | Confirmed | Shipped

  // Morphisms (relationships)
  morphisms {
    placedBy: Order -> Customer
  }

  // Aggregates
  aggregate Order {
    root: Order
    contains: [LineItem]
  }

  // Equations (path equivalences)
  equation orderTotal: order.total = sum(order.items.price)
}
```

## Type System

### Built-in Types

| Type | Description | Example |
|------|-------------|---------|
| `String` | Text | `"hello"` |
| `Int` | Integer | `42` |
| `Float` | Floating point | `3.14` |
| `Decimal` | Precise decimal | `19.99` |
| `Boolean` | True/False | `true` |
| `UUID` | Universal identifier | `550e8400-...` |
| `Date` | Calendar date | `2024-01-15` |
| `DateTime` | Date and time | `2024-01-15T10:30:00Z` |
| `Time` | Time of day | `10:30:00` |
| `Email` | Email address | `user@example.com` |
| `URL` | Web address | `https://example.com` |

### Generic Types

```sddd
// Optional (nullable)
address: Address?

// Lists
items: List<LineItem>

// Sets
tags: Set<String>

// Maps
metadata: Map<String, String>
```

### Custom Types

Custom types are defined through entities, value objects, and enums:

```sddd
entity Customer { ... }      // Customer is now a type
value Money { ... }          // Money is now a type
enum Status = A | B          // Status is now a type
```

## Elements Reference

| Element | Purpose | Identity |
|---------|---------|----------|
| `objects` | Quick declarations | N/A |
| `entity` | Objects with lifecycle | Yes |
| `value` | Immutable data | No (structural) |
| `enum` | Fixed choices | N/A |
| `morphisms` | Relationships | N/A |
| `aggregate` | Consistency boundary | Via root |
| `equation` | Path equivalence | N/A |

## Syntax Rules

### Identifiers

- Start with a letter
- Contain letters, numbers, underscores
- PascalCase for types: `Customer`, `OrderStatus`
- camelCase for fields/morphisms: `firstName`, `placedBy`

### Blocks

Curly braces define blocks:

```sddd
context Name {
  entity Name {
    field: Type
  }
}
```

### Lists

Square brackets for inline lists:

```sddd
objects { A, B, C }
contains: [Item1, Item2]
```

### Arrows

Arrows define mappings:

```sddd
// Morphism
name: Source -> Target

// Context map
map Name: Context1 -> Context2 { }
```

## File Organization

For large projects, split into multiple files:

```
domain/
├── commerce.sddd       # Commerce context
├── shipping.sddd       # Shipping context
├── notifications.sddd  # Notifications context
└── maps.sddd          # Context maps
```

Then combine when processing:

```bash
sketchddd check domain/*.sddd
```

## Next Steps

- [Bounded Contexts](contexts.md)
- [Entities & Value Objects](entities-values.md)
- [Aggregates](aggregates.md)
- [Morphisms](morphisms.md)
- [Enums](enums.md)
- [Context Maps](context-maps.md)
