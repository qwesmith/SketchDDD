# Error Codes

This reference documents all error codes produced by SketchDDD.

## Error Format

Errors are displayed with:
- Error code (E0xxx)
- Error message
- Source location (file:line:column)
- Context and suggestions

Example:
```
error[E0001]: undefined type 'Customer'
  --> orders.sddd:5:12
   |
 5 |   customer: Customer
   |             ^^^^^^^^ type not found
   |
   = help: did you mean 'CustomerInfo'?
```

## Parse Errors (E01xx)

### E0100: Syntax Error

**Message**: Unexpected token

**Cause**: Invalid syntax in source file.

**Example**:
```sddd
context Orders {
  entity Order
    id: UUID    // Missing opening brace
  }
}
```

**Fix**: Check for missing braces, colons, or keywords.

---

### E0101: Unexpected End of File

**Message**: Unexpected end of input

**Cause**: File ends before a construct is complete.

**Example**:
```sddd
context Orders {
  entity Order {
    id: UUID
// File ends without closing braces
```

**Fix**: Ensure all braces and blocks are properly closed.

---

### E0102: Invalid Identifier

**Message**: Invalid identifier

**Cause**: Identifier contains invalid characters or starts incorrectly.

**Example**:
```sddd
entity 1Order { }  // Cannot start with digit
entity my-order { } // Hyphens not allowed
```

**Fix**: Use alphanumeric characters and underscores, starting with a letter.

---

### E0103: Invalid Type Name

**Message**: Type names must start with uppercase letter

**Cause**: Type name doesn't follow naming convention.

**Example**:
```sddd
entity order { }  // Should be Order
value money { }   // Should be Money
```

**Fix**: Start type names with an uppercase letter.

## Type Errors (E02xx)

### E0200: Undefined Type

**Message**: Undefined type '{name}'

**Cause**: Reference to a type that hasn't been declared.

**Example**:
```sddd
entity Order {
  customer: Customer  // Customer not defined
}
```

**Fix**: Declare the type or check spelling.

---

### E0201: Duplicate Type

**Message**: Type '{name}' is already defined

**Cause**: Two types with the same name in one context.

**Example**:
```sddd
context Orders {
  entity Order { }
  value Order { }  // Duplicate
}
```

**Fix**: Use unique names for each type.

---

### E0202: Invalid Generic Argument

**Message**: Invalid type argument for '{generic}'

**Cause**: Generic type used with incompatible argument.

**Example**:
```sddd
entity Order {
  items: Map<Order, String>  // Entities can't be map keys
}
```

**Fix**: Use appropriate types for generic arguments.

---

### E0203: Recursive Type

**Message**: Type '{name}' has infinite recursion

**Cause**: Type directly or indirectly references itself without indirection.

**Example**:
```sddd
value Node {
  children: List<Node>  // OK - List provides indirection
  self: Node            // Error - direct recursion
}
```

**Fix**: Use collections or optional types for self-references.

---

### E0204: Missing Required Field

**Message**: Entity '{name}' must have an 'id' field

**Cause**: Entity declared without identity field.

**Example**:
```sddd
entity Order {
  status: OrderStatus  // Missing id field
}
```

**Fix**: Add an `id: UUID` field to entities.

## Morphism Errors (E03xx)

### E0300: Invalid Morphism Source

**Message**: Morphism source '{type}' is not defined

**Cause**: Morphism references undefined source type.

**Example**:
```sddd
morphisms {
  customer: Order -> Customer  // Order not defined
}
```

**Fix**: Ensure source type is defined in the context.

---

### E0301: Invalid Morphism Target

**Message**: Morphism target '{type}' is not defined

**Cause**: Morphism references undefined target type.

**Example**:
```sddd
morphisms {
  status: Order -> OrderStatus  // OrderStatus not defined
}
```

**Fix**: Define the target type or check spelling.

---

### E0302: Duplicate Morphism

**Message**: Morphism '{name}' is already defined

**Cause**: Two morphisms with the same name.

**Example**:
```sddd
morphisms {
  customer: Order -> Customer
  customer: Order -> Person  // Duplicate name
}
```

**Fix**: Use unique names for morphisms.

---

### E0303: Conflicting Cardinality

**Message**: Morphism cardinality conflicts with field type

**Cause**: Morphism cardinality doesn't match the field definition.

**Example**:
```sddd
entity Order {
  items: List<LineItem>
}

morphisms {
  items: Order -> LineItem @one  // Should be @many
}
```

**Fix**: Ensure cardinality annotations match field types.

## Aggregate Errors (E04xx)

### E0400: Invalid Aggregate Root

**Message**: Aggregate root '{name}' is not an entity

**Cause**: Aggregate root references a non-entity type.

**Example**:
```sddd
value Money { }

aggregate Order {
  root: Money  // Must be an entity
}
```

**Fix**: Use an entity as the aggregate root.

---

### E0401: Invalid Contained Entity

**Message**: Contained type '{name}' is not an entity

**Cause**: Contains list includes non-entity types.

**Example**:
```sddd
enum Status = Active | Inactive

aggregate Order {
  root: Order
  contains: [Status]  // Must be entities
}
```

**Fix**: Only include entities in the contains list.

---

### E0402: Unreachable Contained Entity

**Message**: Contained entity '{name}' is not reachable from root

**Cause**: No morphism path from root to contained entity.

**Example**:
```sddd
aggregate Order {
  root: Order
  contains: [LineItem]  // No morphism Order -> LineItem
}
```

**Fix**: Add morphism connecting root to contained entities.

---

### E0403: Invalid Invariant Expression

**Message**: Invalid expression in invariant

**Cause**: Invariant contains invalid syntax or references.

**Example**:
```sddd
aggregate Order {
  root: Order
  invariant: total = undefined_field  // Field doesn't exist
}
```

**Fix**: Ensure invariant references valid fields and uses correct syntax.

## Context Map Errors (E05xx)

### E0500: Undefined Source Context

**Message**: Source context '{name}' is not defined

**Cause**: Context map references undefined source.

**Example**:
```sddd
map OrderToShipping: Orders -> Shipping {  // Orders not defined
  mappings { }
}
```

**Fix**: Define the source context first.

---

### E0501: Undefined Target Context

**Message**: Target context '{name}' is not defined

**Cause**: Context map references undefined target.

**Example**:
```sddd
context Orders { }

map OrderToShipping: Orders -> Shipping {  // Shipping not defined
  mappings { }
}
```

**Fix**: Define the target context first.

---

### E0502: Invalid Mapping Source

**Message**: Type '{name}' not found in source context

**Cause**: Mapping references type not in source context.

**Example**:
```sddd
map OrderToShipping: Orders -> Shipping {
  mappings {
    Product -> Package  // Product not in Orders context
  }
}
```

**Fix**: Only map types that exist in the source context.

---

### E0503: Invalid Mapping Target

**Message**: Type '{name}' not found in target context

**Cause**: Mapping references type not in target context.

**Example**:
```sddd
map OrderToShipping: Orders -> Shipping {
  mappings {
    Order -> Delivery  // Delivery not in Shipping context
  }
}
```

**Fix**: Only map to types that exist in the target context.

---

### E0504: Invalid Pattern

**Message**: Unknown integration pattern '{name}'

**Cause**: Unrecognized context map pattern.

**Example**:
```sddd
map OrderToShipping: Orders -> Shipping {
  pattern: UnknownPattern  // Not a valid pattern
}
```

**Fix**: Use one of: `CustomerSupplier`, `AntiCorruptionLayer`, `OpenHostService`, `Conformist`, `SharedKernel`, `Partnership`.

## Warnings (W0xxx)

### W0001: Unused Type

**Message**: Type '{name}' is defined but never used

**Cause**: Type declared but not referenced by any field or morphism.

**Fix**: Either use the type or remove it if unnecessary.

---

### W0002: Missing Morphism

**Message**: Field '{name}' could have an explicit morphism

**Cause**: Entity reference field without corresponding morphism.

**Fix**: Consider adding an explicit morphism for clarity.

---

### W0003: Large Aggregate

**Message**: Aggregate contains many entities; consider splitting

**Cause**: Aggregate contains more than 5 entities.

**Fix**: Consider breaking into smaller aggregates.
