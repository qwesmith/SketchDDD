# SketchDDD for Sublime Text

Syntax highlighting for SketchDDD (.sddd files) in Sublime Text.

## Installation

### Package Control (Recommended)

Coming soon to Package Control.

### Manual Installation

1. Open Sublime Text
2. Go to `Preferences` → `Browse Packages...`
3. Create a folder named `SketchDDD`
4. Copy `SketchDDD.sublime-syntax` into the folder

### Via Git

```bash
cd ~/Library/Application\ Support/Sublime\ Text/Packages/  # macOS
# cd ~/.config/sublime-text/Packages/  # Linux
# cd %APPDATA%\Sublime Text\Packages\  # Windows

git clone https://github.com/ibrahimcesar/SketchDDD.git
# Or just copy the editors/sublime folder
```

## Features

- Full syntax highlighting for SketchDDD language
- Support for all language constructs:
  - Bounded contexts
  - Entities and value objects
  - Enums (simple and with data)
  - Aggregates
  - Morphisms
  - Context maps

## Color Scheme

The syntax highlighting uses standard Sublime Text scopes, so it works with any color scheme. Key scopes used:

| Element | Scope |
|---------|-------|
| Keywords | `keyword.control` |
| Types | `entity.name.type`, `support.type` |
| Fields | `variable.other.property` |
| Comments | `comment` |
| Strings | `string.quoted` |
| Operators | `keyword.operator` |

## Example

```sddd
context Orders {
  entity Order {
    id: UUID
    total: Money
    status: OrderStatus
  }

  value Money {
    amount: Decimal
    currency: Currency
  }

  enum OrderStatus = Pending | Confirmed | Shipped
  enum Currency = USD | EUR | GBP
}
```

## License

MIT
