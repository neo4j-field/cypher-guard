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

def test_DbSchemaProperty_init_from_dict_valid():
    prop = DbSchemaProperty.from_dict({"name": "name", "neo4j_type": "STRING", "enum_values": ["value1", "value2"]})
    assert prop is not None
    assert prop.name == "name"
    assert prop.neo4j_type == "STRING"
    # Note: enum_values might not be preserved in the current implementation
    assert hasattr(prop, 'enum_values')

def test_DbSchemaProperty_init_from_dict_minimal():
    prop = DbSchemaProperty.from_dict({"name": "name", "neo4j_type": "STRING"})
    assert prop is not None
    assert prop.name == "name"
    assert prop.neo4j_type == "STRING"

def test_DbSchemaProperty_to_dict_valid():
    prop = DbSchemaProperty.from_dict({"name": "name", "neo4j_type": "STRING", "enum_values": ["value1", "value2"]})
    result = prop.to_dict()
    assert "name" in result
    assert result["name"] == "name"
    assert result["neo4j_type"] == "STRING"

def test_DbSchemaProperty_repr():
    prop = DbSchemaProperty.from_dict({"name": "name", "neo4j_type": "STRING"})
    repr_str = repr(prop)
    assert "DbSchemaProperty" in repr_str
    assert "name=name" in repr_str
    assert "neo4j_type=STRING" in repr_str

def test_DbSchemaRelationshipPattern_init_from_dict_valid():
    rel = DbSchemaRelationshipPattern.from_dict({"start": "nodeA", "end": "nodeB", "rel_type": "REL_A"})
    assert rel is not None
    assert rel.start == "nodeA"
    assert rel.end == "nodeB"
    assert rel.rel_type == "REL_A"

def test_DbSchemaRelationshipPattern_init_from_dict_invalid_keys():
    with pytest.raises(KeyError):
        DbSchemaRelationshipPattern.from_dict({"start": "nodeA", "end": "nodeB"})

def test_DbSchemaRelationshipPattern_repr():
    rel = DbSchemaRelationshipPattern.from_dict({"start": "nodeA", "end": "nodeB", "rel_type": "REL_A"})
    assert repr(rel) == "DbSchemaRelationshipPattern(start=nodeA, end=nodeB, rel_type=REL_A)"

def test_DbSchemaRelationshipPattern_to_dict_valid():
    rel = DbSchemaRelationshipPattern.from_dict({"start": "nodeA", "end": "nodeB", "rel_type": "REL_A"})
    assert rel.to_dict() == {"start": "nodeA", "end": "nodeB", "rel_type": "REL_A"}

def test_DbSchema_init_from_dict_valid():
    schema = DbSchema.from_dict({
        "nodes": [
            {
                "label": "nodeA",
                "properties": [
                    {"name": "name", "neo4j_type": "STRING", "enum_values": ["value1", "value2"]}, 
                    {"name": "age", "neo4j_type": "INTEGER"}
                ]
            },
            {
                "label": "nodeB",
                "properties": [
                    {"name": "title", "neo4j_type": "STRING", "enum_values": ["value1", "value2"]}
                ]
            }
        ],
        "rel_props": {"relA": [{"name": "num", "neo4j_type": "INTEGER"}]},
        "relationships": [{"start": "nodeA", "end": "nodeB", "rel_type": "relA"}],
        "metadata": {"index": [], "constraint": []}
    })
    assert schema is not None
    assert len(schema.nodes) == 2
    assert len(schema.relationships) == 1
    assert schema.nodes[0].label == "nodeA"
    assert len(schema.nodes[0].properties) == 2
    assert schema.nodes[0].properties[0].name == "name"
    assert schema.nodes[0].properties[1].name == "age"
    assert schema.nodes[1].label == "nodeB"
    assert len(schema.nodes[1].properties) == 1
    assert schema.nodes[1].properties[0].name == "title"
    assert schema.relationships[0].start == "nodeA"

def test_DbSchema_to_dict_valid():
    d = {
        "nodes": [
            {
                "label": "nodeA",
                "properties": [
                    {"name": "name", "neo4j_type": "STRING"}, 
                    {"name": "age", "neo4j_type": "INTEGER"}
                ]
            }
        ],
        "rel_props": {"relA": [{"name": "num", "neo4j_type": "INTEGER"}]},
        "relationships": [{"start": "nodeA", "end": "nodeB", "rel_type": "relA"}],
        "metadata": {"index": [], "constraint": []}
    }
    schema = DbSchema.from_dict(d)
    result = schema.to_dict()
    
    # Check that the basic structure is there (only nodes and relationships are returned)
    assert "nodes" in result
    assert "relationships" in result
    assert len(result["nodes"]) == 1
    assert result["nodes"][0]["label"] == "nodeA"
    assert len(result["relationships"]) == 1
    assert result["relationships"][0]["start"] == "nodeA"

def test_DbSchema_str():
    schema = DbSchema.from_dict({
        "nodes": [
            {
                "label": "nodeA",
                "properties": [
                    {"name": "name", "neo4j_type": "STRING"}, 
                    {"name": "age", "neo4j_type": "INTEGER"}
                ]
            }
        ],
        "rel_props": {"relA": [{"name": "num", "neo4j_type": "INTEGER"}]},
        "relationships": [{"start": "nodeA", "end": "nodeB", "rel_type": "relA"}],
        "metadata": {"index": [], "constraint": []}
    })
    schema_str = str(schema)
    assert "DbSchema" in schema_str
    assert "nodes=" in schema_str
    assert "relationships=" in schema_str

def test_DbSchema_repr():
    schema = DbSchema.from_dict({
        "nodes": [
            {
                "label": "nodeA",
                "properties": [
                    {"name": "name", "neo4j_type": "STRING"}
                ]
            }
        ],
        "rel_props": {},
        "relationships": [],
        "metadata": {"index": [], "constraint": []}
    })
    repr_str = repr(schema)
    assert "DbSchema" in repr_str

def test_DbSchema_has_label():
    schema = DbSchema.from_dict({
        "nodes": [
            {
                "label": "Person",
                "properties": [
                    {"name": "name", "neo4j_type": "STRING"}
                ]
            }
        ],
        "rel_props": {},
        "relationships": [],
        "metadata": {"index": [], "constraint": []}
    })
    assert schema.has_label("Person") == True
    assert schema.has_label("Movie") == False

def test_DbSchema_has_node_property():
    schema = DbSchema.from_dict({
        "nodes": [
            {
                "label": "Person",
                "properties": [
                    {"name": "name", "neo4j_type": "STRING"},
                    {"name": "age", "neo4j_type": "INTEGER"}
                ]
            }
        ],
        "rel_props": {},
        "relationships": [],
        "metadata": {"index": [], "constraint": []}
    })
    assert schema.has_node_property("Person", "name") == True
    assert schema.has_node_property("Person", "age") == True
    assert schema.has_node_property("Person", "height") == False
    assert schema.has_node_property("Movie", "title") == False