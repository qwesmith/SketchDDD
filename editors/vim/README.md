# SketchDDD for Vim/Neovim

Syntax highlighting and indentation for SketchDDD (.sddd files) in Vim and Neovim.

## Installation

### vim-plug

```vim
Plug 'ibrahimcesar/SketchDDD', { 'rtp': 'editors/vim' }
```

### Vundle

```vim
Plugin 'ibrahimcesar/SketchDDD'
```

### Pathogen

```bash
cd ~/.vim/bundle
git clone https://github.com/ibrahimcesar/SketchDDD.git
```

### Manual Installation

Copy the files to your Vim configuration:

```bash
# For Vim
mkdir -p ~/.vim/syntax ~/.vim/ftdetect ~/.vim/indent
cp editors/vim/syntax/sketchddd.vim ~/.vim/syntax/
cp editors/vim/ftdetect/sketchddd.vim ~/.vim/ftdetect/
cp editors/vim/indent/sketchddd.vim ~/.vim/indent/

# For Neovim
mkdir -p ~/.config/nvim/syntax ~/.config/nvim/ftdetect ~/.config/nvim/indent
cp editors/vim/syntax/sketchddd.vim ~/.config/nvim/syntax/
cp editors/vim/ftdetect/sketchddd.vim ~/.config/nvim/ftdetect/
cp editors/vim/indent/sketchddd.vim ~/.config/nvim/indent/
```

### lazy.nvim (Neovim)

```lua
{
  "ibrahimcesar/SketchDDD",
  ft = "sketchddd",
  config = function()
    vim.filetype.add({
      extension = {
        sddd = "sketchddd",
      },
    })
  end,
}
```

## Features

- **Syntax Highlighting**: Full support for SketchDDD language
- **Auto-detection**: Automatically detects `.sddd` files
- **Indentation**: Smart indentation for nested blocks

## Highlighting Groups

| Element | Highlight Group |
|---------|-----------------|
| Keywords | `Keyword` |
| Types | `Type` |
| Fields | `Identifier` |
| Morphisms | `Function` |
| Comments | `Comment` |
| Strings | `String` |
| Numbers | `Number` |
| Operators | `Operator` |
| Annotations | `PreProc` |
| Enum variants | `Constant` |

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

  morphisms {
    customer: Order -> Customer @one
  }
}
```

## Configuration

### Custom Highlighting

You can customize highlighting in your `.vimrc` or `init.vim`:

```vim
" Example: Make keywords bold
hi sketchdddKeyword gui=bold cterm=bold

" Example: Custom colors
hi sketchdddType guifg=#569CD6 ctermfg=Blue
hi sketchdddField guifg=#9CDCFE ctermfg=Cyan
```

## License

MIT
