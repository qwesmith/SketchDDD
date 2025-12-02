# WASM API Reference

Complete API documentation for the SketchDDD WebAssembly module.

## Initialization

### `init()`

Initialize the WASM module. Must be called before using any other functions.

```typescript
import init from 'sketchddd-wasm';

await init();
```

**Returns**: `Promise<void>`

## Parsing

### `parse(source: string): ParseResult`

Parse SketchDDD source code into a structured representation.

```typescript
const result = parse(`
context Orders {
  entity Order {
    id: UUID
    status: OrderStatus
  }
  enum OrderStatus = Pending | Confirmed
}
`);

if (result.success) {
  console.log(result.contexts);
  // [{ name: 'Orders', entities: [...], enums: [...] }]
}
```

**Parameters**:
- `source` - SketchDDD source code

**Returns**: `ParseResult`

```typescript
interface ParseResult {
  success: boolean;
  contexts?: ContextInfo[];
  error?: string;
}

interface ContextInfo {
  name: string;
  entities: EntityInfo[];
  value_objects: ValueObjectInfo[];
  aggregates: AggregateInfo[];
  enums: EnumInfo[];
  morphisms: MorphismInfo[];
}

interface EntityInfo {
  name: string;
  fields: FieldInfo[];
}

interface FieldInfo {
  name: string;
  field_type: string;
}

interface EnumInfo {
  name: string;
  variants: string[];
}

interface MorphismInfo {
  name: string;
  source: string;
  target: string;
}
```

## Validation

### `validate_source(source: string): ValidationResult`

Validate SketchDDD source code for syntax and semantic errors.

```typescript
const result = validate_source(`
context Orders {
  entity Order {
    id: UUID
    status: UnknownType  // Will produce error
  }
}
`);

if (!result.valid) {
  for (const error of result.errors) {
    console.error(`${error.file}:${error.line}: ${error.message}`);
  }
}
```

**Parameters**:
- `source` - SketchDDD source code

**Returns**: `ValidationResult`

```typescript
interface ValidationResult {
  valid: boolean;
  errors: ValidationError[];
  warnings: ValidationWarning[];
}

interface ValidationError {
  message: string;
  file: string;
  line: number;
  column: number;
}

interface ValidationWarning {
  message: string;
  file: string;
  line: number;
  column: number;
}
```

## Code Generation

### `generate_code(source: string, target: string): CodeGenResult`

Generate code from SketchDDD source for a target language.

```typescript
const result = generate_code(source, 'typescript');

if (result.success) {
  console.log(result.code);
  // export interface Order { ... }
}
```

**Parameters**:
- `source` - SketchDDD source code
- `target` - Target language (see `supported_targets()`)

**Returns**: `CodeGenResult`

```typescript
interface CodeGenResult {
  success: boolean;
  code?: string;
  error?: string;
}
```

### `supported_targets(): string[]`

Get list of supported code generation targets.

```typescript
const targets = supported_targets();
// ['rust', 'typescript', 'kotlin', 'python', 'java', 'clojure', 'haskell']
```

**Returns**: `string[]`

## Visualization

### `generate_viz(source: string, format: string): VizResult`

Generate a visualization diagram from SketchDDD source.

```typescript
// Generate Mermaid diagram
const mermaid = generate_viz(source, 'mermaid');
if (mermaid.success) {
  console.log(mermaid.diagram);
  // classDiagram
  //   class Order { ... }
}

// Generate Graphviz DOT
const dot = generate_viz(source, 'graphviz');
if (dot.success) {
  console.log(dot.diagram);
  // digraph { ... }
}
```

**Parameters**:
- `source` - SketchDDD source code
- `format` - Output format (see `supported_viz_formats()`)

**Returns**: `VizResult`

```typescript
interface VizResult {
  success: boolean;
  diagram?: string;
  error?: string;
}
```

### `supported_viz_formats(): string[]`

Get list of supported visualization formats.

```typescript
const formats = supported_viz_formats();
// ['mermaid', 'graphviz']
```

**Returns**: `string[]`

## Formatting

### `format_source(source: string): FormatResult`

Format SketchDDD source code.

```typescript
const result = format_source(`
context Orders{entity Order{id:UUID}}
`);

if (result.success) {
  console.log(result.formatted);
  // context Orders {
  //   entity Order {
  //     id: UUID
  //   }
  // }
}
```

**Parameters**:
- `source` - SketchDDD source code

**Returns**: `FormatResult`

```typescript
interface FormatResult {
  success: boolean;
  formatted?: string;
  error?: string;
}
```

## Complete Example

```typescript
import init, {
  parse,
  validate_source,
  generate_code,
  generate_viz,
  format_source,
  supported_targets,
  supported_viz_formats
} from 'sketchddd-wasm';

async function main() {
  // Initialize
  await init();

  const source = `
context Inventory {
  entity Product {
    id: UUID
    name: String
    price: Money
    stock: Int
  }

  value Money {
    amount: Decimal
    currency: Currency
  }

  enum Currency = USD | EUR | GBP
}
`;

  // 1. Parse
  const parseResult = parse(source);
  if (!parseResult.success) {
    console.error('Parse error:', parseResult.error);
    return;
  }
  console.log('Parsed contexts:', parseResult.contexts);

  // 2. Validate
  const validation = validate_source(source);
  if (!validation.valid) {
    for (const err of validation.errors) {
      console.error(`Error at line ${err.line}: ${err.message}`);
    }
    return;
  }
  console.log('Validation passed!');

  // 3. Format
  const formatted = format_source(source);
  if (formatted.success) {
    console.log('Formatted:\n', formatted.formatted);
  }

  // 4. Generate code for all targets
  console.log('\nSupported targets:', supported_targets());
  for (const target of supported_targets()) {
    const code = generate_code(source, target);
    if (code.success) {
      console.log(`\n=== ${target.toUpperCase()} ===`);
      console.log(code.code);
    }
  }

  // 5. Generate visualizations
  console.log('\nSupported viz formats:', supported_viz_formats());
  for (const format of supported_viz_formats()) {
    const viz = generate_viz(source, format);
    if (viz.success) {
      console.log(`\n=== ${format.toUpperCase()} ===`);
      console.log(viz.diagram);
    }
  }
}

main();
```

## Error Handling

All API functions return result objects with a `success` boolean. Check this before accessing result data:

```typescript
// Good
const result = generate_code(source, 'typescript');
if (result.success) {
  // Safe to use result.code
  console.log(result.code);
} else {
  // Handle error
  console.error(result.error);
}

// Bad - may throw if result.success is false
const code = generate_code(source, 'typescript').code;
```

## TypeScript Definitions

TypeScript definitions are included in the npm package:

```typescript
// All types are available
import type {
  ParseResult,
  ContextInfo,
  EntityInfo,
  ValidationResult,
  ValidationError,
  CodeGenResult,
  VizResult,
  FormatResult
} from 'sketchddd-wasm';
```

## Memory Management

The WASM module manages memory automatically. Returned objects are JavaScript objects, not WASM memory references, so no manual cleanup is needed.

```typescript
// Memory is automatically managed
const result1 = parse(source1);
const result2 = parse(source2);
// No need to free result1 or result2
```
