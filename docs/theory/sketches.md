# Sketches

A **sketch** is a mathematical structure that precisely captures the shape of a domain model. SketchDDD uses sketches as its core representation.

## What is a Sketch?

Formally, a sketch S = (G, L, C) consists of:

- **G**: A graph (objects and arrows)
- **L**: A set of limit cones
- **C**: A set of colimit cocones

In domain modeling terms:
- Objects are types (entities, value objects, primitives)
- Arrows are relationships (morphisms)
- Limit cones define product types (entities with fields)
- Colimit cocones define sum types (enums)

## From DDD to Sketches

### Bounded Context → Sketch Category

Each bounded context becomes a sketch:

```sddd
context Orders {
  // The sketch for this context
}
```

### Entity → Limit Cone

An entity defines a limit cone (product):

```sddd
entity Order {
  id: UUID
  customer: Customer
  total: Money
  status: OrderStatus
}
```

This creates a limit cone:

```
            Order (apex)
           /  |  \  \
          /   |   \  \
         ↓    ↓    ↓  ↓
      UUID Customer Money OrderStatus
       π₁    π₂    π₃    π₄
```

Each field is a **projection** from the entity to its type.

### Value Object → Limit Cone

Value objects are also limit cones, but without identity:

```sddd
value Money {
  amount: Decimal
  currency: Currency
}
```

```
         Money
        /     \
       ↓       ↓
   Decimal  Currency
     π₁       π₂
```

### Enum → Colimit Cocone

An enumeration defines a colimit cocone (coproduct):

```sddd
enum OrderStatus = Pending | Confirmed | Shipped | Delivered
```

```
Pending  Confirmed  Shipped  Delivered
    \        |        |        /
     ↘       ↓        ↓       ↙
         OrderStatus (apex)
```

Each variant is an **injection** into the sum type.

### Morphism → Arrow

A morphism is simply an arrow in the graph:

```sddd
morphisms {
  customer: Order -> Customer
}
```

```
       customer
Order ─────────→ Customer
```

## Composition

Arrows compose in sketches:

```sddd
morphisms {
  order: LineItem -> Order
  customer: Order -> Customer
}

// Implicitly, we have:
// customer ∘ order: LineItem -> Customer
```

```
            order          customer
LineItem ─────────→ Order ─────────→ Customer

// Composition:
           customer ∘ order
LineItem ───────────────────→ Customer
```

## Universal Properties

### Limit Universal Property

For a limit cone, any other cone factors uniquely:

```
           X
          /|\
         / | \
        ↓  ↓  ↓
     UUID Customer Money
        ↑   ↑   ↑
         \  |  /
          Order   ← Universal cone

// For any X with arrows to all fields,
// there exists a unique arrow X → Order
```

This means: to construct an Order, you must provide all its fields.

### Colimit Universal Property

For a colimit cocone, any other cocone factors uniquely:

```
Pending → OrderStatus → X
           ↑          ↗
       Confirmed ────┘
           ↑        ↗
        Shipped ───┘

// For any X with arrows from all variants,
// there exists a unique arrow OrderStatus → X
```

This means: to handle an OrderStatus, you must handle all cases.

## Models of Sketches

A **model** of a sketch is a structure-preserving interpretation:

```
Sketch S ─────→ Set (or other category)
```

- Objects map to sets (or types)
- Arrows map to functions
- Limits map to products (tuples, records)
- Colimits map to sums (tagged unions)

Generated code is a model of your sketch in a programming language.

## Sketch Morphisms

A **sketch morphism** preserves structure between sketches:

```sddd
map OrderToShipping: Orders -> Shipping {
  mappings {
    Order -> Shipment
    LineItem -> Package
  }
}
```

This is a functor that:
- Maps objects to objects
- Maps arrows to arrows
- Preserves limits (products map to products)
- Preserves colimits (sums map to sums)

## Aggregates as Structured Cones

An aggregate defines a cone with constraints:

```sddd
aggregate Order {
  root: Order
  contains: [LineItem]
  invariant: total = sum(items.map(i => i.quantity * i.unitPrice))
}
```

This creates a structured diagram:

```
      Order (root)
         |
         | items
         ↓
     [LineItem]
         |
         | for each item
         ↓
   (quantity, unitPrice)
         |
         | multiply
         ↓
       Money
```

The invariant is a **commutative diagram**: different paths yield the same result.

## Practical Implications

### 1. Complete Specifications

Sketches force complete definitions:
- All entity fields must be specified
- All enum variants must be listed
- All relationships must be typed

### 2. Guaranteed Code Generation

Because sketches have precise semantics:
- Code generation is deterministic
- Generated code respects the model
- Type safety is preserved

### 3. Refactoring Guidance

Sketch morphisms show valid transformations:
- Safe renames preserve structure
- Safe splits maintain relationships
- Invalid changes break sketch morphisms

## Example: Complete Sketch

```sddd
context ECommerce {
  // Objects (types)
  entity Customer {
    id: UUID
    email: Email
    name: String
  }

  entity Order {
    id: UUID
    orderNumber: String
    total: Money
    status: OrderStatus
  }

  entity LineItem {
    id: UUID
    quantity: Int
    unitPrice: Money
  }

  entity Product {
    id: UUID
    sku: String
    name: String
    price: Money
  }

  value Money {
    amount: Decimal
    currency: Currency
  }

  // Colimit cocones
  enum OrderStatus = Pending | Confirmed | Shipped | Delivered | Cancelled
  enum Currency = USD | EUR | GBP

  // Arrows
  morphisms {
    customer: Order -> Customer
    items: Order -> List<LineItem>
    product: LineItem -> Product
  }

  // Structured cone with invariant
  aggregate Order {
    root: Order
    contains: [LineItem]
    invariant: total = sum(items.map(i => i.quantity * i.unitPrice))
  }
}
```

This complete sketch specifies:
- 4 entities (limit cones with identity)
- 1 value object (limit cone without identity)
- 2 enums (colimit cocones)
- 3 relationships (arrows)
- 1 aggregate (structured cone with invariant)
