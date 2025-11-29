# sketchddd-core

Core categorical semantics and data structures for [SketchDDD](https://github.com/ibrahimcesar/SketchDDD).

## Overview

This crate provides the mathematical foundation for SketchDDD, modeling Domain-Driven Design concepts using category theory. A bounded context is represented as a **sketch** `S = (G, E, L, C)` where:

- `G`: Directed graph (objects and morphisms)
- `E`: Path equations (business rules)
- `L`: Limit cones (aggregates, value objects)
- `C`: Colimit cocones (sum types, enumerations)

## DDD to Category Theory Mapping

| DDD Concept | Categorical Structure |
|-------------|----------------------|
| Bounded Context | Sketch |
| Ubiquitous Language | Graph + Equations |
| Entity | Object with identity morphism |
| Value Object | Limit with structural equality |
| Aggregate | Limit cone with root |
| Invariant | Equalizer |
| Context Map | Sketch morphism |

## License

Licensed under either of [MIT](../../LICENSE-MIT) or [Apache-2.0](../../LICENSE-APACHE) at your option.
