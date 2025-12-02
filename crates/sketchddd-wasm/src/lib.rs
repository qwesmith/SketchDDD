//! # SketchDDD WASM
//!
//! WebAssembly bindings for use in the browser-based visual builder.
//!
//! This crate provides a JavaScript/TypeScript API for:
//! - Parsing SketchDDD source files
//! - Transforming AST to semantic models
//! - Validating bounded contexts and context maps
//! - Generating code in multiple languages
//! - Generating visualizations (Mermaid, Graphviz)
//!
//! ## Usage from JavaScript
//!
//! ```javascript
//! import init, { parse, validate, generateCode, generateViz } from 'sketchddd-wasm';
//!
//! await init();
//!
//! const source = `
//!   context Commerce {
//!     entity Customer { id: UUID, name: String }
//!     entity Order { id: UUID, status: OrderStatus }
//!     enum OrderStatus = Pending | Confirmed | Shipped
//!   }
//! `;
//!
//! // Parse and validate
//! const result = parse(source);
//! if (result.success) {
//!   const validation = validate(result.data);
//!   console.log('Validation issues:', validation.issues);
//!
//!   // Generate TypeScript code
//!   const tsCode = generateCode(result.data, 'typescript');
//!   console.log(tsCode);
//!
//!   // Generate Mermaid diagram
//!   const diagram = generateViz(result.data, 'mermaid');
//!   console.log(diagram);
//! }
//! ```

use serde::{Deserialize, Serialize};
use sketchddd_core::{BoundedContext, Severity};
use sketchddd_parser::{parse_file, transform, PrettyPrint};
use wasm_bindgen::prelude::*;

/// Initialize the WASM module.
#[wasm_bindgen(start)]
pub fn init() {
    // Set up panic hook for better error messages
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

// =============================================================
// Result Types for JS
// =============================================================

/// Result type for parsing operations
#[derive(Serialize, Deserialize)]
pub struct ParseResult {
    pub success: bool,
    pub data: Option<ParsedModel>,
    pub error: Option<String>,
}

/// Parsed model data
#[derive(Serialize, Deserialize)]
pub struct ParsedModel {
    pub contexts: Vec<ContextInfo>,
    pub context_maps: Vec<ContextMapInfo>,
    pub warnings: Vec<WarningInfo>,
}

/// Information about a bounded context
#[derive(Serialize, Deserialize)]
pub struct ContextInfo {
    pub name: String,
    pub entities: Vec<EntityInfo>,
    pub value_objects: Vec<ValueObjectInfo>,
    pub aggregates: Vec<AggregateInfo>,
    pub enums: Vec<EnumInfo>,
    pub morphisms: Vec<MorphismInfo>,
    pub objects: Vec<String>,
}

/// Entity information
#[derive(Serialize, Deserialize)]
pub struct EntityInfo {
    pub name: String,
    pub fields: Vec<FieldInfo>,
}

/// Value object information
#[derive(Serialize, Deserialize)]
pub struct ValueObjectInfo {
    pub name: String,
    pub fields: Vec<FieldInfo>,
}

/// Field information
#[derive(Serialize, Deserialize)]
pub struct FieldInfo {
    pub name: String,
    pub type_name: String,
    pub optional: bool,
}

/// Aggregate information
#[derive(Serialize, Deserialize)]
pub struct AggregateInfo {
    pub name: String,
    pub root: Option<String>,
    pub contains: Vec<String>,
}

/// Enum information
#[derive(Serialize, Deserialize)]
pub struct EnumInfo {
    pub name: String,
    pub variants: Vec<VariantInfo>,
}

/// Variant information
#[derive(Serialize, Deserialize)]
pub struct VariantInfo {
    pub name: String,
    pub has_payload: bool,
}

/// Morphism information
#[derive(Serialize, Deserialize)]
pub struct MorphismInfo {
    pub name: String,
    pub source: String,
    pub target: String,
}

/// Context map information
#[derive(Serialize, Deserialize)]
pub struct ContextMapInfo {
    pub name: String,
    pub source_context: String,
    pub target_context: String,
    pub pattern: Option<String>,
    pub mappings: Vec<MappingInfo>,
}

/// Mapping information
#[derive(Serialize, Deserialize)]
pub struct MappingInfo {
    pub source: String,
    pub target: String,
}

/// Warning information
#[derive(Serialize, Deserialize)]
pub struct WarningInfo {
    pub message: String,
    pub line: Option<u32>,
    pub column: Option<u32>,
}

/// Validation result for JS
#[derive(Serialize, Deserialize)]
pub struct JsValidationResult {
    pub valid: bool,
    pub error_count: usize,
    pub warning_count: usize,
    pub issues: Vec<JsValidationIssue>,
}

/// Validation issue for JS
#[derive(Serialize, Deserialize)]
pub struct JsValidationIssue {
    pub severity: String,
    pub code: String,
    pub message: String,
    pub context: Option<String>,
    pub line: Option<u32>,
    pub column: Option<u32>,
    pub suggestion: Option<String>,
}

/// Code generation result
#[derive(Serialize, Deserialize)]
pub struct CodegenResult {
    pub success: bool,
    pub code: Option<String>,
    pub error: Option<String>,
}

/// Visualization result
#[derive(Serialize, Deserialize)]
pub struct VizResult {
    pub success: bool,
    pub output: Option<String>,
    pub error: Option<String>,
}

// =============================================================
// Core Functions
// =============================================================

/// Get the version of the WASM module.
#[wasm_bindgen]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

/// Parse a SketchDDD source file and return structured data.
#[wasm_bindgen]
pub fn parse(source: &str) -> JsValue {
    match parse_and_transform(source) {
        Ok(model) => {
            let result = ParseResult {
                success: true,
                data: Some(model),
                error: None,
            };
            serde_wasm_bindgen::to_value(&result).unwrap_or(JsValue::NULL)
        }
        Err(e) => {
            let result = ParseResult {
                success: false,
                data: None,
                error: Some(e),
            };
            serde_wasm_bindgen::to_value(&result).unwrap_or(JsValue::NULL)
        }
    }
}

/// Parse and transform source into a model
fn parse_and_transform(source: &str) -> Result<ParsedModel, String> {
    // Parse to AST
    let ast = parse_file(source).map_err(|e| e.to_string())?;

    // Transform to semantic model
    let transform_result = transform(&ast).map_err(|e| e.to_string())?;

    // Convert to JS-friendly format
    let warnings: Vec<WarningInfo> = transform_result
        .warnings
        .iter()
        .map(|w| WarningInfo {
            message: w.message.clone(),
            line: w.line,
            column: w.column,
        })
        .collect();

    let contexts: Vec<ContextInfo> = transform_result
        .contexts
        .iter()
        .map(context_to_info)
        .collect();

    let context_maps: Vec<ContextMapInfo> = transform_result
        .context_maps
        .iter()
        .map(|map| ContextMapInfo {
            name: map.name().to_string(),
            source_context: map.source_context().to_string(),
            target_context: map.target_context().to_string(),
            pattern: Some(format!("{:?}", map.pattern())),
            mappings: map
                .object_mappings()
                .iter()
                .map(|m| MappingInfo {
                    source: m.source.clone(),
                    target: m.target.clone(),
                })
                .collect(),
        })
        .collect();

    let model = ParsedModel {
        contexts,
        context_maps,
        warnings,
    };

    Ok(model)
}

/// Convert a BoundedContext to ContextInfo
fn context_to_info(ctx: &BoundedContext) -> ContextInfo {
    let graph = ctx.graph();

    // Helper to get object name
    let get_name = |id: sketchddd_core::sketch::ObjectId| -> String {
        graph
            .get_object(id)
            .map(|o| o.name.clone())
            .unwrap_or_else(|| "Unknown".to_string())
    };

    // Get entities
    let entities: Vec<EntityInfo> = ctx
        .entities()
        .iter()
        .map(|id| {
            let name = get_name(*id);
            EntityInfo {
                name,
                fields: Vec::new(), // Fields would need to be stored separately
            }
        })
        .collect();

    // Get value objects
    let value_objects: Vec<ValueObjectInfo> = ctx
        .value_objects()
        .iter()
        .map(|id| {
            let name = get_name(*id);
            ValueObjectInfo {
                name,
                fields: Vec::new(),
            }
        })
        .collect();

    // Get aggregates
    let aggregates: Vec<AggregateInfo> = ctx
        .aggregate_roots()
        .iter()
        .map(|id| {
            let name = get_name(*id);
            AggregateInfo {
                name: name.clone(),
                root: Some(name),
                contains: Vec::new(),
            }
        })
        .collect();

    // Get enums from colimits
    let enums: Vec<EnumInfo> = ctx
        .sketch()
        .colimits
        .iter()
        .map(|colimit| {
            let name = colimit.name.clone();
            let variants: Vec<VariantInfo> = colimit
                .injections
                .iter()
                .map(|injection| VariantInfo {
                    name: injection.name.clone(),
                    has_payload: false,
                })
                .collect();
            EnumInfo { name, variants }
        })
        .collect();

    // Get morphisms
    let morphisms: Vec<MorphismInfo> = graph
        .morphisms()
        .filter(|m| !m.name.starts_with("id_")) // Filter out identity morphisms
        .map(|m| MorphismInfo {
            name: m.name.clone(),
            source: get_name(m.source),
            target: get_name(m.target),
        })
        .collect();

    // Get objects
    let objects: Vec<String> = graph.objects().map(|obj| obj.name.clone()).collect();

    ContextInfo {
        name: ctx.name().to_string(),
        entities,
        value_objects,
        aggregates,
        enums,
        morphisms,
        objects,
    }
}

/// Validate a parsed model and return validation issues.
#[wasm_bindgen]
pub fn validate(model_json: &str) -> JsValue {
    let result: Result<JsValidationResult, String> = (|| {
        let contexts: Vec<BoundedContext> =
            serde_json::from_str(model_json).map_err(|e| format!("Invalid model JSON: {}", e))?;

        let validation = sketchddd_core::validate_model(&contexts, &[]);

        let issues: Vec<JsValidationIssue> = validation
            .issues
            .iter()
            .map(|issue| JsValidationIssue {
                severity: match issue.severity {
                    Severity::Error => "error".to_string(),
                    Severity::Warning => "warning".to_string(),
                    Severity::Hint => "hint".to_string(),
                },
                code: issue.code.clone(),
                message: issue.message.clone(),
                context: issue.location.file.clone(),
                line: issue.location.line,
                column: issue.location.column,
                suggestion: issue.suggestion.clone(),
            })
            .collect();

        Ok(JsValidationResult {
            valid: validation.is_ok(),
            error_count: validation.error_count(),
            warning_count: validation.warning_count(),
            issues,
        })
    })();

    match result {
        Ok(r) => serde_wasm_bindgen::to_value(&r).unwrap_or(JsValue::NULL),
        Err(e) => {
            let error_result = JsValidationResult {
                valid: false,
                error_count: 1,
                warning_count: 0,
                issues: vec![JsValidationIssue {
                    severity: "error".to_string(),
                    code: "PARSE_ERROR".to_string(),
                    message: e,
                    context: None,
                    line: None,
                    column: None,
                    suggestion: None,
                }],
            };
            serde_wasm_bindgen::to_value(&error_result).unwrap_or(JsValue::NULL)
        }
    }
}

/// Validate source directly without pre-parsing.
#[wasm_bindgen]
pub fn validate_source(source: &str) -> JsValue {
    let result: Result<JsValidationResult, String> = (|| {
        // Parse
        let ast = parse_file(source).map_err(|e| e.to_string())?;

        // Transform
        let transform_result = transform(&ast).map_err(|e| e.to_string())?;

        // Validate
        let validation = sketchddd_core::validate_model(
            &transform_result.contexts,
            &transform_result.context_maps,
        );

        let mut issues: Vec<JsValidationIssue> = Vec::new();

        // Add transform warnings
        for warning in &transform_result.warnings {
            issues.push(JsValidationIssue {
                severity: "warning".to_string(),
                code: "TRANSFORM_WARNING".to_string(),
                message: warning.message.clone(),
                context: None,
                line: warning.line,
                column: warning.column,
                suggestion: None,
            });
        }

        // Add validation issues
        for issue in &validation.issues {
            issues.push(JsValidationIssue {
                severity: match issue.severity {
                    Severity::Error => "error".to_string(),
                    Severity::Warning => "warning".to_string(),
                    Severity::Hint => "hint".to_string(),
                },
                code: issue.code.clone(),
                message: issue.message.clone(),
                context: issue.location.file.clone(),
                line: issue.location.line,
                column: issue.location.column,
                suggestion: issue.suggestion.clone(),
            });
        }

        let error_count = issues.iter().filter(|i| i.severity == "error").count();
        let warning_count = issues.iter().filter(|i| i.severity == "warning").count();

        Ok(JsValidationResult {
            valid: error_count == 0,
            error_count,
            warning_count,
            issues,
        })
    })();

    match result {
        Ok(r) => serde_wasm_bindgen::to_value(&r).unwrap_or(JsValue::NULL),
        Err(e) => {
            let error_result = JsValidationResult {
                valid: false,
                error_count: 1,
                warning_count: 0,
                issues: vec![JsValidationIssue {
                    severity: "error".to_string(),
                    code: "PARSE_ERROR".to_string(),
                    message: e,
                    context: None,
                    line: None,
                    column: None,
                    suggestion: None,
                }],
            };
            serde_wasm_bindgen::to_value(&error_result).unwrap_or(JsValue::NULL)
        }
    }
}

/// Generate code from a SketchDDD source.
///
/// Supported targets: rust, typescript, kotlin, python, java, clojure, haskell
#[wasm_bindgen]
pub fn generate_code(source: &str, target: &str) -> JsValue {
    let result: Result<CodegenResult, String> = (|| {
        // Parse and transform
        let ast = parse_file(source).map_err(|e| e.to_string())?;
        let transform_result = transform(&ast).map_err(|e| e.to_string())?;

        // Parse target
        let target_enum: sketchddd_codegen::Target = target.parse().map_err(|_| {
            format!(
                "Unknown target: {}. Supported: rust, typescript, kotlin, python, java, clojure, haskell",
                target
            )
        })?;

        // Generate code for all contexts
        let mut all_code = String::new();
        for (i, context) in transform_result.contexts.iter().enumerate() {
            if i > 0 {
                all_code.push_str(
                    "\n\n// =============================================================\n\n",
                );
            }
            let code =
                sketchddd_codegen::generate(context, target_enum).map_err(|e| e.to_string())?;
            all_code.push_str(&code);
        }

        Ok(CodegenResult {
            success: true,
            code: Some(all_code),
            error: None,
        })
    })();

    match result {
        Ok(r) => serde_wasm_bindgen::to_value(&r).unwrap_or(JsValue::NULL),
        Err(e) => {
            let error_result = CodegenResult {
                success: false,
                code: None,
                error: Some(e),
            };
            serde_wasm_bindgen::to_value(&error_result).unwrap_or(JsValue::NULL)
        }
    }
}

/// Generate visualization from a SketchDDD source.
///
/// Supported formats: mermaid, graphviz (or dot)
#[wasm_bindgen]
pub fn generate_viz(source: &str, format: &str) -> JsValue {
    let result: Result<VizResult, String> = (|| {
        // Parse and transform
        let ast = parse_file(source).map_err(|e| e.to_string())?;
        let transform_result = transform(&ast).map_err(|e| e.to_string())?;

        // Generate visualization for all contexts
        let mut all_output = String::new();
        for (i, context) in transform_result.contexts.iter().enumerate() {
            if i > 0 {
                all_output.push_str("\n\n");
            }
            let viz = match format.to_lowercase().as_str() {
                "mermaid" | "md" => {
                    sketchddd_viz::mermaid::generate(context).map_err(|e| e.to_string())?
                }
                "graphviz" | "dot" => {
                    sketchddd_viz::graphviz::generate(context).map_err(|e| e.to_string())?
                }
                _ => {
                    return Err(format!(
                        "Unknown format: {}. Supported: mermaid, graphviz",
                        format
                    ))
                }
            };
            all_output.push_str(&viz);
        }

        Ok(VizResult {
            success: true,
            output: Some(all_output),
            error: None,
        })
    })();

    match result {
        Ok(r) => serde_wasm_bindgen::to_value(&r).unwrap_or(JsValue::NULL),
        Err(e) => {
            let error_result = VizResult {
                success: false,
                output: None,
                error: Some(e),
            };
            serde_wasm_bindgen::to_value(&error_result).unwrap_or(JsValue::NULL)
        }
    }
}

/// Create a new bounded context.
#[wasm_bindgen]
pub fn create_context(name: &str) -> JsValue {
    let context = BoundedContext::new(name);
    serde_wasm_bindgen::to_value(&context).unwrap_or(JsValue::NULL)
}

/// Get list of supported code generation targets.
#[wasm_bindgen]
pub fn supported_targets() -> JsValue {
    let targets = vec![
        "rust",
        "typescript",
        "kotlin",
        "python",
        "java",
        "clojure",
        "haskell",
    ];
    serde_wasm_bindgen::to_value(&targets).unwrap_or(JsValue::NULL)
}

/// Get list of supported visualization formats.
#[wasm_bindgen]
pub fn supported_viz_formats() -> JsValue {
    let formats = vec!["mermaid", "graphviz"];
    serde_wasm_bindgen::to_value(&formats).unwrap_or(JsValue::NULL)
}

/// Format source code (pretty print).
#[wasm_bindgen]
pub fn format_source(source: &str) -> JsValue {
    let result: Result<String, String> = (|| {
        let ast = parse_file(source).map_err(|e| e.to_string())?;
        Ok(ast.pretty_print())
    })();

    match result {
        Ok(formatted) => JsValue::from_str(&formatted),
        Err(e) => JsValue::from_str(&format!("Error: {}", e)),
    }
}

// =============================================================
// Tests
// =============================================================

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn test_version() {
        let v = version();
        assert!(!v.is_empty());
    }

    #[wasm_bindgen_test]
    fn test_parse_valid() {
        let source = r#"
            context Commerce {
                entity Customer {
                    id: UUID
                    name: String
                }
            }
        "#;

        let result = parse(source);
        assert!(!result.is_null());
    }

    #[wasm_bindgen_test]
    fn test_parse_invalid() {
        let source = "this is not valid {{{";
        let result = parse(source);
        assert!(!result.is_null());
    }

    #[wasm_bindgen_test]
    fn test_supported_targets() {
        let targets = supported_targets();
        assert!(!targets.is_null());
    }

    #[wasm_bindgen_test]
    fn test_supported_viz_formats() {
        let formats = supported_viz_formats();
        assert!(!formats.is_null());
    }
}
