# Built-in Types Reference

Complete reference for all built-in types in SketchDDD.

## Primitive Types

### String

UTF-8 encoded text of arbitrary length.

```sddd
entity User {
  name: String
  bio: String
}
```

**Code Generation**:
| Target | Type |
|--------|------|
| Rust | `String` |
| TypeScript | `string` |
| Kotlin | `String` |
| Python | `str` |
| Java | `String` |
| Clojure | `string?` (spec) |
| Haskell | `Text` |

---

### Int

64-bit signed integer.

```sddd
entity Product {
  quantity: Int
  sortOrder: Int
}
```

**Range**: -9,223,372,036,854,775,808 to 9,223,372,036,854,775,807

**Code Generation**:
| Target | Type |
|--------|------|
| Rust | `i64` |
| TypeScript | `number` |
| Kotlin | `Long` |
| Python | `int` |
| Java | `long` |
| Clojure | `int?` (spec) |
| Haskell | `Int64` |

---

### Float

64-bit IEEE 754 floating-point number.

```sddd
entity Measurement {
  value: Float
  precision: Float
}
```

**Code Generation**:
| Target | Type |
|--------|------|
| Rust | `f64` |
| TypeScript | `number` |
| Kotlin | `Double` |
| Python | `float` |
| Java | `double` |
| Clojure | `float?` (spec) |
| Haskell | `Double` |

---

### Bool

Boolean value (true or false).

```sddd
entity Feature {
  isEnabled: Bool
  isPublic: Bool
}
```

**Code Generation**:
| Target | Type |
|--------|------|
| Rust | `bool` |
| TypeScript | `boolean` |
| Kotlin | `Boolean` |
| Python | `bool` |
| Java | `boolean` |
| Clojure | `boolean?` (spec) |
| Haskell | `Bool` |

---

### UUID

Universally Unique Identifier (128-bit).

```sddd
entity Order {
  id: UUID
  correlationId: UUID
}
```

**Format**: `xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx`

**Code Generation**:
| Target | Type |
|--------|------|
| Rust | `Uuid` (uuid crate) |
| TypeScript | `string` |
| Kotlin | `UUID` |
| Python | `UUID` |
| Java | `UUID` |
| Clojure | `uuid?` (spec) |
| Haskell | `UUID` |

---

### DateTime

Date and time with timezone information.

```sddd
entity Event {
  occurredAt: DateTime
  scheduledFor: DateTime
}
```

**Format**: ISO 8601 (e.g., `2024-01-15T10:30:00Z`)

**Code Generation**:
| Target | Type |
|--------|------|
| Rust | `DateTime<Utc>` (chrono) |
| TypeScript | `Date` |
| Kotlin | `Instant` |
| Python | `datetime` |
| Java | `Instant` |
| Clojure | `inst?` (spec) |
| Haskell | `UTCTime` |

---

### Date

Calendar date without time.

```sddd
entity Reservation {
  checkIn: Date
  checkOut: Date
}
```

**Format**: ISO 8601 (e.g., `2024-01-15`)

**Code Generation**:
| Target | Type |
|--------|------|
| Rust | `NaiveDate` (chrono) |
| TypeScript | `string` |
| Kotlin | `LocalDate` |
| Python | `date` |
| Java | `LocalDate` |
| Clojure | `inst?` (spec) |
| Haskell | `Day` |

---

### Decimal

Arbitrary-precision decimal number.

```sddd
value Money {
  amount: Decimal
}

entity Invoice {
  taxRate: Decimal
}
```

**Use for**: Financial calculations, precise measurements.

**Code Generation**:
| Target | Type |
|--------|------|
| Rust | `Decimal` (rust_decimal) |
| TypeScript | `number` |
| Kotlin | `BigDecimal` |
| Python | `Decimal` |
| Java | `BigDecimal` |
| Clojure | `decimal?` (spec) |
| Haskell | `Scientific` |

---

### Email

Email address string.

```sddd
entity User {
  email: Email
  backupEmail: Email?
}
```

**Validation**: Must be a valid email format.

**Code Generation**:
| Target | Type |
|--------|------|
| Rust | `String` |
| TypeScript | `string` |
| Kotlin | `String` |
| Python | `str` |
| Java | `String` |
| Clojure | `string?` with email predicate |
| Haskell | `Text` |

**Note**: Some generators add validation annotations/decorators.

## Generic Types

### List<T>

Ordered collection of elements.

```sddd
entity Order {
  items: List<LineItem>
  tags: List<String>
}
```

**Code Generation**:
| Target | Type |
|--------|------|
| Rust | `Vec<T>` |
| TypeScript | `T[]` |
| Kotlin | `List<T>` |
| Python | `list[T]` |
| Java | `List<T>` |
| Clojure | `(s/coll-of T)` |
| Haskell | `[T]` |

---

### Map<K, V>

Key-value mapping.

```sddd
entity Settings {
  preferences: Map<String, String>
  permissions: Map<String, Bool>
}
```

**Constraints**: Keys should be primitive types or value objects.

**Code Generation**:
| Target | Type |
|--------|------|
| Rust | `HashMap<K, V>` |
| TypeScript | `Map<K, V>` |
| Kotlin | `Map<K, V>` |
| Python | `dict[K, V]` |
| Java | `Map<K, V>` |
| Clojure | `(s/map-of K V)` |
| Haskell | `Map K V` |

---

### Set<T>

Unordered collection of unique elements.

```sddd
entity Article {
  categories: Set<Category>
  readers: Set<UUID>
}
```

**Code Generation**:
| Target | Type |
|--------|------|
| Rust | `HashSet<T>` |
| TypeScript | `Set<T>` |
| Kotlin | `Set<T>` |
| Python | `set[T]` |
| Java | `Set<T>` |
| Clojure | `(s/coll-of T :kind set?)` |
| Haskell | `Set T` |

---

### Optional (T?)

Nullable or absent value.

```sddd
entity User {
  middleName: String?
  deletedAt: DateTime?
}
```

**Code Generation**:
| Target | Type |
|--------|------|
| Rust | `Option<T>` |
| TypeScript | `T \| null` |
| Kotlin | `T?` |
| Python | `T \| None` |
| Java | `Optional<T>` |
| Clojure | `(s/nilable T)` |
| Haskell | `Maybe T` |

## Nested Generics

Generic types can be nested:

```sddd
entity Warehouse {
  inventory: Map<String, List<Product>>
  optionalItems: List<Product?>
  categories: Set<List<String>>?
}
```

## Type Aliases

Types can be referenced by name:

```sddd
entity Order {
  items: List<LineItem>  // LineItem is a custom type
  status: OrderStatus    // Enum reference
  address: Address       // Value object reference
}
```

## Type Resolution

Types are resolved in this order:

1. Built-in primitive types
2. Built-in generic types
3. User-defined types in the same context
4. (Future) Imported types from other files

## Best Practices

### Use Appropriate Types

```sddd
// Good
entity Invoice {
  amount: Decimal      // Precise for money
  taxRate: Decimal
  createdAt: DateTime  // Full timestamp
  dueDate: Date        // Date only
}

// Avoid
entity Invoice {
  amount: Float        // Imprecise for money
  taxRate: String      // Should be numeric
}
```

### Use Optional for Truly Optional Data

```sddd
// Good
entity User {
  email: Email           // Required
  phone: String?         // Optional
  deletedAt: DateTime?   // Null means not deleted
}

// Avoid using empty strings or sentinel values
```

### Use Specific Types

```sddd
// Good
entity User {
  id: UUID
  email: Email
  createdAt: DateTime
}

// Less specific
entity User {
  id: String     // Could be anything
  email: String  // No format hint
  createdAt: String  // No structure
}
```
