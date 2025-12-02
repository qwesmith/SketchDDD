# Language Specification

This is the formal specification for the SketchDDD language.

## Lexical Structure

### Comments

```sddd
// Single-line comment

/* Multi-line
   comment */
```

### Identifiers

```
identifier     = letter (letter | digit | "_")*
type_name      = upper_letter (letter | digit)*
field_name     = lower_letter (letter | digit | "_")*
```

Examples:
- Type names: `Order`, `OrderStatus`, `UUID`
- Field names: `orderId`, `created_at`, `totalAmount`

### Keywords

Reserved words:

```
context  entity  value  enum  aggregate
morphisms  map  pattern  mappings
root  contains  invariant
List  Map  Set
```

### Operators

```
->    // Morphism arrow
=>    // Lambda/mapping
=     // Assignment
:     // Type annotation
|     // Enum variant separator
?     // Optional type
```

### Literals

```
string_literal = '"' characters '"'
int_literal    = digit+
float_literal  = digit+ "." digit+
bool_literal   = "true" | "false"
```

## Grammar

### Top-Level Declarations

```ebnf
program = (context | map)*

context = "context" identifier "{" context_body "}"

map = "map" identifier ":" identifier "->" identifier "{" map_body "}"
```

### Context Body

```ebnf
context_body = (entity | value | enum | aggregate | morphisms)*

entity = "entity" identifier "{" field_list "}"

value = "value" identifier "{" field_list "}"

enum = "enum" identifier "=" variant_list
     | "enum" identifier "{" variant_def_list "}"

aggregate = "aggregate" identifier "{" aggregate_body "}"

morphisms = "morphisms" "{" morphism_list "}"
```

### Fields

```ebnf
field_list = (field ("," field)* ","?)?

field = identifier ":" type

type = primitive_type
     | identifier
     | generic_type
     | optional_type

primitive_type = "String" | "Int" | "Float" | "Bool"
               | "UUID" | "DateTime" | "Date" | "Decimal" | "Email"

generic_type = "List" "<" type ">"
             | "Map" "<" type "," type ">"
             | "Set" "<" type ">"

optional_type = type "?"
```

### Enums

```ebnf
variant_list = identifier ("|" identifier)*

variant_def_list = variant_def ("," variant_def)* ","?

variant_def = identifier
            | identifier "{" field_list "}"
```

### Aggregates

```ebnf
aggregate_body = root_decl contains_decl? invariant_decl*

root_decl = "root" ":" identifier

contains_decl = "contains" ":" "[" identifier_list "]"

invariant_decl = "invariant" ":" expression

identifier_list = identifier ("," identifier)*
```

### Morphisms

```ebnf
morphism_list = (morphism ("," morphism)* ","?)?

morphism = identifier ":" identifier "->" type annotation*

annotation = "@" identifier ("(" annotation_args ")")?

annotation_args = annotation_arg ("," annotation_arg)*

annotation_arg = identifier "=" literal
```

### Context Maps

```ebnf
map_body = pattern_decl? mappings_block

pattern_decl = "pattern" ":" map_pattern

map_pattern = "CustomerSupplier"
            | "AntiCorruptionLayer"
            | "OpenHostService"
            | "Conformist"
            | "SharedKernel"
            | "Partnership"

mappings_block = "mappings" "{" mapping_list "}"

mapping_list = (mapping ("," mapping)* ","?)?

mapping = identifier "->" identifier
```

### Expressions

```ebnf
expression = primary
           | expression binary_op expression
           | unary_op expression
           | expression "." identifier
           | expression "." method_call

primary = identifier
        | literal
        | "(" expression ")"
        | method_call

method_call = identifier "(" arg_list? ")"

arg_list = expression ("," expression)*

binary_op = "+" | "-" | "*" | "/" | "%"
          | "==" | "!=" | "<" | ">" | "<=" | ">="
          | "&&" | "||"

unary_op = "!" | "-"
```

## Semantic Rules

### Scope Rules

1. Bounded contexts define a namespace
2. Types within a context are visible to all declarations in that context
3. Type references must resolve to a declared type
4. Morphism sources and targets must be declared types

### Type Rules

1. Entity fields can reference:
   - Primitive types
   - Value objects
   - Other entities
   - Enums
   - Generic types containing the above

2. Value object fields can reference:
   - Primitive types
   - Other value objects
   - Enums
   - Generic types containing the above

3. Morphism types must match:
   - Source: declared entity or value
   - Target: declared type in the context

### Aggregate Rules

1. Root must be a declared entity
2. Contains must list declared entities
3. Invariants must be boolean expressions
4. Invariant expressions can only reference:
   - Root entity fields
   - Contained entity fields via morphisms

### Context Map Rules

1. Source and target contexts must be declared
2. Mappings must reference types in respective contexts
3. Pattern annotations are optional but validated if present

## Type System

### Built-in Types

| Type | Description |
|------|-------------|
| `String` | UTF-8 text |
| `Int` | 64-bit signed integer |
| `Float` | 64-bit floating point |
| `Bool` | Boolean (true/false) |
| `UUID` | Universally unique identifier |
| `DateTime` | Date and time with timezone |
| `Date` | Calendar date |
| `Decimal` | Arbitrary precision decimal |
| `Email` | Email address string |

### Generic Types

| Type | Description |
|------|-------------|
| `List<T>` | Ordered collection |
| `Map<K, V>` | Key-value mapping |
| `Set<T>` | Unique collection |
| `T?` | Optional (nullable) |

### User-Defined Types

- Entities: Products with identity
- Values: Products without identity
- Enums: Coproducts (sum types)

## File Format

### Extension

SketchDDD files use the `.sddd` extension.

### Encoding

Files must be UTF-8 encoded.

### Structure

A typical file structure:

```sddd
// Optional file-level comments

context ContextName {
  // Type declarations
  entity EntityName { ... }
  value ValueName { ... }
  enum EnumName = ...

  // Relationships
  morphisms { ... }

  // Aggregates
  aggregate Name { ... }
}

// Context maps
map Name: Source -> Target { ... }
```

## Error Codes

See [Error Codes](errors.md) for a complete list of error codes and their meanings.
