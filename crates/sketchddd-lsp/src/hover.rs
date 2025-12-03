//! Hover information

use tower_lsp::lsp_types::*;

use crate::document::{Definition, DefinitionKind, Document};

/// Provide hover information at a position
pub fn provide_hover(document: &Document, position: Position) -> Option<Hover> {
    // Get the word at the position
    let word = document.word_at(position)?;

    // Check if it's a keyword
    if let Some(hover) = keyword_hover(&word) {
        return Some(hover);
    }

    // Check if it's a built-in type
    if let Some(hover) = builtin_type_hover(&word) {
        return Some(hover);
    }

    // Check if it's a pattern
    if let Some(hover) = pattern_hover(&word) {
        return Some(hover);
    }

    // Check if it's a user-defined type
    if let Some(def) = find_definition(document, &word) {
        return Some(definition_hover(def));
    }

    None
}

/// Hover for keywords
fn keyword_hover(word: &str) -> Option<Hover> {
    let (title, description) = match word {
        "context" => (
            "context",
            "Defines a **Bounded Context** - a logical boundary within which a domain model applies.\n\nIn category theory, a context forms a category with objects (types) and morphisms (relationships).",
        ),
        "entity" => (
            "entity",
            "Defines a **Domain Entity** - an object with identity that persists over time.\n\nEntities are limit cones (products) with an identity field. They are mutable and tracked by their ID.",
        ),
        "value" => (
            "value",
            "Defines a **Value Object** - an immutable object defined by its attributes.\n\nValue objects are limit cones without identity. Two value objects with the same attributes are equal.",
        ),
        "enum" => (
            "enum",
            "Defines an **Enumeration** - a sum type with named variants.\n\nIn category theory, enums are colimit cocones (coproducts). Each variant is an injection into the sum type.",
        ),
        "aggregate" => (
            "aggregate",
            "Defines an **Aggregate Root** - a cluster of entities treated as a unit.\n\nAggregates ensure consistency boundaries and encapsulate invariants. Access to contained entities goes through the root.",
        ),
        "morphisms" => (
            "morphisms",
            "Defines **Morphisms** - relationships between domain objects.\n\nMorphisms are the arrows in the category, mapping from a source type to a target type.",
        ),
        "map" => (
            "map",
            "Defines a **Context Map** - relationships between bounded contexts.\n\nContext maps are functors between categories, preserving structure while translating types.",
        ),
        "root" => (
            "root",
            "Specifies the **Aggregate Root** entity.\n\nThe root is the only entity accessible from outside the aggregate. All access to contained entities goes through the root.",
        ),
        "contains" => (
            "contains",
            "Lists entities **Contained** within an aggregate.\n\nContained entities can only be accessed through the aggregate root and share its lifecycle.",
        ),
        "invariant" => (
            "invariant",
            "Defines an **Invariant** - a business rule that must always hold.\n\nInvariants are commutative diagrams ensuring consistency across the aggregate.",
        ),
        "pattern" => (
            "pattern",
            "Specifies the **Integration Pattern** for a context map.\n\nPatterns describe the relationship type between upstream and downstream contexts.",
        ),
        "mappings" => (
            "mappings",
            "Defines type **Mappings** in a context map.\n\nMappings specify how types in the source context translate to types in the target context.",
        ),
        _ => return None,
    };

    Some(Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: format!("### `{}`\n\n{}", title, description),
        }),
        range: None,
    })
}

/// Hover for built-in types
fn builtin_type_hover(word: &str) -> Option<Hover> {
    let description = match word {
        "String" => "**String** - UTF-8 encoded text of arbitrary length.\n\n```\nRust: String\nTypeScript: string\nPython: str\n```",
        "Int" => "**Int** - 64-bit signed integer.\n\nRange: -9,223,372,036,854,775,808 to 9,223,372,036,854,775,807\n\n```\nRust: i64\nTypeScript: number\nPython: int\n```",
        "Float" => "**Float** - 64-bit IEEE 754 floating-point number.\n\n```\nRust: f64\nTypeScript: number\nPython: float\n```",
        "Bool" => "**Bool** - Boolean value (true or false).\n\n```\nRust: bool\nTypeScript: boolean\nPython: bool\n```",
        "UUID" => "**UUID** - Universally Unique Identifier (128-bit).\n\nFormat: `xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx`\n\n```\nRust: Uuid\nTypeScript: string\nPython: UUID\n```",
        "DateTime" => "**DateTime** - Date and time with timezone.\n\nFormat: ISO 8601 (e.g., `2024-01-15T10:30:00Z`)\n\n```\nRust: DateTime<Utc>\nTypeScript: Date\nPython: datetime\n```",
        "Date" => "**Date** - Calendar date without time.\n\nFormat: ISO 8601 (e.g., `2024-01-15`)\n\n```\nRust: NaiveDate\nTypeScript: string\nPython: date\n```",
        "Decimal" => "**Decimal** - Arbitrary-precision decimal number.\n\nUse for financial calculations requiring precision.\n\n```\nRust: Decimal\nTypeScript: number\nPython: Decimal\n```",
        "Email" => "**Email** - Email address string.\n\nValidated to contain `@` and domain.\n\n```\nRust: String\nTypeScript: string\nPython: str\n```",
        "List" => "**List<T>** - Ordered collection of elements.\n\n```\nRust: Vec<T>\nTypeScript: T[]\nPython: list[T]\n```",
        "Map" => "**Map<K, V>** - Key-value mapping.\n\n```\nRust: HashMap<K, V>\nTypeScript: Map<K, V>\nPython: dict[K, V]\n```",
        "Set" => "**Set<T>** - Unordered collection of unique elements.\n\n```\nRust: HashSet<T>\nTypeScript: Set<T>\nPython: set[T]\n```",
        _ => return None,
    };

    Some(Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: description.to_string(),
        }),
        range: None,
    })
}

/// Hover for integration patterns
fn pattern_hover(word: &str) -> Option<Hover> {
    let description = match word {
        "CustomerSupplier" => "### Customer/Supplier Pattern\n\nThe **upstream** context (supplier) provides services to the **downstream** context (customer).\n\nThe supplier prioritizes the customer's needs but maintains ownership of the shared model.",
        "AntiCorruptionLayer" => "### Anti-Corruption Layer (ACL)\n\nA **translation layer** that protects the downstream context from changes in the upstream context.\n\nThe ACL translates between the external model and the internal domain model.",
        "OpenHostService" => "### Open Host Service (OHS)\n\nThe upstream context provides a **well-defined protocol** (API) for integration.\n\nOften combined with a Published Language for documentation.",
        "Conformist" => "### Conformist Pattern\n\nThe downstream context **conforms** to the upstream model without translation.\n\nUsed when the upstream model is adequate and translation cost outweighs benefits.",
        "SharedKernel" => "### Shared Kernel\n\nA **shared subset** of the domain model used by multiple contexts.\n\nRequires careful coordination between teams. Changes require agreement from all parties.",
        "Partnership" => "### Partnership Pattern\n\nContexts have a **mutual dependency** and must coordinate changes.\n\nTeams plan together and notify each other of changes that affect the shared boundary.",
        _ => return None,
    };

    Some(Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: description.to_string(),
        }),
        range: None,
    })
}

/// Find a definition by name
fn find_definition<'a>(document: &'a Document, name: &str) -> Option<&'a Definition> {
    for def in &document.definitions {
        if def.name == name {
            return Some(def);
        }

        for child in &def.children {
            if child.name == name {
                return Some(child);
            }
        }
    }

    None
}

/// Hover for user-defined types
fn definition_hover(def: &Definition) -> Hover {
    let kind_name = match def.kind {
        DefinitionKind::Context => "Bounded Context",
        DefinitionKind::Entity => "Entity",
        DefinitionKind::Value => "Value Object",
        DefinitionKind::Enum => "Enumeration",
        DefinitionKind::EnumVariant => "Enum Variant",
        DefinitionKind::Aggregate => "Aggregate",
        DefinitionKind::Morphism => "Morphism",
        DefinitionKind::Field => "Field",
        DefinitionKind::ContextMap => "Context Map",
    };

    let mut content = format!("### {} `{}`\n", kind_name, def.name);

    if !def.children.is_empty() {
        content.push_str("\n**Contains:**\n");
        for child in &def.children {
            content.push_str(&format!("- `{}` ({:?})\n", child.name, child.kind));
        }
    }

    Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: content,
        }),
        range: Some(def.selection_range),
    }
}
