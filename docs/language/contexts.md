# Bounded Contexts

A bounded context is the fundamental building block in SketchDDD. It represents a linguistic boundary within which terms have specific, consistent meanings.

## Defining a Context

```sddd
context Commerce {
  // All domain elements go here
}
```

## Multiple Contexts

A file can contain multiple contexts:

```sddd
context Sales {
  entity Customer { ... }
  entity Order { ... }
}

context Warehouse {
  entity Inventory { ... }
  entity Shipment { ... }
}

context Billing {
  entity Invoice { ... }
  entity Payment { ... }
}
```

## Why Bounded Contexts?

In Domain-Driven Design, the same word often means different things in different parts of a business:

```sddd
// In Sales, a Customer is someone who buys
context Sales {
  entity Customer {
    id: UUID
    name: String
    purchaseHistory: List<Order>
    loyaltyPoints: Int
  }
}

// In Support, a Customer is someone who needs help
context Support {
  entity Customer {
    id: UUID
    name: String
    tickets: List<SupportTicket>
    satisfactionScore: Float
  }
}

// In Shipping, a Customer is a delivery destination
context Shipping {
  entity Customer {
    id: UUID
    name: String
    address: Address
    deliveryPreferences: DeliveryPreferences
  }
}
```

Each context has its own `Customer` with different attributes relevant to that subdomain.

## Context Contents

A context can contain:

### Objects

Simple type declarations without fields:

```sddd
context Example {
  objects { Customer, Order, Product, LineItem }
}
```

### Entities

Objects with identity and fields:

```sddd
context Example {
  entity Customer {
    id: UUID
    name: String
    email: Email
  }
}
```

### Value Objects

Immutable objects defined by their attributes:

```sddd
context Example {
  value Money {
    amount: Decimal
    currency: Currency
  }
}
```

### Enums

Fixed sets of values:

```sddd
context Example {
  enum OrderStatus = Pending | Confirmed | Shipped | Delivered
}
```

### Morphisms

Relationships between objects:

```sddd
context Example {
  morphisms {
    placedBy: Order -> Customer
    items: Order -> List<LineItem>
  }
}
```

### Aggregates

Consistency boundaries:

```sddd
context Example {
  aggregate Order {
    root: Order
    contains: [LineItem, Payment]
    invariant: total >= 0
  }
}
```

### Equations

Path equivalences expressing business rules:

```sddd
context Example {
  equation discountRule: order.discount = order.customer.loyaltyDiscount
}
```

## Category Theory: Contexts as Sketches

In category theory, a bounded context is modeled as a **sketch**:

```
S = (G, E, L, C)
```

Where:

- **G** (Graph): Objects and morphisms
- **E** (Equations): Path equivalences
- **L** (Limits): Aggregates, value objects
- **C** (Colimits): Enums, sum types

This mathematical foundation ensures:

- Consistent relationships
- Well-defined boundaries
- Composable transformations

## Best Practices

### 1. One Context Per Subdomain

Each context should represent a distinct area of the business:

```sddd
// Good: Clear separation
context OrderManagement { ... }
context Inventory { ... }
context CustomerRelations { ... }

// Avoid: Everything in one context
context Everything { ... }
```

### 2. Use Ubiquitous Language

Names should match how domain experts speak:

```sddd
// Good: Domain language
context Insurance {
  entity Policy { ... }
  entity Claim { ... }
  enum ClaimStatus = Filed | UnderReview | Approved | Denied
}

// Avoid: Technical jargon
context Insurance {
  entity PolicyRecord { ... }
  entity ClaimEntity { ... }
  enum ClaimStatusEnum = STATUS_1 | STATUS_2 | STATUS_3
}
```

### 3. Keep Contexts Focused

If a context is getting too large, consider splitting:

```sddd
// Instead of one huge Commerce context:
context ProductCatalog { ... }
context ShoppingCart { ... }
context Checkout { ... }
context OrderFulfillment { ... }
```

## Connecting Contexts

Use [Context Maps](context-maps.md) to define how contexts relate:

```sddd
map SalesToShipping: Sales -> Shipping {
  pattern: CustomerSupplier
  mappings {
    Customer -> Customer
    Order -> Shipment
  }
}
```
