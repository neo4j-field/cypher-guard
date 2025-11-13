// Root of the AST
#[derive(Debug, PartialEq, Clone)]
pub struct Query {
    pub match_clauses: Vec<MatchClause>,
    pub merge_clauses: Vec<MergeClause>,
    pub create_clauses: Vec<CreateClause>,
    pub insert_clauses: Vec<InsertClause>,
    pub with_clauses: Vec<WithClause>,
    pub where_clauses: Vec<WhereClause>,
    pub return_clauses: Vec<ReturnClause>,
    pub unwind_clauses: Vec<UnwindClause>,
    pub call_clauses: Vec<CallClause>,
}

// RETURN clause (simple)
#[derive(Debug, PartialEq, Clone)]
pub struct ReturnClause {
    pub items: Vec<String>,
}

// MATCH clause
#[derive(Debug, PartialEq, Clone)]
pub struct MatchClause {
    pub elements: Vec<MatchElement>,
    pub is_optional: bool,
}

// WHERE clause
#[derive(Debug, PartialEq, Clone)]
pub struct WhereClause {
    pub conditions: Vec<WhereCondition>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum WhereCondition {
    Comparison {
        left: PropertyValue,
        operator: String,
        right: PropertyValue,
    },
    FunctionCall {
        function: String,
        arguments: Vec<String>,
    },
    PathProperty {
        path_var: String,
        property: String,
    },
    And(Box<WhereCondition>, Box<WhereCondition>),
    Or(Box<WhereCondition>, Box<WhereCondition>),
    Not(Box<WhereCondition>),
    Parenthesized(Box<WhereCondition>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct PathProperty {
    pub path_var: String,
    pub property: String,
    pub value: PropertyValue,
}

// Elements of a MATCH clause
#[derive(Debug, PartialEq, Clone)]
pub struct MatchElement {
    pub path_var: Option<String>,
    pub pattern: Vec<PatternElement>,
}

// Quantified path pattern details
#[derive(Debug, PartialEq, Clone)]
pub struct QuantifiedPathPattern {
    pub pattern: Vec<PatternElement>,
    pub min: Option<u32>,
    pub max: Option<u32>,
    pub where_clause: Option<WhereClause>,
    pub path_variable: Option<String>,
}

// Nodes and relationships that form a pattern
#[derive(Debug, PartialEq, Clone)]
pub enum PatternElement {
    Node(NodePattern),
    Relationship(RelationshipPattern),
    QuantifiedPathPattern(QuantifiedPathPattern),
}

// Node pattern
#[derive(Debug, PartialEq, Clone)]
pub struct NodePattern {
    pub variable: Option<String>,
    pub label: Option<String>,
    pub properties: Option<Vec<Property>>,
}

// Quantifier
#[derive(Debug, PartialEq, Clone)]
pub struct Quantifier {
    pub min: Option<u32>,
    pub max: Option<u32>,
}

// Relationship pattern
#[derive(Debug, PartialEq, Clone)]
pub struct RelationshipDetails {
    pub variable: Option<String>,
    pub direction: Direction,
    pub properties: Option<Vec<Property>>,
    pub rel_type: Option<String>,
    pub length: Option<LengthRange>,
    pub where_clause: Option<WhereClause>,
    pub quantifier: Option<Quantifier>,
    pub is_optional: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub enum RelationshipPattern {
    Regular(RelationshipDetails),
    OptionalRelationship(RelationshipDetails),
}

impl RelationshipPattern {
    pub fn direction(&self) -> Direction {
        match self {
            RelationshipPattern::Regular(details)
            | RelationshipPattern::OptionalRelationship(details) => details.direction.clone(),
        }
    }

    pub fn rel_type(&self) -> Option<&str> {
        match self {
            RelationshipPattern::Regular(details)
            | RelationshipPattern::OptionalRelationship(details) => details.rel_type.as_deref(),
        }
    }

    pub fn properties(&self) -> Option<&Vec<Property>> {
        match self {
            RelationshipPattern::Regular(details)
            | RelationshipPattern::OptionalRelationship(details) => details.properties.as_ref(),
        }
    }

    pub fn quantifier(&self) -> Option<&Quantifier> {
        match self {
            RelationshipPattern::Regular(details)
            | RelationshipPattern::OptionalRelationship(details) => details.quantifier.as_ref(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Direction {
    Left,
    Right,
    Undirected,
}

#[derive(Debug, PartialEq, Clone)]
pub struct LengthRange {
    pub min: Option<u32>,
    pub max: Option<u32>,
}

// Key-value property pairs
#[derive(Debug, PartialEq, Clone)]
pub struct Property {
    pub key: String,
    pub value: PropertyValue,
}

#[derive(Debug, PartialEq, Clone)]
pub enum PropertyValue {
    String(String),
    Number(i64),
    Boolean(bool),
    Null,
    List(Vec<PropertyValue>),
    Map(std::collections::HashMap<String, PropertyValue>),
    FunctionCall {
        name: String,
        args: Vec<PropertyValue>,
    },
    Parameter(String),
    Identifier(String), // For variable references and property access
}

// MERGE clause
#[derive(Debug, PartialEq, Clone)]
pub struct MergeClause {
    pub elements: Vec<MatchElement>,
    pub on_create: Option<OnCreateClause>,
    pub on_match: Option<OnMatchClause>,
}

// CREATE clause
#[derive(Debug, PartialEq, Clone)]
pub struct CreateClause {
    pub elements: Vec<MatchElement>,
}

// INSERT clause (synonym for CREATE, but uses & for multiple labels)
#[derive(Debug, PartialEq, Clone)]
pub struct InsertClause {
    pub elements: Vec<MatchElement>,
}

// ON CREATE clause
#[derive(Debug, PartialEq, Clone)]
pub struct OnCreateClause {
    pub set_clauses: Vec<SetClause>,
}

// ON MATCH clause
#[derive(Debug, PartialEq, Clone)]
pub struct OnMatchClause {
    pub set_clauses: Vec<SetClause>,
}

// SET clause
#[derive(Debug, PartialEq, Clone)]
pub struct SetClause {
    pub variable: String,
    pub property: String,
    pub value: PropertyValue,
}

#[derive(Debug, PartialEq, Clone)]
pub struct WithClause {
    pub items: Vec<WithItem>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum WithExpression {
    Identifier(String),
    PropertyAccess {
        variable: String,
        property: String,
    },
    FunctionCall {
        name: String,
        args: Vec<WithExpression>,
    },
    Wildcard,
}

#[derive(Debug, PartialEq, Clone)]
pub struct WithItem {
    pub expression: WithExpression,
    pub alias: Option<String>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct UnwindClause {
    pub expression: UnwindExpression,
    pub variable: String,
}

#[derive(Debug, PartialEq, Clone)]
pub enum UnwindExpression {
    List(Vec<PropertyValue>),
    Identifier(String),
    FunctionCall {
        name: String,
        args: Vec<PropertyValue>,
    },
    Parameter(String),
}

// CALL clause for subqueries and procedures
#[derive(Debug, PartialEq, Clone)]
pub struct CallClause {
    pub subquery: Option<Query>,           // For CALL { ... } subqueries
    pub procedure: Option<String>,         // For CALL procedure() calls
    pub yield_clause: Option<Vec<String>>, // For YIELD clause
}
