import pytest
from cypher_guard import validate_cypher, DbSchema


@pytest.fixture
def test_schema():
    """Test schema fixture for all debug tests"""
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


def test_simple_qpp(test_schema):
    """Test a simple QPP pattern without complex functions"""
    query = "MATCH ((a)-[:LINK]-(b:Station))+ RETURN a.name"
    result = validate_cypher(query, test_schema)
    assert result is not None


def test_qpp_with_where(test_schema):
    """Test QPP with WHERE clause but no complex functions"""
    query = "MATCH ((a)-[:LINK]-(b:Station) WHERE a.name = 'test')+ RETURN a.name"
    result = validate_cypher(query, test_schema)
    assert result is not None


def test_simple_pattern(test_schema):
    """Test a simple pattern without QPP"""
    query = "MATCH (a:Station)-[:LINK]-(b:Station) RETURN a.name"
    result = validate_cypher(query, test_schema)
    assert result is not None 