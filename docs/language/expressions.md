# Expressions

Expressions in SketchDDD are used in invariants and equations to express business rules and constraints.

## Where Expressions Are Used

### Invariants

```sddd
aggregate Order {
  root: Order
  contains: [LineItem]
  invariant: total = sum(items.price)
  invariant: items.length > 0
}
```

### Equations

```sddd
equation discountCalculation: order.finalPrice = order.subtotal - order.discount
equation taxRule: order.tax = order.subtotal * order.taxRate
```

## Expression Types

### Literals

```sddd
// Numbers
invariant: quantity > 0
invariant: price >= 0.01
invariant: maxItems <= 100

// Strings (in comparisons)
invariant: status != "deleted"

// Boolean
invariant: isActive == true
```

### Path Expressions

Navigate through relationships:

```sddd
// Simple field access
invariant: order.total >= 0

// Nested navigation
invariant: order.customer.status == Active

// Through collections
equation totalCalc: order.total = sum(order.items.price)
```

### Arithmetic Operators

```sddd
invariant: total = subtotal + tax
invariant: discount = price * discountRate
invariant: perItemPrice = total / quantity
invariant: remainder = total % 100
```

| Operator | Meaning |
|----------|---------|
| `+` | Addition |
| `-` | Subtraction |
| `*` | Multiplication |
| `/` | Division |
| `%` | Modulo |

### Comparison Operators

```sddd
invariant: quantity > 0
invariant: price >= minimumPrice
invariant: stock < reorderLevel
invariant: status == Active
invariant: type != Deleted
```

| Operator | Meaning |
|----------|---------|
| `=` or `==` | Equal |
| `!=` | Not equal |
| `<` | Less than |
| `<=` | Less than or equal |
| `>` | Greater than |
| `>=` | Greater than or equal |

### Logical Operators

```sddd
// AND
invariant: isActive && hasValidPayment

// OR
invariant: status == Shipped || status == Delivered

// NOT
invariant: !isDeleted

// Combined
invariant: (status == Active && balance > 0) || isVIP
```

| Operator | Meaning |
|----------|---------|
| `&&` | Logical AND |
| `\|\|` | Logical OR |
| `!` | Logical NOT |

### Collection Functions

#### sum

Sum numeric values in a collection:

```sddd
invariant: total = sum(items.price)
invariant: totalWeight = sum(items.weight)
```

#### count / length

Count elements:

```sddd
invariant: items.length > 0
invariant: items.length <= maxItems
```

#### all

Check if all elements satisfy a condition:

```sddd
invariant: items.all(i => i.quantity > 0)
invariant: payments.all(p => p.status == Confirmed)
```

#### any

Check if any element satisfies a condition:

```sddd
invariant: items.any(i => i.type == Physical) || shippingNotRequired
invariant: discounts.any(d => d.isApplicable)
```

#### filter

Filter elements:

```sddd
equation activeItems: order.activeItems = order.items.filter(i => i.status == Active)
```

#### map

Transform elements:

```sddd
equation itemPrices: order.prices = order.items.map(i => i.price)
```

## Complex Expressions

### Nested Conditions

```sddd
invariant: (status == Pending && createdAt > yesterday) ||
           (status == Processing && assignedTo != null) ||
           status == Completed
```

### Calculation Chains

```sddd
equation pricing:
  order.finalPrice = order.subtotal
                   - order.discount
                   + order.tax
                   + order.shippingCost
```

### Collection Aggregations

```sddd
aggregate Order {
  root: Order
  contains: [LineItem]

  // Total is sum of item subtotals
  invariant: total = sum(items.map(i => i.quantity * i.unitPrice))

  // All items must have positive quantities
  invariant: items.all(i => i.quantity > 0)

  // At least one item required
  invariant: items.length >= 1

  // Max items limit
  invariant: items.length <= 50
}
```

## Null/Optional Handling

### Null Checks

```sddd
// Check if optional value exists
invariant: discount != null || price == fullPrice

// Safe navigation
invariant: customer.address != null && customer.address.country == "US"
```

### Default Values

```sddd
// If discount is null, treat as 0
equation finalPrice: order.final = order.subtotal - (order.discount ?? 0)
```

## Type Coercion

Expressions generally require compatible types:

```sddd
// Good: Comparing numbers
invariant: total >= 0

// Good: Comparing enums
invariant: status == Active

// Error: Type mismatch
invariant: total == "100"  // Number vs String
```

## Best Practices

### 1. Keep Invariants Focused

```sddd
// Good: Single responsibility
invariant: total = sum(items.price)
invariant: items.length > 0
invariant: items.all(i => i.quantity > 0)

// Avoid: Too complex
invariant: total = sum(items.price) && items.length > 0 && items.all(i => i.quantity > 0) && status != Cancelled
```

### 2. Use Meaningful Equation Names

```sddd
// Good: Describes the business rule
equation orderTotal: order.total = sum(order.items.price)
equation loyaltyDiscount: order.discount = order.customer.loyaltyPoints * 0.01

// Avoid: Generic names
equation eq1: order.total = sum(order.items.price)
```

### 3. Express Real Business Rules

```sddd
// Good: Actual business constraint
invariant: balance >= 0 || accountType == Overdraft

// Avoid: Technical constraint without business meaning
invariant: field1 != null
```

### 4. Document Complex Expressions

```sddd
aggregate Order {
  root: Order

  // Business rule: Orders must have at least one item
  invariant: items.length > 0

  // Business rule: Total must match item sum (no hidden charges)
  invariant: total = sum(items.price)

  // Business rule: VIP customers can exceed normal limits
  invariant: items.length <= 100 || customer.isVIP
}
```

## Validation

SketchDDD validates expressions for:

- Type correctness
- Valid field references
- Valid function calls
- Proper operator usage

```bash
sketchddd check model.sddd
```

Example errors:

```
error[E0301]: Unknown field 'totl' on type 'Order'
  --> model.sddd:15:15
   |
15 |   invariant: totl >= 0
   |              ^^^^ Did you mean 'total'?

error[E0302]: Type mismatch in comparison
  --> model.sddd:16:15
   |
16 |   invariant: total == "100"
   |              ^^^^^^^^^^^^^^ Cannot compare Decimal with String
```
