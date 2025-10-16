import pytest
import cypher_guard
from cypher_guard import DbSchema


@pytest.fixture
def simple_schema():
    """Simple schema fixture for error handling tests"""
    return DbSchema.from_dict({
        "node_props": {
            "Person": [
                {"name": "name", "neo4j_type": "STRING"},
                {"name": "age", "neo4j_type": "INTEGER"}
            ]
        },
        "rel_props": {
            "KNOWS": [
                {"name": "since", "neo4j_type": "STRING"}
            ]
        },
        "relationships": [
            {"start": "Person", "end": "Person", "rel_type": "KNOWS"}
        ],
        "metadata": {
            "index": [],
            "constraint": []
        }
    })

def test_validate_cypher_success(simple_schema):
    """Test successful cypher validation with valid query"""
    result = cypher_guard.validate_cypher("MATCH (p:Person) RETURN p.name", simple_schema)
    assert result is not None


def test_validate_cypher_invalid_property(simple_schema):
    """Test cypher validation with invalid property returns errors"""
    errors = cypher_guard.validate_cypher("MATCH (p:Person) RETURN p.invalid_prop", simple_schema)
    assert len(errors) > 0
    assert any("invalid_prop" in error for error in errors) 