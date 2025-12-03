//! SketchDDD Language Server
//!
//! Provides LSP support for .sddd files including:
//! - Diagnostics (syntax and semantic errors)
//! - Hover information
//! - Go to definition
//! - Code completion
//! - Document symbols
//! - Formatting

use tower_lsp::{LspService, Server};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod backend;
mod capabilities;
mod completion;
mod diagnostics;
mod document;
mod hover;
mod semantic_tokens;
mod symbols;

use backend::SketchDDDBackend;

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().with_writer(std::io::stderr))
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "sketchddd_lsp=info".into()),
        )
        .init();

    tracing::info!("Starting SketchDDD Language Server");

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(SketchDDDBackend::new);
    Server::new(stdin, stdout, socket).serve(service).await;
}
