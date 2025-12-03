//! Server capabilities configuration

use tower_lsp::lsp_types::*;

/// Build server capabilities
pub fn server_capabilities() -> ServerCapabilities {
    ServerCapabilities {
        // Full document sync - we get the entire document on each change
        text_document_sync: Some(TextDocumentSyncCapability::Options(
            TextDocumentSyncOptions {
                open_close: Some(true),
                change: Some(TextDocumentSyncKind::FULL),
                will_save: Some(false),
                will_save_wait_until: Some(false),
                save: Some(TextDocumentSyncSaveOptions::SaveOptions(SaveOptions {
                    include_text: Some(true),
                })),
            },
        )),

        // Hover support
        hover_provider: Some(HoverProviderCapability::Simple(true)),

        // Completion support
        completion_provider: Some(CompletionOptions {
            trigger_characters: Some(vec![
                ":".to_string(),
                " ".to_string(),
                "{".to_string(),
                "@".to_string(),
            ]),
            resolve_provider: Some(false),
            ..Default::default()
        }),

        // Definition support
        definition_provider: Some(OneOf::Left(true)),

        // References support
        references_provider: Some(OneOf::Left(true)),

        // Document symbols (outline)
        document_symbol_provider: Some(OneOf::Left(true)),

        // Formatting
        document_formatting_provider: Some(OneOf::Left(true)),

        // Semantic tokens for enhanced highlighting
        semantic_tokens_provider: Some(
            SemanticTokensServerCapabilities::SemanticTokensOptions(SemanticTokensOptions {
                legend: SemanticTokensLegend {
                    token_types: vec![
                        SemanticTokenType::NAMESPACE,  // context
                        SemanticTokenType::CLASS,      // entity, value
                        SemanticTokenType::ENUM,       // enum
                        SemanticTokenType::ENUM_MEMBER, // enum variant
                        SemanticTokenType::PROPERTY,   // field
                        SemanticTokenType::FUNCTION,   // morphism
                        SemanticTokenType::TYPE,       // type reference
                        SemanticTokenType::KEYWORD,    // keywords
                        SemanticTokenType::COMMENT,    // comments
                        SemanticTokenType::STRING,     // strings
                        SemanticTokenType::NUMBER,     // numbers
                        SemanticTokenType::OPERATOR,   // operators
                        SemanticTokenType::DECORATOR,  // annotations
                    ],
                    token_modifiers: vec![
                        SemanticTokenModifier::DECLARATION,
                        SemanticTokenModifier::DEFINITION,
                        SemanticTokenModifier::READONLY,
                    ],
                },
                full: Some(SemanticTokensFullOptions::Bool(true)),
                range: Some(false),
                ..Default::default()
            }),
        ),

        ..Default::default()
    }
}
