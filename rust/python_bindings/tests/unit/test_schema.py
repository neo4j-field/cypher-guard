from cypher_guard import DbSchema, DbSchemaProperty, PropertyType, DbSchemaRelationshipPattern, DbSchemaConstraint, DbSchemaIndex, DbSchemaMetadata
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

def test_DbSchemaProperty_to_dict_valid():
    prop = DbSchemaProperty("name", "STRING", ["value1", "value2"], None, None, None, None)
    assert prop.to_dict() == {"name": "name", "neo4j_type": "STRING", "enum_values": ["value1", "value2"]}

def test_DbSchemaProperty_repr_with_enum_values():
    prop = DbSchemaProperty("name", "STRING", ["value1", "value2"], None, None, None, None)
    assert prop.__repr__() == "DbSchemaProperty(name=name, neo4j_type=STRING, enum_values=['value1', 'value2'], min_value=None, max_value=None, distinct_value_count=None, example_values=None)"

def test_DbSchemaProperty_repr_without_enum_values():
    prop = DbSchemaProperty("name", "STRING", None, None, None, None, None)
    assert prop.__repr__() == "DbSchemaProperty(name=name, neo4j_type=STRING, enum_values=None, min_value=None, max_value=None, distinct_value_count=None, example_values=None)"


def test_DbSchemaProperty_repr_with_min_max_distinct_value():
    prop = DbSchemaProperty("name", "STRING", None, 1.2, 10, 2, None)
    assert prop.__repr__() == "DbSchemaProperty(name=name, neo4j_type=STRING, enum_values=None, min_value=1.2, max_value=10, distinct_value_count=2, example_values=None)"

def test_DbSchemaProperty_str():
    prop = DbSchemaProperty("name", "STRING", ["value1", "value2"], None, None, None, None)
    assert str(prop) == "name: STRING"

def test_DbSchemaRelationshipPattern_init_from_args_valid():
    rel = DbSchemaRelationshipPattern("nodeA", "nodeB", "REL_A")
    assert rel is not None
    assert rel.start == "nodeA"
    assert rel.end == "nodeB"
    assert rel.rel_type == "REL_A"

def test_DbSchemaRelationshipPattern_init_from_args_invalid_arg_type():
    with pytest.raises(TypeError):
        DbSchemaRelationshipPattern("nodeA", "nodeB", 10)

def test_DbSchemaRelationshipPattern_init_from_dict_valid():
    rel = DbSchemaRelationshipPattern.from_dict({"start": "nodeA", "end": "nodeB", "rel_type": "REL_A"})
    assert rel is not None
    assert rel.start == "nodeA"
    assert rel.end == "nodeB"
    assert rel.rel_type == "REL_A"

def test_DbSchemaRelationshipPattern_init_from_dict_invalid_arg_type():
    with pytest.raises(TypeError):
        DbSchemaRelationshipPattern.from_dict({"start": "nodeA", "end": "nodeB", "rel_type": 10})

def test_DbSchemaRelationshipPattern_init_from_dict_invalid_keys():
    with pytest.raises(KeyError):
        DbSchemaRelationshipPattern.from_dict({"start": "nodeA", "end": "nodeB"})

def test_DbSchemaRelationshipPattern_repr():
    rel = DbSchemaRelationshipPattern("nodeA", "nodeB", "REL_A")
    assert rel.__repr__() == "DbSchemaRelationshipPattern(start=nodeA, end=nodeB, rel_type=REL_A)"

def test_DbSchemaRelationshipPattern_str():
    rel = DbSchemaRelationshipPattern("nodeA", "nodeB", "REL_A")
    assert str(rel) == "(:nodeA)-[:REL_A]->(:nodeB)"

def test_DbSchemaRelationshipPattern_to_dict_valid():
    rel = DbSchemaRelationshipPattern("nodeA", "nodeB", "REL_A")
    assert rel.to_dict() == {"start": "nodeA", "end": "nodeB", "rel_type": "REL_A"}


def test_DbSchemaConstraint_init_from_args_valid():
    constraint = DbSchemaConstraint(1, "CONSTRAINT_NAME", "UNIQUE", "NODE", ["label1", "label2"], ["prop1", "prop2"], "INDEX_NAME")
    assert constraint is not None
    assert constraint.id == 1
    assert constraint.name == "CONSTRAINT_NAME"

def test_DbSchemaConstraint_init_from_args_invalid_arg_type():
    with pytest.raises(TypeError):
        DbSchemaConstraint(1, "CONSTRAINT_NAME", "UNIQUE", "NODE", ["label1", "label2"], ["prop1", "prop2"], 10)

def test_DbSchemaConstraint_init_from_dict_valid():
    constraint = DbSchemaConstraint.from_dict({"id": 1, "name": "CONSTRAINT_NAME", "constraint_type": "UNIQUE", "entity_type": "NODE", "labels_or_types": ["label1"], "properties": ["prop1", "prop2"], "owned_index": "INDEX_NAME"})
    assert constraint is not None
    assert constraint.id == 1
    assert constraint.name == "CONSTRAINT_NAME" 
    assert constraint.constraint_type == "UNIQUE"
    assert constraint.entity_type == "NODE"
    assert constraint.labels_or_types == ["label1"]
    assert constraint.properties == ["prop1", "prop2"]
    assert constraint.owned_index == "INDEX_NAME"

def test_DbSchemaConstraint_init_from_dict_invalid_arg_type():
    with pytest.raises(TypeError):
        DbSchemaConstraint.from_dict({"id": 1, "name": "CONSTRAINT_NAME", "constraint_type": "UNIQUE", "entity_type": "NODE", "labels_or_types": ["label1"], "properties": ["prop1", "prop2"], "owned_index": 10})

def test_DbSchemaConstraint_repr():
    constraint = DbSchemaConstraint(1, "CONSTRAINT_NAME", "UNIQUE", "NODE", ["label1", "label2"], ["prop1", "prop2"], "INDEX_NAME")
    assert constraint.__repr__() == "DbSchemaConstraint(id=1, name=CONSTRAINT_NAME, constraint_type=UNIQUE, entity_type=NODE, labels_or_types=[label1, label2], properties=[prop1, prop2], owned_index=INDEX_NAME)"

def test_DbSchemaConstraint_str():
    constraint = DbSchemaConstraint(1, "CONSTRAINT_NAME", "UNIQUE", "NODE", ["label1", "label2"], ["prop1", "prop2"], "INDEX_NAME")
    assert str(constraint) == "UNIQUE CONSTRAINT CONSTRAINT_NAME ON NODE (label1, label2).{prop1, prop2}"   

def test_DbSchemaConstraint_to_dict_valid():
    constraint = DbSchemaConstraint(1, "CONSTRAINT_NAME", "UNIQUE", "NODE", ["label1", "label2"], ["prop1", "prop2"], "INDEX_NAME")
    assert constraint.to_dict() == {"id": 1, "name": "CONSTRAINT_NAME", "constraint_type": "UNIQUE", "entity_type": "NODE", "labels_or_types": ["label1", "label2"], "properties": ["prop1", "prop2"], "owned_index": "INDEX_NAME"}

def test_DbSchemaIndex_init_from_args_valid():
    index = DbSchemaIndex("INDEX_NAME", ["prop1", "prop2"], 10, "BTREE", 0.5, 1000)
    assert index is not None
    assert index.label == "INDEX_NAME"
    assert index.properties == ["prop1", "prop2"]
    assert index.size == 10 
    assert index.index_type == "BTREE"
    assert index.values_selectivity == 0.5
    assert index.distinct_values == 1000

def test_DbSchemaIndex_init_from_args_invalid_arg_type():
    with pytest.raises(TypeError):
        DbSchemaIndex("INDEX_NAME", ["prop1", "prop2"], 10, "BTREE", 0.5, "1000")

def test_DbSchemaIndex_init_from_dict_valid():
    index = DbSchemaIndex.from_dict({"label": "INDEX_NAME", "properties": ["prop1", "prop2"], "size": 10, "index_type": "BTREE", "values_selectivity": 0.5, "distinct_values": 1000})
    assert index is not None
    assert index.label == "INDEX_NAME"
    assert index.properties == ["prop1", "prop2"]
    assert index.size == 10 
    assert index.index_type == "BTREE"
    assert index.values_selectivity == 0.5
    assert index.distinct_values == 1000

def test_DbSchemaIndex_init_from_dict_invalid_arg_type():
    with pytest.raises(TypeError):
        DbSchemaIndex.from_dict({"label": "INDEX_NAME", "properties": ["prop1", "prop2"], "size": 10, "index_type": "BTREE", "values_selectivity": 0.5, "distinct_values": "1000"}) 

def test_DbSchemaIndex_repr():
    index = DbSchemaIndex("INDEX_NAME", ["prop1", "prop2"], 10, "BTREE", 0.5, 1000)
    assert index.__repr__() == "DbSchemaIndex(label=INDEX_NAME, properties=[prop1, prop2], size=10, index_type=BTREE, values_selectivity=0.5, distinct_values=1000)"

def test_DbSchemaIndex_str():
    index = DbSchemaIndex("INDEX_NAME", ["prop1", "prop2"], 10, "BTREE", 0.5, 1000)
    assert str(index) == "INDEX BTREE ON INDEX_NAME (prop1, prop2)"

def test_DbSchemaIndex_to_dict_valid():
    index = DbSchemaIndex("INDEX_NAME", ["prop1", "prop2"], 10, "BTREE", 0.5, 1000)
    assert index.to_dict() == {"label": "INDEX_NAME", "properties": ["prop1", "prop2"], "size": 10, "index_type": "BTREE", "values_selectivity": 0.5, "distinct_values": 1000}

def test_DbSchemaMetadata_init_from_args_valid():
    constraint = DbSchemaConstraint(1, "CONSTRAINT_NAME", "UNIQUE", "NODE", ["label1", "label2"], ["prop1", "prop2"], "INDEX_NAME")
    index = DbSchemaIndex("INDEX_NAME", ["prop1", "prop2"], 10, "BTREE", 0.5, 1000)
    metadata = DbSchemaMetadata([constraint], [index])
    assert metadata is not None
    assert len(metadata.constraints) == 1
    assert len(metadata.indexes) == 1
    assert metadata.constraints[0].id == constraint.id
    assert metadata.indexes[0].label == index.label

def test_DbSchemaMetadata_init_from_dict_valid():
    constraint = {"id": 1, "name": "CONSTRAINT_NAME", "constraint_type": "UNIQUE", "entity_type": "NODE", "labels_or_types": ["label1"], "properties": ["prop1", "prop2"], "owned_index": "INDEX_NAME"}
    index = {"label": "INDEX_NAME", "properties": ["prop1", "prop2"], "size": 10, "index_type": "BTREE", "values_selectivity": 0.5, "distinct_values": 1000}
    metadata = DbSchemaMetadata.from_dict({"constraints": [constraint], "indexes": [index]})
    assert metadata is not None
    assert len(metadata.constraints) == 1
    assert len(metadata.indexes) == 1
    assert metadata.constraints[0].id == constraint["id"]
    assert metadata.indexes[0].label == index["label"]

def test_DbSchemaMetadata_to_dict_valid():
    constraint = DbSchemaConstraint(1, "CONSTRAINT_NAME", "UNIQUE", "NODE", ["label1", "label2"], ["prop1", "prop2"], "INDEX_NAME")
    index = DbSchemaIndex("INDEX_NAME", ["prop1", "prop2"], 10, "BTREE", 0.5, 1000)
    metadata = DbSchemaMetadata([constraint], [index])
    assert metadata.to_dict() == {"constraints": [constraint.to_dict()], "indexes": [index.to_dict()]}

def test_DbSchemaMetadata_repr():
    constraint = DbSchemaConstraint(1, "CONSTRAINT_NAME", "UNIQUE", "NODE", ["label1", "label2"], ["prop1", "prop2"], "INDEX_NAME")
    index = DbSchemaIndex("INDEX_NAME", ["prop1", "prop2"], 10, "BTREE", 0.5, 1000)
    metadata = DbSchemaMetadata([constraint], [index])
    assert metadata.__repr__() == "DbSchemaMetadata(constraints=[DbSchemaConstraint(id=1, name=CONSTRAINT_NAME, constraint_type=UNIQUE, entity_type=NODE, labels_or_types=[label1, label2], properties=[prop1, prop2], owned_index=INDEX_NAME)], indexes=[DbSchemaIndex(label=INDEX_NAME, properties=[prop1, prop2], size=10, index_type=BTREE, values_selectivity=0.5, distinct_values=1000)])"    

def test_DbSchemaMetadata_str():
    constraint = DbSchemaConstraint(1, "CONSTRAINT_NAME", "UNIQUE", "NODE", ["label1", "label2"], ["prop1", "prop2"], "INDEX_NAME")
    index = DbSchemaIndex("INDEX_NAME", ["prop1", "prop2"], 10, "BTREE", 0.5, 1000)
    metadata = DbSchemaMetadata([constraint], [index])
    assert str(metadata) == "DbSchemaMetadata(constraints=[UNIQUE CONSTRAINT CONSTRAINT_NAME ON NODE (label1, label2).{prop1, prop2}], indexes=[INDEX BTREE ON INDEX_NAME (prop1, prop2)])"


def test_DbSchema_init_from_args_valid():
    node_a_props = [DbSchemaProperty("name", "STRING", ["value1", "value2"], None, None, None, None), DbSchemaProperty("age", "INTEGER")]
    node_b_props = [DbSchemaProperty("title", "STRING", ["value1", "value2"], None, None, None, None)]
    rel_a_props = [DbSchemaProperty("num", "INTEGER", None, None, None, None, None)]
    rel_a_pattern = DbSchemaRelationshipPattern("nodeA", "nodeB", "relA")
    constraint = DbSchemaConstraint(1, "CONSTRAINT_NAME", "UNIQUE", "NODE", ["label1", "label2"], ["prop1", "prop2"], "INDEX_NAME")
    index = DbSchemaIndex("INDEX_NAME", ["prop1", "prop2"], 10, "BTREE", 0.5, 1000)
    metadata = DbSchemaMetadata([constraint], [index])

    schema = DbSchema(
        node_props={"nodeA": node_a_props, "nodeB": node_b_props},
        rel_props={"relA": rel_a_props},
        relationships=[rel_a_pattern],
        metadata=metadata,
    )
    assert schema is not None
    assert len(schema.node_props) == 2
    assert len(schema.node_props["nodeA"]) == 2
    assert len(schema.node_props["nodeB"]) == 1
    assert len(schema.rel_props) == 1
    assert len(schema.rel_props["relA"]) == 1
    assert len(schema.relationships) == 1
    assert schema.node_props["nodeA"][0].name == "name"
    assert schema.node_props["nodeB"][0].name == "title"
    assert schema.rel_props["relA"][0].name == "num"
    assert schema.relationships[0].start == "nodeA"
    assert schema.metadata.constraints[0].name == "CONSTRAINT_NAME"
    assert schema.metadata.indexes[0].label == "INDEX_NAME"


def test_DbSchema_init_from_dict_valid():
    schema = DbSchema.from_dict({
        "node_props": {"nodeA": [{"name": "name", "neo4j_type": "STRING", "enum_values": ["value1", "value2"], "min_value": None, "max_value": None, "distinct_value_count": None, "example_values": None}, {"name": "age", "neo4j_type": "INTEGER"}], 
                       "nodeB": [{"name": "title", "neo4j_type": "STRING", "enum_values": ["value1", "value2"], "min_value": None, "max_value": None, "distinct_value_count": None, "example_values": None}]},
        "rel_props": {"relA": [{"name": "num", "neo4j_type": "INTEGER"}]},
        "relationships": [{"start": "nodeA", "end": "nodeB", "rel_type": "relA"}],
        "metadata": {"constraints": [{"id": 1, "name": "CONSTRAINT_NAME", "constraint_type": "UNIQUE", "entity_type": "NODE", "labels_or_types": ["label1", "label2"], "properties": ["prop1", "prop2"], "owned_index": "INDEX_NAME"}], "indexes": [{"label": "INDEX_NAME", "properties": ["prop1", "prop2"], "size": 10, "index_type": "BTREE", "values_selectivity": 0.5, "distinct_values": 1000}]}
    })
    assert schema is not None
    assert len(schema.node_props) == 2
    assert len(schema.node_props["nodeA"]) == 2
    assert len(schema.rel_props) == 1
    assert len(schema.rel_props["relA"]) == 1
    assert len(schema.relationships) == 1
    assert schema.node_props["nodeA"][0].name == "name"
    assert schema.node_props["nodeA"][1].name == "age"
    assert schema.rel_props["relA"][0].name == "num"
    assert schema.relationships[0].start == "nodeA"
    assert schema.metadata.constraints[0].name == "CONSTRAINT_NAME"
    assert schema.metadata.indexes[0].label == "INDEX_NAME"

def test_DbSchema_to_dict_valid():

    d = {
        "node_props": {"nodeA": [{"name": "name", "neo4j_type": "STRING", "enum_values": ["value1", "value2"]}, {"name": "age", "neo4j_type": "INTEGER"}]},
        "rel_props": {"relA": [{"name": "num", "neo4j_type": "INTEGER"}]},
        "relationships": [{"start": "nodeA", "end": "nodeB", "rel_type": "relA"}],
        "metadata": {"constraints": [{"id": 1, "name": "CONSTRAINT_NAME", "constraint_type": "UNIQUE", "entity_type": "NODE", "labels_or_types": ["label1", "label2"], "properties": ["prop1", "prop2"], "owned_index": "INDEX_NAME"}], "indexes": [{"label": "INDEX_NAME", "properties": ["prop1", "prop2"], "size": 10, "index_type": "BTREE", "values_selectivity": 0.5, "distinct_values": 1000}]}
    }
    schema = DbSchema.from_dict(d)
    assert schema.to_dict() == d

def test_DbSchema_str():
    schema = DbSchema.from_dict({
        "node_props": {"nodeA": [{"name": "name", "neo4j_type": "STRING", "enum_values": ["value1", "value2"]}, {"name": "age", "neo4j_type": "INTEGER"}],
                       "nodeB": [{"name": "title", "neo4j_type": "STRING", "enum_values": ["value1", "value2"]}]},
        "rel_props": {"relA": [{"name": "num", "neo4j_type": "INTEGER"}]},
        "relationships": [{"start": "nodeA", "end": "nodeB", "rel_type": "relA"}],
        "metadata": {"constraints": [{"id": 1, "name": "CONSTRAINT_NAME", "constraint_type": "UNIQUE", "entity_type": "NODE", "labels_or_types": ["label1", "label2"], "properties": ["prop1", "prop2"], "owned_index": "INDEX_NAME"}], "indexes": [{"label": "INDEX_NAME", "properties": ["prop1", "prop2"], "size": 10, "index_type": "BTREE", "values_selectivity": 0.5, "distinct_values": 1000}]}
    })
    assert "Nodes:" in str(schema)
    assert "nodeA:\nname: STRING\nage: INTEGER" in str(schema)
    assert "nodeB:\ntitle: STRING" in str(schema)
    assert "Relationship Properties:" in str(schema)
    assert "relA:\nnum: INTEGER" in str(schema)
    assert "Relationships:" in str(schema)
    assert "(:nodeA)-[:relA]->(:nodeB)" in str(schema)
    assert "Constraints:" in str(schema)
    assert "UNIQUE CONSTRAINT CONSTRAINT_NAME ON NODE (label1, label2).{prop1, prop2}" in str(schema)
    assert "Indexes:" in str(schema)
    assert "INDEX BTREE ON INDEX_NAME (prop1, prop2)" in str(schema)

def test_DbSchema_repr():
    schema = DbSchema.from_dict({
        "node_props": {"nodeA": [{"name": "name", "neo4j_type": "STRING", "enum_values": ["value1", "value2"]}, {"name": "age", "neo4j_type": "INTEGER"}],
                       "nodeB": [{"name": "title", "neo4j_type": "STRING", "enum_values": ["value1", "value2"]}]},
        "rel_props": {"relA": [{"name": "num", "neo4j_type": "INTEGER"}]},
        "relationships": [{"start": "nodeA", "end": "nodeB", "rel_type": "relA"}],
        "metadata": {"constraints": [{"id": 1, "name": "CONSTRAINT_NAME", "constraint_type": "UNIQUE", "entity_type": "NODE", "labels_or_types": ["label1", "label2"], "properties": ["prop1", "prop2"], "owned_index": "INDEX_NAME"}], "indexes": [{"label": "INDEX_NAME", "properties": ["prop1", "prop2"], "size": 10, "index_type": "BTREE", "values_selectivity": 0.5, "distinct_values": 1000}]}
    })
    assert "DbSchema(node_props={" in repr(schema)
    assert "{'nodeA': DbSchemaProperty(name=name, neo4j_type=STRING, enum_values=['value1', 'value2'], min_value=None, max_value=None, distinct_value_count=None, example_values=None), DbSchemaProperty(name=age, neo4j_type=INTEGER, enum_values=None, min_value=None, max_value=None, distinct_value_count=None, example_values=None)" in repr(schema)
    assert "'nodeB': DbSchemaProperty(name=title, neo4j_type=STRING, enum_values=['value1', 'value2'], min_value=None, max_value=None, distinct_value_count=None, example_values=None)" in repr(schema)
    assert "relationships=[DbSchemaRelationshipPattern(start=nodeA, end=nodeB, rel_type=relA)]," in repr(schema)
    assert "metadata=DbSchemaMetadata(constraints=[DbSchemaConstraint(id=1, name=CONSTRAINT_NAME, constraint_type=UNIQUE, entity_type=NODE, labels_or_types=[label1, label2], properties=[prop1, prop2], owned_index=INDEX_NAME)], indexes=[DbSchemaIndex(label=INDEX_NAME, properties=[prop1, prop2], size=10, index_type=BTREE, values_selectivity=0.5, distinct_values=1000)])" in repr(schema)
