import { validateCypher, getValidationErrors } from './index';

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
            {"name": "since", "neo4j_type": {"type": "DATETIME"}}
        ]
    },
    "relationships": [
        {"start": "Person", "end": "Person", "rel_type": "KNOWS"}
    ],
    "metadata": {
        "indexes": [],
        "constraints": []
    }
}`;

// Test valid query
const validQuery = "MATCH (a:Person)-[r:KNOWS]->(b:Person) RETURN a.name, r.since";
console.log("Valid query test:", validateCypher(validQuery, schemaJson));

// Test invalid query
const invalidQuery = "MATCH (a:Person)-[r:FOLLOWS]->(b:Person) RETURN a.name";
console.log("Invalid query test:", getValidationErrors(invalidQuery, schemaJson)); 