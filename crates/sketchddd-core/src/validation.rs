//! Validation logic for sketches and bounded contexts.
//!
//! This module provides comprehensive validation for SketchDDD models:
//! - Object name uniqueness
//! - Morphism source/target existence
//! - Aggregate structure validation
//! - Value object field validation
//! - Enum variant uniqueness
//! - Context map reference validation

use crate::context::BoundedContext;
use crate::mapping::NamedContextMap;
use crate::sketch::{ObjectId, Sketch};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use thiserror::Error;

/// Location in source code for error reporting.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SourceLocation {
    /// File path
    pub file: Option<String>,
    /// Line number (1-indexed)
    pub line: Option<u32>,
    /// Column number (1-indexed)
    pub column: Option<u32>,
}

impl SourceLocation {
    /// Create a new source location.
    pub fn new(file: impl Into<String>, line: u32, column: u32) -> Self {
        Self {
            file: Some(file.into()),
            line: Some(line),
            column: Some(column),
        }
    }
}

/// The severity of a validation issue.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Severity {
    /// Error: Must be fixed
    Error,
    /// Warning: Should be reviewed
    Warning,
    /// Hint: Suggestion for improvement
    Hint,
}

/// A validation error or warning.
#[derive(Debug, Clone, Error, Serialize, Deserialize)]
#[error("{message}")]
pub struct ValidationError {
    /// Error code (e.g., "E0001")
    pub code: String,

    /// Human-readable message
    pub message: String,

    /// Severity level
    pub severity: Severity,

    /// Location in source
    pub location: SourceLocation,

    /// Suggested fix
    pub suggestion: Option<String>,
}

impl ValidationError {
    /// Create a new error.
    pub fn error(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            severity: Severity::Error,
            location: SourceLocation::default(),
            suggestion: None,
        }
    }

    /// Create a new warning.
    pub fn warning(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            severity: Severity::Warning,
            location: SourceLocation::default(),
            suggestion: None,
        }
    }

    /// Add a location to this error.
    pub fn with_location(mut self, location: SourceLocation) -> Self {
        self.location = location;
        self
    }

    /// Add a suggestion to this error.
    pub fn with_suggestion(mut self, suggestion: impl Into<String>) -> Self {
        self.suggestion = Some(suggestion.into());
        self
    }
}

/// Result of validating a sketch.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ValidationResult {
    /// List of errors and warnings
    pub issues: Vec<ValidationError>,
}

impl ValidationResult {
    /// Create a new empty result.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an issue.
    pub fn add(&mut self, error: ValidationError) {
        self.issues.push(error);
    }

    /// Check if validation passed (no errors).
    pub fn is_ok(&self) -> bool {
        !self.issues.iter().any(|e| e.severity == Severity::Error)
    }

    /// Check if there are any issues.
    pub fn has_issues(&self) -> bool {
        !self.issues.is_empty()
    }

    /// Get only errors.
    pub fn errors(&self) -> impl Iterator<Item = &ValidationError> {
        self.issues
            .iter()
            .filter(|e| e.severity == Severity::Error)
    }

    /// Get only warnings.
    pub fn warnings(&self) -> impl Iterator<Item = &ValidationError> {
        self.issues
            .iter()
            .filter(|e| e.severity == Severity::Warning)
    }

    /// Count errors.
    pub fn error_count(&self) -> usize {
        self.errors().count()
    }

    /// Count warnings.
    pub fn warning_count(&self) -> usize {
        self.warnings().count()
    }
}

/// Validate a sketch for basic consistency.
pub fn validate_sketch(sketch: &Sketch) -> ValidationResult {
    let mut result = ValidationResult::new();

    // Check that morphism sources and targets exist
    for morphism in sketch.graph.morphisms() {
        if sketch.graph.get_object(morphism.source).is_none() {
            result.add(ValidationError::error(
                "E0001",
                format!(
                    "Morphism '{}' references non-existent source object",
                    morphism.name
                ),
            ));
        }
        if sketch.graph.get_object(morphism.target).is_none() {
            result.add(ValidationError::error(
                "E0002",
                format!(
                    "Morphism '{}' references non-existent target object",
                    morphism.name
                ),
            ));
        }
    }

    // Check that equations are well-formed
    for equation in &sketch.equations {
        if !equation.is_well_formed() {
            result.add(ValidationError::error(
                "E0010",
                format!(
                    "Equation '{}' is not well-formed: paths have different sources or targets",
                    equation.name
                ),
            ));
        }
    }

    // Check for duplicate object names
    let mut seen_names: std::collections::HashSet<&str> = std::collections::HashSet::new();
    for object in sketch.graph.objects() {
        if !seen_names.insert(&object.name) {
            result.add(ValidationError::error(
                "E0020",
                format!("Duplicate object name: '{}'", object.name),
            ));
        }
    }

    // Warn about potentially large aggregates
    for limit in &sketch.limits {
        if limit.is_aggregate && limit.projections.len() > 5 {
            result.add(
                ValidationError::warning(
                    "W0001",
                    format!(
                        "Aggregate '{}' contains {} objects, which may be too large",
                        limit.name,
                        limit.projections.len()
                    ),
                )
                .with_suggestion("Consider splitting into smaller aggregates"),
            );
        }
    }

    result
}

/// Validate that an object exists in a sketch.
pub fn object_exists(sketch: &Sketch, id: ObjectId) -> bool {
    sketch.graph.get_object(id).is_some()
}

// =============================================================
// BoundedContext Validation
// =============================================================

/// Validate a bounded context for consistency.
///
/// This performs comprehensive validation including:
/// - Object name uniqueness
/// - Morphism source/target existence
/// - Aggregate root validity
/// - Aggregate member validity
/// - Value object structure
/// - Enum variant uniqueness
pub fn validate_context(context: &BoundedContext) -> ValidationResult {
    let mut result = ValidationResult::new();

    // First validate the underlying sketch
    let sketch_result = validate_sketch(context.sketch());
    for issue in sketch_result.issues {
        result.add(issue);
    }

    // Validate aggregate roots exist
    validate_aggregate_roots(context, &mut result);

    // Validate aggregate members
    validate_aggregate_members(context, &mut result);

    // Validate entities have identity morphisms
    validate_entity_identities(context, &mut result);

    // Validate value objects have limit cones
    validate_value_objects(context, &mut result);

    // Validate enum variants are unique
    validate_enum_variants(context, &mut result);

    result
}

/// Validate that aggregate roots are valid objects.
fn validate_aggregate_roots(context: &BoundedContext, result: &mut ValidationResult) {
    for &root_id in context.aggregate_roots() {
        if context.graph().get_object(root_id).is_none() {
            result.add(ValidationError::error(
                "E0030",
                format!("Aggregate root references non-existent object (id: {:?})", root_id),
            ));
        }
    }
}

/// Validate that aggregate members are valid objects.
fn validate_aggregate_members(context: &BoundedContext, result: &mut ValidationResult) {
    for limit in &context.sketch().limits {
        if limit.is_aggregate {
            // Check root
            if let Some(root_id) = limit.root {
                if context.graph().get_object(root_id).is_none() {
                    result.add(ValidationError::error(
                        "E0031",
                        format!(
                            "Aggregate '{}' has root that references non-existent object",
                            limit.name
                        ),
                    ));
                }
            }

            // Check projections point to valid objects
            for projection in &limit.projections {
                if context.graph().get_object(projection.target).is_none() {
                    result.add(ValidationError::error(
                        "E0032",
                        format!(
                            "Aggregate '{}' contains reference to non-existent object",
                            limit.name
                        ),
                    ));
                }
            }
        }
    }
}

/// Validate that entities have proper identity morphisms.
fn validate_entity_identities(context: &BoundedContext, result: &mut ValidationResult) {
    for &entity_id in context.entities() {
        if context.get_entity_identity(entity_id).is_none() {
            if let Some(obj) = context.graph().get_object(entity_id) {
                result.add(ValidationError::error(
                    "E0040",
                    format!("Entity '{}' is missing its identity morphism", obj.name),
                ));
            }
        }
    }
}

/// Validate value objects have proper limit cones.
fn validate_value_objects(context: &BoundedContext, result: &mut ValidationResult) {
    for &vo_id in context.value_objects() {
        let has_limit = context
            .sketch()
            .limits
            .iter()
            .any(|l| !l.is_aggregate && l.apex == vo_id);

        if !has_limit {
            if let Some(obj) = context.graph().get_object(vo_id) {
                result.add(ValidationError::warning(
                    "W0010",
                    format!(
                        "Value object '{}' does not have an associated limit cone",
                        obj.name
                    ),
                ));
            }
        }
    }
}

/// Validate enum variants are unique within each enum.
fn validate_enum_variants(context: &BoundedContext, result: &mut ValidationResult) {
    for colimit in &context.sketch().colimits {
        let mut seen_variants: HashSet<&str> = HashSet::new();

        for injection in &colimit.injections {
            if !seen_variants.insert(&injection.name) {
                result.add(ValidationError::error(
                    "E0050",
                    format!(
                        "Enum '{}' has duplicate variant: '{}'",
                        colimit.name, injection.name
                    ),
                ));
            }
        }
    }
}

// =============================================================
// Context Map Validation
// =============================================================

/// Validate a context map against a set of bounded contexts.
///
/// This checks that:
/// - Source and target contexts exist
/// - Mapped objects exist in their respective contexts
/// - Mapped morphisms exist in their respective contexts
pub fn validate_context_map(
    context_map: &NamedContextMap,
    contexts: &HashMap<String, &BoundedContext>,
) -> ValidationResult {
    let mut result = ValidationResult::new();

    // Check source context exists
    let source_ctx = contexts.get(context_map.source_context());
    if source_ctx.is_none() {
        result.add(
            ValidationError::error(
                "E0060",
                format!(
                    "Context map '{}' references non-existent source context: '{}'",
                    context_map.name(),
                    context_map.source_context()
                ),
            )
            .with_suggestion(format!(
                "Define a context named '{}' or check for typos",
                context_map.source_context()
            )),
        );
    }

    // Check target context exists
    let target_ctx = contexts.get(context_map.target_context());
    if target_ctx.is_none() {
        result.add(
            ValidationError::error(
                "E0061",
                format!(
                    "Context map '{}' references non-existent target context: '{}'",
                    context_map.name(),
                    context_map.target_context()
                ),
            )
            .with_suggestion(format!(
                "Define a context named '{}' or check for typos",
                context_map.target_context()
            )),
        );
    }

    // Validate object mappings if both contexts exist
    if let (Some(source), Some(target)) = (source_ctx, target_ctx) {
        validate_object_mappings(context_map, source, target, &mut result);
        validate_morphism_mappings(context_map, source, target, &mut result);
    }

    result
}

/// Validate object mappings in a context map.
fn validate_object_mappings(
    context_map: &NamedContextMap,
    source_ctx: &BoundedContext,
    target_ctx: &BoundedContext,
    result: &mut ValidationResult,
) {
    for mapping in context_map.object_mappings() {
        // Check source object exists
        if source_ctx.graph().find_object_by_name(&mapping.source).is_none() {
            result.add(
                ValidationError::error(
                    "E0062",
                    format!(
                        "Object mapping in '{}' references non-existent source object: '{}'",
                        context_map.name(),
                        mapping.source
                    ),
                )
                .with_suggestion(format!(
                    "Check that '{}' is defined in context '{}'",
                    mapping.source,
                    context_map.source_context()
                )),
            );
        }

        // Check target object exists
        if target_ctx.graph().find_object_by_name(&mapping.target).is_none() {
            result.add(
                ValidationError::error(
                    "E0063",
                    format!(
                        "Object mapping in '{}' references non-existent target object: '{}'",
                        context_map.name(),
                        mapping.target
                    ),
                )
                .with_suggestion(format!(
                    "Check that '{}' is defined in context '{}'",
                    mapping.target,
                    context_map.target_context()
                )),
            );
        }
    }
}

/// Validate morphism mappings in a context map.
fn validate_morphism_mappings(
    context_map: &NamedContextMap,
    source_ctx: &BoundedContext,
    target_ctx: &BoundedContext,
    result: &mut ValidationResult,
) {
    for mapping in context_map.morphism_mappings() {
        // Check source morphism exists
        if source_ctx.graph().find_morphism_by_name(&mapping.source).is_none() {
            result.add(
                ValidationError::error(
                    "E0064",
                    format!(
                        "Morphism mapping in '{}' references non-existent source morphism: '{}'",
                        context_map.name(),
                        mapping.source
                    ),
                )
                .with_suggestion(format!(
                    "Check that morphism '{}' is defined in context '{}'",
                    mapping.source,
                    context_map.source_context()
                )),
            );
        }

        // Check target morphism exists
        if target_ctx.graph().find_morphism_by_name(&mapping.target).is_none() {
            result.add(
                ValidationError::error(
                    "E0065",
                    format!(
                        "Morphism mapping in '{}' references non-existent target morphism: '{}'",
                        context_map.name(),
                        mapping.target
                    ),
                )
                .with_suggestion(format!(
                    "Check that morphism '{}' is defined in context '{}'",
                    mapping.target,
                    context_map.target_context()
                )),
            );
        }
    }
}

// =============================================================
// Full Model Validation
// =============================================================

/// Validate a complete model with multiple contexts and context maps.
pub fn validate_model(
    contexts: &[BoundedContext],
    context_maps: &[NamedContextMap],
) -> ValidationResult {
    let mut result = ValidationResult::new();

    // Build context lookup
    let context_lookup: HashMap<String, &BoundedContext> = contexts
        .iter()
        .map(|c| (c.name().to_string(), c))
        .collect();

    // Check for duplicate context names
    let mut seen_context_names: HashSet<&str> = HashSet::new();
    for ctx in contexts {
        if !seen_context_names.insert(ctx.name()) {
            result.add(ValidationError::error(
                "E0070",
                format!("Duplicate context name: '{}'", ctx.name()),
            ));
        }
    }

    // Validate each context
    for ctx in contexts {
        let ctx_result = validate_context(ctx);
        for mut issue in ctx_result.issues {
            // Prefix error messages with context name
            issue.message = format!("[{}] {}", ctx.name(), issue.message);
            result.add(issue);
        }
    }

    // Check for duplicate context map names
    let mut seen_map_names: HashSet<&str> = HashSet::new();
    for map in context_maps {
        if !seen_map_names.insert(map.name()) {
            result.add(ValidationError::error(
                "E0071",
                format!("Duplicate context map name: '{}'", map.name()),
            ));
        }
    }

    // Validate each context map
    for map in context_maps {
        let map_result = validate_context_map(map, &context_lookup);
        for issue in map_result.issues {
            result.add(issue);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mapping::{NamedObjectMapping, NamedMorphismMapping, RelationshipPattern};

    // =============================================================
    // Sketch Validation Tests
    // =============================================================

    #[test]
    fn test_empty_sketch_is_valid() {
        let sketch = Sketch::new("Test");
        let result = validate_sketch(&sketch);
        assert!(result.is_ok());
    }

    #[test]
    fn test_duplicate_object_names_detected() {
        let mut sketch = Sketch::new("Test");
        sketch.add_object("Customer");
        sketch.add_object("Customer"); // Duplicate!

        let result = validate_sketch(&sketch);
        assert!(!result.is_ok());
        assert_eq!(result.error_count(), 1);
        assert!(result.errors().any(|e| e.code == "E0020"));
    }

    #[test]
    fn test_validation_error_builder() {
        let err = ValidationError::error("E0001", "Test error")
            .with_location(SourceLocation::new("test.sketch", 10, 5))
            .with_suggestion("Try this instead");

        assert_eq!(err.code, "E0001");
        assert_eq!(err.location.line, Some(10));
        assert!(err.suggestion.is_some());
    }

    #[test]
    fn test_morphism_source_target_validation() {
        let mut sketch = Sketch::new("Test");
        let customer = sketch.add_object("Customer");
        let _order = sketch.add_object("Order");

        // Create a morphism that references a non-existent target
        // We need to add a valid morphism first, then test the validation
        // For this test, we'll use the graph's add_morphism_unchecked approach
        // Since morphisms field is private, we test by adding a morphism to non-existent target
        sketch.graph.add_morphism("bad_morphism", customer, ObjectId(999));

        let result = validate_sketch(&sketch);
        assert!(!result.is_ok());
        assert!(result.errors().any(|e| e.code == "E0002"));
    }

    // =============================================================
    // BoundedContext Validation Tests
    // =============================================================

    #[test]
    fn test_empty_context_is_valid() {
        let ctx = BoundedContext::new("Commerce");
        let result = validate_context(&ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_context_with_entities_is_valid() {
        let mut ctx = BoundedContext::new("Commerce");
        ctx.add_entity("Customer");
        ctx.add_entity("Order");

        let result = validate_context(&ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_context_with_value_objects_is_valid() {
        let mut ctx = BoundedContext::new("Commerce");
        ctx.add_value_object("Money");

        let result = validate_context(&ctx);
        // May have warning about missing limit cone
        assert!(result.error_count() == 0);
    }

    #[test]
    fn test_context_with_morphisms_is_valid() {
        let mut ctx = BoundedContext::new("Commerce");
        let customer = ctx.add_entity("Customer");
        let order = ctx.add_entity("Order");
        ctx.sketch_mut().graph.add_morphism("placedBy", order, customer);

        let result = validate_context(&ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_context_with_aggregates_is_valid() {
        let mut ctx = BoundedContext::new("Commerce");
        let order = ctx.add_entity("Order");
        let line_item = ctx.add_entity("LineItem");
        ctx.define_aggregate_with_members("OrderAggregate", order, &[line_item]);

        let result = validate_context(&ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_context_with_enum_is_valid() {
        let mut ctx = BoundedContext::new("Commerce");
        ctx.add_enum("OrderStatus", vec!["Pending".to_string(), "Shipped".to_string()]);

        let result = validate_context(&ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_context_duplicate_object_names_error() {
        let mut ctx = BoundedContext::new("Commerce");
        ctx.sketch_mut().add_object("Customer");
        ctx.sketch_mut().add_object("Customer"); // Duplicate!

        let result = validate_context(&ctx);
        assert!(!result.is_ok());
        assert!(result.errors().any(|e| e.code == "E0020"));
    }

    // =============================================================
    // Context Map Validation Tests
    // =============================================================

    #[test]
    fn test_context_map_valid() {
        let mut commerce = BoundedContext::new("Commerce");
        commerce.sketch_mut().add_object("Order");
        commerce.sketch_mut().add_object("Customer");

        let mut shipping = BoundedContext::new("Shipping");
        shipping.sketch_mut().add_object("Shipment");
        shipping.sketch_mut().add_object("Recipient");

        let mut context_map = NamedContextMap::new(
            "CommerceToShipping",
            "Commerce",
            "Shipping",
            RelationshipPattern::CustomerSupplier,
        );
        context_map.add_object_mapping(NamedObjectMapping {
            source: "Order".to_string(),
            target: "Shipment".to_string(),
            description: None,
        });
        context_map.add_object_mapping(NamedObjectMapping {
            source: "Customer".to_string(),
            target: "Recipient".to_string(),
            description: None,
        });

        let contexts: HashMap<String, &BoundedContext> = [
            ("Commerce".to_string(), &commerce),
            ("Shipping".to_string(), &shipping),
        ]
        .into_iter()
        .collect();

        let result = validate_context_map(&context_map, &contexts);
        assert!(result.is_ok());
    }

    #[test]
    fn test_context_map_missing_source_context() {
        let shipping = BoundedContext::new("Shipping");

        let context_map = NamedContextMap::new(
            "CommerceToShipping",
            "Commerce", // Does not exist!
            "Shipping",
            RelationshipPattern::CustomerSupplier,
        );

        let contexts: HashMap<String, &BoundedContext> = [
            ("Shipping".to_string(), &shipping),
        ]
        .into_iter()
        .collect();

        let result = validate_context_map(&context_map, &contexts);
        assert!(!result.is_ok());
        assert!(result.errors().any(|e| e.code == "E0060"));
    }

    #[test]
    fn test_context_map_missing_target_context() {
        let commerce = BoundedContext::new("Commerce");

        let context_map = NamedContextMap::new(
            "CommerceToShipping",
            "Commerce",
            "Shipping", // Does not exist!
            RelationshipPattern::CustomerSupplier,
        );

        let contexts: HashMap<String, &BoundedContext> = [
            ("Commerce".to_string(), &commerce),
        ]
        .into_iter()
        .collect();

        let result = validate_context_map(&context_map, &contexts);
        assert!(!result.is_ok());
        assert!(result.errors().any(|e| e.code == "E0061"));
    }

    #[test]
    fn test_context_map_missing_source_object() {
        let mut commerce = BoundedContext::new("Commerce");
        commerce.sketch_mut().add_object("Customer");

        let mut shipping = BoundedContext::new("Shipping");
        shipping.sketch_mut().add_object("Shipment");

        let mut context_map = NamedContextMap::new(
            "CommerceToShipping",
            "Commerce",
            "Shipping",
            RelationshipPattern::CustomerSupplier,
        );
        context_map.add_object_mapping(NamedObjectMapping {
            source: "Order".to_string(), // Does not exist in Commerce!
            target: "Shipment".to_string(),
            description: None,
        });

        let contexts: HashMap<String, &BoundedContext> = [
            ("Commerce".to_string(), &commerce),
            ("Shipping".to_string(), &shipping),
        ]
        .into_iter()
        .collect();

        let result = validate_context_map(&context_map, &contexts);
        assert!(!result.is_ok());
        assert!(result.errors().any(|e| e.code == "E0062"));
    }

    #[test]
    fn test_context_map_missing_target_object() {
        let mut commerce = BoundedContext::new("Commerce");
        commerce.sketch_mut().add_object("Order");

        let mut shipping = BoundedContext::new("Shipping");
        shipping.sketch_mut().add_object("Recipient");

        let mut context_map = NamedContextMap::new(
            "CommerceToShipping",
            "Commerce",
            "Shipping",
            RelationshipPattern::CustomerSupplier,
        );
        context_map.add_object_mapping(NamedObjectMapping {
            source: "Order".to_string(),
            target: "Shipment".to_string(), // Does not exist in Shipping!
            description: None,
        });

        let contexts: HashMap<String, &BoundedContext> = [
            ("Commerce".to_string(), &commerce),
            ("Shipping".to_string(), &shipping),
        ]
        .into_iter()
        .collect();

        let result = validate_context_map(&context_map, &contexts);
        assert!(!result.is_ok());
        assert!(result.errors().any(|e| e.code == "E0063"));
    }

    #[test]
    fn test_context_map_morphism_mapping_validation() {
        let mut commerce = BoundedContext::new("Commerce");
        let customer = commerce.sketch_mut().add_object("Customer");
        let order = commerce.sketch_mut().add_object("Order");
        commerce.sketch_mut().graph.add_morphism("placedBy", order, customer);

        let mut shipping = BoundedContext::new("Shipping");
        shipping.sketch_mut().add_object("Shipment");
        shipping.sketch_mut().add_object("Recipient");

        let mut context_map = NamedContextMap::new(
            "CommerceToShipping",
            "Commerce",
            "Shipping",
            RelationshipPattern::CustomerSupplier,
        );
        context_map.add_morphism_mapping(NamedMorphismMapping {
            source: "placedBy".to_string(),
            target: "assignedTo".to_string(), // Does not exist in Shipping!
            description: None,
        });

        let contexts: HashMap<String, &BoundedContext> = [
            ("Commerce".to_string(), &commerce),
            ("Shipping".to_string(), &shipping),
        ]
        .into_iter()
        .collect();

        let result = validate_context_map(&context_map, &contexts);
        assert!(!result.is_ok());
        assert!(result.errors().any(|e| e.code == "E0065"));
    }

    // =============================================================
    // Full Model Validation Tests
    // =============================================================

    #[test]
    fn test_validate_model_empty() {
        let result = validate_model(&[], &[]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_model_with_contexts() {
        let mut ctx1 = BoundedContext::new("Commerce");
        ctx1.add_entity("Order");

        let mut ctx2 = BoundedContext::new("Shipping");
        ctx2.add_entity("Shipment");

        let result = validate_model(&[ctx1, ctx2], &[]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_model_duplicate_context_names() {
        let ctx1 = BoundedContext::new("Commerce");
        let ctx2 = BoundedContext::new("Commerce"); // Duplicate!

        let result = validate_model(&[ctx1, ctx2], &[]);
        assert!(!result.is_ok());
        assert!(result.errors().any(|e| e.code == "E0070"));
    }

    #[test]
    fn test_validate_model_duplicate_map_names() {
        let map1 = NamedContextMap::new("TestMap", "A", "B", RelationshipPattern::Partnership);
        let map2 = NamedContextMap::new("TestMap", "C", "D", RelationshipPattern::Partnership); // Duplicate!

        let result = validate_model(&[], &[map1, map2]);
        assert!(!result.is_ok());
        assert!(result.errors().any(|e| e.code == "E0071"));
    }

    #[test]
    fn test_validate_model_full_integration() {
        let mut commerce = BoundedContext::new("Commerce");
        commerce.add_entity("Customer");
        commerce.add_entity("Order");
        let order_id = commerce.graph().find_object_by_name("Order").unwrap().id;
        let customer_id = commerce.graph().find_object_by_name("Customer").unwrap().id;
        commerce.sketch_mut().graph.add_morphism("placedBy", order_id, customer_id);

        let mut shipping = BoundedContext::new("Shipping");
        shipping.add_entity("Shipment");
        shipping.add_entity("Recipient");
        let shipment_id = shipping.graph().find_object_by_name("Shipment").unwrap().id;
        let recipient_id = shipping.graph().find_object_by_name("Recipient").unwrap().id;
        shipping.sketch_mut().graph.add_morphism("assignedTo", shipment_id, recipient_id);

        let mut context_map = NamedContextMap::new(
            "CommerceToShipping",
            "Commerce",
            "Shipping",
            RelationshipPattern::CustomerSupplier,
        );
        context_map.add_object_mapping(NamedObjectMapping {
            source: "Order".to_string(),
            target: "Shipment".to_string(),
            description: Some("Order maps to Shipment".to_string()),
        });
        context_map.add_object_mapping(NamedObjectMapping {
            source: "Customer".to_string(),
            target: "Recipient".to_string(),
            description: None,
        });
        context_map.add_morphism_mapping(NamedMorphismMapping {
            source: "placedBy".to_string(),
            target: "assignedTo".to_string(),
            description: None,
        });

        let result = validate_model(&[commerce, shipping], &[context_map]);
        assert!(result.is_ok(), "Errors: {:?}", result.errors().collect::<Vec<_>>());
    }

    // =============================================================
    // Validation Result Tests
    // =============================================================

    #[test]
    fn test_validation_result_counts() {
        let mut result = ValidationResult::new();
        result.add(ValidationError::error("E0001", "Error 1"));
        result.add(ValidationError::error("E0002", "Error 2"));
        result.add(ValidationError::warning("W0001", "Warning 1"));

        assert_eq!(result.error_count(), 2);
        assert_eq!(result.warning_count(), 1);
        assert!(!result.is_ok());
        assert!(result.has_issues());
    }

    #[test]
    fn test_validation_result_only_warnings_is_ok() {
        let mut result = ValidationResult::new();
        result.add(ValidationError::warning("W0001", "Warning 1"));
        result.add(ValidationError::warning("W0002", "Warning 2"));

        assert!(result.is_ok());
        assert!(result.has_issues());
    }
}
