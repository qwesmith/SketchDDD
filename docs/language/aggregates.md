# Aggregates

Aggregates define consistency boundaries in your domain model. They ensure that invariants are maintained and that changes happen atomically.

## What is an Aggregate?

An aggregate is a cluster of domain objects that are treated as a single unit for data changes:

```sddd
context Commerce {
  aggregate Order {
    root: Order
    contains: [LineItem, Payment]
    invariant: total = sum(items.price)
  }
}
```

## Aggregate Components

### Root

The **root** is the entry point to the aggregate. All external access must go through the root:

```sddd
aggregate Order {
  root: Order  // Order is the entry point
}
```

Rules for roots:
- Must be an entity (has identity)
- Is the only object accessible from outside
- Controls access to contained objects

### Contains

The **contains** clause lists entities that exist only within this aggregate:

```sddd
aggregate Order {
  root: Order
  contains: [LineItem, ShippingInfo]
}
```

Rules for contained entities:
- Cannot be referenced from outside the aggregate
- Are deleted when the root is deleted
- Can only be modified through the root

### Invariants

**Invariants** are rules that must always be true:

```sddd
aggregate Order {
  root: Order
  contains: [LineItem]
  invariant: total = sum(items.price)
  invariant: items.length > 0
  invariant: status != Cancelled || refundIssued
}
```

## Defining Aggregates

### Basic Aggregate

```sddd
context Banking {
  entity Account {
    id: UUID
    balance: Money
    status: AccountStatus
  }

  entity Transaction {
    id: UUID
    amount: Money
    timestamp: DateTime
    type: TransactionType
  }

  aggregate Account {
    root: Account
    contains: [Transaction]
    invariant: balance >= 0 || overdraftAllowed
  }
}
```

### Multiple Aggregates

```sddd
context Commerce {
  // Customer aggregate
  aggregate Customer {
    root: Customer
    contains: [Address, PaymentMethod]
  }

  // Order aggregate
  aggregate Order {
    root: Order
    contains: [LineItem, Shipment]
    invariant: total = sum(items.price)
  }

  // Product aggregate (separate from Order)
  aggregate Product {
    root: Product
    contains: [ProductVariant, PricingRule]
  }
}
```

## Aggregate Design Guidelines

### 1. Keep Aggregates Small

Large aggregates cause:
- Performance issues (loading too much data)
- Concurrency conflicts (many updates to same aggregate)
- Complexity (too many invariants to maintain)

```sddd
// Good: Small, focused aggregate
aggregate Order {
  root: Order
  contains: [LineItem]
}

// Avoid: Too large
aggregate Order {
  root: Order
  contains: [LineItem, Customer, Product, Payment, Shipment, Invoice, ...]
}
```

### 2. Reference Other Aggregates by ID

Don't include other aggregates; reference them:

```sddd
context Commerce {
  entity Order {
    id: UUID
    customerId: UUID      // Reference, not contained
    productIds: List<UUID> // References, not contained
  }

  aggregate Order {
    root: Order
    contains: [LineItem]  // Only truly contained entities
  }
}
```

### 3. Design for True Invariants

Only include entities that share invariants:

```sddd
// Good: LineItem affects Order's total invariant
aggregate Order {
  root: Order
  contains: [LineItem]
  invariant: total = sum(items.price)
}

// Separate: Shipment doesn't affect Order invariants
aggregate Shipment {
  root: Shipment
  contains: [TrackingEvent]
}
```

### 4. Consider Eventual Consistency

Not everything needs immediate consistency:

```sddd
context Commerce {
  // Immediate consistency within Order
  aggregate Order {
    root: Order
    contains: [LineItem]
    invariant: total = sum(items.price)
  }

  // Inventory updated eventually (not in Order aggregate)
  aggregate Inventory {
    root: Inventory
    invariant: quantity >= 0
  }
}
```

## Invariant Expressions

### Comparison Operators

```sddd
aggregate Account {
  root: Account
  invariant: balance >= 0
  invariant: transactions.length <= maxTransactions
  invariant: status != Closed || balance == 0
}
```

### Logical Operators

```sddd
aggregate Order {
  root: Order
  invariant: status == Pending || paymentReceived
  invariant: items.length > 0 && items.length <= 100
  invariant: !cancelled || refundProcessed
}
```

### Collection Functions

```sddd
aggregate Order {
  root: Order
  contains: [LineItem]
  invariant: total = sum(items.price)
  invariant: items.all(i => i.quantity > 0)
  invariant: items.any(i => i.type == Physical) || shippingNotRequired
}
```

## Category Theory: Aggregates as Limits

In category theory, an aggregate is modeled as a **limit cone**:

```
        Order (root)
       /  |  \
      /   |   \
LineItem LineItem Payment
```

The root is the apex of the cone, and contained entities are connected through projection morphisms. This structure ensures:

- Single point of access (the apex)
- Well-defined containment (the projections)
- Structural consistency (the limit property)

## Generated Code

=== "Rust"
    ```rust
    pub struct Order {
        pub id: OrderId,
        pub items: Vec<LineItem>,
        pub total: Money,
    }

    impl Order {
        // Invariant checked on construction/modification
        pub fn add_item(&mut self, item: LineItem) -> Result<(), DomainError> {
            self.items.push(item);
            self.recalculate_total();
            self.validate()?;
            Ok(())
        }

        fn validate(&self) -> Result<(), DomainError> {
            // invariant: total = sum(items.price)
            let calculated = self.items.iter().map(|i| i.price).sum();
            if self.total != calculated {
                return Err(DomainError::InvariantViolation("total mismatch"));
            }
            Ok(())
        }
    }
    ```

=== "TypeScript"
    ```typescript
    export class Order {
      readonly id: OrderId;
      private _items: LineItem[];
      private _total: Money;

      addItem(item: LineItem): void {
        this._items.push(item);
        this.recalculateTotal();
        this.validate();
      }

      private validate(): void {
        // invariant: total = sum(items.price)
        const calculated = this._items.reduce((sum, i) => sum + i.price, 0);
        if (this._total !== calculated) {
          throw new DomainError('Invariant violation: total mismatch');
        }
      }
    }
    ```

## Best Practices

1. **Start small** - Begin with minimal aggregates and expand if needed
2. **Question every contains** - Does this entity truly share invariants with the root?
3. **Use IDs for references** - Don't try to include everything
4. **Express real business rules** - Invariants should reflect actual domain constraints
5. **Consider transactions** - Aggregates define transactional boundaries
