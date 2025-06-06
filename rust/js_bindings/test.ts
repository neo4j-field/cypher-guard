const nativeBinding = require('./dist/index.js');

console.log('Exported functions:', nativeBinding);

const schemaJson = `{
    "node_props": {
        "Person": [
            {"name": "name", "neo4j_type": {"type": "STRING"}},
            {"name": "age", "neo4j_type": {"type": "INTEGER"}},
            {"name": "created", "neo4j_type": {"type": "BOOLEAN"}}
        ]
    },
    "rel_props": {
        "KNOWS": [
            {"name": "since", "neo4j_type": {"type": "DATE_TIME"}}
        ]
    },
    "relationships": [
        {"start": "Person", "end": "Person", "rel_type": "KNOWS"}
    ],
    "metadata": {
        "index": [],
        "constraint": []
    }
}`;

// Test valid query
const validQuery = "MATCH (a:Person)-[r:KNOWS]->(b:Person) RETURN a.name, r.since";
console.log("Valid query test:", nativeBinding.validateCypherJs(validQuery, schemaJson));

// Test invalid query
const invalidQuery = "MATCH (a:Person)-[r:FOLLOWS]->(b:Person) RETURN a.name";
console.log("Invalid query test:", nativeBinding.getValidationErrorsJs(invalidQuery, schemaJson)); 