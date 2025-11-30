//! # SketchDDD CLI
//!
//! Command-line interface for validating, generating, and visualizing
//! SketchDDD domain models.

use clap::{Parser, Subcommand, ValueEnum};
use colored::Colorize;
use sketchddd_core::{validate_model, Severity, ValidationError};
use sketchddd_parser::{parse_file, transform};
use std::path::PathBuf;

/// Verbosity level for output
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum, Default)]
pub enum Verbosity {
    /// Suppress all non-essential output
    Quiet,
    /// Normal output (default)
    #[default]
    Normal,
    /// Verbose output with additional details
    Verbose,
}

#[derive(Parser)]
#[command(name = "sketchddd")]
#[command(author, version, about = "Build Domain Models Visually or with Code", long_about = None)]
struct Cli {
    /// Verbosity level
    #[arg(short, long, value_enum, default_value_t = Verbosity::Normal, global = true)]
    verbosity: Verbosity,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Validate a SketchDDD model file
    Check {
        /// Path to the .sddd or .sketch file
        file: PathBuf,

        /// Output format for errors
        #[arg(short, long, default_value = "pretty")]
        format: String,
    },

    /// Generate code from a SketchDDD model
    Codegen {
        /// Path to the .sddd or .sketch file
        file: PathBuf,

        /// Target language (rust, typescript, kotlin)
        #[arg(short, long, default_value = "rust")]
        target: String,

        /// Output directory
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Generate visualizations from a SketchDDD model
    Viz {
        /// Path to the .sddd or .sketch file
        file: PathBuf,

        /// Output format (graphviz, mermaid)
        #[arg(short, long, default_value = "mermaid")]
        format: String,

        /// Output file
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Initialize a new SketchDDD project
    Init {
        /// Project name
        name: String,

        /// Template to use (minimal, ecommerce, microservices)
        #[arg(short, long, default_value = "minimal")]
        template: String,
    },

    /// Start the visual builder server
    Serve {
        /// Port to listen on
        #[arg(short, long, default_value = "3000")]
        port: u16,
    },

    /// Export model to JSON format
    Export {
        /// Path to the .sddd or .sketch file
        file: PathBuf,

        /// Output file
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Import model from JSON format
    Import {
        /// Path to the JSON file
        file: PathBuf,

        /// Output .sddd file
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Compare two model versions
    Diff {
        /// First .sddd or .sketch file
        old: PathBuf,

        /// Second .sddd or .sketch file
        new: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Check { file, format } => cmd_check(&file, &format, cli.verbosity),
        Commands::Codegen {
            file,
            target,
            output,
        } => cmd_codegen(&file, &target, output, cli.verbosity),
        Commands::Viz {
            file,
            format,
            output,
        } => cmd_viz(&file, &format, output, cli.verbosity),
        Commands::Init { name, template } => cmd_init(&name, &template, cli.verbosity),
        Commands::Serve { port } => cmd_serve(port, cli.verbosity),
        Commands::Export { file, output } => cmd_export(&file, output, cli.verbosity),
        Commands::Import { file, output } => cmd_import(&file, output, cli.verbosity),
        Commands::Diff { old, new } => cmd_diff(&old, &new, cli.verbosity),
    };

    if let Err(e) = result {
        eprintln!("{}: {}", "error".red().bold(), e);
        std::process::exit(1);
    }
}

/// Check/validate a SketchDDD model file
fn cmd_check(file: &PathBuf, format: &str, verbosity: Verbosity) -> Result<(), String> {
    if verbosity != Verbosity::Quiet {
        println!("{} {}", "Checking".cyan().bold(), file.display());
    }

    // Read file
    let source =
        std::fs::read_to_string(file).map_err(|e| format!("Failed to read file: {}", e))?;

    // Parse to AST
    let ast = parse_file(&source).map_err(|e| format!("Parse error: {}", e))?;

    if verbosity == Verbosity::Verbose {
        println!(
            "  {} {} context(s), {} context map(s)",
            "Parsed".blue(),
            ast.contexts.len(),
            ast.context_maps.len()
        );
    }

    // Transform AST to semantic model
    let transform_result = transform(&ast).map_err(|e| format!("Transform error: {}", e))?;

    // Show transform warnings
    for warning in &transform_result.warnings {
        let location = match (warning.line, warning.column) {
            (Some(l), Some(c)) => format!("{}:{}:{}", file.display(), l, c),
            (Some(l), None) => format!("{}:{}", file.display(), l),
            _ => file.display().to_string(),
        };
        eprintln!(
            "{}: {} {}",
            location,
            "warning".yellow().bold(),
            warning.message
        );
    }

    // Validate the model
    let validation_result =
        validate_model(&transform_result.contexts, &transform_result.context_maps);

    // Report results based on format
    match format {
        "json" => {
            let json = serde_json::to_string_pretty(&validation_result.issues)
                .map_err(|e| format!("JSON serialization error: {}", e))?;
            println!("{}", json);
        }
        _ => {
            // Pretty format (default)
            print_validation_issues(file, &validation_result.issues, verbosity);
        }
    }

    // Summary
    let error_count = validation_result.error_count();
    let warning_count = validation_result.warning_count();

    if verbosity != Verbosity::Quiet {
        if error_count == 0 && warning_count == 0 {
            println!(
                "{} {} No issues found!",
                "✓".green().bold(),
                file.display()
            );
        } else {
            let errors_str = if error_count == 1 {
                "1 error".red().bold().to_string()
            } else if error_count > 1 {
                format!("{} errors", error_count).red().bold().to_string()
            } else {
                String::new()
            };

            let warnings_str = if warning_count == 1 {
                "1 warning".yellow().bold().to_string()
            } else if warning_count > 1 {
                format!("{} warnings", warning_count)
                    .yellow()
                    .bold()
                    .to_string()
            } else {
                String::new()
            };

            let summary = match (error_count > 0, warning_count > 0) {
                (true, true) => format!("{} and {}", errors_str, warnings_str),
                (true, false) => errors_str,
                (false, true) => warnings_str,
                (false, false) => "no issues".to_string(),
            };

            println!("  {} generated {} {}", file.display(), summary, "");
        }
    }

    if validation_result.is_ok() {
        Ok(())
    } else {
        Err(format!(
            "Validation failed with {} error(s)",
            error_count
        ))
    }
}

/// Print validation issues in a pretty format
fn print_validation_issues(file: &PathBuf, issues: &[ValidationError], verbosity: Verbosity) {
    for issue in issues {
        let severity_str = match issue.severity {
            Severity::Error => "error".red().bold(),
            Severity::Warning => "warning".yellow().bold(),
            Severity::Hint => "hint".blue().bold(),
        };

        let location = match (&issue.location.line, &issue.location.column) {
            (Some(l), Some(c)) => format!("{}:{}:{}", file.display(), l, c),
            (Some(l), None) => format!("{}:{}", file.display(), l),
            _ => file.display().to_string(),
        };

        println!(
            "{}: {}[{}]: {}",
            location, severity_str, issue.code, issue.message
        );

        if verbosity == Verbosity::Verbose {
            if let Some(ref suggestion) = issue.suggestion {
                println!("  {} {}", "suggestion:".cyan(), suggestion);
            }
        }
    }
}

fn cmd_codegen(
    file: &PathBuf,
    target: &str,
    output: Option<PathBuf>,
    verbosity: Verbosity,
) -> Result<(), String> {
    if verbosity != Verbosity::Quiet {
        println!(
            "{} {} -> {}",
            "Generating".cyan().bold(),
            file.display(),
            target
        );
    }

    // TODO: Implement full codegen
    let _ = output;
    println!(
        "{} Code generation not yet implemented",
        "⚠".yellow().bold()
    );
    Ok(())
}

fn cmd_viz(
    file: &PathBuf,
    format: &str,
    output: Option<PathBuf>,
    verbosity: Verbosity,
) -> Result<(), String> {
    if verbosity != Verbosity::Quiet {
        println!(
            "{} {} -> {}",
            "Visualizing".cyan().bold(),
            file.display(),
            format
        );
    }

    // TODO: Implement full viz
    let _ = output;
    println!(
        "{} Visualization not yet implemented",
        "⚠".yellow().bold()
    );
    Ok(())
}

fn cmd_init(name: &str, template: &str, verbosity: Verbosity) -> Result<(), String> {
    if verbosity != Verbosity::Quiet {
        println!(
            "{} {} (template: {})",
            "Initializing".cyan().bold(),
            name,
            template
        );
    }

    // Create directory
    std::fs::create_dir_all(name).map_err(|e| format!("Failed to create directory: {}", e))?;

    // Get template content
    let (content, description) = match template {
        "ecommerce" => (get_ecommerce_template(name), "e-commerce domain"),
        "microservices" => (get_microservices_template(name), "microservices architecture"),
        _ => (get_minimal_template(name), "minimal project"),
    };

    // Create the main .sddd file
    let filename = format!("{}/{}.sddd", name, name.to_lowercase());
    std::fs::write(&filename, content).map_err(|e| format!("Failed to write file: {}", e))?;

    // Create a .gitignore
    let gitignore = r#"# Generated files
/generated/
*.gen.*

# Editor files
.vscode/
.idea/
*.swp
*.swo

# OS files
.DS_Store
Thumbs.db
"#;
    std::fs::write(format!("{}/.gitignore", name), gitignore)
        .map_err(|e| format!("Failed to write .gitignore: {}", e))?;

    if verbosity != Verbosity::Quiet {
        println!("{} Created {}/", "✓".green().bold(), name);
        println!("  {} {}.sddd ({} template)", "→".blue(), name.to_lowercase(), description);
        println!("  {} .gitignore", "→".blue());
        println!();
        println!("Next steps:");
        println!(
            "  {} {}",
            "cd".cyan(),
            name
        );
        println!(
            "  {} check {}.sddd",
            "sketchddd".cyan(),
            name.to_lowercase()
        );
    }

    Ok(())
}

/// Minimal template for new projects
fn get_minimal_template(name: &str) -> String {
    format!(
        r#"// {name} Domain Model
// Created with SketchDDD
// Documentation: https://sketchddd.dev

context {name} {{
    // Define your domain objects
    objects {{
        // Add objects here, e.g.: Customer, Order, Product
    }}

    // Define relationships between objects
    morphisms {{
        // Add morphisms here, e.g.: placedBy: Order -> Customer
    }}

    // Define entities with identity
    // entity Customer {{
    //     id: UUID
    //     name: String
    // }}

    // Define value objects
    // value Money {{
    //     amount: Decimal
    //     currency: Currency
    // }}

    // Define aggregates
    // aggregate OrderAggregate {{
    //     root: Order
    //     contains: [LineItem]
    // }}

    // Define enumerations
    // enum Status = Active | Inactive | Pending
}}
"#,
        name = name
    )
}

/// E-commerce template
fn get_ecommerce_template(name: &str) -> String {
    format!(
        r#"// {name} - E-Commerce Domain Model
// Created with SketchDDD

context {name} {{
    // Core domain objects
    objects {{
        Product,
        Category,
        Inventory
    }}

    // Entities with identity
    entity Customer {{
        id: UUID
        email: Email
        name: String
    }}

    entity Order {{
        id: UUID
        orderNumber: String
        placedAt: DateTime
    }}

    entity LineItem {{
        id: UUID
        quantity: Integer
    }}

    // Value objects (immutable)
    value Money {{
        amount: Decimal
        currency: Currency
    }}

    value Address {{
        street: String
        city: String
        country: String
        postalCode: String
    }}

    // Relationships
    morphisms {{
        placedBy: Order -> Customer
        items: Order -> List<LineItem>
        product: LineItem -> Product
        unitPrice: LineItem -> Money
        shippingAddress: Order -> Address
        billingAddress: Order -> Address?
        belongsTo: Product -> Category
    }}

    // Aggregates (consistency boundaries)
    aggregate OrderAggregate {{
        root: Order
        contains: [LineItem]
        invariant: totalItems = sum(items.quantity)
    }}

    // Enumerations
    enum OrderStatus = Draft | Pending | Confirmed | Shipped | Delivered | Cancelled

    enum PaymentStatus = Pending | Authorized | Captured | Refunded | Failed
}}
"#,
        name = name
    )
}

/// Microservices template with multiple contexts
fn get_microservices_template(name: &str) -> String {
    format!(
        r#"// {name} - Microservices Domain Model
// Created with SketchDDD
// This template demonstrates multiple bounded contexts and context maps

// ============================================
// Orders Context
// ============================================
context Orders {{
    entity Order {{
        id: UUID
        customerId: UUID
        status: OrderStatus
    }}

    entity LineItem {{
        id: UUID
        productId: UUID
        quantity: Integer
    }}

    value Money {{
        amount: Decimal
        currency: Currency
    }}

    morphisms {{
        items: Order -> List<LineItem>
        total: Order -> Money
    }}

    aggregate OrderAggregate {{
        root: Order
        contains: [LineItem]
    }}

    enum OrderStatus = Created | Confirmed | Fulfilled | Cancelled
}}

// ============================================
// Inventory Context
// ============================================
context Inventory {{
    entity StockItem {{
        id: UUID
        productId: UUID
        quantity: Integer
        warehouseId: UUID
    }}

    entity Warehouse {{
        id: UUID
        name: String
        location: String
    }}

    morphisms {{
        storedIn: StockItem -> Warehouse
    }}

    aggregate WarehouseAggregate {{
        root: Warehouse
        contains: [StockItem]
    }}
}}

// ============================================
// Shipping Context
// ============================================
context Shipping {{
    entity Shipment {{
        id: UUID
        orderId: UUID
        trackingNumber: String
    }}

    entity Carrier {{
        id: UUID
        name: String
    }}

    value Address {{
        street: String
        city: String
        country: String
    }}

    morphisms {{
        destination: Shipment -> Address
        carrier: Shipment -> Carrier
    }}

    enum ShipmentStatus = Pending | InTransit | Delivered | Returned
}}

// ============================================
// Context Maps (Integration Patterns)
// ============================================

// Orders publishes events that Inventory consumes
map OrdersToInventory: Orders -> Inventory {{
    pattern: CustomerSupplier
    mappings {{
        Order -> StockItem
    }}
}}

// Orders publishes events that Shipping consumes
map OrdersToShipping: Orders -> Shipping {{
    pattern: CustomerSupplier
    mappings {{
        Order -> Shipment
    }}
}}
"#,
        name = name
    )
}

fn cmd_serve(port: u16, verbosity: Verbosity) -> Result<(), String> {
    if verbosity != Verbosity::Quiet {
        println!(
            "{} Visual builder at http://localhost:{}",
            "Starting".cyan().bold(),
            port
        );
    }
    println!("{} Server not yet implemented", "⚠".yellow().bold());
    Ok(())
}

fn cmd_export(file: &PathBuf, output: Option<PathBuf>, verbosity: Verbosity) -> Result<(), String> {
    if verbosity != Verbosity::Quiet {
        println!("{} {}", "Exporting".cyan().bold(), file.display());
    }

    // Read and parse the source file
    let source =
        std::fs::read_to_string(file).map_err(|e| format!("Failed to read file: {}", e))?;
    let ast = parse_file(&source).map_err(|e| format!("Parse error: {}", e))?;
    let transform_result = transform(&ast).map_err(|e| format!("Transform error: {}", e))?;

    // Serialize to JSON
    let json_output = serde_json::json!({
        "contexts": transform_result.contexts.iter().map(|ctx| {
            serde_json::json!({
                "name": ctx.name(),
                "entities": ctx.entities().len(),
                "valueObjects": ctx.value_objects().len(),
                "aggregates": ctx.aggregate_roots().len(),
            })
        }).collect::<Vec<_>>(),
        "contextMaps": transform_result.context_maps.iter().map(|map| {
            serde_json::json!({
                "name": map.name(),
                "source": map.source_context(),
                "target": map.target_context(),
                "pattern": format!("{:?}", map.pattern()),
            })
        }).collect::<Vec<_>>(),
    });

    let json_str = serde_json::to_string_pretty(&json_output)
        .map_err(|e| format!("JSON serialization error: {}", e))?;

    // Write to output file or stdout
    match output {
        Some(path) => {
            std::fs::write(&path, &json_str)
                .map_err(|e| format!("Failed to write output: {}", e))?;
            if verbosity != Verbosity::Quiet {
                println!("{} Exported to {}", "✓".green().bold(), path.display());
            }
        }
        None => {
            println!("{}", json_str);
        }
    }

    Ok(())
}

fn cmd_import(file: &PathBuf, output: Option<PathBuf>, verbosity: Verbosity) -> Result<(), String> {
    if verbosity != Verbosity::Quiet {
        println!("{} {}", "Importing".cyan().bold(), file.display());
    }
    let _ = output;
    println!("{} Import not yet implemented", "⚠".yellow().bold());
    Ok(())
}

fn cmd_diff(old: &PathBuf, new: &PathBuf, verbosity: Verbosity) -> Result<(), String> {
    if verbosity != Verbosity::Quiet {
        println!(
            "{} {} vs {}",
            "Comparing".cyan().bold(),
            old.display(),
            new.display()
        );
    }
    println!("{} Diff not yet implemented", "⚠".yellow().bold());
    Ok(())
}
