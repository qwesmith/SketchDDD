# SketchDDD Visual Builder

A visual drag-and-drop interface for building domain models with SketchDDD.

## Development

```bash
# Install dependencies
npm install

# Start development server
npm run dev

# Run tests
npm test

# Build for production
npm run build
```

## Architecture

```
src/
├── components/
│   ├── canvas/       # React Flow canvas
│   ├── layout/       # Header, Sidebar
│   ├── nodes/        # Custom node types (Entity, Value, Enum, Aggregate)
│   ├── palette/      # Building blocks palette
│   ├── panels/       # Properties panel
│   └── wizards/      # Guided wizards
├── hooks/            # Custom React hooks
├── stores/           # Zustand state management
├── types/            # TypeScript type definitions
├── lib/              # Utility functions
└── wasm/             # WASM bindings
```

## Features

- **Visual Editor**: Drag-and-drop domain modeling with React Flow
- **Building Blocks**: Entity, Value Object, Enum, Aggregate
- **Properties Panel**: Edit node properties inline
- **Undo/Redo**: Full history support
- **WASM Integration**: Uses the Rust parser for validation
- **Dark Mode**: Automatic theme detection

## Tech Stack

- React 19 + TypeScript
- Vite
- Tailwind CSS v4
- Zustand (state management)
- React Flow (canvas)
- Lucide Icons
