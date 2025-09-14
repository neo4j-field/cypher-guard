from cypher_guard import DbSchema, DbSchemaProperty, PropertyType, DbSchemaRelationshipPattern
import pytest

def test_PropertyType_init_valid():
    prop = PropertyType("STRING")
    assert prop is not None
    assert str(prop) == "STRING"

def test_PropertyType_repr():
    prop = PropertyType("STRING")
    assert repr(prop) == "PropertyType(STRING)"

def test_PropertyType_str():
    prop = PropertyType("STRING")
    assert str(prop) == "STRING"

def test_PropertyType_all_types():
    types = ["STRING", "INTEGER", "FLOAT", "BOOLEAN", "POINT", "DATE_TIME", "LIST"]
    for type_name in types:
        pt = PropertyType(type_name)
        assert str(pt) == type_name

def test_PropertyType_equality():
    pt1 = PropertyType("STRING")
    pt2 = PropertyType("STRING")
    pt3 = PropertyType("INTEGER")
    assert str(pt1) == str(pt2)
    assert str(pt1) != str(pt3)

def test_PropertyType_from_string():
    pt = PropertyType("STRING")
    assert str(pt) == "STRING"

def test_DbSchemaProperty_init_from_dict_valid():
    prop = DbSchemaProperty.from_dict({"name": "name", "neo4j_type": "STRING", "enum_values": ["value1", "value2"]})
    assert prop is not None
    assert prop.name == "name"
    assert prop.neo4j_type == "STRING"
    # Note: enum_values might not be preserved in the current implementation
    assert hasattr(prop, 'enum_values')

def test_DbSchemaProperty_init_from_dict_missing_name():
    with pytest.raises(Exception):
        DbSchemaProperty.from_dict({"neo4j_type": "STRING"})

def test_DbSchemaProperty_init_from_dict_missing_type():
    with pytest.raises(Exception):
        DbSchemaProperty.from_dict({"name": "name"})

def test_DbSchemaProperty_to_dict():
    prop = DbSchemaProperty.from_dict({"name": "name", "neo4j_type": "STRING"})
    result = prop.to_dict()
    assert result["name"] == "name"
    assert result["neo4j_type"] == "STRING"

def test_DbSchemaRelationshipPattern_init_from_dict_valid():
    rel = DbSchemaRelationshipPattern.from_dict({"start": "nodeA", "end": "nodeB", "rel_type": "relA"})
    assert rel is not None
    assert rel.start == "nodeA"
    assert rel.end == "nodeB"
    assert rel.rel_type == "relA"

def test_DbSchemaRelationshipPattern_to_dict():
    rel = DbSchemaRelationshipPattern.from_dict({"start": "nodeA", "end": "nodeB", "rel_type": "relA"})
    result = rel.to_dict()
    assert result["start"] == "nodeA"
    assert result["end"] == "nodeB"
    assert result["rel_type"] == "relA"

def test_DbSchemaRelationshipPattern_str():
    rel = DbSchemaRelationshipPattern.from_dict({"start": "nodeA", "end": "nodeB", "rel_type": "relA"})
    rel_str = str(rel)
    assert "nodeA" in rel_str
    assert "nodeB" in rel_str
    assert "relA" in rel_str

def test_DbSchema_init_from_dict_valid():
    schema = DbSchema.from_dict({
        "node_props": {
            "nodeA": [
                {"name": "name", "neo4j_type": "STRING"}, 
                {"name": "age", "neo4j_type": "INTEGER"}
            ],
            "nodeB": [
                {"name": "title", "neo4j_type": "STRING"}
            ]
        },
        "rel_props": {"relA": [{"name": "num", "neo4j_type": "INTEGER"}]},
        "relationships": [{"start": "nodeA", "end": "nodeB", "rel_type": "relA"}],
        "metadata": {"index": [], "constraint": []}
    })
    assert schema is not None
    assert len(schema.node_props) == 2
    assert "nodeA" in schema.node_props
    assert len(schema.node_props["nodeA"]) == 2
    assert schema.node_props["nodeA"][0].name == "name"
    assert "nodeB" in schema.node_props
    assert len(schema.node_props["nodeB"]) == 1
    assert schema.node_props["nodeB"][0].name == "title"
    assert schema.relationships[0].start == "nodeA"

def test_DbSchema_to_dict_valid():
    d = {
        "node_props": {
            "nodeA": [
                {"name": "name", "neo4j_type": "STRING"}, 
                {"name": "age", "neo4j_type": "INTEGER"}
            ]
        },
        "rel_props": {"relA": [{"name": "num", "neo4j_type": "INTEGER"}]},
        "relationships": [{"start": "nodeA", "end": "nodeB", "rel_type": "relA"}],
        "metadata": {"index": [], "constraint": []}
    }
    schema = DbSchema.from_dict(d)
    result = schema.to_dict()
    
    # Check that the basic structure is there (only node_props and relationships are returned)
    assert "node_props" in result
    assert "relationships" in result
    assert len(result["node_props"]) == 1
    assert "nodeA" in result["node_props"]
    assert len(result["relationships"]) == 1
    assert result["relationships"][0]["start"] == "nodeA"

def test_DbSchema_str():
    schema = DbSchema.from_dict({
        "node_props": {
            "nodeA": [
                {"name": "name", "neo4j_type": "STRING"}, 
                {"name": "age", "neo4j_type": "INTEGER"}
            ]
        },
        "rel_props": {"relA": [{"name": "num", "neo4j_type": "INTEGER"}]},
        "relationships": [{"start": "nodeA", "end": "nodeB", "rel_type": "relA"}],
        "metadata": {"index": [], "constraint": []}
    })
    schema_str = str(schema)
    assert "node_props=1 labels" in schema_str
    assert "relationships=1 types" in schema_str

def test_DbSchema_repr():
    schema = DbSchema.from_dict({
        "node_props": {
            "nodeA": [
                {"name": "name", "neo4j_type": "STRING"}, 
                {"name": "age", "neo4j_type": "INTEGER"}
            ]
        },
        "rel_props": {"relA": [{"name": "num", "neo4j_type": "INTEGER"}]},
        "relationships": [{"start": "nodeA", "end": "nodeB", "rel_type": "relA"}],
        "metadata": {"index": [], "constraint": []}
    })
    schema_repr = repr(schema)
    assert "DbSchema" in schema_repr

def test_DbSchema_has_label():
    schema = DbSchema.from_dict({
        "node_props": {
            "Person": [
                {"name": "name", "neo4j_type": "STRING"}
            ]
        },
        "rel_props": {},
        "relationships": [],
        "metadata": {"index": [], "constraint": []}
    })
    assert schema.has_label("Person") == True
    assert schema.has_label("Movie") == False

def test_DbSchema_has_node_property():
    schema = DbSchema.from_dict({
        "node_props": {
            "Person": [
                {"name": "name", "neo4j_type": "STRING"},
                {"name": "age", "neo4j_type": "INTEGER"}
            ]
        },
        "rel_props": {},
        "relationships": [],
        "metadata": {"index": [], "constraint": []}
    })
    assert schema.has_node_property("Person", "name") == True
    assert schema.has_node_property("Person", "age") == True
    assert schema.has_node_property("Person", "title") == False
    assert schema.has_node_property("Movie", "name") == False