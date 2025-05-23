from cypher_guard import validate_cypher_py, get_validation_errors_py
import pytest

@pytest.fixture(scope="session")
def schema_json():
    return '''
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

def get_valid_cypher_queries():
    return [
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
        "MATCH ((a:Station)-[r:LINK]->(b:Station)){1,3} WHERE point.distance(a.location, b.location) > 10 RETURN a.name"
    ]

# Valid QPPs
@pytest.fixture(scope="session")
def valid_qpp_cypher_queries():
    return get_valid_qpp_cypher_queries()

@pytest.mark.parametrize("query", [
    "MATCH (a:Person)-[r:KNOWS]->(b:Person) RETURN a.height",  # 'height' is not a valid property
    "MATCH (a:Person)-[r:KNOWS]->(b:Person) RETURN a.name, r.invalid_property",  # 'invalid_property' is not a valid property
])
def test_cypher_query_invalid_property(query: str, schema_json: str):
    errors = get_validation_errors_py(query, schema_json)
    assert len(errors) > 0

@pytest.mark.parametrize("query", [
   "MATCH (a:Person)-[r:FOLLOWS]->(b:Person) RETURN a.name",  # 'FOLLOWS' is not a valid relationship type
    "MATCH (a:Station)-[r:CONNECTS]->(b:Station) RETURN a.name",  # 'CONNECTS' is not a valid relationship type
])
def test_cypher_query_invalid_relationship_type(query: str, schema_json: str):
    errors = get_validation_errors_py(query, schema_json)
    assert len(errors) > 0

@pytest.mark.parametrize("query", [
   "MATCH (a:User) RETURN a.name",  # 'User' is not a valid label
    "MATCH (a:Train) RETURN a.name",  # 'Train' is not a valid label
])
def test_cypher_query_invalid_node_label(query: str, schema_json: str):
    errors = get_validation_errors_py(query, schema_json)
    assert len(errors) > 0

@pytest.mark.parametrize("query", [
   "MATCH (a:Person) WHERE a.age = '30' RETURN a.name",  # 'age' should be INTEGER, not STRING
    "MATCH (a:Person) WHERE a.name = 123 RETURN a.name",  # 'name' should be STRING, not INTEGER
])
def test_cypher_query_invalid_property_type(query: str, schema_json: str):
    errors = get_validation_errors_py(query, schema_json)
    assert len(errors) > 0

@pytest.mark.parametrize("query", [
   "MATCH (a:Person)<-[r:ACTED_IN]-(b:Movie) RETURN a.name",  # ACTED_IN is defined as Person->Movie, not Person<-Movie
    "MATCH (a:Stop)<-[r:CALLS_AT]-(b:Station) RETURN a.name",  # CALLS_AT is defined as Stop->Station, not Stop<-Station
])
def test_cypher_query_invalid_relationship_direction(query: str, schema_json: str):
    errors = get_validation_errors_py(query, schema_json)
    assert len(errors) > 0

@pytest.mark.parametrize("query", [
    "MATCH (a:Person)-[r:KNOWS]->(b:Person) WHERE r.role = 'friend' RETURN a.name",  # KNOWS doesn't have a 'role' property, but ACTED_IN does
    "MATCH (a:Station)-[r:LINK]->(b:Station) WHERE r.duration = 10 RETURN a.name"  # LINK doesn't have a 'duration' property, and doesn't exist on any rel or node
])
def test_cypher_query_invalid_relationship_property(query: str, schema_json: str):
    errors = get_validation_errors_py(query, schema_json)
    assert len(errors) > 0

@pytest.mark.parametrize("query", get_valid_cypher_queries())
def test_valid_queries(query: str, schema_json: str):
    assert  validate_cypher_py(query, schema_json)
       
@pytest.mark.parametrize("query", get_valid_qpp_cypher_queries())
def test_valid_qpps(query: str, schema_json: str):
    assert validate_cypher_py(query, schema_json)

def test_basic_validation_valid(schema_json: str):
    query = "MATCH (p:Person) RETURN p.name"
    assert validate_cypher_py(query, schema_json)

def test_relationship_pattern_valid(schema_json: str):
    query = "MATCH (a:Person)-[r:KNOWS {since: 2020}]->(b:Person) RETURN a.name, r.since"
    assert validate_cypher_py(query, schema_json)

def test_quantified_path_pattern_valid(schema_json: str):
    query = """
    MATCH (a:Station { name: 'Denmark Hill' })<-[:CALLS_AT]-(d:Stop)
    ((:Stop)-[:NEXT]->(:Stop)){1,3}
    (a:Stop)-[:CALLS_AT]->(:Station { name: 'Clapham Junction' })
    RETURN d.departs AS departureTime, a.arrives AS arrivalTime
    """
    assert validate_cypher_py(query, schema_json)

def test_merge_clause_valid(schema_json: str):
    query = "MERGE (a:Person {name: 'Alice'}) ON CREATE SET a.created = true"
    assert validate_cypher_py(query, schema_json)

def test_path_variable_with_predicate_valid(schema_json: str):
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
    assert validate_cypher_py(query, schema_json)



    