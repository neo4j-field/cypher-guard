from cypher_guard import validate_cypher_py, get_validation_errors_py

# Example schema JSON
schema_json = '''
{
    "node_props": {
        "Person": [
            {"name": "name", "neo4j_type": {"type": "STRING"}},
            {"name": "age", "neo4j_type": {"type": "INTEGER"}},
            {"name": "created", "neo4j_type": {"type": "BOOLEAN"}}
        ],
        "Movie": [
            {"name": "title", "neo4j_type": {"type": "STRING"}},
            {"name": "year", "neo4j_type": {"type": "INTEGER"}}
        ],
        "Station": [
            {"name": "name", "neo4j_type": {"type": "STRING"}},
            {"name": "location", "neo4j_type": {"type": "POINT"}}
        ],
        "Stop": [
            {"name": "departs", "neo4j_type": {"type": "STRING"}},
            {"name": "arrives", "neo4j_type": {"type": "STRING"}}
        ]
    },
    "rel_props": {
        "KNOWS": [
            {"name": "since", "neo4j_type": {"type": "DATETIME"}}
        ],
        "ACTED_IN": [
            {"name": "role", "neo4j_type": {"type": "STRING"}}
        ],
        "CALLS_AT": [],
        "NEXT": [],
        "LINK": [
            {"name": "distance", "neo4j_type": {"type": "FLOAT"}}
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
        "indexes": [],
        "constraints": []
    }
}
'''

# Valid Cypher queries
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
    "MATCH (a:Person)-[r:ACTED_IN]->(m:Movie) RETURN a.name, m.title, r.role",
    "MATCH (a:Person) WHERE a.age > 30 AND a.name = 'Alice' RETURN a.name",
    "MATCH (a:Station)-[r:LINK]->(b:Station) WHERE point.distance(a.location, b.location) > 10 RETURN a.name",
    "MATCH (a:Person)-[r:KNOWS]->(b:Person) WHERE r.since > 2020 RETURN a.name"
]

# Valid QPPs
valid_qpps = [
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
]

# Invalid Cypher queries
invalid_queries = [
    # Invalid properties
    "MATCH (a:Person)-[r:KNOWS]->(b:Person) RETURN a.height",  # 'height' is not a valid property
    "MATCH (a:Person)-[r:KNOWS]->(b:Person) RETURN a.name, r.invalid_property",  # 'invalid_property' is not a valid property
    
    # Invalid relationship types
    "MATCH (a:Person)-[r:FOLLOWS]->(b:Person) RETURN a.name",  # 'FOLLOWS' is not a valid relationship type
    "MATCH (a:Station)-[r:CONNECTS]->(b:Station) RETURN a.name",  # 'CONNECTS' is not a valid relationship type
    
    # Invalid node labels
    "MATCH (a:User) RETURN a.name",  # 'User' is not a valid label
    "MATCH (a:Train) RETURN a.name",  # 'Train' is not a valid label
    
    # Invalid property types
    "MATCH (a:Person) WHERE a.age = '30' RETURN a.name",  # 'age' should be INTEGER, not STRING
    "MATCH (a:Person) WHERE a.name = 123 RETURN a.name",  # 'name' should be STRING, not INTEGER
    
    # Invalid relationship directions
    "MATCH (a:Person)<-[r:KNOWS]-(b:Person) RETURN a.name",  # KNOWS is defined as Person->Person, not Person<-Person
    "MATCH (a:Station)<-[r:LINK]-(b:Station) RETURN a.name",  # LINK is defined as Station->Station, not Station<-Station
    
    # Invalid property access on relationships
    "MATCH (a:Person)-[r:KNOWS]->(b:Person) WHERE r.role = 'friend' RETURN a.name",  # KNOWS doesn't have a 'role' property
    "MATCH (a:Station)-[r:LINK]->(b:Station) WHERE r.duration = 10 RETURN a.name"  # LINK doesn't have a 'duration' property
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

def test_basic_validation():
    query = "MATCH (p:Person) RETURN p.name"
    result = validate_cypher_py(query, schema_json)
    assert result

def test_relationship_pattern():
    query = "MATCH (a:Person)-[r:KNOWS {since: 2020}]->(b:Person) RETURN a.name, r.since"
    result = validate_cypher_py(query, schema_json)
    assert result

def test_quantified_path_pattern():
    query = """
    MATCH (a:Station { name: 'Denmark Hill' })<-[:CALLS_AT]-(d:Stop)
    ((:Stop)-[:NEXT]->(:Stop)){1,3}
    (a:Stop)-[:CALLS_AT]->(:Station { name: 'Clapham Junction' })
    RETURN d.departs AS departureTime, a.arrives AS arrivalTime
    """
    result = validate_cypher_py(query, schema_json)
    assert result

def test_merge_clause():
    query = "MERGE (a:Person {name: 'Alice'}) ON CREATE SET a.created = true"
    result = validate_cypher_py(query, schema_json)
    assert result

def test_path_variable_with_predicate():
    query = """
    MATCH (bfr:Station {name: 'London Blackfriars'}),
          (ndl:Station {name: 'North Dulwich'})
    MATCH p = (bfr)
    ((a)-[:LINK]-(b:Station)
    WHERE point.distance(a.location, ndl.location) >
    point.distance(b.location, ndl.location))+ (ndl)
    RETURN reduce(acc = 0, r in relationships(p) | round(acc + r.distance, 2))
    AS distance
    """
    result = validate_cypher_py(query, schema_json)
    assert result

if __name__ == "__main__":
    test_valid_queries()
    test_valid_qpps()
    test_invalid_queries() 