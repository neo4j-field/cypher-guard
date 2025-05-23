from cypher_guard import DbSchema, DbSchemaProperty, PropertyType
import pytest

def test_PropertyType_init_valid():
    prop = PropertyType.STRING
    assert prop is not None
    assert prop == PropertyType.STRING

def test_PropertyType_repr():
    prop = PropertyType.STRING
    assert prop.__repr__() == "PropertyType.STRING"

def test_PropertyType_str():
    prop = PropertyType.float()
    assert str(prop) == "FLOAT"

def test_PropertyType_integer():
    prop = PropertyType.integer()
    assert prop == PropertyType.INTEGER

def test_PropertyType_datetime():
    prop = PropertyType.datetime()
    assert prop == PropertyType.DATETIME

def test_PropertyType_float():
    prop = PropertyType.float()
    assert prop == PropertyType.FLOAT

def test_DbSchemaProperty_init_from_args_valid():
    prop = DbSchemaProperty("name", "STRING", ["value1", "value2"], None, None, None, None)
    assert prop is not None
    assert prop.name == "name"
    assert prop.neo4j_type == PropertyType.STRING
    assert prop.enum_values == ["value1", "value2"]
    assert prop.min_value is None
    assert prop.max_value is None

def test_DbSchemaProperty_init_from_args_invalid_arg_type():
    with pytest.raises(TypeError):
        DbSchemaProperty("name", 10, ["value1", "value2"], None, None, None, None)


def test_DbSchemaProperty_init_from_dict_valid():
    prop = DbSchemaProperty.from_dict({"name": "name", "neo4j_type": "STRING", "enum_values": ["value1", "value2"], "min_value": None, "max_value": None, "distinct_value_count": None, "example_values": None})
    assert prop is not None
    assert prop.name == "name"
    assert prop.neo4j_type == PropertyType.STRING
    assert prop.enum_values == ["value1", "value2"]
    assert prop.min_value is None
    assert prop.max_value is None

def test_DbSchemaProperty_init_from_dict_valid_undeclared_keys():
    prop = DbSchemaProperty.from_dict({"name": "name", "neo4j_type": "STRING", "enum_values": ["value1", "value2"]})
    assert prop is not None
    assert prop.name == "name"
    assert prop.neo4j_type == PropertyType.STRING
    assert prop.enum_values == ["value1", "value2"]
    assert prop.min_value is None
    assert prop.max_value is None

def test_DbSchemaProperty_init_from_dict_invalid_neo4j_type():
    with pytest.raises(ValueError):
        DbSchemaProperty.from_dict({"name": "name", "neo4j_type": "bigint", "enum_values": ["value1", "value2"], "min_value": None, "max_value": None, "distinct_value_count": None, "example_values": None})

# def test_schema_init():
#     schema = DbSchema(
#         node_props={"nodeA": {"name": "STRING", "age": "INTEGER"}, "nodeB": {"title": "STRING"}},
#         rel_props={"relA": { "num": "INTEGER"}},
#         relationships=[{"start": "nodeA", "end": "nodeB", "rel": "relA"}],
#         metadata={"constraints": [], "indexes": []},
#     )
#     assert schema is not None
#     assert len(schema.node_props) == 2
#     assert len(schema.rel_props) == 1
#     assert len(schema.relationships) == 1
#     assert schema.node_props["nodeA"] == {"name": "STRING", "age": "INTEGER"}
#     assert schema.node_props["nodeB"] == {"title": "STRING"}
#     assert schema.rel_props["relA"] == {"num": "INTEGER"}
#     assert schema.relationships[0] == {"start": "nodeA", "end": "nodeB", "rel": "relA"}

