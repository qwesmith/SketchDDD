# Limits and Colimits

Limits and colimits are fundamental constructions in category theory that correspond directly to common programming patterns.

## Limits: Product Types

A **limit** combines multiple objects into one, with projections to each component.

### Products

The simplest limit is a product of two objects:

```
    A × B
   /     \
  π₁     π₂
 ↓         ↓
 A         B
```

In programming, this is a tuple or record:

```sddd
value Coordinate {
  x: Float   // π₁: Coordinate → Float
  y: Float   // π₂: Coordinate → Float
}
```

```rust
// Generated Rust
struct Coordinate {
    x: f64,  // Projection to first component
    y: f64,  // Projection to second component
}
```

### Terminal Object

The **terminal object** (1) has exactly one element:

```
  A ───→ 1
```

Every object has a unique arrow to 1.

In SketchDDD, unit types like simple enum variants represent this:

```sddd
enum Status = Active  // Active : 1 → Status
```

### Pullbacks

A **pullback** is the limit of a diagram:

```
    A ──f──→ C ←──g── B
```

The pullback P is the "fiber product":

```
    P ───→ B
    │      │
    ↓      ↓ g
    A ──f─→ C
```

In domain terms, this captures objects that share a common reference:

```sddd
// Orders and Reviews for the same Customer
entity CustomerActivity {
  order: Order
  review: Review
  // Pullback ensures: order.customer = review.customer
}
```

### Equalizers

An **equalizer** is the limit of parallel arrows:

```
    f
A ═══→ B
    g
```

The equalizer E is the subset where f = g:

```
E ──→ A ═══→ B
        f,g
```

This models constraints:

```sddd
entity ValidatedOrder {
  order: Order
  // Equalizer: order.calculatedTotal = order.declaredTotal
}
```

## Colimits: Sum Types

A **colimit** combines objects with injections from each component.

### Coproducts

The simplest colimit is a coproduct (sum) of two objects:

```
 A         B
 ↓         ↓
 ι₁       ι₂
   \     /
    A + B
```

In programming, this is a tagged union:

```sddd
enum Result {
  Success { value: T }   // ι₁: T → Result
  Failure { error: E }   // ι₂: E → Result
}
```

```rust
// Generated Rust
enum Result<T, E> {
    Success { value: T },
    Failure { error: E },
}
```

### Initial Object

The **initial object** (0) has no elements:

```
  0 ───→ A
```

There's a unique arrow from 0 to every object.

This represents impossible states or `Never`/`Void` types.

### Pushouts

A **pushout** is the colimit of a diagram:

```
    A ←──f── C ──g──→ B
```

The pushout P glues A and B along C:

```
    A ──→ P
    ↑      ↑
    f
    C ──g─→ B
```

In domain terms, this merges contexts:

```sddd
// Merging UserContext and ProfileContext
// along their shared User type
context UnifiedUser {
  // Pushout of User from both contexts
}
```

### Coequalizers

A **coequalizer** is the colimit of parallel arrows:

```
    f
A ═══→ B
    g
```

The coequalizer Q identifies elements that f and g make equal:

```
A ═══→ B ──→ Q
   f,g
```

This models quotients:

```sddd
// Normalized email (ignoring case, dots in gmail)
// Coequalizer of "same normalized form"
```

## Limits in SketchDDD

### Entities as Limits

Every entity is a limit of its fields:

```sddd
entity Customer {
  id: UUID          // π₁
  email: Email      // π₂
  name: String      // π₃
  tier: CustomerTier // π₄
}
```

```
         Customer
       /   |   \   \
      ↓    ↓    ↓    ↓
   UUID  Email String CustomerTier
```

### Value Objects as Limits

Value objects are also limits, but without identity:

```sddd
value Address {
  street: String
  city: String
  country: Country
}
```

### Aggregates as Structured Limits

Aggregates define limits with additional constraints:

```sddd
aggregate Order {
  root: Order
  contains: [LineItem]
}
```

The aggregate is a limit where:
- The root is the apex
- Contained entities are accessible only through the root
- Invariants are commutative diagrams

## Colimits in SketchDDD

### Simple Enums

Simple enums are coproducts of terminal objects:

```sddd
enum Color = Red | Green | Blue
```

```
  1    1    1
  ↓    ↓    ↓
 Red Green Blue
   \   |   /
    Color
```

### Enums with Data

Enums with data are general coproducts:

```sddd
enum Shape {
  Circle { radius: Float }
  Rectangle { width: Float, height: Float }
  Point
}
```

```
  Float     Float×Float    1
    ↓           ↓          ↓
 Circle    Rectangle    Point
     \         |         /
          Shape
```

## Universal Properties

### Limit Universal Property

To construct a limit, you must provide all components:

```rust
// Must provide all fields
let customer = Customer {
    id: Uuid::new_v4(),
    email: "user@example.com".into(),
    name: "Alice".into(),
    tier: CustomerTier::Gold,
};
```

### Colimit Universal Property

To consume a colimit, you must handle all cases:

```rust
// Must handle all variants
match shape {
    Shape::Circle { radius } => /* ... */,
    Shape::Rectangle { width, height } => /* ... */,
    Shape::Point => /* ... */,
}
```

## Practical Applications

### 1. Type-Safe APIs

Limits ensure complete data:
```rust
// Cannot create with missing fields
fn create_order(order: Order) -> OrderId
```

### 2. Exhaustive Pattern Matching

Colimits ensure complete handling:
```rust
// Compiler error if case is missing
fn handle_status(status: OrderStatus) -> Action
```

### 3. Refactoring Safety

Adding a variant to an enum (colimit):
- Compiler finds all places that need updating
- No runtime surprises

### 4. Data Migrations

Understanding structure as limits/colimits helps with:
- Schema migrations (adding/removing fields)
- API versioning (extending enums)
- Data transformations (structure-preserving maps)
