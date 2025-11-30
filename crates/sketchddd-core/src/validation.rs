//! Validation logic for sketches and bounded contexts.
//!
//! This module provides comprehensive validation for SketchDDD models:
//! - Object name uniqueness
//! - Morphism source/target existence
//! - Aggregate structure validation
//! - Value object field validation
//! - Enum variant uniqueness
//! - Context map reference validation
//! - Path equation validation (morphism composition)

use crate::context::BoundedContext;
use crate::mapping::NamedContextMap;
use crate::sketch::{Graph, ObjectId, Path, PathEquation, Sketch};
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

    // Check that equations are well-formed (basic check - detailed validation in validate_equations)
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

    // Validate equation paths (morphism composition)
    let equation_result = validate_equations(sketch);
    for issue in equation_result.issues {
        result.add(issue);
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

    // Validate limit cones (aggregates, value objects)
    let limit_result = validate_limits(sketch);
    for issue in limit_result.issues {
        result.add(issue);
    }

    // Validate colimit cocones (enums, sum types)
    let colimit_result = validate_colimits(sketch);
    for issue in colimit_result.issues {
        result.add(issue);
    }

    result
}

/// Validate that an object exists in a sketch.
pub fn object_exists(sketch: &Sketch, id: ObjectId) -> bool {
    sketch.graph.get_object(id).is_some()
}

// =============================================================
// Path Equation Validation
// =============================================================

/// Validate a path for correctness within a graph.
///
/// This checks:
/// - E0100: Source object exists
/// - E0101: Target object exists
/// - E0102: All morphisms in path exist
/// - E0103: Morphisms compose correctly (target of morphism[i] == source of morphism[i+1])
/// - E0104: Path source matches first morphism's source
/// - E0105: Path target matches last morphism's target
pub fn validate_path(path: &Path, graph: &Graph, path_name: &str) -> ValidationResult {
    let mut result = ValidationResult::new();

    // E0100: Check source object exists
    if graph.get_object(path.source).is_none() {
        result.add(ValidationError::error(
            "E0100",
            format!(
                "Path '{}' references non-existent source object (id: {:?})",
                path_name, path.source
            ),
        ));
        return result; // Can't continue validation without source
    }

    // E0101: Check target object exists
    if graph.get_object(path.target).is_none() {
        result.add(ValidationError::error(
            "E0101",
            format!(
                "Path '{}' references non-existent target object (id: {:?})",
                path_name, path.target
            ),
        ));
    }

    // Identity paths are valid if source/target exist
    if path.morphisms.is_empty() {
        if path.source != path.target {
            result.add(ValidationError::error(
                "E0106",
                format!(
                    "Path '{}' has no morphisms but source and target differ",
                    path_name
                ),
            ));
        }
        return result;
    }

    // Validate morphism sequence
    let mut current_object = path.source;

    for (i, &morph_id) in path.morphisms.iter().enumerate() {
        // E0102: Check morphism exists
        let morphism = match graph.get_morphism(morph_id) {
            Some(m) => m,
            None => {
                result.add(ValidationError::error(
                    "E0102",
                    format!(
                        "Path '{}' references non-existent morphism at position {} (id: {:?})",
                        path_name, i, morph_id
                    ),
                ));
                continue;
            }
        };

        // E0103: Check morphism source matches current position
        if i == 0 {
            // E0104: First morphism's source must match path source
            if morphism.source != path.source {
                result.add(ValidationError::error(
                    "E0104",
                    format!(
                        "Path '{}' source ({:?}) doesn't match first morphism '{}' source ({:?})",
                        path_name,
                        path.source,
                        morphism.name,
                        morphism.source
                    ),
                ));
            }
        } else if morphism.source != current_object {
            result.add(ValidationError::error(
                "E0103",
                format!(
                    "Path '{}' has non-composable morphisms at position {}: morphism '{}' expects source {:?} but previous morphism ends at {:?}",
                    path_name, i, morphism.name, morphism.source, current_object
                ),
            ));
        }

        current_object = morphism.target;
    }

    // E0105: Check final morphism's target matches path target
    if current_object != path.target {
        result.add(ValidationError::error(
            "E0105",
            format!(
                "Path '{}' declared target ({:?}) doesn't match computed target ({:?})",
                path_name, path.target, current_object
            ),
        ));
    }

    result
}

/// Validate a path equation for correctness.
///
/// This validates both LHS and RHS paths, and checks they have matching endpoints.
pub fn validate_equation(equation: &PathEquation, graph: &Graph) -> ValidationResult {
    let mut result = ValidationResult::new();

    // Validate LHS path
    let lhs_name = format!("{} (LHS)", equation.name);
    let lhs_result = validate_path(&equation.lhs, graph, &lhs_name);
    for issue in lhs_result.issues {
        result.add(issue);
    }

    // Validate RHS path
    let rhs_name = format!("{} (RHS)", equation.name);
    let rhs_result = validate_path(&equation.rhs, graph, &rhs_name);
    for issue in rhs_result.issues {
        result.add(issue);
    }

    // E0107: Check sources match (already covered by is_well_formed but with better message)
    if equation.lhs.source != equation.rhs.source {
        let lhs_source_name = graph
            .get_object(equation.lhs.source)
            .map(|o| o.name.as_str())
            .unwrap_or("unknown");
        let rhs_source_name = graph
            .get_object(equation.rhs.source)
            .map(|o| o.name.as_str())
            .unwrap_or("unknown");

        result.add(
            ValidationError::error(
                "E0107",
                format!(
                    "Equation '{}' has mismatched sources: LHS starts at '{}', RHS starts at '{}'",
                    equation.name, lhs_source_name, rhs_source_name
                ),
            )
            .with_suggestion("Both sides of an equation must start from the same object"),
        );
    }

    // E0108: Check targets match
    if equation.lhs.target != equation.rhs.target {
        let lhs_target_name = graph
            .get_object(equation.lhs.target)
            .map(|o| o.name.as_str())
            .unwrap_or("unknown");
        let rhs_target_name = graph
            .get_object(equation.rhs.target)
            .map(|o| o.name.as_str())
            .unwrap_or("unknown");

        result.add(
            ValidationError::error(
                "E0108",
                format!(
                    "Equation '{}' has mismatched targets: LHS ends at '{}', RHS ends at '{}'",
                    equation.name, lhs_target_name, rhs_target_name
                ),
            )
            .with_suggestion("Both sides of an equation must end at the same object"),
        );
    }

    // W0100: Warn about trivial equations (both sides are identity paths)
    if equation.lhs.is_identity() && equation.rhs.is_identity() {
        result.add(ValidationError::warning(
            "W0100",
            format!(
                "Equation '{}' is trivial: both sides are identity paths",
                equation.name
            ),
        ));
    }

    // W0101: Warn about very long paths
    if equation.lhs.len() > 5 || equation.rhs.len() > 5 {
        result.add(
            ValidationError::warning(
                "W0101",
                format!(
                    "Equation '{}' has a long path ({} morphisms). Consider simplifying.",
                    equation.name,
                    std::cmp::max(equation.lhs.len(), equation.rhs.len())
                ),
            )
            .with_suggestion("Long paths may indicate overly complex business rules"),
        );
    }

    result
}

/// Validate all equations in a sketch.
pub fn validate_equations(sketch: &Sketch) -> ValidationResult {
    let mut result = ValidationResult::new();

    for equation in &sketch.equations {
        let eq_result = validate_equation(equation, &sketch.graph);
        for issue in eq_result.issues {
            result.add(issue);
        }
    }

    // Check for duplicate equation names
    let mut seen_names: HashSet<&str> = HashSet::new();
    for equation in &sketch.equations {
        if !equation.name.is_empty() && !seen_names.insert(&equation.name) {
            result.add(ValidationError::warning(
                "W0102",
                format!("Duplicate equation name: '{}'", equation.name),
            ));
        }
    }

    result
}

// =============================================================
// Limit/Colimit Validation
// =============================================================

use crate::sketch::{ColimitCocone, LimitCone};

/// Validate a limit cone (aggregate or value object) for structural correctness.
///
/// This checks:
/// - E0110: Apex object exists
/// - E0111: Root object exists (for aggregates)
/// - E0112: Root must equal apex or be in projections (for aggregates)
/// - E0113: Projection morphism exists
/// - E0114: Projection target object exists
/// - E0115: Projection morphism source must be the apex
/// - E0116: Projection morphism target must match projection target
/// - E0117: Duplicate projection targets
/// - W0110: Empty limit cone (no projections)
/// - W0111: Aggregate without root
pub fn validate_limit_cone(limit: &LimitCone, graph: &Graph) -> ValidationResult {
    let mut result = ValidationResult::new();

    // E0110: Check apex object exists
    let apex_name = if let Some(obj) = graph.get_object(limit.apex) {
        obj.name.clone()
    } else {
        result.add(ValidationError::error(
            "E0110",
            format!(
                "Limit cone '{}' has apex that references non-existent object (id: {:?})",
                limit.name, limit.apex
            ),
        ));
        return result; // Can't continue without apex
    };

    // Aggregate-specific validations
    if limit.is_aggregate {
        if let Some(root_id) = limit.root {
            // E0111: Check root object exists
            if graph.get_object(root_id).is_none() {
                result.add(ValidationError::error(
                    "E0111",
                    format!(
                        "Aggregate '{}' has root that references non-existent object (id: {:?})",
                        limit.name, root_id
                    ),
                ));
            } else {
                // E0112: Root must be the apex or reachable via projections
                let root_is_apex = root_id == limit.apex;
                let root_in_projections = limit.projections.iter().any(|p| p.target == root_id);

                if !root_is_apex && !root_in_projections {
                    result.add(
                        ValidationError::error(
                            "E0112",
                            format!(
                                "Aggregate '{}' root is neither the apex nor a projection target",
                                limit.name
                            ),
                        )
                        .with_suggestion("The root should be the apex object or one of the contained entities"),
                    );
                }
            }
        } else {
            // W0111: Aggregate should have a root
            result.add(
                ValidationError::warning(
                    "W0111",
                    format!("Aggregate '{}' does not specify a root", limit.name),
                )
                .with_suggestion("Consider specifying a root entity for the aggregate"),
            );
        }
    }

    // W0110: Warn about empty limit cones
    if limit.projections.is_empty() {
        result.add(ValidationError::warning(
            "W0110",
            format!(
                "Limit cone '{}' has no projections (empty {})",
                limit.name,
                if limit.is_aggregate { "aggregate" } else { "value object" }
            ),
        ));
    }

    // Track projection targets for duplicate detection
    let mut seen_targets: HashSet<ObjectId> = HashSet::new();

    // Validate each projection
    for (i, projection) in limit.projections.iter().enumerate() {
        // E0113: Check projection morphism exists
        let morphism = match graph.get_morphism(projection.morphism) {
            Some(m) => m,
            None => {
                result.add(ValidationError::error(
                    "E0113",
                    format!(
                        "Limit cone '{}' projection {} references non-existent morphism (id: {:?})",
                        limit.name, i, projection.morphism
                    ),
                ));
                continue;
            }
        };

        // E0114: Check projection target object exists
        if graph.get_object(projection.target).is_none() {
            result.add(ValidationError::error(
                "E0114",
                format!(
                    "Limit cone '{}' projection '{}' references non-existent target object (id: {:?})",
                    limit.name, morphism.name, projection.target
                ),
            ));
        }

        // E0115: Projection morphism source must be the apex
        if morphism.source != limit.apex {
            let morph_source_name = graph
                .get_object(morphism.source)
                .map(|o| o.name.as_str())
                .unwrap_or("unknown");

            result.add(
                ValidationError::error(
                    "E0115",
                    format!(
                        "Limit cone '{}' projection morphism '{}' has wrong source: expected '{}' (apex), found '{}'",
                        limit.name, morphism.name, apex_name, morph_source_name
                    ),
                )
                .with_suggestion("Projection morphisms must originate from the limit's apex"),
            );
        }

        // E0116: Projection morphism target must match declared target
        if morphism.target != projection.target {
            let morph_target_name = graph
                .get_object(morphism.target)
                .map(|o| o.name.as_str())
                .unwrap_or("unknown");
            let declared_target_name = graph
                .get_object(projection.target)
                .map(|o| o.name.as_str())
                .unwrap_or("unknown");

            result.add(ValidationError::error(
                "E0116",
                format!(
                    "Limit cone '{}' projection morphism '{}' targets '{}' but projection declares target '{}'",
                    limit.name, morphism.name, morph_target_name, declared_target_name
                ),
            ));
        }

        // E0117: Check for duplicate projection targets
        if !seen_targets.insert(projection.target) {
            let target_name = graph
                .get_object(projection.target)
                .map(|o| o.name.as_str())
                .unwrap_or("unknown");

            result.add(
                ValidationError::error(
                    "E0117",
                    format!(
                        "Limit cone '{}' has duplicate projection to object '{}'",
                        limit.name, target_name
                    ),
                )
                .with_suggestion("Each component object should appear in at most one projection"),
            );
        }
    }

    result
}

/// Validate a colimit cocone (enum/sum type) for structural correctness.
///
/// This checks:
/// - E0120: Apex object exists
/// - E0121: Injection source object exists
/// - E0122: Empty variant name
/// - E0123: Duplicate variant names (handled elsewhere but included for completeness)
/// - W0120: Empty colimit (no injections)
/// - W0121: Single variant colimit (trivial sum type)
pub fn validate_colimit_cocone(colimit: &ColimitCocone, graph: &Graph) -> ValidationResult {
    let mut result = ValidationResult::new();

    // E0120: Check apex object exists
    if graph.get_object(colimit.apex).is_none() {
        result.add(ValidationError::error(
            "E0120",
            format!(
                "Colimit cocone '{}' has apex that references non-existent object (id: {:?})",
                colimit.name, colimit.apex
            ),
        ));
        return result; // Can't continue without apex
    }

    // W0120: Warn about empty colimits
    if colimit.injections.is_empty() {
        result.add(
            ValidationError::warning(
                "W0120",
                format!("Colimit cocone '{}' has no injections (empty enum)", colimit.name),
            )
            .with_suggestion("Consider adding at least one variant to the enumeration"),
        );
    }

    // W0121: Warn about single-variant colimits
    if colimit.injections.len() == 1 {
        result.add(ValidationError::warning(
            "W0121",
            format!(
                "Colimit cocone '{}' has only one variant, which is a trivial sum type",
                colimit.name
            ),
        ));
    }

    // Track variant names for duplicate detection
    let mut seen_names: HashSet<&str> = HashSet::new();

    // Validate each injection
    for injection in &colimit.injections {
        // E0121: Check injection source object exists
        if graph.get_object(injection.source).is_none() {
            result.add(ValidationError::error(
                "E0121",
                format!(
                    "Colimit cocone '{}' variant '{}' references non-existent source object (id: {:?})",
                    colimit.name, injection.name, injection.source
                ),
            ));
        }

        // E0122: Check variant name is not empty
        if injection.name.trim().is_empty() {
            result.add(ValidationError::error(
                "E0122",
                format!("Colimit cocone '{}' has a variant with an empty name", colimit.name),
            ));
        }

        // E0123: Check for duplicate variant names
        if !seen_names.insert(&injection.name) {
            result.add(ValidationError::error(
                "E0123",
                format!(
                    "Colimit cocone '{}' has duplicate variant name: '{}'",
                    colimit.name, injection.name
                ),
            ));
        }
    }

    result
}

/// Validate all limits in a sketch.
pub fn validate_limits(sketch: &Sketch) -> ValidationResult {
    let mut result = ValidationResult::new();

    // Check for duplicate limit names
    let mut seen_names: HashSet<&str> = HashSet::new();
    for limit in &sketch.limits {
        if !seen_names.insert(&limit.name) {
            result.add(ValidationError::warning(
                "W0112",
                format!("Duplicate limit cone name: '{}'", limit.name),
            ));
        }

        // Validate each limit cone
        let limit_result = validate_limit_cone(limit, &sketch.graph);
        for issue in limit_result.issues {
            result.add(issue);
        }
    }

    result
}

/// Validate all colimits in a sketch.
pub fn validate_colimits(sketch: &Sketch) -> ValidationResult {
    let mut result = ValidationResult::new();

    // Check for duplicate colimit names
    let mut seen_names: HashSet<&str> = HashSet::new();
    for colimit in &sketch.colimits {
        if !seen_names.insert(&colimit.name) {
            result.add(ValidationError::warning(
                "W0122",
                format!("Duplicate colimit cocone name: '{}'", colimit.name),
            ));
        }

        // Validate each colimit cocone
        let colimit_result = validate_colimit_cocone(colimit, &sketch.graph);
        for issue in colimit_result.issues {
            result.add(issue);
        }
    }

    result
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

        // Advanced validations
        validate_mapping_completeness(context_map, source, &mut result);
        validate_identity_preservation(context_map, source, target, &mut result);
        validate_relationship_pattern(context_map, source, target, &mut result);
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
        let source_morph = source_ctx.graph().find_morphism_by_name(&mapping.source);
        if source_morph.is_none() {
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
        let target_morph = target_ctx.graph().find_morphism_by_name(&mapping.target);
        if target_morph.is_none() {
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

        // E0066: Check functorial consistency - morphism endpoints must be mapped correctly
        if let (Some(src_m), Some(tgt_m)) = (source_morph, target_morph) {
            validate_morphism_endpoint_consistency(
                context_map,
                src_m,
                tgt_m,
                source_ctx,
                target_ctx,
                result,
            );
        }
    }
}

/// Validate that morphism mapping preserves graph structure.
/// For a morphism f: A -> B in source, F(f): F(A) -> F(B) in target.
fn validate_morphism_endpoint_consistency(
    context_map: &NamedContextMap,
    source_morph: &crate::sketch::Morphism,
    target_morph: &crate::sketch::Morphism,
    source_ctx: &BoundedContext,
    target_ctx: &BoundedContext,
    result: &mut ValidationResult,
) {
    // Get source morphism's endpoints in source context
    let src_source_obj = source_ctx.graph().get_object(source_morph.source);
    let src_target_obj = source_ctx.graph().get_object(source_morph.target);

    // Get target morphism's endpoints in target context
    let tgt_source_obj = target_ctx.graph().get_object(target_morph.source);
    let tgt_target_obj = target_ctx.graph().get_object(target_morph.target);

    if let (Some(ss), Some(st), Some(ts), Some(tt)) =
        (src_source_obj, src_target_obj, tgt_source_obj, tgt_target_obj)
    {
        // Check if source object mapping exists and matches
        let expected_target_source = context_map
            .object_mappings()
            .iter()
            .find(|m| m.source == ss.name)
            .map(|m| m.target.as_str());

        if let Some(expected) = expected_target_source {
            if expected != ts.name {
                result.add(
                    ValidationError::error(
                        "E0066",
                        format!(
                            "Context map '{}': morphism '{}' maps to '{}', but source object '{}' maps to '{}', not '{}'",
                            context_map.name(),
                            source_morph.name,
                            target_morph.name,
                            ss.name,
                            expected,
                            ts.name
                        ),
                    )
                    .with_suggestion("Morphism mappings must preserve graph structure: F(f: A→B) should have F(A) as source"),
                );
            }
        }

        // Check if target object mapping exists and matches
        let expected_target_target = context_map
            .object_mappings()
            .iter()
            .find(|m| m.source == st.name)
            .map(|m| m.target.as_str());

        if let Some(expected) = expected_target_target {
            if expected != tt.name {
                result.add(
                    ValidationError::error(
                        "E0067",
                        format!(
                            "Context map '{}': morphism '{}' maps to '{}', but target object '{}' maps to '{}', not '{}'",
                            context_map.name(),
                            source_morph.name,
                            target_morph.name,
                            st.name,
                            expected,
                            tt.name
                        ),
                    )
                    .with_suggestion("Morphism mappings must preserve graph structure: F(f: A→B) should have F(B) as target"),
                );
            }
        }
    }
}

/// Check for missing object mappings (warnings).
fn validate_mapping_completeness(
    context_map: &NamedContextMap,
    source_ctx: &BoundedContext,
    result: &mut ValidationResult,
) {
    // Build set of mapped source objects
    let mapped_objects: HashSet<&str> = context_map
        .object_mappings()
        .iter()
        .map(|m| m.source.as_str())
        .collect();

    // Build set of mapped source morphisms
    let mapped_morphisms: HashSet<&str> = context_map
        .morphism_mappings()
        .iter()
        .map(|m| m.source.as_str())
        .collect();

    // Count unmapped objects
    let unmapped_objects: Vec<&str> = source_ctx
        .graph()
        .objects()
        .filter(|o| !mapped_objects.contains(o.name.as_str()))
        .map(|o| o.name.as_str())
        .collect();

    if !unmapped_objects.is_empty() {
        let unmapped_list = if unmapped_objects.len() <= 5 {
            unmapped_objects.join(", ")
        } else {
            format!(
                "{}, ... ({} more)",
                unmapped_objects[..5].join(", "),
                unmapped_objects.len() - 5
            )
        };

        result.add(
            ValidationError::warning(
                "W0130",
                format!(
                    "Context map '{}' has {} unmapped objects: {}",
                    context_map.name(),
                    unmapped_objects.len(),
                    unmapped_list
                ),
            )
            .with_suggestion("Consider mapping all objects for a complete context translation"),
        );
    }

    // Count unmapped morphisms (excluding identity morphisms)
    let unmapped_morphisms: Vec<&str> = source_ctx
        .graph()
        .morphisms()
        .filter(|m| !m.is_identity && !mapped_morphisms.contains(m.name.as_str()))
        .map(|m| m.name.as_str())
        .collect();

    if !unmapped_morphisms.is_empty() {
        let unmapped_list = if unmapped_morphisms.len() <= 5 {
            unmapped_morphisms.join(", ")
        } else {
            format!(
                "{}, ... ({} more)",
                unmapped_morphisms[..5].join(", "),
                unmapped_morphisms.len() - 5
            )
        };

        result.add(
            ValidationError::warning(
                "W0131",
                format!(
                    "Context map '{}' has {} unmapped morphisms: {}",
                    context_map.name(),
                    unmapped_morphisms.len(),
                    unmapped_list
                ),
            )
            .with_suggestion("Consider mapping morphisms to preserve relationships"),
        );
    }
}

/// Validate identity morphism preservation.
/// For functorial mapping: F(id_X) = id_{F(X)}
fn validate_identity_preservation(
    context_map: &NamedContextMap,
    source_ctx: &BoundedContext,
    target_ctx: &BoundedContext,
    result: &mut ValidationResult,
) {
    // For each object mapping, check if identity morphisms are mapped correctly
    for obj_mapping in context_map.object_mappings() {
        let source_obj = source_ctx.graph().find_object_by_name(&obj_mapping.source);
        let target_obj = target_ctx.graph().find_object_by_name(&obj_mapping.target);

        if let (Some(src_obj), Some(tgt_obj)) = (source_obj, target_obj) {
            // Check if source has identity morphism
            let src_identity = source_ctx.graph().get_identity_morphism(src_obj.id);
            let tgt_identity = target_ctx.graph().get_identity_morphism(tgt_obj.id);

            // If source has identity and target doesn't, warn
            if src_identity.is_some() && tgt_identity.is_none() {
                result.add(
                    ValidationError::warning(
                        "W0132",
                        format!(
                            "Context map '{}': object '{}' has identity morphism in source, but mapped target '{}' does not",
                            context_map.name(),
                            obj_mapping.source,
                            obj_mapping.target
                        ),
                    )
                    .with_suggestion("For functorial consistency, F(id_X) should equal id_{F(X)}"),
                );
            }
        }
    }
}

/// Validate relationship pattern-specific constraints.
fn validate_relationship_pattern(
    context_map: &NamedContextMap,
    source_ctx: &BoundedContext,
    target_ctx: &BoundedContext,
    result: &mut ValidationResult,
) {
    use crate::mapping::RelationshipPattern;

    match context_map.pattern() {
        RelationshipPattern::SharedKernel => {
            // Shared kernel: mappings should be bidirectional (same names usually)
            // Just a hint for now
            let non_identical: Vec<_> = context_map
                .object_mappings()
                .iter()
                .filter(|m| m.source != m.target)
                .collect();

            if !non_identical.is_empty() {
                result.add(ValidationError::warning(
                    "W0133",
                    format!(
                        "Context map '{}' uses SharedKernel but has {} non-identical object names",
                        context_map.name(),
                        non_identical.len()
                    ),
                ));
            }
        }
        RelationshipPattern::AntiCorruptionLayer => {
            // ACL: should map all incoming types from upstream
            let source_entities = source_ctx.entities().len();
            let mapped_count = context_map.object_mappings().len();

            if mapped_count == 0 && source_entities > 0 {
                result.add(
                    ValidationError::warning(
                        "W0134",
                        format!(
                            "Context map '{}' is an AntiCorruptionLayer but has no object mappings",
                            context_map.name()
                        ),
                    )
                    .with_suggestion("ACL should translate upstream concepts to local representations"),
                );
            }
        }
        RelationshipPattern::OpenHostService => {
            // OHS: should expose types to downstream
            let target_entities = target_ctx.entities().len();
            let mapped_count = context_map.object_mappings().len();

            if mapped_count == 0 && target_entities > 0 {
                result.add(
                    ValidationError::warning(
                        "W0135",
                        format!(
                            "Context map '{}' is an OpenHostService but has no object mappings",
                            context_map.name()
                        ),
                    )
                    .with_suggestion("OHS should expose domain types to downstream consumers"),
                );
            }
        }
        RelationshipPattern::Conformist => {
            // Conformist: downstream adopts upstream model entirely
            // Should map most/all objects
            let source_objects: usize = source_ctx.graph().objects().count();
            let mapped_count = context_map.object_mappings().len();

            if source_objects > 0 && mapped_count < source_objects / 2 {
                result.add(
                    ValidationError::warning(
                        "W0136",
                        format!(
                            "Context map '{}' is Conformist but only maps {}/{} objects from upstream",
                            context_map.name(),
                            mapped_count,
                            source_objects
                        ),
                    )
                    .with_suggestion("Conformist pattern implies adopting the upstream model largely as-is"),
                );
            }
        }
        _ => {
            // No special constraints for other patterns
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
    use crate::sketch::MorphismId;

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
    // Path Equation Validation Tests
    // =============================================================

    #[test]
    fn test_valid_identity_path() {
        let mut graph = crate::sketch::Graph::new();
        let order = graph.add_object("Order");

        let path = Path::identity(order);
        let result = validate_path(&path, &graph, "test_path");

        assert!(result.is_ok());
    }

    #[test]
    fn test_valid_single_morphism_path() {
        let mut graph = crate::sketch::Graph::new();
        let order = graph.add_object("Order");
        let customer = graph.add_object("Customer");
        let placed_by = graph.add_morphism("placedBy", order, customer);

        let path = Path::new(order, customer, vec![placed_by]);
        let result = validate_path(&path, &graph, "test_path");

        assert!(result.is_ok());
    }

    #[test]
    fn test_valid_multi_morphism_path() {
        let mut graph = crate::sketch::Graph::new();
        let order = graph.add_object("Order");
        let line_item = graph.add_object("LineItem");
        let product = graph.add_object("Product");

        let items = graph.add_morphism("items", order, line_item);
        let product_morph = graph.add_morphism("product", line_item, product);

        let path = Path::new(order, product, vec![items, product_morph]);
        let result = validate_path(&path, &graph, "test_path");

        assert!(result.is_ok());
    }

    #[test]
    fn test_path_with_non_existent_source_object() {
        let graph = crate::sketch::Graph::new();

        let path = Path::identity(ObjectId(999));
        let result = validate_path(&path, &graph, "test_path");

        assert!(!result.is_ok());
        assert!(result.errors().any(|e| e.code == "E0100"));
    }

    #[test]
    fn test_path_with_non_existent_morphism() {
        let mut graph = crate::sketch::Graph::new();
        let order = graph.add_object("Order");
        let customer = graph.add_object("Customer");

        // Reference a non-existent morphism
        let path = Path::new(order, customer, vec![MorphismId(999)]);
        let result = validate_path(&path, &graph, "test_path");

        assert!(!result.is_ok());
        assert!(result.errors().any(|e| e.code == "E0102"));
    }

    #[test]
    fn test_path_with_non_composable_morphisms() {
        let mut graph = crate::sketch::Graph::new();
        let order = graph.add_object("Order");
        let customer = graph.add_object("Customer");
        let product = graph.add_object("Product");

        // Order -> Customer
        let placed_by = graph.add_morphism("placedBy", order, customer);
        // Product -> Customer (not Order -> Product, so can't compose)
        let sold_to = graph.add_morphism("soldTo", product, customer);

        // Try to compose: Order -placedBy-> Customer, Product -soldTo-> Customer
        // This should fail because placedBy ends at Customer, but soldTo starts at Product
        let path = Path::new(order, customer, vec![placed_by, sold_to]);
        let result = validate_path(&path, &graph, "test_path");

        assert!(!result.is_ok());
        assert!(result.errors().any(|e| e.code == "E0103"));
    }

    #[test]
    fn test_path_source_mismatch() {
        let mut graph = crate::sketch::Graph::new();
        let order = graph.add_object("Order");
        let customer = graph.add_object("Customer");
        let product = graph.add_object("Product");

        // Morphism from Order -> Customer
        let placed_by = graph.add_morphism("placedBy", order, customer);

        // But path says it starts at Product
        let path = Path::new(product, customer, vec![placed_by]);
        let result = validate_path(&path, &graph, "test_path");

        assert!(!result.is_ok());
        assert!(result.errors().any(|e| e.code == "E0104"));
    }

    #[test]
    fn test_path_target_mismatch() {
        let mut graph = crate::sketch::Graph::new();
        let order = graph.add_object("Order");
        let customer = graph.add_object("Customer");
        let product = graph.add_object("Product");

        // Morphism from Order -> Customer
        let placed_by = graph.add_morphism("placedBy", order, customer);

        // But path says it ends at Product
        let path = Path::new(order, product, vec![placed_by]);
        let result = validate_path(&path, &graph, "test_path");

        assert!(!result.is_ok());
        assert!(result.errors().any(|e| e.code == "E0105"));
    }

    #[test]
    fn test_empty_path_with_different_source_target() {
        let mut graph = crate::sketch::Graph::new();
        let order = graph.add_object("Order");
        let customer = graph.add_object("Customer");

        // No morphisms but different source/target
        let path = Path::new(order, customer, vec![]);
        let result = validate_path(&path, &graph, "test_path");

        assert!(!result.is_ok());
        assert!(result.errors().any(|e| e.code == "E0106"));
    }

    #[test]
    fn test_valid_equation() {
        let mut graph = crate::sketch::Graph::new();
        let order = graph.add_object("Order");

        let lhs = Path::identity(order);
        let rhs = Path::identity(order);
        let equation = PathEquation::new("identity_eq", lhs, rhs);

        let result = validate_equation(&equation, &graph);

        // Valid but trivial (W0100 warning)
        assert!(result.is_ok());
        assert!(result.warnings().any(|e| e.code == "W0100"));
    }

    #[test]
    fn test_equation_with_mismatched_sources() {
        let mut graph = crate::sketch::Graph::new();
        let order = graph.add_object("Order");
        let customer = graph.add_object("Customer");

        let lhs = Path::identity(order);
        let rhs = Path::identity(customer);
        let equation = PathEquation::new("bad_eq", lhs, rhs);

        let result = validate_equation(&equation, &graph);

        assert!(!result.is_ok());
        assert!(result.errors().any(|e| e.code == "E0107"));
    }

    #[test]
    fn test_equation_with_mismatched_targets() {
        let mut graph = crate::sketch::Graph::new();
        let order = graph.add_object("Order");
        let customer = graph.add_object("Customer");
        let product = graph.add_object("Product");

        let placed_by = graph.add_morphism("placedBy", order, customer);
        let contains = graph.add_morphism("contains", order, product);

        let lhs = Path::new(order, customer, vec![placed_by]);
        let rhs = Path::new(order, product, vec![contains]);
        let equation = PathEquation::new("target_mismatch", lhs, rhs);

        let result = validate_equation(&equation, &graph);

        assert!(!result.is_ok());
        assert!(result.errors().any(|e| e.code == "E0108"));
    }

    #[test]
    fn test_equation_with_long_path_warning() {
        let mut graph = crate::sketch::Graph::new();

        // Create a long chain of objects
        let a = graph.add_object("A");
        let b = graph.add_object("B");
        let c = graph.add_object("C");
        let d = graph.add_object("D");
        let e = graph.add_object("E");
        let f = graph.add_object("F");
        let g = graph.add_object("G");

        let ab = graph.add_morphism("ab", a, b);
        let bc = graph.add_morphism("bc", b, c);
        let cd = graph.add_morphism("cd", c, d);
        let de = graph.add_morphism("de", d, e);
        let ef = graph.add_morphism("ef", e, f);
        let fg = graph.add_morphism("fg", f, g);

        let lhs = Path::new(a, g, vec![ab, bc, cd, de, ef, fg]);
        let rhs = Path::new(a, g, vec![ab, bc, cd, de, ef, fg]);
        let equation = PathEquation::new("long_path", lhs, rhs);

        let result = validate_equation(&equation, &graph);

        assert!(result.is_ok()); // Warnings don't fail validation
        assert!(result.warnings().any(|e| e.code == "W0101"));
    }

    #[test]
    fn test_duplicate_equation_names_warning() {
        let mut sketch = Sketch::new("Test");
        let order = sketch.add_object("Order");

        // Add two equations with the same name
        let eq1 = PathEquation::new("my_rule", Path::identity(order), Path::identity(order));
        let eq2 = PathEquation::new("my_rule", Path::identity(order), Path::identity(order));

        sketch.equations.push(eq1);
        sketch.equations.push(eq2);

        let result = validate_equations(&sketch);

        assert!(result.is_ok()); // Warnings don't fail validation
        assert!(result.warnings().any(|e| e.code == "W0102"));
    }

    #[test]
    fn test_validate_sketch_with_equations() {
        let mut sketch = Sketch::new("Commerce");
        let order = sketch.add_object("Order");
        let customer = sketch.add_object("Customer");
        let total = sketch.add_object("Money");

        let _placed_by = sketch.graph.add_morphism("placedBy", order, customer);
        let total_price = sketch.graph.add_morphism("totalPrice", order, total);

        // Valid equation: different paths from Order to same target would be equal
        // For this test, we just verify the validation runs without error on valid morphisms
        let eq = PathEquation::new(
            "price_consistency",
            Path::new(order, total, vec![total_price]),
            Path::new(order, total, vec![total_price]), // Same path for simplicity
        );
        sketch.equations.push(eq);

        let result = validate_sketch(&sketch);

        assert!(result.is_ok(), "Errors: {:?}", result.errors().collect::<Vec<_>>());
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

    #[test]
    fn test_context_map_functorial_source_consistency() {
        // Test E0066: morphism mapping source consistency
        let mut commerce = BoundedContext::new("Commerce");
        let customer = commerce.sketch_mut().add_object("Customer");
        let order = commerce.sketch_mut().add_object("Order");
        commerce.sketch_mut().graph.add_morphism("placedBy", order, customer);

        let mut shipping = BoundedContext::new("Shipping");
        let _shipment = shipping.sketch_mut().add_object("Shipment");
        let recipient = shipping.sketch_mut().add_object("Recipient");
        let other = shipping.sketch_mut().add_object("Other");
        // Wrong source: Other -> Recipient instead of Shipment -> Recipient
        shipping.sketch_mut().graph.add_morphism("assignedTo", other, recipient);

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
        // placedBy: Order -> Customer maps to assignedTo: Other -> Recipient
        // This violates F(Order) = Shipment but assignedTo starts at Other
        context_map.add_morphism_mapping(NamedMorphismMapping {
            source: "placedBy".to_string(),
            target: "assignedTo".to_string(),
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
        assert!(result.errors().any(|e| e.code == "E0066"));
    }

    #[test]
    fn test_context_map_functorial_target_consistency() {
        // Test E0067: morphism mapping target consistency
        let mut commerce = BoundedContext::new("Commerce");
        let customer = commerce.sketch_mut().add_object("Customer");
        let order = commerce.sketch_mut().add_object("Order");
        commerce.sketch_mut().graph.add_morphism("placedBy", order, customer);

        let mut shipping = BoundedContext::new("Shipping");
        let shipment = shipping.sketch_mut().add_object("Shipment");
        let _recipient = shipping.sketch_mut().add_object("Recipient");
        let other = shipping.sketch_mut().add_object("Other");
        // Wrong target: Shipment -> Other instead of Shipment -> Recipient
        shipping.sketch_mut().graph.add_morphism("assignedTo", shipment, other);

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
        // placedBy: Order -> Customer maps to assignedTo: Shipment -> Other
        // This violates F(Customer) = Recipient but assignedTo ends at Other
        context_map.add_morphism_mapping(NamedMorphismMapping {
            source: "placedBy".to_string(),
            target: "assignedTo".to_string(),
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
        assert!(result.errors().any(|e| e.code == "E0067"));
    }

    #[test]
    fn test_context_map_unmapped_objects_warning() {
        let mut commerce = BoundedContext::new("Commerce");
        commerce.sketch_mut().add_object("Order");
        commerce.sketch_mut().add_object("Customer");
        commerce.sketch_mut().add_object("Product"); // Not mapped

        let mut shipping = BoundedContext::new("Shipping");
        shipping.sketch_mut().add_object("Shipment");

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
        // Customer and Product are not mapped

        let contexts: HashMap<String, &BoundedContext> = [
            ("Commerce".to_string(), &commerce),
            ("Shipping".to_string(), &shipping),
        ]
        .into_iter()
        .collect();

        let result = validate_context_map(&context_map, &contexts);
        assert!(result.is_ok()); // Warnings don't fail
        assert!(result.warnings().any(|e| e.code == "W0130"));
    }

    #[test]
    fn test_context_map_unmapped_morphisms_warning() {
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
        // placedBy morphism is not mapped

        let contexts: HashMap<String, &BoundedContext> = [
            ("Commerce".to_string(), &commerce),
            ("Shipping".to_string(), &shipping),
        ]
        .into_iter()
        .collect();

        let result = validate_context_map(&context_map, &contexts);
        assert!(result.is_ok()); // Warnings don't fail
        assert!(result.warnings().any(|e| e.code == "W0131"));
    }

    #[test]
    fn test_context_map_shared_kernel_non_identical_warning() {
        let mut context_a = BoundedContext::new("ContextA");
        context_a.sketch_mut().add_object("Entity");

        let mut context_b = BoundedContext::new("ContextB");
        context_b.sketch_mut().add_object("DifferentName");

        let mut context_map = NamedContextMap::new(
            "SharedEntities",
            "ContextA",
            "ContextB",
            RelationshipPattern::SharedKernel,
        );
        context_map.add_object_mapping(NamedObjectMapping {
            source: "Entity".to_string(),
            target: "DifferentName".to_string(), // Non-identical
            description: None,
        });

        let contexts: HashMap<String, &BoundedContext> = [
            ("ContextA".to_string(), &context_a),
            ("ContextB".to_string(), &context_b),
        ]
        .into_iter()
        .collect();

        let result = validate_context_map(&context_map, &contexts);
        assert!(result.is_ok()); // Warnings don't fail
        assert!(result.warnings().any(|e| e.code == "W0133"));
    }

    #[test]
    fn test_context_map_acl_no_mappings_warning() {
        let mut upstream = BoundedContext::new("Upstream");
        upstream.add_entity("ExternalEntity");

        let downstream = BoundedContext::new("Downstream");

        let context_map = NamedContextMap::new(
            "ACL",
            "Upstream",
            "Downstream",
            RelationshipPattern::AntiCorruptionLayer,
        );
        // No mappings!

        let contexts: HashMap<String, &BoundedContext> = [
            ("Upstream".to_string(), &upstream),
            ("Downstream".to_string(), &downstream),
        ]
        .into_iter()
        .collect();

        let result = validate_context_map(&context_map, &contexts);
        assert!(result.is_ok()); // Warnings don't fail
        assert!(result.warnings().any(|e| e.code == "W0134"));
    }

    #[test]
    fn test_context_map_conformist_partial_mapping_warning() {
        let mut upstream = BoundedContext::new("Upstream");
        upstream.sketch_mut().add_object("Entity1");
        upstream.sketch_mut().add_object("Entity2");
        upstream.sketch_mut().add_object("Entity3");
        upstream.sketch_mut().add_object("Entity4");

        let mut downstream = BoundedContext::new("Downstream");
        downstream.sketch_mut().add_object("LocalEntity");

        let mut context_map = NamedContextMap::new(
            "ConformistMap",
            "Upstream",
            "Downstream",
            RelationshipPattern::Conformist,
        );
        // Only map 1 out of 4 entities (less than half)
        context_map.add_object_mapping(NamedObjectMapping {
            source: "Entity1".to_string(),
            target: "LocalEntity".to_string(),
            description: None,
        });

        let contexts: HashMap<String, &BoundedContext> = [
            ("Upstream".to_string(), &upstream),
            ("Downstream".to_string(), &downstream),
        ]
        .into_iter()
        .collect();

        let result = validate_context_map(&context_map, &contexts);
        assert!(result.is_ok()); // Warnings don't fail
        assert!(result.warnings().any(|e| e.code == "W0136"));
    }

    #[test]
    fn test_valid_functorial_context_map() {
        // Test that a properly functorial mapping has no errors
        let mut commerce = BoundedContext::new("Commerce");
        let customer = commerce.sketch_mut().add_object("Customer");
        let order = commerce.sketch_mut().add_object("Order");
        commerce.sketch_mut().graph.add_morphism("placedBy", order, customer);

        let mut shipping = BoundedContext::new("Shipping");
        let shipment = shipping.sketch_mut().add_object("Shipment");
        let recipient = shipping.sketch_mut().add_object("Recipient");
        // Correct structure: Shipment -> Recipient (matching Order -> Customer)
        shipping.sketch_mut().graph.add_morphism("assignedTo", shipment, recipient);

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
        context_map.add_morphism_mapping(NamedMorphismMapping {
            source: "placedBy".to_string(),
            target: "assignedTo".to_string(),
            description: None,
        });

        let contexts: HashMap<String, &BoundedContext> = [
            ("Commerce".to_string(), &commerce),
            ("Shipping".to_string(), &shipping),
        ]
        .into_iter()
        .collect();

        let result = validate_context_map(&context_map, &contexts);
        // Should pass (may have warnings but no errors)
        assert!(result.is_ok(), "Errors: {:?}", result.errors().collect::<Vec<_>>());
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

    // =============================================================
    // Limit Cone Validation Tests
    // =============================================================

    #[test]
    fn test_valid_limit_cone_aggregate() {
        let mut graph = crate::sketch::Graph::new();
        let order = graph.add_object("Order");
        let line_item = graph.add_object("LineItem");
        let items = graph.add_morphism("items", order, line_item);

        let mut limit = LimitCone::aggregate("OrderAggregate", order, order);
        limit.add_projection(items, line_item);

        let result = validate_limit_cone(&limit, &graph);
        // Expect only warnings (W0110 is not triggered since we have projections)
        assert!(result.is_ok(), "Errors: {:?}", result.errors().collect::<Vec<_>>());
    }

    #[test]
    fn test_valid_limit_cone_value_object() {
        let mut graph = crate::sketch::Graph::new();
        let money = graph.add_object("Money");
        let decimal = graph.add_object("Decimal");
        let currency = graph.add_object("Currency");

        let amount = graph.add_morphism("amount", money, decimal);
        let currency_morph = graph.add_morphism("currency", money, currency);

        let mut limit = LimitCone::value_object("Money", money);
        limit.add_projection(amount, decimal);
        limit.add_projection(currency_morph, currency);

        let result = validate_limit_cone(&limit, &graph);
        assert!(result.is_ok());
    }

    #[test]
    fn test_limit_cone_non_existent_apex() {
        let graph = crate::sketch::Graph::new();

        let limit = LimitCone::aggregate("BadAggregate", ObjectId(999), ObjectId(999));

        let result = validate_limit_cone(&limit, &graph);
        assert!(!result.is_ok());
        assert!(result.errors().any(|e| e.code == "E0110"));
    }

    #[test]
    fn test_limit_cone_non_existent_root() {
        let mut graph = crate::sketch::Graph::new();
        let order = graph.add_object("Order");

        let limit = LimitCone::aggregate("BadAggregate", order, ObjectId(999));

        let result = validate_limit_cone(&limit, &graph);
        assert!(!result.is_ok());
        assert!(result.errors().any(|e| e.code == "E0111"));
    }

    #[test]
    fn test_limit_cone_root_not_in_structure() {
        let mut graph = crate::sketch::Graph::new();
        let order = graph.add_object("Order");
        let line_item = graph.add_object("LineItem");
        let customer = graph.add_object("Customer"); // Not part of aggregate

        let items = graph.add_morphism("items", order, line_item);

        let mut limit = LimitCone::aggregate("BadAggregate", order, customer);
        limit.add_projection(items, line_item);

        let result = validate_limit_cone(&limit, &graph);
        assert!(!result.is_ok());
        assert!(result.errors().any(|e| e.code == "E0112"));
    }

    #[test]
    fn test_limit_cone_non_existent_projection_morphism() {
        let mut graph = crate::sketch::Graph::new();
        let order = graph.add_object("Order");
        let line_item = graph.add_object("LineItem");

        let mut limit = LimitCone::aggregate("BadAggregate", order, order);
        limit.add_projection(MorphismId(999), line_item);

        let result = validate_limit_cone(&limit, &graph);
        assert!(!result.is_ok());
        assert!(result.errors().any(|e| e.code == "E0113"));
    }

    #[test]
    fn test_limit_cone_non_existent_projection_target() {
        let mut graph = crate::sketch::Graph::new();
        let order = graph.add_object("Order");
        let line_item = graph.add_object("LineItem");
        let items = graph.add_morphism("items", order, line_item);

        let mut limit = LimitCone::aggregate("BadAggregate", order, order);
        limit.add_projection(items, ObjectId(999));

        let result = validate_limit_cone(&limit, &graph);
        assert!(!result.is_ok());
        assert!(result.errors().any(|e| e.code == "E0114"));
    }

    #[test]
    fn test_limit_cone_projection_morphism_wrong_source() {
        let mut graph = crate::sketch::Graph::new();
        let order = graph.add_object("Order");
        let line_item = graph.add_object("LineItem");
        let customer = graph.add_object("Customer");

        // Morphism from Customer, not Order
        let wrong_morph = graph.add_morphism("wrong", customer, line_item);

        let mut limit = LimitCone::aggregate("BadAggregate", order, order);
        limit.add_projection(wrong_morph, line_item);

        let result = validate_limit_cone(&limit, &graph);
        assert!(!result.is_ok());
        assert!(result.errors().any(|e| e.code == "E0115"));
    }

    #[test]
    fn test_limit_cone_projection_morphism_target_mismatch() {
        let mut graph = crate::sketch::Graph::new();
        let order = graph.add_object("Order");
        let line_item = graph.add_object("LineItem");
        let product = graph.add_object("Product");

        // Morphism goes to LineItem, but projection says Product
        let items = graph.add_morphism("items", order, line_item);

        let mut limit = LimitCone::aggregate("BadAggregate", order, order);
        limit.add_projection(items, product); // Mismatch!

        let result = validate_limit_cone(&limit, &graph);
        assert!(!result.is_ok());
        assert!(result.errors().any(|e| e.code == "E0116"));
    }

    #[test]
    fn test_limit_cone_duplicate_projection_targets() {
        let mut graph = crate::sketch::Graph::new();
        let order = graph.add_object("Order");
        let line_item = graph.add_object("LineItem");

        let items1 = graph.add_morphism("items1", order, line_item);
        let items2 = graph.add_morphism("items2", order, line_item);

        let mut limit = LimitCone::aggregate("BadAggregate", order, order);
        limit.add_projection(items1, line_item);
        limit.add_projection(items2, line_item); // Duplicate target!

        let result = validate_limit_cone(&limit, &graph);
        assert!(!result.is_ok());
        assert!(result.errors().any(|e| e.code == "E0117"));
    }

    #[test]
    fn test_limit_cone_empty_warning() {
        let mut graph = crate::sketch::Graph::new();
        let order = graph.add_object("Order");

        let limit = LimitCone::aggregate("EmptyAggregate", order, order);

        let result = validate_limit_cone(&limit, &graph);
        assert!(result.is_ok()); // Warnings don't fail
        assert!(result.warnings().any(|e| e.code == "W0110"));
    }

    #[test]
    fn test_limit_cone_aggregate_without_root_warning() {
        let mut graph = crate::sketch::Graph::new();
        let order = graph.add_object("Order");

        // Create aggregate without root using the raw struct
        let limit = LimitCone {
            name: "NoRootAggregate".to_string(),
            apex: order,
            projections: Vec::new(),
            is_aggregate: true,
            root: None,
        };

        let result = validate_limit_cone(&limit, &graph);
        assert!(result.is_ok()); // Warnings don't fail
        assert!(result.warnings().any(|e| e.code == "W0111"));
    }

    #[test]
    fn test_validate_limits_duplicate_names_warning() {
        let mut sketch = Sketch::new("Test");
        let order = sketch.add_object("Order");

        // Add two limits with the same name
        let limit1 = LimitCone::aggregate("MyAggregate", order, order);
        let limit2 = LimitCone::aggregate("MyAggregate", order, order);

        sketch.limits.push(limit1);
        sketch.limits.push(limit2);

        let result = validate_limits(&sketch);
        assert!(result.is_ok()); // Warnings don't fail
        assert!(result.warnings().any(|e| e.code == "W0112"));
    }

    // =============================================================
    // Colimit Cocone Validation Tests
    // =============================================================

    #[test]
    fn test_valid_colimit_cocone() {
        let mut graph = crate::sketch::Graph::new();
        let order_status = graph.add_object("OrderStatus");

        let colimit = ColimitCocone::enumeration(
            "OrderStatus",
            order_status,
            vec!["Pending".into(), "Confirmed".into(), "Shipped".into()],
        );

        let result = validate_colimit_cocone(&colimit, &graph);
        assert!(result.is_ok());
    }

    #[test]
    fn test_colimit_cocone_non_existent_apex() {
        let graph = crate::sketch::Graph::new();

        let colimit = ColimitCocone::new("BadEnum", ObjectId(999));

        let result = validate_colimit_cocone(&colimit, &graph);
        assert!(!result.is_ok());
        assert!(result.errors().any(|e| e.code == "E0120"));
    }

    #[test]
    fn test_colimit_cocone_non_existent_injection_source() {
        let mut graph = crate::sketch::Graph::new();
        let order_status = graph.add_object("OrderStatus");

        let mut colimit = ColimitCocone::new("BadEnum", order_status);
        colimit.add_variant("BadVariant", ObjectId(999));

        let result = validate_colimit_cocone(&colimit, &graph);
        assert!(!result.is_ok());
        assert!(result.errors().any(|e| e.code == "E0121"));
    }

    #[test]
    fn test_colimit_cocone_empty_variant_name() {
        let mut graph = crate::sketch::Graph::new();
        let order_status = graph.add_object("OrderStatus");

        let mut colimit = ColimitCocone::new("BadEnum", order_status);
        colimit.add_variant("", order_status);

        let result = validate_colimit_cocone(&colimit, &graph);
        assert!(!result.is_ok());
        assert!(result.errors().any(|e| e.code == "E0122"));
    }

    #[test]
    fn test_colimit_cocone_duplicate_variant_names() {
        let mut graph = crate::sketch::Graph::new();
        let order_status = graph.add_object("OrderStatus");

        let mut colimit = ColimitCocone::new("BadEnum", order_status);
        colimit.add_variant("Pending", order_status);
        colimit.add_variant("Pending", order_status); // Duplicate!

        let result = validate_colimit_cocone(&colimit, &graph);
        assert!(!result.is_ok());
        assert!(result.errors().any(|e| e.code == "E0123"));
    }

    #[test]
    fn test_colimit_cocone_empty_warning() {
        let mut graph = crate::sketch::Graph::new();
        let order_status = graph.add_object("OrderStatus");

        let colimit = ColimitCocone::new("EmptyEnum", order_status);

        let result = validate_colimit_cocone(&colimit, &graph);
        assert!(result.is_ok()); // Warnings don't fail
        assert!(result.warnings().any(|e| e.code == "W0120"));
    }

    #[test]
    fn test_colimit_cocone_single_variant_warning() {
        let mut graph = crate::sketch::Graph::new();
        let order_status = graph.add_object("OrderStatus");

        let colimit = ColimitCocone::enumeration(
            "SingleEnum",
            order_status,
            vec!["OnlyOne".into()],
        );

        let result = validate_colimit_cocone(&colimit, &graph);
        assert!(result.is_ok()); // Warnings don't fail
        assert!(result.warnings().any(|e| e.code == "W0121"));
    }

    #[test]
    fn test_validate_colimits_duplicate_names_warning() {
        let mut sketch = Sketch::new("Test");
        let order_status = sketch.add_object("OrderStatus");

        // Add two colimits with the same name
        let colimit1 = ColimitCocone::enumeration(
            "Status",
            order_status,
            vec!["A".into(), "B".into()],
        );
        let colimit2 = ColimitCocone::enumeration(
            "Status",
            order_status,
            vec!["X".into(), "Y".into()],
        );

        sketch.colimits.push(colimit1);
        sketch.colimits.push(colimit2);

        let result = validate_colimits(&sketch);
        assert!(result.is_ok()); // Warnings don't fail
        assert!(result.warnings().any(|e| e.code == "W0122"));
    }

    #[test]
    fn test_sketch_validation_includes_limit_colimit() {
        let mut sketch = Sketch::new("Commerce");
        let order = sketch.add_object("Order");
        let line_item = sketch.add_object("LineItem");
        let items = sketch.graph.add_morphism("items", order, line_item);

        // Add valid aggregate
        let mut aggregate = LimitCone::aggregate("OrderAggregate", order, order);
        aggregate.add_projection(items, line_item);
        sketch.limits.push(aggregate);

        // Add valid enum
        let order_status = sketch.add_object("OrderStatus");
        let status_enum = ColimitCocone::enumeration(
            "OrderStatus",
            order_status,
            vec!["Pending".into(), "Shipped".into()],
        );
        sketch.colimits.push(status_enum);

        let result = validate_sketch(&sketch);
        assert!(result.is_ok(), "Errors: {:?}", result.errors().collect::<Vec<_>>());
    }

    #[test]
    fn test_sketch_validation_catches_invalid_limit() {
        let mut sketch = Sketch::new("Commerce");
        let order = sketch.add_object("Order");

        // Add limit with non-existent projection morphism
        let mut bad_limit = LimitCone::aggregate("BadAggregate", order, order);
        bad_limit.add_projection(MorphismId(999), ObjectId(888));
        sketch.limits.push(bad_limit);

        let result = validate_sketch(&sketch);
        assert!(!result.is_ok());
        assert!(result.errors().any(|e| e.code == "E0113"));
    }

    #[test]
    fn test_sketch_validation_catches_invalid_colimit() {
        let mut sketch = Sketch::new("Commerce");
        let order_status = sketch.add_object("OrderStatus");

        // Add colimit with duplicate variant names
        let mut bad_colimit = ColimitCocone::new("BadEnum", order_status);
        bad_colimit.add_variant("Dup", order_status);
        bad_colimit.add_variant("Dup", order_status);
        sketch.colimits.push(bad_colimit);

        let result = validate_sketch(&sketch);
        assert!(!result.is_ok());
        assert!(result.errors().any(|e| e.code == "E0123"));
    }
}
