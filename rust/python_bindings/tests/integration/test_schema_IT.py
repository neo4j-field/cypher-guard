from neo4j_graphrag.schema import get_structured_schema
from neo4j import Driver

from cypher_guard import DbSchema
def test_load_DbSchema_from_neo4j_graphrag_package(init_data: None, neo4j_driver: Driver):
    schema = get_structured_schema(neo4j_driver, is_enhanced=True)
    assert schema is not None

    print(schema)

    db_schema = DbSchema.from_dict(schema)

    print()
    print(db_schema)

    assert len(db_schema.node_props["Person"]) == 2
    assert db_schema.node_props["Person"][0].name == "name"
    assert len(db_schema.metadata.index) == 1
    assert len(db_schema.metadata.constraint) == 1
    assert len(db_schema.relationships) == 1
    
    
    
    

