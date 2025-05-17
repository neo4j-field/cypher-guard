from cypher_guard import validate_cypher_py, get_validation_errors_py

# Example schema JSON
schema_json = '''
{
    "node_props": {
        "Person": [
            {"name": "name", "neo4j_type": {"type": "STRING"}},
            {"name": "age", "neo4j_type": {"type": "INTEGER"}}
        ],
        "Movie": [
            {"name": "title", "neo4j_type": {"type": "STRING"}},
            {"name": "year", "neo4j_type": {"type": "INTEGER"}}
        ]
    },
    "rel_props": {
        "KNOWS": [
            {"name": "since", "neo4j_type": {"type": "DATETIME"}}
        ],
        "ACTED_IN": [
            {"name": "role", "neo4j_type": {"type": "STRING"}}
        ]
    },
    "relationships": [
        {"start": "Person", "end": "Person", "rel_type": "KNOWS"},
        {"start": "Person", "end": "Movie", "rel_type": "ACTED_IN"}
    ],
    "metadata": {
        "indexes": [],
        "constraints": []
    }
}
'''

# Valid Cypher queries (10 match statements)
valid_queries = [
    "MATCH (a:Person)-[r:KNOWS]->(b:Person) RETURN a.name, r.since",
    "MATCH (a:Person)-[r:ACTED_IN]->(m:Movie) RETURN a.name, m.title, r.role",
    "MATCH (a:Person) WHERE a.age > 30 RETURN a.name",
    "MATCH (a:Person)-[r:KNOWS]->(b:Person) WHERE a.name = 'Alice' RETURN b.name",
    "MATCH (a:Person)-[r:ACTED_IN]->(m:Movie) WHERE m.year > 2000 RETURN a.name, m.title",
    "MATCH (a:Person)-[r:KNOWS]->(b:Person) RETURN a.name, b.name",
    "MATCH (a:Person)-[r:ACTED_IN]->(m:Movie) RETURN a.name, m.title",
    "MATCH (a:Person) RETURN a.name",
    "MATCH (a:Person)-[r:KNOWS]->(b:Person) RETURN a.name, r.since, b.name",
    "MATCH (a:Person)-[r:ACTED_IN]->(m:Movie) RETURN a.name, m.title, r.role"
]

# Valid QPPs (10 QPPs)
valid_qpps = [
    "MATCH ((a:Person)-[r:KNOWS]->(b:Person)){1,3} RETURN a.name, b.name",
    "MATCH ((a:Person)-[r:KNOWS]->(b:Person)){2,4} RETURN a.name, b.name",
    "MATCH ((a:Person)-[r:KNOWS]->(b:Person)){3,5} RETURN a.name, b.name",
    "MATCH ((a:Person)-[r:KNOWS]->(b:Person)){4,6} RETURN a.name, b.name",
    "MATCH ((a:Person)-[r:KNOWS]->(b:Person)){5,7} RETURN a.name, b.name",
    "MATCH ((a:Person)-[r:KNOWS]->(b:Person)){6,8} RETURN a.name, b.name",
    "MATCH ((a:Person)-[r:KNOWS]->(b:Person)){7,9} RETURN a.name, b.name",
    "MATCH ((a:Person)-[r:KNOWS]->(b:Person)){8,10} RETURN a.name, b.name",
    "MATCH ((a:Person)-[r:KNOWS]->(b:Person)){9,11} RETURN a.name, b.name",
    "MATCH ((a:Person)-[r:KNOWS]->(b:Person)){10,12} RETURN a.name, b.name"
]

# Invalid Cypher queries (10 invalid queries)
invalid_queries = [
    "MATCH (a:Person)-[r:KNOWS]->(b:Person) RETURN a.age",  # 'age' is not a valid property
    "MATCH (a:Person)-[r:KNOWS]->(b:Person) RETURN a.height",  # 'height' is not a valid property
    "MATCH (a:Person)-[r:KNOWS]->(b:Person) RETURN a.name, r.invalid_property",  # 'invalid_property' is not a valid property
    "MATCH (a:Person)-[r:KNOWS]->(b:Person) RETURN a.name, r.since, b.invalid_property",  # 'invalid_property' is not a valid property
    "MATCH (a:Person)-[r:KNOWS]->(b:Person) RETURN a.name, r.since, b.age",  # 'age' is not a valid property
    "MATCH (a:Person)-[r:KNOWS]->(b:Person) RETURN a.name, r.since, b.height",  # 'height' is not a valid property
    "MATCH (a:Person)-[r:KNOWS]->(b:Person) RETURN a.name, r.since, b.invalid_property",  # 'invalid_property' is not a valid property
    "MATCH (a:Person)-[r:KNOWS]->(b:Person) RETURN a.name, r.since, b.age",  # 'age' is not a valid property
    "MATCH (a:Person)-[r:KNOWS]->(b:Person) RETURN a.name, r.since, b.height",  # 'height' is not a valid property
    "MATCH (a:Person)-[r:KNOWS]->(b:Person) RETURN a.name, r.since, b.invalid_property"  # 'invalid_property' is not a valid property
]

def test_valid_queries():
    print("Testing valid queries:")
    print("Schema JSON:", schema_json)  # Debug: Print schema JSON
    for query in valid_queries:
        print(f"\nQuery: {query}")
        try:
            errors = get_validation_errors_py(query, schema_json)  # Debug: Get validation errors
            print(f"Validation errors: {errors}")  # Debug: Print validation errors
            is_valid = validate_cypher_py(query, schema_json)
            print(f"Valid: {is_valid}")
            assert is_valid, f"Query should be valid: {query}"
        except Exception as e:
            print(f"Error during validation: {str(e)}")  # Debug: Print any exceptions
            raise
        print()

def test_valid_qpps():
    print("Testing valid QPPs:")
    for query in valid_qpps:
        is_valid = validate_cypher_py(query, schema_json)
        print(f"Query: {query}")
        print(f"Valid: {is_valid}")
        assert is_valid, f"Query should be valid: {query}"
        print()

def test_invalid_queries():
    print("Testing invalid queries:")
    for query in invalid_queries:
        errors = get_validation_errors_py(query, schema_json)
        print(f"Query: {query}")
        print(f"Errors: {errors}")
        assert len(errors) > 0, f"Query should have validation errors: {query}"
        print()

if __name__ == "__main__":
    test_valid_queries()
    test_valid_qpps()
    test_invalid_queries() 