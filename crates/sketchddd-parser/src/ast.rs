//! Abstract Syntax Tree for SketchDDD DSL.
//!
//! This module defines the AST nodes that represent parsed SketchDDD source code.
//! The AST closely mirrors the grammar structure and serves as an intermediate
//! representation before conversion to the semantic model.

use serde::{Deserialize, Serialize};

// =============================================================
// Source Location
// =============================================================

/// Source location for error reporting.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Span {
    /// Byte offset of the start position
    pub start: usize,
    /// Byte offset of the end position
    pub end: usize,
    /// Line number (1-indexed)
    pub line: u32,
    /// Column number (1-indexed)
    pub column: u32,
}

impl Span {
    /// Create a new span with the given positions.
    pub fn new(start: usize, end: usize, line: u32, column: u32) -> Self {
        Self {
            start,
            end,
            line,
            column,
        }
    }
}

// =============================================================
// File (Top Level)
// =============================================================

/// A complete parsed SketchDDD file.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct File {
    /// Context declarations in the file
    pub contexts: Vec<ContextDecl>,
    /// Context map declarations in the file
    pub context_maps: Vec<ContextMapDecl>,
}

// =============================================================
// Context Declaration
// =============================================================

/// A context declaration representing a bounded context.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextDecl {
    /// Name of the bounded context
    pub name: String,
    /// Objects declared in the context
    pub objects: Vec<ObjectDecl>,
    /// Entities declared in the context
    pub entities: Vec<EntityDecl>,
    /// Morphisms (relationships) declared in the context
    pub morphisms: Vec<MorphismDecl>,
    /// Aggregate definitions
    pub aggregates: Vec<AggregateDecl>,
    /// Value object definitions
    pub value_objects: Vec<ValueObjectDecl>,
    /// Enum/sum type definitions
    pub enums: Vec<EnumDecl>,
    /// Path equation definitions
    pub equations: Vec<EquationDecl>,
    /// Source location
    pub span: Span,
}

impl Default for ContextDecl {
    fn default() -> Self {
        Self {
            name: String::new(),
            objects: Vec::new(),
            entities: Vec::new(),
            morphisms: Vec::new(),
            aggregates: Vec::new(),
            value_objects: Vec::new(),
            enums: Vec::new(),
            equations: Vec::new(),
            span: Span::default(),
        }
    }
}

// =============================================================
// Object Declaration
// =============================================================

/// An object declaration representing a domain concept.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObjectDecl {
    /// Name of the object
    pub name: String,
    /// Source location
    pub span: Span,
}

impl ObjectDecl {
    /// Create a new object declaration.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            span: Span::default(),
        }
    }
}

// =============================================================
// Entity Declaration
// =============================================================

/// An entity declaration with optional fields.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityDecl {
    /// Name of the entity
    pub name: String,
    /// Fields of the entity
    pub fields: Vec<FieldDecl>,
    /// Source location
    pub span: Span,
}

impl EntityDecl {
    /// Create a new entity declaration.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            fields: Vec::new(),
            span: Span::default(),
        }
    }
}

// =============================================================
// Morphism Declaration
// =============================================================

/// A morphism declaration representing a relationship between objects.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MorphismDecl {
    /// Name of the morphism
    pub name: String,
    /// Source type
    pub source: TypeExpr,
    /// Target type
    pub target: TypeExpr,
    /// Optional annotations
    pub annotations: Vec<Annotation>,
    /// Source location
    pub span: Span,
}

impl MorphismDecl {
    /// Create a new morphism declaration.
    pub fn new(name: impl Into<String>, source: TypeExpr, target: TypeExpr) -> Self {
        Self {
            name: name.into(),
            source,
            target,
            annotations: Vec::new(),
            span: Span::default(),
        }
    }
}

/// An annotation on a morphism.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Annotation {
    /// Annotation name
    pub name: String,
    /// Optional annotation value
    pub value: Option<String>,
}

// =============================================================
// Type Expression
// =============================================================

/// A type expression representing a type reference.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TypeExpr {
    /// A simple type reference (e.g., `Customer`)
    Simple(String),
    /// A generic type (e.g., `List<Order>`)
    Generic {
        name: String,
        args: Vec<TypeExpr>,
    },
    /// An optional type (e.g., `Customer?`)
    Optional(Box<TypeExpr>),
}

impl TypeExpr {
    /// Create a simple type expression.
    pub fn simple(name: impl Into<String>) -> Self {
        Self::Simple(name.into())
    }

    /// Create a generic type expression with a single argument.
    pub fn generic(name: impl Into<String>, arg: TypeExpr) -> Self {
        Self::Generic {
            name: name.into(),
            args: vec![arg],
        }
    }

    /// Create a generic type expression with multiple arguments.
    pub fn generic_multi(name: impl Into<String>, args: Vec<TypeExpr>) -> Self {
        Self::Generic {
            name: name.into(),
            args,
        }
    }

    /// Create an optional type expression.
    pub fn optional(inner: TypeExpr) -> Self {
        Self::Optional(Box::new(inner))
    }

    /// Get the base type name.
    pub fn base_name(&self) -> &str {
        match self {
            TypeExpr::Simple(name) => name,
            TypeExpr::Generic { name, .. } => name,
            TypeExpr::Optional(inner) => inner.base_name(),
        }
    }
}

// =============================================================
// Aggregate Declaration
// =============================================================

/// An aggregate declaration defining an aggregate root.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregateDecl {
    /// Name of the aggregate
    pub name: String,
    /// Root entity name
    pub root: Option<String>,
    /// Contained entities
    pub contains: Vec<String>,
    /// Invariants
    pub invariants: Vec<InvariantDecl>,
    /// Source location
    pub span: Span,
}

impl AggregateDecl {
    /// Create a new aggregate declaration.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            root: None,
            contains: Vec::new(),
            invariants: Vec::new(),
            span: Span::default(),
        }
    }
}

/// An invariant declaration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvariantDecl {
    /// The invariant expression as a string
    pub expression: Expr,
    /// Source location
    pub span: Span,
}

// =============================================================
// Value Object Declaration
// =============================================================

/// A value object declaration defining a value type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueObjectDecl {
    /// Name of the value object
    pub name: String,
    /// Fields of the value object
    pub fields: Vec<FieldDecl>,
    /// Source location
    pub span: Span,
}

impl ValueObjectDecl {
    /// Create a new value object declaration.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            fields: Vec::new(),
            span: Span::default(),
        }
    }
}

/// A field declaration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldDecl {
    /// Field name
    pub name: String,
    /// Field type
    pub type_expr: TypeExpr,
    /// Source location
    pub span: Span,
}

impl FieldDecl {
    /// Create a new field declaration.
    pub fn new(name: impl Into<String>, type_expr: TypeExpr) -> Self {
        Self {
            name: name.into(),
            type_expr,
            span: Span::default(),
        }
    }
}

// =============================================================
// Enum Declaration
// =============================================================

/// An enum declaration defining a sum type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnumDecl {
    /// Name of the enum
    pub name: String,
    /// Variants of the enum
    pub variants: Vec<VariantDecl>,
    /// Source location
    pub span: Span,
}

impl EnumDecl {
    /// Create a new enum declaration.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            variants: Vec::new(),
            span: Span::default(),
        }
    }
}

/// An enum variant declaration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VariantDecl {
    /// Variant name
    pub name: String,
    /// Optional payload types
    pub payload: Vec<TypeExpr>,
    /// Source location
    pub span: Span,
}

impl VariantDecl {
    /// Create a new variant declaration.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            payload: Vec::new(),
            span: Span::default(),
        }
    }

    /// Create a new variant declaration with payload.
    pub fn with_payload(name: impl Into<String>, payload: Vec<TypeExpr>) -> Self {
        Self {
            name: name.into(),
            payload,
            span: Span::default(),
        }
    }
}

// =============================================================
// Equation Declaration
// =============================================================

/// A path equation declaration (business rule).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EquationDecl {
    /// Optional name for the equation
    pub name: Option<String>,
    /// Left-hand side path
    pub lhs: Path,
    /// Right-hand side path
    pub rhs: Path,
    /// Source location
    pub span: Span,
}

/// A path through morphisms.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Path {
    /// Components of the path
    pub components: Vec<String>,
}

impl Path {
    /// Create a new path from components.
    pub fn new(components: Vec<String>) -> Self {
        Self { components }
    }

    /// Create a path from a single component.
    pub fn single(name: impl Into<String>) -> Self {
        Self {
            components: vec![name.into()],
        }
    }
}

// =============================================================
// Context Map Declaration
// =============================================================

/// A context map declaration representing a relationship between contexts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextMapDecl {
    /// Name of the context map
    pub name: String,
    /// Source context name
    pub source_context: String,
    /// Target context name
    pub target_context: String,
    /// Relationship pattern
    pub pattern: Option<String>,
    /// Object mappings
    pub object_mappings: Vec<ObjectMappingDecl>,
    /// Morphism mappings
    pub morphism_mappings: Vec<MorphismMappingDecl>,
    /// Source location
    pub span: Span,
}

impl ContextMapDecl {
    /// Create a new context map declaration.
    pub fn new(
        name: impl Into<String>,
        source: impl Into<String>,
        target: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            source_context: source.into(),
            target_context: target.into(),
            pattern: None,
            object_mappings: Vec::new(),
            morphism_mappings: Vec::new(),
            span: Span::default(),
        }
    }
}

/// An object mapping in a context map.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObjectMappingDecl {
    /// Source object name
    pub source: String,
    /// Target object name
    pub target: String,
    /// Optional description
    pub description: Option<String>,
    /// Source location
    pub span: Span,
}

/// A morphism mapping in a context map.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MorphismMappingDecl {
    /// Source morphism name
    pub source: String,
    /// Target morphism name
    pub target: String,
    /// Optional description
    pub description: Option<String>,
    /// Source location
    pub span: Span,
}

// =============================================================
// Expression AST
// =============================================================

/// An expression node.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Expr {
    /// A literal number
    Number(f64),
    /// A literal string
    String(String),
    /// An identifier or path expression
    Path(Path),
    /// A binary operation
    BinaryOp {
        left: Box<Expr>,
        op: BinaryOperator,
        right: Box<Expr>,
    },
    /// A unary operation
    UnaryOp {
        op: UnaryOperator,
        operand: Box<Expr>,
    },
    /// A function call
    FunctionCall {
        name: String,
        args: Vec<Expr>,
    },
    /// An index expression
    Index {
        expr: Box<Expr>,
        index: Box<Expr>,
    },
}

/// Binary operators.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}

/// Unary operators.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnaryOperator {
    Not,
    Neg,
}
