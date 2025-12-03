//! Diagnostics generation

use tower_lsp::lsp_types::*;

use sketchddd_parser::{ast::TypeExpr, parse};

use crate::document::Document;

/// Generate diagnostics for a document
pub fn publish_diagnostics(document: &Document) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    let text = document.text();

    // Try to parse and collect errors
    match parse(&text) {
        Ok(contexts) => {
            // Check for validation issues
            for context in &contexts {
                // Validate entity references
                validate_context(context, &text, &mut diagnostics);
            }
        }
        Err(error) => {
            // Convert parse error to diagnostic
            let line = error.line.unwrap_or(1).saturating_sub(1);
            let col = error.column.unwrap_or(1).saturating_sub(1);

            diagnostics.push(Diagnostic {
                range: Range {
                    start: Position::new(line, col),
                    end: Position::new(line, col + 10),
                },
                severity: Some(DiagnosticSeverity::ERROR),
                code: Some(NumberOrString::String("E0100".to_string())),
                source: Some("sketchddd".to_string()),
                message: error.message.clone(),
                related_information: None,
                tags: None,
                code_description: None,
                data: None,
            });
        }
    }

    // Add semantic validation
    add_semantic_diagnostics(&text, &mut diagnostics);

    diagnostics
}

/// Convert TypeExpr to string for validation
fn type_expr_to_string(expr: &TypeExpr) -> String {
    match expr {
        TypeExpr::Simple(name) => name.clone(),
        TypeExpr::Generic { name, args } => {
            let args_str: Vec<String> = args.iter().map(type_expr_to_string).collect();
            format!("{}<{}>", name, args_str.join(", "))
        }
        TypeExpr::Optional(inner) => format!("{}?", type_expr_to_string(inner)),
    }
}

/// Validate a context for semantic errors
fn validate_context(
    context: &sketchddd_parser::ast::ContextDecl,
    text: &str,
    diagnostics: &mut Vec<Diagnostic>,
) {
    // Collect all defined type names
    let mut defined_types: Vec<String> = Vec::new();

    for entity in &context.entities {
        defined_types.push(entity.name.clone());
    }

    for value in &context.value_objects {
        defined_types.push(value.name.clone());
    }

    for e in &context.enums {
        defined_types.push(e.name.clone());
    }

    // Check for missing types in fields
    for entity in &context.entities {
        for field in &entity.fields {
            let type_str = type_expr_to_string(&field.type_expr);
            let base_type = extract_base_type(&type_str);
            if !is_builtin_type(&base_type) && !defined_types.contains(&base_type) {
                // Find the line where this field is defined
                if let Some((line, col)) = find_type_reference(text, &entity.name, &base_type) {
                    diagnostics.push(Diagnostic {
                        range: Range {
                            start: Position::new(line, col),
                            end: Position::new(line, col + base_type.len() as u32),
                        },
                        severity: Some(DiagnosticSeverity::ERROR),
                        code: Some(NumberOrString::String("E0200".to_string())),
                        source: Some("sketchddd".to_string()),
                        message: format!("Undefined type '{}'", base_type),
                        related_information: None,
                        tags: None,
                        code_description: None,
                        data: None,
                    });
                }
            }
        }
    }

    // Check for entities without id field
    for entity in &context.entities {
        let has_id = entity.fields.iter().any(|f| f.name == "id");
        if !has_id {
            if let Some(line) = find_entity_line(text, &entity.name) {
                diagnostics.push(Diagnostic {
                    range: Range {
                        start: Position::new(line, 0),
                        end: Position::new(line, 50),
                    },
                    severity: Some(DiagnosticSeverity::WARNING),
                    code: Some(NumberOrString::String("W0001".to_string())),
                    source: Some("sketchddd".to_string()),
                    message: format!(
                        "Entity '{}' should have an 'id' field for identity",
                        entity.name
                    ),
                    related_information: None,
                    tags: None,
                    code_description: None,
                    data: None,
                });
            }
        }
    }
}

/// Add semantic diagnostics (beyond parsing)
fn add_semantic_diagnostics(text: &str, diagnostics: &mut Vec<Diagnostic>) {
    // Check for duplicate type names
    let mut type_names: Vec<(String, u32)> = Vec::new();

    for (line_idx, line) in text.lines().enumerate() {
        let trimmed = line.trim();
        let line_num = line_idx as u32;

        // Extract type name from entity/value/enum definitions
        for keyword in &["entity", "value", "enum", "aggregate"] {
            if trimmed.starts_with(keyword) {
                let rest = trimmed[keyword.len()..].trim_start();
                let name: String = rest.chars().take_while(|c| c.is_alphanumeric()).collect();

                if !name.is_empty() {
                    // Check for duplicate
                    if let Some((_, first_line)) =
                        type_names.iter().find(|(n, _)| n == &name)
                    {
                        diagnostics.push(Diagnostic {
                            range: Range {
                                start: Position::new(line_num, 0),
                                end: Position::new(line_num, line.len() as u32),
                            },
                            severity: Some(DiagnosticSeverity::ERROR),
                            code: Some(NumberOrString::String("E0201".to_string())),
                            source: Some("sketchddd".to_string()),
                            message: format!(
                                "Duplicate type name '{}' (first defined on line {})",
                                name,
                                first_line + 1
                            ),
                            related_information: Some(vec![DiagnosticRelatedInformation {
                                location: Location {
                                    uri: Url::parse("file:///").unwrap(),
                                    range: Range {
                                        start: Position::new(*first_line, 0),
                                        end: Position::new(*first_line, 50),
                                    },
                                },
                                message: "First definition here".to_string(),
                            }]),
                            tags: None,
                            code_description: None,
                            data: None,
                        });
                    } else {
                        type_names.push((name, line_num));
                    }
                }
            }
        }
    }
}

/// Extract base type from a type expression (handles List<T>, Map<K,V>, T?)
fn extract_base_type(type_expr: &str) -> String {
    let t = type_expr.trim().trim_end_matches('?');

    if t.starts_with("List<") && t.ends_with('>') {
        t[5..t.len() - 1].trim().to_string()
    } else if t.starts_with("Set<") && t.ends_with('>') {
        t[4..t.len() - 1].trim().to_string()
    } else if t.starts_with("Map<") && t.ends_with('>') {
        // For Map<K, V>, just check the value type for simplicity
        let inner = &t[4..t.len() - 1];
        if let Some(pos) = inner.find(',') {
            inner[pos + 1..].trim().to_string()
        } else {
            t.to_string()
        }
    } else {
        t.to_string()
    }
}

/// Check if a type is a built-in type
fn is_builtin_type(name: &str) -> bool {
    matches!(
        name,
        "String"
            | "Int"
            | "Float"
            | "Bool"
            | "UUID"
            | "DateTime"
            | "Date"
            | "Decimal"
            | "Email"
            | "List"
            | "Map"
            | "Set"
    )
}

/// Find the line where a type is referenced in an entity
fn find_type_reference(text: &str, entity_name: &str, type_name: &str) -> Option<(u32, u32)> {
    let mut in_entity = false;

    for (line_idx, line) in text.lines().enumerate() {
        let trimmed = line.trim();

        if trimmed.starts_with("entity") && trimmed.contains(entity_name) {
            in_entity = true;
        } else if in_entity {
            if trimmed == "}" {
                in_entity = false;
            } else if let Some(pos) = line.find(type_name) {
                return Some((line_idx as u32, pos as u32));
            }
        }
    }

    None
}

/// Find the line where an entity is defined
fn find_entity_line(text: &str, entity_name: &str) -> Option<u32> {
    for (line_idx, line) in text.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.starts_with("entity") && trimmed.contains(entity_name) {
            return Some(line_idx as u32);
        }
    }
    None
}
