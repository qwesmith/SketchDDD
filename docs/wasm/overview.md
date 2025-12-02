# WebAssembly Overview

SketchDDD compiles to WebAssembly, enabling use in browsers and JavaScript/TypeScript applications.

## Features

- **Parse** SketchDDD source in the browser
- **Validate** domain models client-side
- **Generate code** for multiple targets
- **Generate visualizations** as Mermaid or Graphviz
- **Format** SketchDDD source code

## Installation

### npm/yarn

```bash
npm install sketchddd-wasm
# or
yarn add sketchddd-wasm
```

### CDN

```html
<script type="module">
  import init, { parse, validate_source, generate_code } from 'https://unpkg.com/sketchddd-wasm@latest';

  await init();
  // Use the API
</script>
```

## Quick Start

```javascript
import init, {
  parse,
  validate_source,
  generate_code,
  generate_viz,
  format_source,
  supported_targets,
  supported_viz_formats
} from 'sketchddd-wasm';

// Initialize WASM module
await init();

// Define a SketchDDD model
const source = `
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
  enum Currency = USD | EUR
}
`;

// Parse the source
const parseResult = parse(source);
if (parseResult.success) {
  console.log('Contexts:', parseResult.contexts);
}

// Validate the model
const validation = validate_source(source);
if (validation.valid) {
  console.log('Model is valid!');
} else {
  console.error('Validation errors:', validation.errors);
}

// Generate TypeScript code
const codeResult = generate_code(source, 'typescript');
if (codeResult.success) {
  console.log(codeResult.code);
}

// Generate Mermaid diagram
const vizResult = generate_viz(source, 'mermaid');
if (vizResult.success) {
  console.log(vizResult.diagram);
}
```

## API Reference

See the [API Reference](api.md) for detailed documentation of all functions.

## Use Cases

### Online Editor

Build a browser-based SketchDDD editor with real-time validation:

```javascript
const editor = document.getElementById('editor');
const output = document.getElementById('output');
const errors = document.getElementById('errors');

editor.addEventListener('input', () => {
  const source = editor.value;

  // Validate in real-time
  const result = validate_source(source);

  if (result.valid) {
    errors.textContent = '';
    // Generate preview
    const code = generate_code(source, 'typescript');
    output.textContent = code.success ? code.code : '';
  } else {
    errors.innerHTML = result.errors
      .map(e => `<div class="error">${e.message}</div>`)
      .join('');
  }
});
```

### Documentation Generator

Generate visual documentation from domain models:

```javascript
async function generateDocs(source) {
  const mermaid = generate_viz(source, 'mermaid');
  const typescript = generate_code(source, 'typescript');
  const rust = generate_code(source, 'rust');

  return {
    diagram: mermaid.success ? mermaid.diagram : null,
    typescript: typescript.success ? typescript.code : null,
    rust: rust.success ? rust.code : null
  };
}
```

### VS Code Extension

Power language features in an editor extension:

```typescript
import * as vscode from 'vscode';
import init, { validate_source, format_source } from 'sketchddd-wasm';

export async function activate(context: vscode.ExtensionContext) {
  await init();

  // Register diagnostics provider
  const diagnostics = vscode.languages.createDiagnosticCollection('sketchddd');

  vscode.workspace.onDidChangeTextDocument(event => {
    if (event.document.languageId === 'sketchddd') {
      const result = validate_source(event.document.getText());
      diagnostics.set(
        event.document.uri,
        result.errors.map(e => new vscode.Diagnostic(
          new vscode.Range(e.line - 1, e.column, e.line - 1, e.column + 10),
          e.message,
          vscode.DiagnosticSeverity.Error
        ))
      );
    }
  });

  // Register formatter
  vscode.languages.registerDocumentFormattingEditProvider('sketchddd', {
    provideDocumentFormattingEdits(document) {
      const result = format_source(document.getText());
      if (result.success) {
        return [vscode.TextEdit.replace(
          new vscode.Range(0, 0, document.lineCount, 0),
          result.formatted
        )];
      }
      return [];
    }
  });
}
```

## Framework Integration

### React

```tsx
import { useEffect, useState } from 'react';
import init, { parse, validate_source } from 'sketchddd-wasm';

function useSketchDDD() {
  const [ready, setReady] = useState(false);

  useEffect(() => {
    init().then(() => setReady(true));
  }, []);

  return { ready, parse, validate_source };
}

function Editor() {
  const { ready, validate_source } = useSketchDDD();
  const [source, setSource] = useState('');
  const [errors, setErrors] = useState([]);

  useEffect(() => {
    if (ready && source) {
      const result = validate_source(source);
      setErrors(result.errors || []);
    }
  }, [ready, source]);

  if (!ready) return <div>Loading...</div>;

  return (
    <div>
      <textarea value={source} onChange={e => setSource(e.target.value)} />
      {errors.map((e, i) => <div key={i} className="error">{e.message}</div>)}
    </div>
  );
}
```

### Vue

```vue
<script setup>
import { ref, onMounted, watch } from 'vue';
import init, { validate_source, generate_code } from 'sketchddd-wasm';

const ready = ref(false);
const source = ref('');
const errors = ref([]);
const output = ref('');

onMounted(async () => {
  await init();
  ready.value = true;
});

watch(source, (newSource) => {
  if (!ready.value) return;

  const result = validate_source(newSource);
  errors.value = result.errors || [];

  if (result.valid) {
    const code = generate_code(newSource, 'typescript');
    output.value = code.success ? code.code : '';
  }
});
</script>

<template>
  <div v-if="!ready">Loading...</div>
  <div v-else>
    <textarea v-model="source" />
    <div v-for="error in errors" class="error">{{ error.message }}</div>
    <pre>{{ output }}</pre>
  </div>
</template>
```

## Performance

The WASM module is optimized for performance:

- **Size**: ~1.3MB (uncompressed), ~400KB (gzip)
- **Init time**: <100ms on modern hardware
- **Parse time**: <10ms for typical models
- **Memory**: Efficient Rust memory management

### Tips

1. **Initialize once**: Call `init()` once at app startup
2. **Debounce validation**: For real-time editors, debounce input
3. **Web Workers**: Run in a worker for large models

```javascript
// worker.js
import init, { validate_source } from 'sketchddd-wasm';

let initialized = false;

self.onmessage = async (e) => {
  if (!initialized) {
    await init();
    initialized = true;
  }

  const result = validate_source(e.data);
  self.postMessage(result);
};
```

## Browser Support

| Browser | Minimum Version |
|---------|-----------------|
| Chrome | 57+ |
| Firefox | 52+ |
| Safari | 11+ |
| Edge | 16+ |

## Next Steps

- [API Reference](api.md) - Full API documentation
- [Examples](https://github.com/sketchddd/examples) - Example applications
