// Root of the AST
#[derive(Debug, PartialEq, Clone)]
pub struct Query {
    pub match_clause: Option<MatchClause>,
    pub merge_clause: Option<MergeClause>,
    pub create_clause: Option<CreateClause>,
    pub with_clause: Option<WithClause>,
    pub where_clause: Option<WhereClause>,
    pub return_clause: Option<ReturnClause>,
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
        left: String,
        operator: String,
        right: String,
    },
    FunctionCall {
        function: String,
        arguments: Vec<String>,
    },
    PathProperty {
        path_var: String,
        property: String,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub struct PathProperty {
    pub path_var: String,
    pub property: String,
    pub value: PropertyValue,
}

// Elements of a MATCH clause
#[derive(Debug, PartialEq, Clone)]
pub enum MatchElement {
    Pattern(Vec<PatternElement>),
    QuantifiedPathPattern(QuantifiedPathPattern),
}

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
}

// Node pattern
#[derive(Debug, PartialEq, Clone)]
pub struct NodePattern {
    pub variable: Option<String>,
    pub label: Option<String>,
    pub properties: Option<Vec<Property>>,
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
}

#[derive(Debug, PartialEq, Clone)]
pub enum RelationshipPattern {
    Regular(RelationshipDetails),
    OptionalRelationship(RelationshipDetails),
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
pub enum WithItem {
    Variable(String),                            // e.g. WITH a
    Alias { expression: String, alias: String }, // e.g. WITH a AS b
    Wildcard,                                    // e.g. WITH *
}
