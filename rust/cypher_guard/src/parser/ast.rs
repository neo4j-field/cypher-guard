// Root of the AST
#[derive(Debug, PartialEq, Clone)]
pub struct Query {
    pub match_clause: MatchClause,
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
