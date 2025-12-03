# Editor Support for SketchDDD

This directory contains syntax highlighting and language support for various editors.

## Supported Editors

| Editor | Directory | Features |
|--------|-----------|----------|
| VS Code | [vscode/](vscode/) | Syntax highlighting, snippets, bracket matching |
| Sublime Text | [sublime/](sublime/) | Syntax highlighting |
| Vim/Neovim | [vim/](vim/) | Syntax highlighting, indentation, filetype detection |

## Quick Installation

### VS Code

```bash
cd editors/vscode
npm install
npx vsce package
# Then install the .vsix file via VS Code
```

### Sublime Text

Copy `editors/sublime/SketchDDD.sublime-syntax` to your Packages directory.

### Vim/Neovim

Using vim-plug:
```vim
Plug 'ibrahimcesar/SketchDDD', { 'rtp': 'editors/vim' }
```

Or copy the files manually:
```bash
cp -r editors/vim/* ~/.vim/
```

## Features

All editor integrations provide:

- **Syntax Highlighting** for:
  - Keywords (`context`, `entity`, `value`, `enum`, `aggregate`, `morphisms`, `map`)
  - Types (primitive and custom)
  - Operators (`->`, `=>`, `|`, `=`)
  - Comments (line and block)
  - Strings
  - Numbers
  - Annotations (`@one`, `@many`, etc.)

- **File Association** for `.sddd` files

## TextMate Grammar

The VS Code extension includes a TextMate grammar (`sketchddd.tmLanguage.json`) that can be used with any editor supporting TextMate grammars:

- TextMate
- Atom
- VS Code
- IntelliJ IDEA (via plugin)
- And many others

## Contributing

To add support for another editor:

1. Create a new directory under `editors/`
2. Add syntax highlighting configuration
3. Add a README with installation instructions
4. Submit a pull request

## Language Server Protocol (LSP)

For advanced features like:
- Error diagnostics
- Auto-completion
- Go to definition
- Hover information
- Code formatting

See the LSP implementation in `crates/sketchddd-lsp/` (coming soon).
