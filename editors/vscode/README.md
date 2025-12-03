# SketchDDD for Visual Studio Code

Syntax highlighting and language support for SketchDDD (.sddd files).

## Features

- **Syntax Highlighting**: Full support for SketchDDD language constructs
- **Snippets**: Quick templates for common patterns
- **Bracket Matching**: Auto-closing and matching brackets
- **Comment Toggling**: Line and block comments

## Installation

### From VSIX (Local)

1. Build the extension:
   ```bash
   cd editors/vscode
   npm install
   npx vsce package
   ```

2. Install in VS Code:
   - Open VS Code
   - Press `Ctrl+Shift+P` / `Cmd+Shift+P`
   - Type "Install from VSIX"
   - Select the generated `.vsix` file

### From Marketplace (Coming Soon)

Search for "SketchDDD" in the VS Code Extensions marketplace.

## Snippets

| Prefix | Description |
|--------|-------------|
| `context` | Create a bounded context |
| `entity` | Create an entity |
| `value` | Create a value object |
| `enum` | Create a simple enum |
| `enumdata` | Create enum with data |
| `aggregate` | Create an aggregate |
| `morphisms` | Create morphisms block |
| `morph` | Create a single morphism |
| `map` | Create a context map |
| `field` | Create a field |
| `fieldopt` | Create optional field |
| `fieldlist` | Create list field |
| `money` | Money value template |
| `address` | Address value template |
| `ecommerce` | Full e-commerce context |

## Syntax Highlighting

The extension highlights:

- **Keywords**: `context`, `entity`, `value`, `enum`, `aggregate`, `morphisms`, `map`
- **Types**: Primitive types (`String`, `Int`, `UUID`, etc.) and custom types
- **Operators**: `->`, `=>`, `|`, `=`
- **Annotations**: `@one`, `@many`, etc.
- **Comments**: Line (`//`) and block (`/* */`)

## Example

```sddd
context Orders {
  entity Order {
    id: UUID
    customer: Customer
    total: Money
    status: OrderStatus
  }

  value Money {
    amount: Decimal
    currency: Currency
  }

  enum OrderStatus = Pending | Confirmed | Shipped | Delivered
  enum Currency = USD | EUR | GBP

  morphisms {
    customer: Order -> Customer @one
    items: Order -> List<LineItem> @many
  }

  aggregate Order {
    root: Order
    contains: [LineItem]
    invariant: total = sum(items.map(i => i.unitPrice * i.quantity))
  }
}
```

## Development

```bash
# Install dependencies
npm install

# Build
npm run build

# Package
npx vsce package
```

## Contributing

See the main [SketchDDD repository](https://github.com/ibrahimcesar/SketchDDD) for contribution guidelines.

## License

MIT
