# Architecture Decision Records

This document captures key architectural decisions made during SketchDDD development.

## ADR-001: Use Category Theory as Foundation

**Status**: Accepted

**Context**: Domain-Driven Design needs a rigorous mathematical foundation for domain modeling.

**Decision**: Use category theory, specifically sketch theory, as the mathematical foundation.

**Consequences**:
- (+) Formal semantics for domain models
- (+) Composition and modularity built-in
- (+) Well-understood mathematical properties
- (-) Learning curve for users unfamiliar with category theory
- (-) Some concepts require simplification for practical use

---

## ADR-002: Rust as Implementation Language

**Status**: Accepted

**Context**: Need a language for implementing the SketchDDD tooling.

**Decision**: Use Rust for the core implementation.

**Consequences**:
- (+) Memory safety without garbage collection
- (+) Excellent WebAssembly support
- (+) Strong type system matches domain modeling
- (+) Fast compilation of generated code validation
- (-) Steeper learning curve for contributors
- (-) Longer compile times during development

---

## ADR-003: Pest for Parsing

**Status**: Accepted

**Context**: Need a parser for the SketchDDD language.

**Decision**: Use Pest PEG parser generator.

**Consequences**:
- (+) Clear, readable grammar definition
- (+) Good error messages
- (+) Rust-native integration
- (+) Active maintenance
- (-) PEG limitations (no left recursion)
- (-) Less flexible than hand-written parser

---

## ADR-004: Workspace Structure

**Status**: Accepted

**Context**: Need to organize the codebase for multiple components.

**Decision**: Use Cargo workspace with separate crates for each concern.

**Consequences**:
- (+) Clear separation of concerns
- (+) Independent testing
- (+) Parallel compilation
- (+) Selective dependencies
- (-) More complex project structure
- (-) Cross-crate refactoring requires care

---

## ADR-005: Code Generation via Templates

**Status**: Accepted

**Context**: Need to generate code for multiple target languages.

**Decision**: Use string-based templates with structured generation logic.

**Consequences**:
- (+) Easy to add new languages
- (+) Templates are readable
- (+) Full control over output format
- (-) Templates can become complex
- (-) No compile-time template validation

---

## ADR-006: WebAssembly for Browser Support

**Status**: Accepted

**Context**: Need to support browser-based tooling.

**Decision**: Compile to WebAssembly using wasm-bindgen.

**Consequences**:
- (+) Same core logic in browser and CLI
- (+) No server required for basic operations
- (+) Good JavaScript interop
- (-) WASM bundle size (~1.3MB)
- (-) Some Rust features unavailable in WASM

---

## ADR-007: MkDocs for Documentation

**Status**: Accepted

**Context**: Need comprehensive documentation for users and contributors.

**Decision**: Use MkDocs with Material theme.

**Consequences**:
- (+) Markdown-based, easy to maintain
- (+) Excellent search and navigation
- (+) Good code highlighting
- (+) Active community
- (-) Requires Python for local development
- (-) Limited customization compared to custom sites

---

## ADR-008: Bounded Context as Primary Unit

**Status**: Accepted

**Context**: Need to define the scope of domain models.

**Decision**: Use bounded context as the primary modeling unit.

**Consequences**:
- (+) Aligns with DDD principles
- (+) Natural boundary for code generation
- (+) Clear ownership and responsibility
- (-) Cross-context relationships require explicit mapping
- (-) May encourage too-small contexts

---

## ADR-009: Immutable Core Structures

**Status**: Accepted

**Context**: Need to define data structures for domain models.

**Decision**: Make core data structures immutable by default.

**Consequences**:
- (+) Thread safety
- (+) Easier reasoning about state
- (+) Matches functional programming idioms
- (-) May require cloning for modifications
- (-) Some operations less efficient

---

## ADR-010: Rich Error Messages

**Status**: Accepted

**Context**: Need to provide helpful feedback for invalid models.

**Decision**: Include source locations, suggestions, and context in all errors.

**Consequences**:
- (+) Better developer experience
- (+) Faster debugging
- (+) IDE integration support
- (-) More complex error handling code
- (-) Larger error structures

---

## ADR-011: SDDD File Extension

**Status**: Accepted

**Context**: Need a file extension for SketchDDD files.

**Decision**: Use `.sddd` as the file extension.

**Consequences**:
- (+) Unique, unlikely to conflict
- (+) Clear association with SketchDDD
- (+) Short enough for convenience
- (-) New extension requires editor configuration
- (-) Not immediately recognizable

---

## ADR-012: Derive Macros in Generated Rust

**Status**: Accepted

**Context**: Generated Rust code needs common trait implementations.

**Decision**: Use derive macros for Debug, Clone, PartialEq, Serialize, Deserialize.

**Consequences**:
- (+) Minimal boilerplate
- (+) Standard Rust idioms
- (+) Easy serialization support
- (-) All derives always included (no customization yet)
- (-) Requires serde dependency

---

## ADR-013: Enum Variants as Keywords

**Status**: Accepted

**Context**: Need syntax for defining enum variants.

**Decision**: Use PascalCase identifiers separated by `|` for simple enums.

**Consequences**:
- (+) Compact syntax
- (+) Familiar to Rust/Haskell users
- (+) Easy to parse
- (-) Different from TypeScript union syntax
- (-) May confuse users from OO backgrounds

---

## ADR-014: Optional Types with Question Mark

**Status**: Accepted

**Context**: Need syntax for optional/nullable fields.

**Decision**: Use `Type?` suffix for optional types.

**Consequences**:
- (+) Concise syntax
- (+) Familiar from many languages (Swift, TypeScript, Kotlin)
- (+) Visually distinct
- (-) Different from Rust's `Option<T>`
- (-) May conflict with future syntax extensions

---

## ADR-015: Context Maps as First-Class

**Status**: Accepted

**Context**: Need to model relationships between bounded contexts.

**Decision**: Make context maps explicit constructs with pattern annotations.

**Consequences**:
- (+) Visible cross-context relationships
- (+) Explicit integration patterns
- (+) Validates context boundaries
- (-) More verbose than implicit mappings
- (-) Requires understanding of DDD patterns
