from neo4j_graphrag.schema import get_structured_schema
from neo4j import Driver

from cypher_guard import DbSchema, validate_cypher

def test_load_DbSchema_from_neo4j_graphrag_package(init_data: None, neo4j_driver: Driver):
    schema = get_structured_schema(neo4j_driver, is_enhanced=True)
    assert schema is not None

    print(schema)

    db_schema = DbSchema.from_dict(schema)

    print()
    print(db_schema)

    # Check Person properties in node_props
    assert "Person" in db_schema.node_props
    person_properties = db_schema.node_props["Person"]
    assert len(person_properties) == 2
    assert person_properties[0].name == "name"
    assert len(db_schema.metadata.index) == 1
    assert len(db_schema.metadata.constraint) == 1
    assert len(db_schema.relationships) == 1

def test_validate_cypher_with_schema_from_neo4j_graphrag_package(init_data: None, neo4j_driver: Driver):
    schema = get_structured_schema(neo4j_driver, is_enhanced=True)
    db_schema = DbSchema.from_dict(schema)

    query = "MATCH (p:Person) RETURN p.name"
    result = validate_cypher(query, db_schema)
    assert len(result) == 0
    
    
def test_validate_cypher_errors_with_schema_from_neo4j_graphrag_package(init_data: None, neo4j_driver: Driver):
    schema = get_structured_schema(neo4j_driver, is_enhanced=True)
    db_schema = DbSchema.from_dict(schema)

    query = "MATCH (p:Person) RETURN p.wrong"
    result = validate_cypher(query, db_schema)
    assert len(result) == 1


    
    

