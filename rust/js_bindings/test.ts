const assert = require('assert');
const nativeBinding = require('./dist/index.js');

const schemaJson = `{
    "node_props": {
        "Person": [
            {"name": "name", "neo4j_type": "STRING"},
            {"name": "age", "neo4j_type": "INTEGER"},
            {"name": "created", "neo4j_type": "BOOLEAN"}
        ],
        "Movie": [
            {"name": "title", "neo4j_type": "STRING"},
            {"name": "year", "neo4j_type": "INTEGER"}
        ],
        "Station": [
            {"name": "name", "neo4j_type": "STRING"},
            {"name": "location", "neo4j_type": "POINT"}
        ],
        "Stop": [
            {"name": "departs", "neo4j_type": "STRING"},
            {"name": "arrives", "neo4j_type": "STRING"}
        ]
    },
    "rel_props": {
        "KNOWS": [
            {"name": "since", "neo4j_type": "DATE_TIME"}
        ],
        "ACTED_IN": [
            {"name": "role", "neo4j_type": "STRING"}
        ],
        "CALLS_AT": [],
        "NEXT": [],
        "LINK": [
            {"name": "distance", "neo4j_type": "FLOAT"}
        ]
    },
    "relationships": [
        {"start": "Person", "end": "Person", "rel_type": "KNOWS"},
        {"start": "Person", "end": "Movie", "rel_type": "ACTED_IN"},
        {"start": "Stop", "end": "Station", "rel_type": "CALLS_AT"},
        {"start": "Stop", "end": "Stop", "rel_type": "NEXT"},
        {"start": "Station", "end": "Station", "rel_type": "LINK"}
    ],
    "metadata": {
        "index": [],
        "constraint": []
    }
}`;

const validQueries = [
    "MATCH (a:Person)-[r:KNOWS]->(b:Person) RETURN a.name, r.since",
    "MATCH (a:Person)-[r:ACTED_IN]->(m:Movie) RETURN a.name, m.title, r.role",
    "MATCH (a:Person) WHERE a.age > 30 RETURN a.name",
    "MATCH (a:Person)-[r:KNOWS]->(b:Person) WHERE a.name = 'Alice' RETURN b.name",
    "MATCH (a:Person)-[r:ACTED_IN]->(m:Movie) WHERE m.year > 2000 RETURN a.name, m.title",
    "MATCH (a:Person)-[r:KNOWS]->(b:Person) RETURN a.name, b.name",
    "MATCH (a:Person)-[r:ACTED_IN]->(m:Movie) RETURN a.name, m.title",
    "MATCH (a:Person) RETURN a.name",
    "MATCH (a:Person)-[r:KNOWS]->(b:Person) RETURN a.name, r.since, b.name",
    "MATCH (a:Person)-[r:ACTED_IN]->(m:Movie) RETURN a.name, m.title, r.role",
    "MATCH (a:Person) WHERE a.age > 30 AND a.name = 'Alice' RETURN a.name",
    "MATCH (a:Station)-[r:LINK]->(b:Station) WHERE point.distance(a.location, b.location) > 10 RETURN a.name",
    "MATCH (a:Person)-[r:KNOWS]->(b:Person) WHERE r.since > 2020 RETURN a.name"
];

const validQPPQueries = [
    "MATCH ((a:Person)-[r:KNOWS]->(b:Person)){1,3} RETURN a.name, b.name",
    "MATCH ((a:Person)-[r:KNOWS]->(b:Person)){2,4} RETURN a.name, b.name",
    "MATCH ((a:Person)-[r:KNOWS]->(b:Person)){3,5} RETURN a.name, b.name",
    "MATCH ((a:Person)-[r:KNOWS]->(b:Person)){4,6} RETURN a.name, b.name",
    "MATCH ((a:Person)-[r:KNOWS]->(b:Person)){5,7} RETURN a.name, b.name",
    "MATCH ((a:Stop)-[r:NEXT]->(b:Stop)){1,3} RETURN a.departs, b.arrives",
    "MATCH ((a:Station)-[r:LINK]->(b:Station)){1,3} RETURN a.name, b.name",
    "MATCH ((a:Stop)-[r:CALLS_AT]->(b:Station)){1,3} RETURN a.departs, b.name",
    "MATCH ((a:Person)-[r:ACTED_IN]->(b:Movie)){1,3} RETURN a.name, b.title",
    "MATCH ((a:Station)-[r:LINK]->(b:Station)){1,3} WHERE point.distance(a.location, b.location) > 10 RETURN a.name"
];

const invalidPropertyQueries = [
    "MATCH (a:Person)-[r:KNOWS]->(b:Person) RETURN a.height",
    "MATCH (a:Person)-[r:KNOWS]->(b:Person) RETURN a.name, r.invalid_property"
];

const invalidRelationshipTypeQueries = [
    "MATCH (a:Person)-[r:FOLLOWS]->(b:Person) RETURN a.name",
    "MATCH (a:Station)-[r:CONNECTS]->(b:Station) RETURN a.name"
];

const invalidNodeLabelQueries = [
    "MATCH (a:User) RETURN a.name",
    "MATCH (a:Train) RETURN a.name"
];

const invalidPropertyTypeQueries = [
    "MATCH (a:Person) WHERE a.age = '30' RETURN a.name",
    "MATCH (a:Person) WHERE a.name = 123 RETURN a.name"
];

const invalidRelationshipDirectionQueries = [
    "MATCH (a:Person)<-[r:ACTED_IN]-(b:Movie) RETURN a.name",
    "MATCH (a:Stop)<-[r:CALLS_AT]-(b:Station) RETURN a.name"
];

const invalidRelationshipPropertyQueries = [
    "MATCH (a:Person)-[r:KNOWS]->(b:Person) WHERE r.role = 'friend' RETURN a.name",
    "MATCH (a:Station)-[r:LINK]->(b:Station) WHERE r.duration = 10 RETURN a.name"
];

function testValidQueries() {
    for (const query of validQueries) {
        assert(nativeBinding.validateCypher(query, schemaJson), `Query should be valid: ${query}`);
    }
    for (const query of validQPPQueries) {
        assert(nativeBinding.validateCypher(query, schemaJson), `QPP Query should be valid: ${query}`);
    }
}

function testInvalidQueries() {
    for (const query of invalidPropertyQueries) {
        const errors = nativeBinding.getValidationErrors(query, schemaJson);
        assert(errors.length > 0, `Should have errors for invalid property: ${query}`);
    }
    for (const query of invalidRelationshipTypeQueries) {
        const errors = nativeBinding.getValidationErrors(query, schemaJson);
        assert(errors.length > 0, `Should have errors for invalid relationship type: ${query}`);
    }
    for (const query of invalidNodeLabelQueries) {
        const errors = nativeBinding.getValidationErrors(query, schemaJson);
        assert(errors.length > 0, `Should have errors for invalid node label: ${query}`);
    }
    for (const query of invalidPropertyTypeQueries) {
        const errors = nativeBinding.getValidationErrors(query, schemaJson);
        assert(errors.length > 0, `Should have errors for invalid property type: ${query}`);
    }
    for (const query of invalidRelationshipDirectionQueries) {
        const errors = nativeBinding.getValidationErrors(query, schemaJson);
        assert(errors.length > 0, `Should have errors for invalid relationship direction: ${query}`);
    }
    for (const query of invalidRelationshipPropertyQueries) {
        const errors = nativeBinding.getValidationErrors(query, schemaJson);
        assert(errors.length > 0, `Should have errors for invalid relationship property: ${query}`);
    }
}

testValidQueries();
testInvalidQueries();

console.log('✅ All TypeScript validation tests passed!'); 