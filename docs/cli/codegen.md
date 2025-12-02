# Code Generation

SketchDDD generates idiomatic code in multiple programming languages from your domain model.

## Basic Usage

```bash
sketchddd codegen model.sddd --target <LANGUAGE>
```

## Supported Languages

| Language | Target Flag | Output |
|----------|-------------|--------|
| Rust | `rust`, `rs` | Structs, enums, type aliases |
| TypeScript | `typescript`, `ts` | Interfaces, types, Zod schemas |
| Kotlin | `kotlin`, `kt` | Data classes, sealed classes |
| Python | `python`, `py` | Dataclasses, Pydantic models |
| Java | `java` | Records (Java 17+), POJOs |
| Clojure | `clojure`, `clj` | Records, specs |
| Haskell | `haskell`, `hs` | ADTs, Aeson instances |

## Output Options

### To Stdout

```bash
sketchddd codegen model.sddd --target typescript
```

### To File

```bash
sketchddd codegen model.sddd --target typescript --output src/domain.ts
```

### To Directory (multiple files)

```bash
sketchddd codegen model.sddd --target rust --output src/domain/
```

## Language-Specific Features

### Rust

Generated code includes:

- Strongly-typed ID wrappers
- Derive macros for common traits
- Serde serialization support
- Optional field handling

```rust
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CustomerId(pub Uuid);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Customer {
    pub id: CustomerId,
    pub name: String,
    pub email: String,
    pub address: Option<Address>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Address {
    pub street: String,
    pub city: String,
    pub postal_code: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CustomerStatus {
    Active,
    Suspended,
    Deleted,
}
```

### TypeScript

Generated code includes:

- Branded types for IDs
- Readonly interfaces
- Zod validation schemas
- Union types for enums

```typescript
import { z } from 'zod';

// Branded ID type
export type CustomerId = string & { readonly __brand: 'CustomerId' };

// Entity interface
export interface Customer {
  readonly id: CustomerId;
  readonly name: string;
  readonly email: string;
  readonly address?: Address;
  readonly status: CustomerStatus;
}

// Value object interface
export interface Address {
  readonly street: string;
  readonly city: string;
  readonly postalCode: string;
}

// Enum as union type
export type CustomerStatus = 'Active' | 'Suspended' | 'Deleted';

// Zod schemas for runtime validation
export const CustomerStatusSchema = z.enum(['Active', 'Suspended', 'Deleted']);

export const AddressSchema = z.object({
  street: z.string(),
  city: z.string(),
  postalCode: z.string(),
});

export const CustomerSchema = z.object({
  id: z.string() as z.ZodType<CustomerId>,
  name: z.string(),
  email: z.string().email(),
  address: AddressSchema.optional(),
  status: CustomerStatusSchema,
});
```

### Kotlin

Generated code includes:

- Data classes with val properties
- Sealed classes for enums with payloads
- Inline value classes for IDs
- kotlinx.serialization annotations

```kotlin
import kotlinx.serialization.Serializable
import java.util.UUID

@JvmInline
value class CustomerId(val value: UUID)

@Serializable
data class Customer(
    val id: CustomerId,
    val name: String,
    val email: String,
    val address: Address? = null,
    val status: CustomerStatus
)

@Serializable
data class Address(
    val street: String,
    val city: String,
    val postalCode: String
)

enum class CustomerStatus {
    Active,
    Suspended,
    Deleted
}
```

### Python

Generated code includes:

- Dataclasses with frozen option
- NewType for ID types
- Pydantic models (optional)
- Enum classes

```python
from dataclasses import dataclass
from typing import Optional, NewType
from enum import Enum
from uuid import UUID

CustomerId = NewType('CustomerId', UUID)

class CustomerStatus(Enum):
    ACTIVE = "Active"
    SUSPENDED = "Suspended"
    DELETED = "Deleted"

@dataclass(frozen=True)
class Address:
    street: str
    city: str
    postal_code: str

@dataclass
class Customer:
    id: CustomerId
    name: str
    email: str
    status: CustomerStatus
    address: Optional[Address] = None
```

### Java

Generated code includes:

- Java 17 records
- Optional for nullable fields
- Enum classes
- Builder pattern (configurable)

```java
import java.util.UUID;
import java.util.Optional;

public record CustomerId(UUID value) {}

public record Customer(
    CustomerId id,
    String name,
    String email,
    Optional<Address> address,
    CustomerStatus status
) {}

public record Address(
    String street,
    String city,
    String postalCode
) {}

public enum CustomerStatus {
    ACTIVE,
    SUSPENDED,
    DELETED
}
```

### Clojure

Generated code includes:

- defrecord definitions
- clojure.spec specifications
- Constructor functions

```clojure
(ns domain.customer
  (:require [clojure.spec.alpha :as s]))

;; Specs
(s/def ::customer-id uuid?)
(s/def ::name string?)
(s/def ::email string?)
(s/def ::street string?)
(s/def ::city string?)
(s/def ::postal-code string?)

(s/def ::address
  (s/keys :req-un [::street ::city ::postal-code]))

(s/def ::customer-status #{:active :suspended :deleted})

(s/def ::customer
  (s/keys :req-un [::customer-id ::name ::email ::customer-status]
          :opt-un [::address]))

;; Records
(defrecord Address [street city postal-code])
(defrecord Customer [id name email address status])
```

### Haskell

Generated code includes:

- Newtype wrappers for IDs
- Data types with record syntax
- Deriving strategies
- Aeson instances for JSON

```haskell
{-# LANGUAGE DeriveGeneric #-}
{-# LANGUAGE DerivingStrategies #-}

module Domain.Customer where

import Data.Aeson (FromJSON, ToJSON)
import Data.Text (Text)
import Data.UUID (UUID)
import GHC.Generics (Generic)

newtype CustomerId = CustomerId { unCustomerId :: UUID }
  deriving stock (Eq, Show, Generic)
  deriving newtype (FromJSON, ToJSON)

data Address = Address
  { street :: Text
  , city :: Text
  , postalCode :: Text
  }
  deriving stock (Eq, Show, Generic)
  deriving anyclass (FromJSON, ToJSON)

data CustomerStatus
  = Active
  | Suspended
  | Deleted
  deriving stock (Eq, Show, Enum, Bounded, Generic)
  deriving anyclass (FromJSON, ToJSON)

data Customer = Customer
  { customerId :: CustomerId
  , name :: Text
  , email :: Text
  , address :: Maybe Address
  , status :: CustomerStatus
  }
  deriving stock (Eq, Show, Generic)
  deriving anyclass (FromJSON, ToJSON)
```

## Best Practices

### 1. Regenerate After Model Changes

```bash
# Add to your build process
sketchddd codegen model.sddd -t rust -o src/domain.rs
cargo fmt
```

### 2. Don't Edit Generated Code

Generated files should be treated as read-only. If you need customization:

1. Extend generated types in separate files
2. Use wrapper types
3. Customize code generation config

### 3. Version Control Generated Code

Include generated code in version control for:

- Easy code review
- No build-time dependency on SketchDDD
- Clear diff of changes

### 4. Use Type-Safe IDs

The generated ID types prevent mixing up IDs:

```rust
let customer_id: CustomerId = ...;
let order_id: OrderId = ...;

// Compile error: can't use CustomerId where OrderId expected
find_order(customer_id); // Error!
```
