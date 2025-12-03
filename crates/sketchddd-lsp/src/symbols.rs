//! Document symbols (outline)

use tower_lsp::lsp_types::*;

use crate::document::{Definition, DefinitionKind, Document};

/// Generate document symbols for the outline view
pub fn document_symbols(document: &Document) -> Vec<DocumentSymbol> {
    document
        .definitions
        .iter()
        .map(definition_to_symbol)
        .collect()
}

/// Convert a definition to a document symbol
#[allow(deprecated)]
fn definition_to_symbol(def: &Definition) -> DocumentSymbol {
    let kind = match def.kind {
        DefinitionKind::Context => SymbolKind::NAMESPACE,
        DefinitionKind::Entity => SymbolKind::CLASS,
        DefinitionKind::Value => SymbolKind::STRUCT,
        DefinitionKind::Enum => SymbolKind::ENUM,
        DefinitionKind::EnumVariant => SymbolKind::ENUM_MEMBER,
        DefinitionKind::Aggregate => SymbolKind::CLASS,
        DefinitionKind::Morphism => SymbolKind::FUNCTION,
        DefinitionKind::Field => SymbolKind::FIELD,
        DefinitionKind::ContextMap => SymbolKind::INTERFACE,
    };

    let detail = match def.kind {
        DefinitionKind::Context => Some("bounded context".to_string()),
        DefinitionKind::Entity => Some("entity".to_string()),
        DefinitionKind::Value => Some("value object".to_string()),
        DefinitionKind::Enum => Some("enumeration".to_string()),
        DefinitionKind::Aggregate => Some("aggregate".to_string()),
        DefinitionKind::ContextMap => Some("context map".to_string()),
        _ => None,
    };

    let children: Vec<DocumentSymbol> = def.children.iter().map(definition_to_symbol).collect();

    DocumentSymbol {
        name: def.name.clone(),
        detail,
        kind,
        tags: None,
        deprecated: None,
        range: def.range,
        selection_range: def.selection_range,
        children: if children.is_empty() {
            None
        } else {
            Some(children)
        },
    }
}
