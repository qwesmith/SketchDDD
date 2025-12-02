# Category Theory Overview

SketchDDD uses category theory as its mathematical foundation. This provides a rigorous basis for domain modeling with formal guarantees about structure and relationships.

## Why Category Theory?

Category theory offers several advantages for domain modeling:

1. **Composability**: Morphisms compose, enabling modular design
2. **Abstraction**: Focus on structure, not implementation
3. **Universality**: Same patterns apply across all domains
4. **Precision**: Formal definitions prevent ambiguity

## Core Concepts

### Categories

A category consists of:
- **Objects**: Things (entities, value objects)
- **Morphisms**: Relationships between objects
- **Composition**: Combining morphisms
- **Identity**: Every object has an identity morphism

In SketchDDD, each bounded context forms a category:

```sddd
context Orders {
  // Objects
  entity Order { ... }
  entity Customer { ... }

  // Morphisms
  morphisms {
    customer: Order -> Customer
  }
}
```

### Morphisms

Morphisms represent relationships:

```
     customer
Order ───────→ Customer
```

Morphisms can compose:

```
       items            product
Order ───────→ LineItem ───────→ Product

// Composed:
       items.product
Order ─────────────→ Product
```

### Sketches

A **sketch** is a category with additional structure:
- **Limit cones**: Product types (entities with fields)
- **Colimit cocones**: Sum types (enumerations)
- **Diagrams**: Patterns that must commute

SketchDDD models are sketches:

```sddd
// Limit cone: Product type with projections
entity Order {
  id: UUID          // π₁: Order → UUID
  customer: Customer // π₂: Order → Customer
  total: Money       // π₃: Order → Money
}

// Colimit cocone: Sum type with injections
enum OrderStatus = Pending | Confirmed | Shipped
// ι₁: Pending → OrderStatus
// ι₂: Confirmed → OrderStatus
// ι₃: Shipped → OrderStatus
```

## Limits

**Limits** represent "product" or "AND" relationships. They combine multiple objects into one.

### Product (Entity Fields)

An entity is a product (limit) of its fields:

```
        π_id
Order ───────→ UUID
   │
   │ π_total
   ▼
  Money
```

```sddd
entity Order {
  id: UUID
  total: Money
}
```

### Pullback (Shared Context)

A pullback captures objects that share a common reference:

```
OrderHistory ──→ Order
     │             │
     ▼             ▼
  Customer ←─── Customer
```

This represents "OrderHistory for a Customer's Orders".

## Colimits

**Colimits** represent "sum" or "OR" relationships. They model choices.

### Coproduct (Enum)

An enumeration is a coproduct (colimit):

```
Pending ───→ OrderStatus ←─── Confirmed
                 ▲
                 │
             Shipped
```

```sddd
enum OrderStatus = Pending | Confirmed | Shipped
```

### Tagged Union

Enums with data are tagged unions:

```sddd
enum PaymentMethod {
  Card { last4: String }
  Bank { account: String }
  Cash
}
```

## Functors

**Functors** map between categories while preserving structure.

### Context Maps as Functors

A context map is a functor between bounded contexts:

```sddd
map OrderToShipping: Orders -> Shipping {
  mappings {
    Order -> Shipment
    Customer -> Recipient
  }
}
```

This means:
- Objects map: `Order ↦ Shipment`, `Customer ↦ Recipient`
- Morphisms map: Relationships are preserved

### Forgetful Functors

Some mappings "forget" structure:

```
Orders ──F──→ Reporting

Order         ↦  OrderSummary (fewer fields)
OrderStatus   ↦  String       (loses type safety)
```

## Natural Transformations

**Natural transformations** are mappings between functors that commute with morphisms.

In DDD terms, this is like having consistent translations:

```
         F(Order)                G(Order)
Orders ──────────→ Shipping   ──────────→ Billing
   │                  │                     │
   │ customer         │ recipient           │ payer
   ▼                  ▼                     ▼
Customer           Recipient              Payer
```

If the translation is natural, these diagrams commute.

## Aggregates as Cones

An aggregate root defines a limit cone over contained entities:

```sddd
aggregate Order {
  root: Order
  contains: [LineItem]
  invariant: total = sum(items.quantity * items.price)
}
```

The aggregate ensures the invariant holds for all morphisms:

```
        items
Order ─────────→ LineItem
  │                  │
  │ total            │ quantity * price
  ▼                  ▼
Money ──────────→ Money
        sum
```

## Practical Benefits

### Type Safety

Category theory ensures type correctness:
- Morphisms have well-defined source and target
- Composition respects types
- Generated code is type-safe

### Refactoring Safety

Functorial mappings preserve structure:
- Changes in one context map correctly to others
- Invariants are maintained across contexts

### Documentation

Diagrams communicate structure:
- Visual representation of relationships
- Mathematical precision avoids ambiguity

## Further Reading

- [Sketches](sketches.md) - Detailed sketch theory
- [Limits and Colimits](limits-colimits.md) - Product and sum types
- [Functors](functors.md) - Context mappings

### Academic References

- Barr & Wells, "Category Theory for Computing Science"
- Awodey, "Category Theory" (Oxford Logic Guides)
- Spivak, "Category Theory for the Sciences"
