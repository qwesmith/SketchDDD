# Code Generation Overview

SketchDDD generates idiomatic code from your domain models for multiple programming languages.

## Supported Languages

| Language | Target | Status | Features |
|----------|--------|--------|----------|
| Rust | `rust` | Stable | Structs, enums, serde, validation |
| TypeScript | `typescript` | Stable | Interfaces, types, Zod schemas |
| Kotlin | `kotlin` | Stable | Data classes, sealed classes |
| Python | `python` | Stable | Dataclasses, Pydantic, enums |
| Java | `java` | Stable | Records, enums, builders |
| Clojure | `clojure` | Stable | Specs, records, protocols |
| Haskell | `haskell` | Stable | ADTs, newtypes, deriving |

## Basic Usage

```bash
# Generate code for a specific language
sketchddd codegen domain.sddd --target rust

# Specify output directory
sketchddd codegen domain.sddd --target typescript --output ./src/types

# Generate for multiple targets
sketchddd codegen domain.sddd --target rust --target typescript
```

## What Gets Generated

For each bounded context, SketchDDD generates:

### Entities

Domain entities become classes/structs with:
- All defined fields with appropriate types
- Identity field (typically `id`)
- Serialization support where applicable

### Value Objects

Value objects become immutable types with:
- All defined fields
- Equality based on all fields
- No identity field

### Enums

Enumerations become:
- Simple enums for variants without data
- Sum types/sealed classes for variants with data

### Aggregates

Aggregate roots include:
- Root entity definition
- Contained entity references
- Invariant documentation (as comments)

## Type Mapping

SketchDDD maps domain types to language-specific types:

| SketchDDD | Rust | TypeScript | Python | Java |
|-----------|------|------------|--------|------|
| `String` | `String` | `string` | `str` | `String` |
| `Int` | `i64` | `number` | `int` | `long` |
| `Float` | `f64` | `number` | `float` | `double` |
| `Bool` | `bool` | `boolean` | `bool` | `boolean` |
| `UUID` | `Uuid` | `string` | `UUID` | `UUID` |
| `DateTime` | `DateTime<Utc>` | `Date` | `datetime` | `Instant` |
| `Date` | `NaiveDate` | `string` | `date` | `LocalDate` |
| `Email` | `String` | `string` | `str` | `String` |
| `List<T>` | `Vec<T>` | `T[]` | `list[T]` | `List<T>` |
| `Map<K,V>` | `HashMap<K,V>` | `Map<K,V>` | `dict[K,V]` | `Map<K,V>` |
| `T?` | `Option<T>` | `T \| null` | `T \| None` | `Optional<T>` |

## Example

Given this SketchDDD model:

```sddd
context Orders {
  entity Order {
    id: UUID
    customerId: UUID
    total: Money
    status: OrderStatus
    createdAt: DateTime
  }

  value Money {
    amount: Decimal
    currency: Currency
  }

  enum OrderStatus = Pending | Confirmed | Shipped | Delivered
  enum Currency = USD | EUR | GBP
}
```

### Generated Rust

```rust
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Order {
    pub id: Uuid,
    pub customer_id: Uuid,
    pub total: Money,
    pub status: OrderStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Money {
    pub amount: Decimal,
    pub currency: Currency,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderStatus {
    Pending,
    Confirmed,
    Shipped,
    Delivered,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Currency {
    Usd,
    Eur,
    Gbp,
}
```

### Generated TypeScript

```typescript
export interface Order {
  id: string;
  customerId: string;
  total: Money;
  status: OrderStatus;
  createdAt: Date;
}

export interface Money {
  amount: number;
  currency: Currency;
}

export type OrderStatus = 'Pending' | 'Confirmed' | 'Shipped' | 'Delivered';

export type Currency = 'USD' | 'EUR' | 'GBP';
```

## Language-Specific Guides

For detailed information about each language:

- [Rust](rust.md) - Structs, enums, traits, and crate organization
- [TypeScript](typescript.md) - Interfaces, types, and Zod integration
- [Kotlin](kotlin.md) - Data classes and sealed hierarchies
- [Python](python.md) - Dataclasses and Pydantic models
- [Java](java.md) - Records, builders, and immutability
- [Clojure](clojure.md) - Specs, records, and functional design
- [Haskell](haskell.md) - Algebraic data types and type classes
