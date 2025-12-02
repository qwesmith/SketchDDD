# Enums

Enums (enumerations) represent a fixed set of possible values. In SketchDDD, enums are modeled as sum types using category theory colimits.

## Basic Enums

Define simple enums with the `enum` keyword:

```sddd
context Commerce {
  enum OrderStatus = Pending | Confirmed | Shipped | Delivered | Cancelled

  enum PaymentMethod = CreditCard | DebitCard | PayPal | BankTransfer | Cash

  enum Priority = Low | Medium | High | Critical
}
```

## Enum with Payloads

Enums can carry associated data (sum types / discriminated unions):

```sddd
context Notifications {
  enum NotificationChannel =
    | Email(address: Email)
    | SMS(phoneNumber: String)
    | Push(deviceToken: String)
    | Slack(channel: String, workspace: String)
}

context Payments {
  enum PaymentResult =
    | Success(transactionId: String, amount: Money)
    | Failure(errorCode: String, message: String)
    | Pending(retryAfter: DateTime)
}
```

## Using Enums

### In Entity Fields

```sddd
entity Order {
  id: UUID
  status: OrderStatus
  priority: Priority
}
```

### In Morphisms

```sddd
morphisms {
  status: Order -> OrderStatus
  preferredChannel: Customer -> NotificationChannel
}
```

### In Value Objects

```sddd
value Shipment {
  carrier: ShippingCarrier
  trackingNumber: String
  status: ShipmentStatus
}

enum ShippingCarrier = FedEx | UPS | USPS | DHL
enum ShipmentStatus = Preparing | InTransit | OutForDelivery | Delivered
```

## Generated Code

### Simple Enums

=== "Rust"
    ```rust
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum OrderStatus {
        Pending,
        Confirmed,
        Shipped,
        Delivered,
        Cancelled,
    }
    ```

=== "TypeScript"
    ```typescript
    export type OrderStatus =
      | 'Pending'
      | 'Confirmed'
      | 'Shipped'
      | 'Delivered'
      | 'Cancelled';

    export const OrderStatus = {
      Pending: 'Pending',
      Confirmed: 'Confirmed',
      Shipped: 'Shipped',
      Delivered: 'Delivered',
      Cancelled: 'Cancelled',
    } as const;
    ```

=== "Kotlin"
    ```kotlin
    enum class OrderStatus {
        Pending,
        Confirmed,
        Shipped,
        Delivered,
        Cancelled
    }
    ```

=== "Python"
    ```python
    from enum import Enum

    class OrderStatus(Enum):
        PENDING = "Pending"
        CONFIRMED = "Confirmed"
        SHIPPED = "Shipped"
        DELIVERED = "Delivered"
        CANCELLED = "Cancelled"
    ```

### Enums with Payloads

=== "Rust"
    ```rust
    #[derive(Debug, Clone, PartialEq)]
    pub enum NotificationChannel {
        Email { address: String },
        SMS { phone_number: String },
        Push { device_token: String },
        Slack { channel: String, workspace: String },
    }
    ```

=== "TypeScript"
    ```typescript
    export type NotificationChannel =
      | { type: 'Email'; address: string }
      | { type: 'SMS'; phoneNumber: string }
      | { type: 'Push'; deviceToken: string }
      | { type: 'Slack'; channel: string; workspace: string };
    ```

=== "Kotlin"
    ```kotlin
    sealed class NotificationChannel {
        data class Email(val address: String) : NotificationChannel()
        data class SMS(val phoneNumber: String) : NotificationChannel()
        data class Push(val deviceToken: String) : NotificationChannel()
        data class Slack(val channel: String, val workspace: String) : NotificationChannel()
    }
    ```

=== "Haskell"
    ```haskell
    data NotificationChannel
      = Email { address :: String }
      | SMS { phoneNumber :: String }
      | Push { deviceToken :: String }
      | Slack { channel :: String, workspace :: String }
      deriving (Eq, Show)
    ```

## Common Patterns

### State Machines

Model state transitions with enums:

```sddd
context Orders {
  enum OrderState =
    | Draft
    | Submitted
    | PaymentPending
    | PaymentReceived
    | Processing
    | Shipped
    | Delivered
    | Cancelled
    | Refunded

  entity Order {
    id: UUID
    state: OrderState
  }

  // State transitions can be modeled as morphisms or methods
}
```

### Result Types

Model success/failure scenarios:

```sddd
context Operations {
  enum Result<T, E> =
    | Ok(value: T)
    | Err(error: E)

  // Specific result types
  enum CreateUserResult =
    | Success(userId: UUID)
    | EmailTaken(existingUserId: UUID)
    | InvalidEmail(reason: String)
    | WeakPassword(suggestions: List<String>)
}
```

### Option/Maybe Types

Model optional values:

```sddd
context Core {
  enum Maybe<T> =
    | Just(value: T)
    | Nothing
}
```

## Best Practices

### 1. Use Descriptive Variant Names

```sddd
// Good: Clear meaning
enum OrderStatus = Pending | Confirmed | Shipped | Delivered | Cancelled

// Avoid: Unclear abbreviations
enum OrderStatus = P | C | S | D | X
```

### 2. Model All Valid States

```sddd
// Good: Complete set of states
enum PaymentStatus =
  | Pending
  | Processing
  | Succeeded
  | Failed
  | Refunded
  | Disputed
  | Cancelled

// Avoid: Incomplete (missing edge cases)
enum PaymentStatus = Pending | Success | Failed
```

### 3. Use Payloads When Variants Need Data

```sddd
// Good: Data associated with variants
enum ShippingMethod =
  | Standard(estimatedDays: Int)
  | Express(estimatedDays: Int, carrier: String)
  | Pickup(location: Address)

// Not as good: Separate fields that only apply to some states
value Shipping {
  method: ShippingMethodType
  estimatedDays: Int?      // Only for Standard/Express
  carrier: String?          // Only for Express
  pickupLocation: Address?  // Only for Pickup
}
```

### 4. Consider State Machine Validity

```sddd
// Design enums that represent valid states
enum DocumentStatus =
  | Draft
  | UnderReview
  | Approved
  | Published
  | Archived

// Invalid transitions should be prevented by your domain logic
// e.g., Draft -> Published is invalid (must go through review)
```

## Category Theory: Enums as Colimits

In category theory, an enum is a **colimit (coproduct)**:

```
Variant1  Variant2  Variant3
    \       |       /
     \      |      /
      \     |     /
       EnumType
```

Each variant is an injection into the colimit. This structure ensures:

- **Exhaustiveness**: All cases are known
- **Disjointness**: A value is exactly one variant
- **Pattern matching**: You can decompose by variant

The colimit property guarantees that any function on the enum can be defined by specifying behavior for each variant (the universal property of coproducts).
