# Entities & Value Objects

Entities and value objects are the two fundamental building blocks for modeling domain concepts.

## Entities

An **entity** is an object with a unique identity that persists through time.

### Defining Entities

```sddd
context Commerce {
  entity Customer {
    id: UUID
    email: Email
    name: String
    registeredAt: DateTime
    status: CustomerStatus
  }

  entity Order {
    id: UUID
    orderNumber: String
    createdAt: DateTime
    total: Money
  }
}
```

### Entity Characteristics

1. **Identity**: Two entities with the same ID are the same entity, even if other attributes differ
2. **Lifecycle**: Entities are created, modified, and potentially deleted
3. **Mutability**: Entity attributes can change over time

### Identity Fields

The `id` field typically serves as the entity's identity:

```sddd
entity Customer {
  id: UUID           // Primary identity
  email: Email       // Could be unique, but not identity
  name: String       // Can change
}
```

!!! tip "Naming Conventions"
    While any field name works, `id` is conventional and recognized by code generators.

### Generated Code

=== "Rust"
    ```rust
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct CustomerId(pub Uuid);

    #[derive(Debug, Clone)]
    pub struct Customer {
        pub id: CustomerId,
        pub email: String,
        pub name: String,
        pub registered_at: DateTime<Utc>,
        pub status: CustomerStatus,
    }
    ```

=== "TypeScript"
    ```typescript
    export type CustomerId = string & { readonly __brand: 'CustomerId' };

    export interface Customer {
      readonly id: CustomerId;
      readonly email: string;
      readonly name: string;
      readonly registeredAt: Date;
      readonly status: CustomerStatus;
    }
    ```

## Value Objects

A **value object** is defined entirely by its attributes and has no conceptual identity.

### Defining Value Objects

```sddd
context Commerce {
  value Money {
    amount: Decimal
    currency: Currency
  }

  value Address {
    street: String
    city: String
    state: String
    postalCode: String
    country: String
  }

  value DateRange {
    start: Date
    end: Date
  }
}
```

### Value Object Characteristics

1. **No Identity**: Two value objects with identical attributes are equal
2. **Immutable**: Once created, a value object never changes
3. **Replaceable**: To "change" a value object, you replace it entirely

### When to Use Value Objects

Use value objects for:

- **Measurements**: `Money`, `Weight`, `Distance`
- **Descriptions**: `Address`, `Name`, `Color`
- **Ranges**: `DateRange`, `PriceRange`
- **Quantities**: `Quantity`, `Percentage`

### Generated Code

=== "Rust"
    ```rust
    #[derive(Debug, Clone, PartialEq)]
    pub struct Money {
        pub amount: Decimal,
        pub currency: Currency,
    }

    impl Eq for Money {}  // Value equality
    ```

=== "TypeScript"
    ```typescript
    export interface Money {
      readonly amount: number;
      readonly currency: Currency;
    }

    // Value objects are compared by structure
    function moneyEquals(a: Money, b: Money): boolean {
      return a.amount === b.amount && a.currency === b.currency;
    }
    ```

=== "Kotlin"
    ```kotlin
    data class Money(
        val amount: BigDecimal,
        val currency: Currency
    )
    // data class provides equals/hashCode based on all fields
    ```

## Comparison

| Aspect | Entity | Value Object |
|--------|--------|--------------|
| Identity | Has unique ID | Defined by attributes |
| Equality | Same ID = same entity | Same attributes = equal |
| Mutability | Can change over time | Immutable |
| Lifecycle | Created, modified, deleted | Created, replaced |
| Example | Customer, Order | Money, Address |

## Entity vs Value Object Decision

Ask yourself:

1. **Does it need to be tracked over time?** → Entity
2. **Would two instances with same data be interchangeable?** → Value Object
3. **Does it have a lifecycle (created, modified, deleted)?** → Entity
4. **Is it a measurement or description?** → Value Object

### Examples

```sddd
context Hospital {
  // Entity: Each patient is unique, even if they have the same name
  entity Patient {
    id: UUID
    name: String
    dateOfBirth: Date
  }

  // Value Object: A blood pressure reading is just data
  value BloodPressure {
    systolic: Int
    diastolic: Int
    measuredAt: DateTime
  }

  // Entity: Each appointment needs to be tracked
  entity Appointment {
    id: UUID
    scheduledFor: DateTime
    duration: Duration
  }

  // Value Object: Duration is just a measurement
  value Duration {
    minutes: Int
  }
}
```

## Field Types

Both entities and value objects can use:

### Required Fields

```sddd
entity Customer {
  id: UUID       // Required
  name: String   // Required
}
```

### Optional Fields

```sddd
entity Customer {
  id: UUID
  name: String
  nickname: String?     // Optional
  middleName: String?   // Optional
}
```

### Collection Fields

```sddd
entity Customer {
  id: UUID
  emails: List<Email>           // List
  tags: Set<String>             // Set
  preferences: Map<String, String>  // Map
}
```

### Nested Types

```sddd
entity Customer {
  id: UUID
  address: Address        // Value object
  orders: List<Order>     // List of entities
}
```

## Best Practices

### 1. Keep Value Objects Small

```sddd
// Good: Focused value object
value Money {
  amount: Decimal
  currency: Currency
}

// Avoid: Too many concerns
value Money {
  amount: Decimal
  currency: Currency
  exchangeRate: Float
  lastUpdated: DateTime
  source: String
}
```

### 2. Make Value Objects Immutable by Design

The code generators enforce immutability, but design with it in mind:

```sddd
// Value objects shouldn't have status or state
value Address {
  street: String
  city: String
  // No: status, lastModified, etc.
}
```

### 3. Use Entities for Domain Objects with Behavior

```sddd
entity Order {
  id: UUID
  status: OrderStatus  // Status changes over time
  items: List<LineItem>
  total: Money
}
```

### 4. Compose Value Objects

```sddd
value FullName {
  first: String
  middle: String?
  last: String
}

value ContactInfo {
  email: Email
  phone: PhoneNumber?
  address: Address?
}

entity Person {
  id: UUID
  name: FullName         // Composed value object
  contact: ContactInfo   // Composed value object
}
```
