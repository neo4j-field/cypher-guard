from cypher_guard import validate_cypher, InvalidNodeLabel, InvalidRelationshipType, InvalidNodeProperty, InvalidRelationshipProperty, InvalidPropertyAccess, DbSchema
import pytest

@pytest.fixture(scope="session")
def schema():
    return DbSchema.from_dict({
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
    })

def get_valid_cypher_queries():
    return [
        "MATCH (a:Person) WHERE a.age > 30 RETURN a.name",
        "MATCH (a:Person)-[r:KNOWS]->(b:Person) WHERE a.name = 'Alice' RETURN b.name",
        "MATCH (a:Person)-[r:ACTED_IN]->(m:Movie) WHERE m.year > 2000 RETURN a.name, m.title",
        "MATCH (a:Person)-[r:KNOWS]->(b:Person) RETURN a.name, b.name",
        "MATCH (a:Person)-[r:ACTED_IN]->(m:Movie) RETURN a.name, m.title",
        "MATCH (a:Person) RETURN a.name",
        "MATCH (a:Person)-[r:KNOWS]->(b:Person) RETURN a.name, r.since, b.name",
        "MATCH (a:Person)-[r:ACTED_IN]->(m:Movie) RETURN a.name, m.title, r.role",
        "MATCH (a:Person) WHERE a.age > 30 AND a.name = 'Alice' RETURN a.name",
        "MATCH (a:Station)-[r:LINK]->(b:Station) WHERE a.name = 'test' RETURN a.name"
    ]

# Valid Cypher queries  
@pytest.fixture(scope="session")
def valid_cypher_queries():
    return get_valid_cypher_queries()

def get_valid_qpp_cypher_queries():
    return [
        "MATCH ((a:Person)-[r:KNOWS]->(b:Person)){1,3} RETURN a.name, b.name",
        "MATCH ((a:Person)-[r:KNOWS]->(b:Person)){2,4} RETURN a.name, b.name",
        "MATCH ((a:Person)-[r:KNOWS]->(b:Person)){3,5} RETURN a.name, b.name",
        "MATCH ((a:Person)-[r:KNOWS]->(b:Person)){4,6} RETURN a.name, b.name",
        "MATCH ((a:Person)-[r:KNOWS]->(b:Person)){5,7} RETURN a.name, b.name",
        "MATCH ((a:Stop)-[r:NEXT]->(b:Stop)){1,3} RETURN a.departs, b.arrives",
        "MATCH ((a:Station)-[r:LINK]->(b:Station)){1,3} RETURN a.name, b.name",
        "MATCH ((a:Stop)-[r:CALLS_AT]->(b:Station)){1,3} RETURN a.departs, b.name",
        "MATCH ((a:Person)-[r:ACTED_IN]->(b:Movie)){1,3} RETURN a.name, b.title",
        "MATCH ((a:Station)-[r:LINK]->(b:Station)){1,3} WHERE a.name = 'test' RETURN a.name"
    ]

# Valid QPPs
@pytest.fixture(scope="session")
def valid_qpp_cypher_queries():
    return get_valid_qpp_cypher_queries()

@pytest.mark.parametrize("query", [
    "MATCH (a:Person)-[r:KNOWS]->(b:Person) RETURN a.height",  # 'height' is not a valid property
    "MATCH (a:Person)-[r:KNOWS]->(b:Person) RETURN a.name, r.invalid_property",  # 'invalid_property' is not a valid property
])
def test_cypher_query_invalid_property(query: str, schema: DbSchema):
    errors = validate_cypher(query, schema)
    assert len(errors) > 0

@pytest.mark.parametrize("query", [
   "MATCH (a:Person)-[r:FOLLOWS]->(b:Person) RETURN a.name",  # 'FOLLOWS' is not a valid relationship type
    "MATCH (a:Station)-[r:CONNECTS]->(b:Station) RETURN a.name",  # 'CONNECTS' is not a valid relationship type
])
def test_cypher_query_invalid_relationship_type(query: str, schema: DbSchema):
    errors = validate_cypher(query, schema)
    assert len(errors) > 0

@pytest.mark.parametrize("query", [
   "MATCH (a:User) RETURN a.name",  # 'User' is not a valid label
    "MATCH (a:Train) RETURN a.name",  # 'Train' is not a valid label
])
def test_cypher_query_invalid_node_label(query: str, schema: DbSchema):
    errors = validate_cypher(query, schema)
    assert len(errors) > 0

@pytest.mark.parametrize("query", [
   "MATCH (a:Person) WHERE a.age = '30' RETURN a.name",  # 'age' should be INTEGER, not STRING
    "MATCH (a:Person) WHERE a.name = 123 RETURN a.name",  # 'name' should be STRING, not INTEGER
])
def test_cypher_query_invalid_property_type(query: str, schema: DbSchema):
    errors = validate_cypher(query, schema)
    assert len(errors) > 0

@pytest.mark.parametrize("query", [
   "MATCH (a:Person)<-[r:ACTED_IN]-(b:Movie) RETURN a.name",  # ACTED_IN is defined as Person->Movie, not Person<-Movie
    "MATCH (a:Stop)<-[r:CALLS_AT]-(b:Station) RETURN a.name",  # CALLS_AT is defined as Stop->Station, not Stop<-Station
])
def test_cypher_query_invalid_relationship_direction(query: str, schema: DbSchema):
    errors = validate_cypher(query, schema)
    assert len(errors) > 0

@pytest.mark.parametrize("query", [
    "MATCH (a:Person)-[r:KNOWS]->(b:Person) WHERE r.role = 'friend' RETURN a.name",  # KNOWS doesn't have a 'role' property, but ACTED_IN does
    "MATCH (a:Station)-[r:LINK]->(b:Station) WHERE r.duration = 10 RETURN a.name"  # LINK doesn't have a 'duration' property, and doesn't exist on any rel or node
])
def test_cypher_query_invalid_relationship_property(query: str, schema: DbSchema):
    errors = validate_cypher(query, schema)
    assert len(errors) > 0

def test_complex_multiline_with_context_aware_validation(schema: DbSchema):
    """Test context-aware relationship property validation in complex multiline query with WITH clauses"""
    # This query should fail because r.role doesn't exist on KNOWS relationships (only on ACTED_IN)
    query = """
    MATCH (a:Person)-[r:KNOWS]->(b:Person)
    WHERE a.age > 30
    WITH a, r, b
    MATCH (b)-[r2:ACTED_IN]->(m:Movie)
    WHERE r.role = 'friend'
    AND r2.role = 'actor'
    RETURN a.name, b.name, m.title
    """
    
    errors = validate_cypher(query, schema)
    
    # Should have exactly 1 error: r.role is invalid for KNOWS relationship
    assert len(errors) == 1
    error_messages = [str(e) for e in errors]
    
    # Should complain about r.role being invalid (r is bound to KNOWS relationship)
    assert any("r.role" in msg or ("r" in msg and "role" in msg) for msg in error_messages)
    
def test_complex_multiline_valid_context_aware(schema: DbSchema):
    """Test that the same query structure works when using correct relationship properties"""
    # This query should pass - using r.since (valid for KNOWS) and r2.role (valid for ACTED_IN)
    query = """
    MATCH (a:Person)-[r:KNOWS]->(b:Person) 
    WHERE a.age > 30
    WITH a, r, b
    MATCH (b)-[r2:ACTED_IN]->(m:Movie)
    WHERE r.since IS NOT NULL
    AND r2.role = 'actor'
    RETURN a.name, b.name, m.title
    """
    
    # Should pass now that we use valid Cypher syntax (r.since IS NOT NULL)
    result = validate_cypher(query, schema)
    assert len(result) == 0  # Should pass with valid temporal property check

@pytest.mark.parametrize("query", get_valid_cypher_queries())
def test_valid_queries(query: str, schema: DbSchema):
    assert len(validate_cypher(query, schema)) == 0
       
@pytest.mark.parametrize("query", get_valid_qpp_cypher_queries())
def test_valid_qpps(query: str, schema: DbSchema):
    assert len(validate_cypher(query, schema)) == 0

def test_basic_validation_valid(schema: DbSchema):
    query = "MATCH (p:Person) RETURN p.name"
    assert len(validate_cypher(query, schema)) == 0

def test_relationship_pattern_valid(schema: DbSchema):
    query = "MATCH (a:Person)-[r:KNOWS {since: 2020}]->(b:Person) RETURN a.name, r.since"
    assert len(validate_cypher(query, schema)) == 0

def test_quantified_path_pattern_valid(schema: DbSchema):
    query = """
    MATCH ((a:Stop)-[:NEXT]->(b:Stop)){1,3}
    RETURN a.departs
    """
    assert len(validate_cypher(query, schema)) == 0

def test_merge_clause_valid(schema: DbSchema):
    query = "MERGE (a:Person {name: 'Alice'}) ON CREATE SET a.created = true"
    assert len(validate_cypher(query, schema)) == 0

@pytest.mark.skip(reason="Known issue: Rust validation bug with multiple MATCH clauses causing integer overflow")
def test_path_variable_with_predicate_valid(schema: DbSchema):
    query = """
    MATCH (bfr:Station),
          (ndl:Station)
    MATCH (bfr)-[:LINK]-(ndl)
    WHERE bfr.name = 'test'
    RETURN bfr.name
    """
    assert len(validate_cypher(query, schema)) == 0

def test_with_clause_valid(schema: DbSchema):
    query = "MATCH (a:Person) WITH a RETURN a.name"
    assert len(validate_cypher(query, schema)) == 0

def test_with_clause_alias_valid(schema: DbSchema):
    query = "MATCH (a:Person) WITH a AS b RETURN b.name"
    assert len(validate_cypher(query, schema)) == 0

def test_with_clause_wildcard_valid(schema: DbSchema):
    query = "MATCH (a:Person) WITH * RETURN a.name"
    assert len(validate_cypher(query, schema)) == 0

def test_with_clause_invalid_variable(schema: DbSchema):
    query = "MATCH (a:Person) WITH b RETURN b.name"
    errors = validate_cypher(query, schema)
    assert errors and any("Undefined variable" in e for e in errors)

def test_with_clause_invalid_alias_expression(schema: DbSchema):
    query = "MATCH (a:Person) WITH b AS c RETURN c.name"
    errors = validate_cypher(query, schema)
    assert errors and any("Undefined variable" in e for e in errors)

def test_invalid_node_label(schema):
    errors = validate_cypher("MATCH (a:User) RETURN a.name", schema)
    assert len(errors) > 0
    assert any("Invalid node label" in error for error in errors)

def test_invalid_relationship_type(schema):
    errors = validate_cypher("MATCH (a:Person)-[r:FOLLOWS]->(b:Person) RETURN a.name", schema)
    assert len(errors) > 0
    assert any("Invalid relationship type" in error for error in errors)

def test_invalid_node_property(schema):
    errors = validate_cypher("MATCH (a:Person) RETURN a.invalid_prop", schema)
    assert len(errors) > 0
    assert any("Invalid property access" in error for error in errors)

def test_invalid_relationship_property(schema):
    errors = validate_cypher("MATCH (a:Person)-[r:KNOWS]->(b:Person) RETURN r.invalid_prop", schema)
    assert len(errors) > 0
    assert any("Invalid property access" in error for error in errors)

def test_invalid_property_access(schema):
    errors = validate_cypher("MATCH (a:Person) RETURN a.height", schema)
    assert len(errors) > 0
    assert any("Invalid property access" in error for error in errors)

def test_direct_invalid_node_label():
    from cypher_guard import InvalidNodeLabel
    import pytest
    with pytest.raises(InvalidNodeLabel):
        raise InvalidNodeLabel("Direct test")



    