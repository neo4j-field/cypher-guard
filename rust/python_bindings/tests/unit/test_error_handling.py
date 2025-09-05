import pytest
import cypher_guard


@pytest.fixture
def simple_schema():
    """Simple schema fixture for error handling tests"""
    return '''
    {
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
    }
    '''

@pytest.mark.skip(reason="parse_query is not implemented yet")
def test_parse_query_success():
    """Test successful query parsing"""
    result = cypher_guard.parse_query("MATCH (n) RETURN n")
    assert result is not None


@pytest.mark.skip(reason="parse_query is not implemented yet")
def test_parse_query_failure():
    """Test query parsing with invalid syntax raises appropriate exception"""
    with pytest.raises(Exception) as exc_info:
        cypher_guard.parse_query("INVALID QUERY SYNTAX")
    
    error_msg = str(exc_info.value)
    assert any(keyword in error_msg for keyword in ["Expected", "Invalid", "Unexpected", "Nom parsing error"])


def test_validate_cypher_success(simple_schema):
    """Test successful cypher validation with valid query"""
    result = cypher_guard.validate_cypher("MATCH (p:Person) RETURN p.name", simple_schema)
    assert result is not None


def test_validate_cypher_invalid_property(simple_schema):
    """Test cypher validation with invalid property raises exception"""
    with pytest.raises(Exception) as exc_info:
        cypher_guard.validate_cypher("MATCH (p:Person) RETURN p.invalid_prop", simple_schema)
    
    assert exc_info.value is not None 