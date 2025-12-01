//! # SketchDDD CLI
//!
//! Command-line interface for validating, generating, and visualizing
//! SketchDDD domain models.

use clap::{Parser, Subcommand, ValueEnum};
use colored::Colorize;
use sketchddd_codegen::Target;
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
    command: Option<Commands>,

    /// Path to .sddd file (auto-detected if not using subcommand)
    #[arg(global = true)]
    file: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Commands {
    /// Validate a SketchDDD model file
    Check {
        /// Path to the .sddd or .sketch file (optional if .sddd file in current dir)
        file: Option<PathBuf>,

        /// Output format for errors
        #[arg(short, long, default_value = "pretty")]
        format: String,
    },

    /// Generate code from a SketchDDD model
    Codegen {
        /// Path to the .sddd or .sketch file (optional if .sddd file in current dir)
        file: Option<PathBuf>,

        /// Target language (rust, typescript, kotlin, python, java, clojure, haskell)
        #[arg(short, long, default_value = "rust")]
        target: String,

        /// Output directory or file
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Generate visualizations from a SketchDDD model
    Viz {
        /// Path to the .sddd or .sketch file (optional if .sddd file in current dir)
        file: Option<PathBuf>,

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
        /// Path to the .sddd or .sketch file (optional if .sddd file in current dir)
        file: Option<PathBuf>,

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

    /// Manage templates
    #[command(subcommand)]
    Template(TemplateCommands),

    /// Check for updates
    Update {
        /// Only check, don't install
        #[arg(long)]
        check: bool,
    },
}

/// Template subcommands
#[derive(Subcommand)]
enum TemplateCommands {
    /// List available templates
    List {
        /// Include templates from remote registry
        #[arg(long)]
        remote: bool,
    },

    /// Show detailed information about a template
    Info {
        /// Template name
        name: String,
    },

    /// Validate a template
    Validate {
        /// Path to template directory or file
        path: PathBuf,
    },

    /// Install a template from registry or URL
    Install {
        /// Template name or URL
        source: String,

        /// Force reinstall if already exists
        #[arg(long)]
        force: bool,
    },

    /// Update an installed template
    #[command(name = "update")]
    UpdateTemplate {
        /// Template name (or 'all' for all templates)
        name: String,
    },

    /// Remove an installed template
    Remove {
        /// Template name
        name: String,

        /// Don't ask for confirmation
        #[arg(long)]
        force: bool,
    },

    /// Create a new template from an existing model
    Create {
        /// Template name
        name: String,

        /// Source .sddd file to use as template
        #[arg(short, long)]
        source: Option<PathBuf>,

        /// Output directory for the template
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Some(Commands::Check { file, format }) => {
            match resolve_sddd_file(file) {
                Ok(file) => cmd_check(&file, &format, cli.verbosity),
                Err(e) => Err(e),
            }
        }
        Some(Commands::Codegen {
            file,
            target,
            output,
        }) => {
            match resolve_sddd_file(file) {
                Ok(file) => cmd_codegen(&file, &target, output, cli.verbosity),
                Err(e) => Err(e),
            }
        }
        Some(Commands::Viz {
            file,
            format,
            output,
        }) => {
            match resolve_sddd_file(file) {
                Ok(file) => cmd_viz(&file, &format, output, cli.verbosity),
                Err(e) => Err(e),
            }
        }
        Some(Commands::Init { name, template }) => cmd_init(&name, &template, cli.verbosity),
        Some(Commands::Serve { port }) => cmd_serve(port, cli.verbosity),
        Some(Commands::Export { file, output }) => {
            match resolve_sddd_file(file) {
                Ok(file) => cmd_export(&file, output, cli.verbosity),
                Err(e) => Err(e),
            }
        }
        Some(Commands::Import { file, output }) => cmd_import(&file, output, cli.verbosity),
        Some(Commands::Diff { old, new }) => cmd_diff(&old, &new, cli.verbosity),
        Some(Commands::Template(subcmd)) => cmd_template(subcmd, cli.verbosity),
        Some(Commands::Update { check }) => cmd_update(check, cli.verbosity),
        None => {
            // Auto-detect .sddd file and run check
            match resolve_sddd_file(cli.file) {
                Ok(file) => cmd_check(&file, "pretty", cli.verbosity),
                Err(e) => Err(e),
            }
        }
    };

    if let Err(e) = result {
        eprintln!("{}: {}", "error".red().bold(), e);
        std::process::exit(1);
    }
}

/// Resolve .sddd file path, auto-detecting if not provided
fn resolve_sddd_file(file: Option<PathBuf>) -> Result<PathBuf, String> {
    match file {
        Some(f) => Ok(f),
        None => auto_detect_sddd_file(),
    }
}

/// Auto-detect .sddd file in current directory
fn auto_detect_sddd_file() -> Result<PathBuf, String> {
    let current_dir = std::env::current_dir()
        .map_err(|e| format!("Failed to get current directory: {}", e))?;

    // Look for .sddd files in current directory
    let entries = std::fs::read_dir(&current_dir)
        .map_err(|e| format!("Failed to read directory: {}", e))?;

    let sddd_files: Vec<PathBuf> = entries
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| {
            p.extension()
                .map(|ext| ext == "sddd" || ext == "sketch")
                .unwrap_or(false)
        })
        .collect();

    match sddd_files.len() {
        0 => Err("No .sddd file found in current directory. Specify a file path or run 'sketchddd init' to create one.".to_string()),
        1 => Ok(sddd_files.into_iter().next().unwrap()),
        _ => {
            // Multiple files found - prefer one matching directory name
            let dir_name = current_dir
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("");

            let matching = sddd_files.iter().find(|p| {
                p.file_stem()
                    .and_then(|s| s.to_str())
                    .map(|s| s.eq_ignore_ascii_case(dir_name))
                    .unwrap_or(false)
            });

            match matching {
                Some(f) => Ok(f.clone()),
                None => {
                    let names: Vec<_> = sddd_files
                        .iter()
                        .filter_map(|p| p.file_name())
                        .filter_map(|n| n.to_str())
                        .collect();
                    Err(format!(
                        "Multiple .sddd files found: {}. Please specify which file to use.",
                        names.join(", ")
                    ))
                }
            }
        }
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

    // Read and parse file
    let source =
        std::fs::read_to_string(file).map_err(|e| format!("Failed to read file: {}", e))?;
    let ast = parse_file(&source).map_err(|e| format!("Parse error: {}", e))?;
    let transform_result = transform(&ast).map_err(|e| format!("Transform error: {}", e))?;

    // Parse target language
    let target_enum: Target = target
        .parse()
        .map_err(|_| format!("Unknown target language: {}. Supported: rust, typescript, kotlin, python, java, clojure, haskell", target))?;

    // Generate code for each context
    for context in &transform_result.contexts {
        let code = sketchddd_codegen::generate(context, target_enum)
            .map_err(|e| format!("Code generation error: {}", e))?;

        // Determine output path
        let output_path = match &output {
            Some(dir) if dir.is_dir() => {
                let ext = match target_enum {
                    Target::Rust => "rs",
                    Target::TypeScript => "ts",
                    Target::Kotlin => "kt",
                    Target::Python => "py",
                    Target::Java => "java",
                    Target::Clojure => "clj",
                    Target::Haskell => "hs",
                };
                dir.join(format!("{}.{}", to_snake_case(context.name()), ext))
            }
            Some(path) => path.clone(),
            None => {
                // Output to stdout
                println!("{}", code);
                continue;
            }
        };

        std::fs::write(&output_path, &code)
            .map_err(|e| format!("Failed to write output: {}", e))?;

        if verbosity != Verbosity::Quiet {
            println!(
                "  {} Generated {}",
                "✓".green().bold(),
                output_path.display()
            );
        }
    }

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

    // Read and parse file
    let source =
        std::fs::read_to_string(file).map_err(|e| format!("Failed to read file: {}", e))?;
    let ast = parse_file(&source).map_err(|e| format!("Parse error: {}", e))?;
    let transform_result = transform(&ast).map_err(|e| format!("Transform error: {}", e))?;

    // Generate visualization for each context
    for context in &transform_result.contexts {
        let viz = match format {
            "graphviz" | "dot" => sketchddd_viz::graphviz::generate(context)
                .map_err(|e| format!("Visualization error: {}", e))?,
            "mermaid" | "md" => sketchddd_viz::mermaid::generate(context)
                .map_err(|e| format!("Visualization error: {}", e))?,
            _ => return Err(format!("Unknown visualization format: {}. Supported: graphviz, mermaid", format)),
        };

        match &output {
            Some(path) => {
                std::fs::write(path, &viz)
                    .map_err(|e| format!("Failed to write output: {}", e))?;
                if verbosity != Verbosity::Quiet {
                    println!("  {} Generated {}", "✓".green().bold(), path.display());
                }
            }
            None => {
                println!("{}", viz);
            }
        }
    }

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

/// Handle template subcommands
fn cmd_template(cmd: TemplateCommands, verbosity: Verbosity) -> Result<(), String> {
    match cmd {
        TemplateCommands::List { remote } => cmd_template_list(remote, verbosity),
        TemplateCommands::Info { name } => cmd_template_info(&name, verbosity),
        TemplateCommands::Validate { path } => cmd_template_validate(&path, verbosity),
        TemplateCommands::Install { source, force } => cmd_template_install(&source, force, verbosity),
        TemplateCommands::UpdateTemplate { name } => cmd_template_update(&name, verbosity),
        TemplateCommands::Remove { name, force } => cmd_template_remove(&name, force, verbosity),
        TemplateCommands::Create { name, source, output } => cmd_template_create(&name, source, output, verbosity),
    }
}

/// Get templates directory
fn get_templates_dir() -> Result<PathBuf, String> {
    let home = dirs::home_dir().ok_or("Could not find home directory")?;
    let templates_dir = home.join(".sketchddd").join("templates");
    Ok(templates_dir)
}

/// List available templates
fn cmd_template_list(remote: bool, verbosity: Verbosity) -> Result<(), String> {
    if verbosity != Verbosity::Quiet {
        println!("{}", "Available Templates".cyan().bold());
        println!();
    }

    // Built-in templates
    println!("{}", "Built-in:".blue().bold());
    println!("  {} - Empty project with example comments", "minimal".green());
    println!("  {} - E-commerce domain with orders, products, customers", "ecommerce".green());
    println!("  {} - Multi-context architecture with context maps", "microservices".green());
    println!();

    // Installed templates
    let templates_dir = get_templates_dir()?;
    if templates_dir.exists() {
        let entries: Vec<_> = std::fs::read_dir(&templates_dir)
            .map_err(|e| format!("Failed to read templates directory: {}", e))?
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_dir())
            .collect();

        if !entries.is_empty() {
            println!("{}", "Installed:".blue().bold());
            for entry in entries {
                let name = entry.file_name().to_string_lossy().to_string();
                let desc = read_template_description(&entry.path()).unwrap_or_default();
                if desc.is_empty() {
                    println!("  {}", name.green());
                } else {
                    println!("  {} - {}", name.green(), desc);
                }
            }
            println!();
        }
    }

    // Remote templates (if --remote flag is set)
    if remote {
        println!("{}", "Remote Registry:".blue().bold());
        println!("  {} Fetching from registry...", "→".blue());
        // TODO: Implement actual registry fetch
        println!("  {} Registry not yet available", "⚠".yellow());
        println!();
    }

    Ok(())
}

/// Read template description from manifest
fn read_template_description(path: &PathBuf) -> Option<String> {
    let manifest = path.join("template.json");
    if manifest.exists() {
        if let Ok(content) = std::fs::read_to_string(&manifest) {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                return json.get("description").and_then(|d| d.as_str()).map(|s| s.to_string());
            }
        }
    }
    None
}

/// Show template info
fn cmd_template_info(name: &str, verbosity: Verbosity) -> Result<(), String> {
    let _ = verbosity;

    // Check built-in templates
    match name {
        "minimal" => {
            println!("{}", "Template: minimal".cyan().bold());
            println!();
            println!("{}: Built-in", "Source".blue());
            println!("{}: Empty project with commented examples", "Description".blue());
            println!("{}: Single bounded context structure", "Contents".blue());
            println!();
            println!("Usage: sketchddd init <project-name> --template minimal");
        }
        "ecommerce" => {
            println!("{}", "Template: ecommerce".cyan().bold());
            println!();
            println!("{}: Built-in", "Source".blue());
            println!("{}: E-commerce domain model", "Description".blue());
            println!("{}", "Contents:".blue());
            println!("  - Customer, Order, Product entities");
            println!("  - Money, Address value objects");
            println!("  - OrderAggregate with LineItems");
            println!("  - OrderStatus, PaymentStatus enums");
            println!();
            println!("Usage: sketchddd init <project-name> --template ecommerce");
        }
        "microservices" => {
            println!("{}", "Template: microservices".cyan().bold());
            println!();
            println!("{}: Built-in", "Source".blue());
            println!("{}: Multi-context microservices architecture", "Description".blue());
            println!("{}", "Contents:".blue());
            println!("  - Orders context");
            println!("  - Inventory context");
            println!("  - Shipping context");
            println!("  - Context maps with CustomerSupplier pattern");
            println!();
            println!("Usage: sketchddd init <project-name> --template microservices");
        }
        _ => {
            // Check installed templates
            let templates_dir = get_templates_dir()?;
            let template_path = templates_dir.join(name);

            if template_path.exists() {
                let manifest_path = template_path.join("template.json");
                if manifest_path.exists() {
                    let content = std::fs::read_to_string(&manifest_path)
                        .map_err(|e| format!("Failed to read manifest: {}", e))?;
                    let json: serde_json::Value = serde_json::from_str(&content)
                        .map_err(|e| format!("Invalid manifest JSON: {}", e))?;

                    println!("{}", format!("Template: {}", name).cyan().bold());
                    println!();

                    if let Some(desc) = json.get("description").and_then(|d| d.as_str()) {
                        println!("{}: {}", "Description".blue(), desc);
                    }
                    if let Some(author) = json.get("author").and_then(|a| a.as_str()) {
                        println!("{}: {}", "Author".blue(), author);
                    }
                    if let Some(version) = json.get("version").and_then(|v| v.as_str()) {
                        println!("{}: {}", "Version".blue(), version);
                    }
                } else {
                    println!("{}", format!("Template: {}", name).cyan().bold());
                    println!("{}: Installed (no manifest)", "Source".blue());
                }
            } else {
                return Err(format!("Template '{}' not found", name));
            }
        }
    }

    Ok(())
}

/// Validate a template
fn cmd_template_validate(path: &PathBuf, verbosity: Verbosity) -> Result<(), String> {
    if verbosity != Verbosity::Quiet {
        println!("{} {}", "Validating template".cyan().bold(), path.display());
    }

    let mut errors = Vec::new();
    let mut warnings = Vec::new();

    // Check if path exists
    if !path.exists() {
        return Err(format!("Path does not exist: {}", path.display()));
    }

    // Check for template.json manifest
    let manifest_path = if path.is_dir() {
        path.join("template.json")
    } else {
        path.with_file_name("template.json")
    };

    if !manifest_path.exists() {
        warnings.push("Missing template.json manifest (recommended)".to_string());
    } else {
        // Validate manifest
        let content = std::fs::read_to_string(&manifest_path)
            .map_err(|e| format!("Failed to read manifest: {}", e))?;

        match serde_json::from_str::<serde_json::Value>(&content) {
            Ok(json) => {
                if json.get("name").is_none() {
                    errors.push("Manifest missing 'name' field".to_string());
                }
                if json.get("description").is_none() {
                    warnings.push("Manifest missing 'description' field".to_string());
                }
            }
            Err(e) => {
                errors.push(format!("Invalid manifest JSON: {}", e));
            }
        }
    }

    // Check for .sddd template file
    let sddd_files: Vec<_> = if path.is_dir() {
        std::fs::read_dir(path)
            .map_err(|e| format!("Failed to read directory: {}", e))?
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.path()
                    .extension()
                    .map(|ext| ext == "sddd")
                    .unwrap_or(false)
            })
            .collect()
    } else if path.extension().map(|e| e == "sddd").unwrap_or(false) {
        vec![]  // Single file is the template
    } else {
        vec![]
    };

    if path.is_dir() && sddd_files.is_empty() {
        errors.push("No .sddd files found in template directory".to_string());
    }

    // Validate any .sddd files
    let files_to_check: Vec<PathBuf> = if path.is_dir() {
        sddd_files.into_iter().map(|e| e.path()).collect()
    } else {
        vec![path.clone()]
    };

    for file in files_to_check {
        if let Ok(content) = std::fs::read_to_string(&file) {
            if let Err(e) = parse_file(&content) {
                errors.push(format!("{}: Parse error - {}", file.display(), e));
            }
        }
    }

    // Report results
    if verbosity != Verbosity::Quiet {
        for warning in &warnings {
            println!("  {} {}", "warning:".yellow().bold(), warning);
        }
        for error in &errors {
            println!("  {} {}", "error:".red().bold(), error);
        }
    }

    if errors.is_empty() {
        if verbosity != Verbosity::Quiet {
            println!("{} Template is valid", "✓".green().bold());
        }
        Ok(())
    } else {
        Err(format!("Template validation failed with {} error(s)", errors.len()))
    }
}

/// Install a template
fn cmd_template_install(source: &str, force: bool, verbosity: Verbosity) -> Result<(), String> {
    if verbosity != Verbosity::Quiet {
        println!("{} {}", "Installing template".cyan().bold(), source);
    }

    let templates_dir = get_templates_dir()?;
    std::fs::create_dir_all(&templates_dir)
        .map_err(|e| format!("Failed to create templates directory: {}", e))?;

    // Determine if source is URL or local path
    if source.starts_with("http://") || source.starts_with("https://") {
        // TODO: Implement URL download
        println!("{} URL installation not yet implemented", "⚠".yellow().bold());
        return Ok(());
    }

    // Local path
    let source_path = PathBuf::from(source);
    if !source_path.exists() {
        return Err(format!("Source path does not exist: {}", source));
    }

    // Get template name
    let name = source_path
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or("Invalid source path")?;

    let dest_path = templates_dir.join(name);

    if dest_path.exists() && !force {
        return Err(format!(
            "Template '{}' already exists. Use --force to overwrite.",
            name
        ));
    }

    // Copy template
    if source_path.is_dir() {
        copy_dir_recursive(&source_path, &dest_path)?;
    } else {
        std::fs::create_dir_all(&dest_path)
            .map_err(|e| format!("Failed to create template directory: {}", e))?;
        std::fs::copy(&source_path, dest_path.join(source_path.file_name().unwrap()))
            .map_err(|e| format!("Failed to copy template: {}", e))?;
    }

    if verbosity != Verbosity::Quiet {
        println!("{} Installed template '{}'", "✓".green().bold(), name);
    }

    Ok(())
}

/// Recursively copy a directory
fn copy_dir_recursive(src: &PathBuf, dst: &PathBuf) -> Result<(), String> {
    std::fs::create_dir_all(dst).map_err(|e| format!("Failed to create directory: {}", e))?;

    for entry in std::fs::read_dir(src).map_err(|e| format!("Failed to read directory: {}", e))? {
        let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if src_path.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            std::fs::copy(&src_path, &dst_path)
                .map_err(|e| format!("Failed to copy file: {}", e))?;
        }
    }

    Ok(())
}

/// Update a template
fn cmd_template_update(name: &str, verbosity: Verbosity) -> Result<(), String> {
    if verbosity != Verbosity::Quiet {
        println!("{} {}", "Updating template".cyan().bold(), name);
    }

    if name == "all" {
        println!("{} Updating all templates...", "→".blue());
        // TODO: Implement update all
        println!("{} Template updates not yet implemented", "⚠".yellow().bold());
    } else {
        let templates_dir = get_templates_dir()?;
        let template_path = templates_dir.join(name);

        if !template_path.exists() {
            return Err(format!("Template '{}' is not installed", name));
        }

        // TODO: Check registry for updates and download
        println!("{} Template updates not yet implemented", "⚠".yellow().bold());
    }

    Ok(())
}

/// Remove a template
fn cmd_template_remove(name: &str, force: bool, verbosity: Verbosity) -> Result<(), String> {
    // Check for built-in templates
    if matches!(name, "minimal" | "ecommerce" | "microservices") {
        return Err(format!("Cannot remove built-in template '{}'", name));
    }

    let templates_dir = get_templates_dir()?;
    let template_path = templates_dir.join(name);

    if !template_path.exists() {
        return Err(format!("Template '{}' is not installed", name));
    }

    if !force {
        println!(
            "{} Remove template '{}'? This cannot be undone. [y/N]",
            "?".yellow().bold(),
            name
        );

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .map_err(|e| format!("Failed to read input: {}", e))?;

        if !input.trim().eq_ignore_ascii_case("y") {
            println!("Cancelled.");
            return Ok(());
        }
    }

    std::fs::remove_dir_all(&template_path)
        .map_err(|e| format!("Failed to remove template: {}", e))?;

    if verbosity != Verbosity::Quiet {
        println!("{} Removed template '{}'", "✓".green().bold(), name);
    }

    Ok(())
}

/// Create a new template
fn cmd_template_create(
    name: &str,
    source: Option<PathBuf>,
    output: Option<PathBuf>,
    verbosity: Verbosity,
) -> Result<(), String> {
    if verbosity != Verbosity::Quiet {
        println!("{} {}", "Creating template".cyan().bold(), name);
    }

    // Determine output directory
    let output_dir = output.unwrap_or_else(|| PathBuf::from(name));
    std::fs::create_dir_all(&output_dir)
        .map_err(|e| format!("Failed to create directory: {}", e))?;

    // Create template manifest
    let manifest = serde_json::json!({
        "name": name,
        "version": "1.0.0",
        "description": format!("{} domain model template", name),
        "author": "",
        "license": "MIT",
        "sketchddd": ">=0.1.0"
    });

    std::fs::write(
        output_dir.join("template.json"),
        serde_json::to_string_pretty(&manifest).unwrap(),
    )
    .map_err(|e| format!("Failed to write manifest: {}", e))?;

    // Copy or create template .sddd file
    let sddd_content = if let Some(source_path) = source {
        std::fs::read_to_string(&source_path)
            .map_err(|e| format!("Failed to read source file: {}", e))?
    } else {
        get_minimal_template(name)
    };

    std::fs::write(output_dir.join(format!("{}.sddd", name.to_lowercase())), sddd_content)
        .map_err(|e| format!("Failed to write template file: {}", e))?;

    // Create README
    let readme = format!(
        r#"# {} Template

A SketchDDD domain model template.

## Usage

```bash
sketchddd init my-project --template {}
```

## Contents

- `{}.sddd` - Main domain model file
- `template.json` - Template metadata

## License

MIT
"#,
        name, name, name.to_lowercase()
    );

    std::fs::write(output_dir.join("README.md"), readme)
        .map_err(|e| format!("Failed to write README: {}", e))?;

    if verbosity != Verbosity::Quiet {
        println!("{} Created template in {}/", "✓".green().bold(), output_dir.display());
        println!("  {} template.json", "→".blue());
        println!("  {} {}.sddd", "→".blue(), name.to_lowercase());
        println!("  {} README.md", "→".blue());
    }

    Ok(())
}

/// Check for updates
fn cmd_update(check_only: bool, verbosity: Verbosity) -> Result<(), String> {
    let current_version = env!("CARGO_PKG_VERSION");

    if verbosity != Verbosity::Quiet {
        println!("{} version {}", "SketchDDD".cyan().bold(), current_version);
    }

    // TODO: Implement actual version check from registry/GitHub
    println!("{} Checking for updates...", "→".blue());

    // Simulated check - in real implementation, fetch from GitHub releases API
    let latest_version = current_version; // Would be fetched from remote

    if latest_version == current_version {
        println!("{} You are running the latest version!", "✓".green().bold());
    } else {
        println!(
            "{} New version {} available (current: {})",
            "⚠".yellow().bold(),
            latest_version,
            current_version
        );

        if !check_only {
            println!();
            println!("To update, run:");
            println!("  {} install sketchddd", "cargo".cyan());
        }
    }

    Ok(())
}

/// Convert PascalCase to snake_case
fn to_snake_case(s: &str) -> String {
    let mut result = String::with_capacity(s.len() + 4);
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() {
            if i > 0 {
                result.push('_');
            }
            result.push(c.to_ascii_lowercase());
        } else {
            result.push(c);
        }
    }
    result
}
