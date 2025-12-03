//! Code completion

use tower_lsp::lsp_types::*;

use crate::document::Document;

/// Provide completions at a position
pub fn provide_completions(document: &Document, position: Position) -> Vec<CompletionItem> {
    let mut completions = Vec::new();

    // Get the current line and context
    let line = match document.line(position.line as usize) {
        Some(l) => l,
        None => return completions,
    };

    let prefix: String = line
        .chars()
        .take(position.character as usize)
        .collect();
    let trimmed = prefix.trim();

    // Determine context and provide appropriate completions
    if trimmed.is_empty() || is_top_level_context(&document.text(), position) {
        // Top-level completions
        completions.extend(top_level_completions());
    } else if trimmed.ends_with(':') {
        // Type completions after colon
        completions.extend(type_completions(document));
    } else if trimmed.starts_with('@') {
        // Annotation completions
        completions.extend(annotation_completions());
    } else if in_morphisms_block(&document.text(), position) {
        // Morphism completions
        completions.extend(morphism_completions(document));
    } else if in_context_block(&document.text(), position) {
        // Context member completions
        completions.extend(context_member_completions());
    } else if in_aggregate_block(&document.text(), position) {
        // Aggregate keyword completions
        completions.extend(aggregate_keyword_completions());
    } else if in_map_block(&document.text(), position) {
        // Map keyword completions
        completions.extend(map_keyword_completions());
    }

    completions
}

/// Check if position is at top level (outside any block)
fn is_top_level_context(text: &str, position: Position) -> bool {
    let mut brace_depth: i32 = 0;

    for (line_idx, line) in text.lines().enumerate() {
        if line_idx >= position.line as usize {
            break;
        }

        for c in line.chars() {
            match c {
                '{' => brace_depth += 1,
                '}' => brace_depth = brace_depth.saturating_sub(1),
                _ => {}
            }
        }
    }

    brace_depth == 0
}

/// Check if position is inside a context block
fn in_context_block(text: &str, position: Position) -> bool {
    let mut in_context = false;
    let mut brace_depth: i32 = 0;

    for (line_idx, line) in text.lines().enumerate() {
        if line_idx >= position.line as usize {
            break;
        }

        let trimmed = line.trim();

        if trimmed.starts_with("context") {
            in_context = true;
            brace_depth = 0;
        }

        for c in line.chars() {
            match c {
                '{' => brace_depth += 1,
                '}' => {
                    brace_depth = brace_depth.saturating_sub(1);
                    if brace_depth == 0 && in_context {
                        in_context = false;
                    }
                }
                _ => {}
            }
        }
    }

    in_context && brace_depth == 1
}

/// Check if position is inside a morphisms block
fn in_morphisms_block(text: &str, position: Position) -> bool {
    let mut in_morphisms = false;
    let mut brace_depth: i32 = 0;

    for (line_idx, line) in text.lines().enumerate() {
        if line_idx >= position.line as usize {
            break;
        }

        let trimmed = line.trim();

        if trimmed.starts_with("morphisms") {
            in_morphisms = true;
            brace_depth = 0;
        }

        for c in line.chars() {
            match c {
                '{' => brace_depth += 1,
                '}' => {
                    brace_depth = brace_depth.saturating_sub(1);
                    if brace_depth == 0 && in_morphisms {
                        in_morphisms = false;
                    }
                }
                _ => {}
            }
        }
    }

    in_morphisms && brace_depth >= 1
}

/// Check if position is inside an aggregate block
fn in_aggregate_block(text: &str, position: Position) -> bool {
    let mut in_aggregate = false;
    let mut brace_depth: i32 = 0;

    for (line_idx, line) in text.lines().enumerate() {
        if line_idx >= position.line as usize {
            break;
        }

        let trimmed = line.trim();

        if trimmed.starts_with("aggregate") {
            in_aggregate = true;
            brace_depth = 0;
        }

        for c in line.chars() {
            match c {
                '{' => brace_depth += 1,
                '}' => {
                    brace_depth = brace_depth.saturating_sub(1);
                    if brace_depth == 0 && in_aggregate {
                        in_aggregate = false;
                    }
                }
                _ => {}
            }
        }
    }

    in_aggregate && brace_depth >= 1
}

/// Check if position is inside a map block
fn in_map_block(text: &str, position: Position) -> bool {
    let mut in_map = false;
    let mut brace_depth: i32 = 0;

    for (line_idx, line) in text.lines().enumerate() {
        if line_idx >= position.line as usize {
            break;
        }

        let trimmed = line.trim();

        if trimmed.starts_with("map") && trimmed.contains("->") {
            in_map = true;
            brace_depth = 0;
        }

        for c in line.chars() {
            match c {
                '{' => brace_depth += 1,
                '}' => {
                    brace_depth = brace_depth.saturating_sub(1);
                    if brace_depth == 0 && in_map {
                        in_map = false;
                    }
                }
                _ => {}
            }
        }
    }

    in_map && brace_depth >= 1
}

/// Top-level keyword completions
fn top_level_completions() -> Vec<CompletionItem> {
    vec![
        CompletionItem {
            label: "context".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            detail: Some("Define a bounded context".to_string()),
            insert_text: Some("context ${1:ContextName} {\n  $0\n}".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "map".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            detail: Some("Define a context map".to_string()),
            insert_text: Some(
                "map ${1:MapName}: ${2:Source} -> ${3:Target} {\n  pattern: ${4:CustomerSupplier}\n  mappings {\n    $0\n  }\n}".to_string(),
            ),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
    ]
}

/// Context member completions
fn context_member_completions() -> Vec<CompletionItem> {
    vec![
        CompletionItem {
            label: "entity".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            detail: Some("Define an entity".to_string()),
            insert_text: Some("entity ${1:EntityName} {\n  id: UUID\n  $0\n}".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "value".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            detail: Some("Define a value object".to_string()),
            insert_text: Some("value ${1:ValueName} {\n  $0\n}".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "enum".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            detail: Some("Define an enumeration".to_string()),
            insert_text: Some("enum ${1:EnumName} = ${2:Variant1} | ${3:Variant2}".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "aggregate".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            detail: Some("Define an aggregate root".to_string()),
            insert_text: Some(
                "aggregate ${1:AggregateName} {\n  root: ${2:RootEntity}\n  contains: [${3:Entity}]\n}".to_string(),
            ),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "morphisms".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            detail: Some("Define relationships".to_string()),
            insert_text: Some("morphisms {\n  $0\n}".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
    ]
}

/// Type completions
fn type_completions(document: &Document) -> Vec<CompletionItem> {
    let mut completions = vec![
        // Primitive types
        simple_type("String", "UTF-8 text"),
        simple_type("Int", "64-bit integer"),
        simple_type("Float", "64-bit floating point"),
        simple_type("Bool", "Boolean (true/false)"),
        simple_type("UUID", "Universally unique identifier"),
        simple_type("DateTime", "Date and time with timezone"),
        simple_type("Date", "Calendar date"),
        simple_type("Decimal", "Arbitrary precision decimal"),
        simple_type("Email", "Email address"),
        // Generic types
        CompletionItem {
            label: "List<T>".to_string(),
            kind: Some(CompletionItemKind::CLASS),
            detail: Some("Ordered collection".to_string()),
            insert_text: Some("List<${1:Type}>".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "Map<K, V>".to_string(),
            kind: Some(CompletionItemKind::CLASS),
            detail: Some("Key-value mapping".to_string()),
            insert_text: Some("Map<${1:Key}, ${2:Value}>".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "Set<T>".to_string(),
            kind: Some(CompletionItemKind::CLASS),
            detail: Some("Unique collection".to_string()),
            insert_text: Some("Set<${1:Type}>".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
    ];

    // Add user-defined types
    for def in &document.definitions {
        completions.push(CompletionItem {
            label: def.name.clone(),
            kind: Some(match def.kind {
                crate::document::DefinitionKind::Entity => CompletionItemKind::CLASS,
                crate::document::DefinitionKind::Value => CompletionItemKind::STRUCT,
                crate::document::DefinitionKind::Enum => CompletionItemKind::ENUM,
                _ => CompletionItemKind::CLASS,
            }),
            detail: Some(format!("{:?}", def.kind)),
            ..Default::default()
        });

        for child in &def.children {
            completions.push(CompletionItem {
                label: child.name.clone(),
                kind: Some(match child.kind {
                    crate::document::DefinitionKind::Entity => CompletionItemKind::CLASS,
                    crate::document::DefinitionKind::Value => CompletionItemKind::STRUCT,
                    crate::document::DefinitionKind::Enum => CompletionItemKind::ENUM,
                    _ => CompletionItemKind::CLASS,
                }),
                detail: Some(format!("{:?}", child.kind)),
                ..Default::default()
            });
        }
    }

    completions
}

/// Simple type completion helper
fn simple_type(name: &str, detail: &str) -> CompletionItem {
    CompletionItem {
        label: name.to_string(),
        kind: Some(CompletionItemKind::CLASS),
        detail: Some(detail.to_string()),
        ..Default::default()
    }
}

/// Annotation completions
fn annotation_completions() -> Vec<CompletionItem> {
    vec![
        CompletionItem {
            label: "@one".to_string(),
            kind: Some(CompletionItemKind::PROPERTY),
            detail: Some("One-to-one relationship".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "@many".to_string(),
            kind: Some(CompletionItemKind::PROPERTY),
            detail: Some("One-to-many relationship".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "@optional".to_string(),
            kind: Some(CompletionItemKind::PROPERTY),
            detail: Some("Optional relationship".to_string()),
            ..Default::default()
        },
    ]
}

/// Morphism completions
fn morphism_completions(document: &Document) -> Vec<CompletionItem> {
    let mut completions = Vec::new();

    // Add entity names for source/target
    for def in &document.definitions {
        for child in &def.children {
            if matches!(
                child.kind,
                crate::document::DefinitionKind::Entity
                    | crate::document::DefinitionKind::Value
            ) {
                completions.push(CompletionItem {
                    label: child.name.clone(),
                    kind: Some(CompletionItemKind::CLASS),
                    detail: Some(format!("{:?}", child.kind)),
                    ..Default::default()
                });
            }
        }
    }

    completions
}

/// Aggregate keyword completions
fn aggregate_keyword_completions() -> Vec<CompletionItem> {
    vec![
        CompletionItem {
            label: "root".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            detail: Some("Aggregate root entity".to_string()),
            insert_text: Some("root: ${1:Entity}".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "contains".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            detail: Some("Contained entities".to_string()),
            insert_text: Some("contains: [${1:Entity}]".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "invariant".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            detail: Some("Aggregate invariant".to_string()),
            insert_text: Some("invariant: ${1:expression}".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
    ]
}

/// Map keyword completions
fn map_keyword_completions() -> Vec<CompletionItem> {
    vec![
        CompletionItem {
            label: "pattern".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            detail: Some("Integration pattern".to_string()),
            insert_text: Some("pattern: ${1|CustomerSupplier,AntiCorruptionLayer,OpenHostService,Conformist,SharedKernel,Partnership|}".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "mappings".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            detail: Some("Type mappings block".to_string()),
            insert_text: Some("mappings {\n  $0\n}".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        // Pattern values
        simple_type("CustomerSupplier", "Customer/Supplier pattern"),
        simple_type("AntiCorruptionLayer", "Anti-corruption layer pattern"),
        simple_type("OpenHostService", "Open host service pattern"),
        simple_type("Conformist", "Conformist pattern"),
        simple_type("SharedKernel", "Shared kernel pattern"),
        simple_type("Partnership", "Partnership pattern"),
    ]
}
