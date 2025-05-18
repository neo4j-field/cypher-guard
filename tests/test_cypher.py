import pytest
from cypher_guard_python_bindings import validate_cypher_with_schema, DbSchema, DbSchemaProperty, PropertyType

def test_relationship_pattern():
    schema = DbSchema()
    # Add required schema elements
    schema.add_label("Person")
    schema.add_node_property("Person", DbSchemaProperty(
        name="name",
        neo4j_type=PropertyType.STRING
    ))
    schema.add_relationship_property("KNOWS", DbSchemaProperty(
        name="since",
        neo4j_type=PropertyType.INTEGER
    ))
    # Add relationship definition
    schema.add_relationship("Person", "Person", "KNOWS")
    
    query = "MATCH (a:Person)-[r:KNOWS {since: 2020}]->(b:Person) RETURN a.name, r.since"
    result = validate_cypher_with_schema(query, schema)
    assert result.is_ok()

def test_quantified_path_pattern():
    schema = DbSchema()
    # Add required schema elements
    schema.add_label("Station")
    schema.add_label("Stop")
    schema.add_node_property("Station", DbSchemaProperty(
        name="name",
        neo4j_type=PropertyType.STRING
    ))
    schema.add_node_property("Stop", DbSchemaProperty(
        name="departs",
        neo4j_type=PropertyType.STRING
    ))
    schema.add_node_property("Stop", DbSchemaProperty(
        name="arrives",
        neo4j_type=PropertyType.STRING
    ))
    # Add relationship definitions
    schema.add_relationship("Stop", "Station", "CALLS_AT")
    schema.add_relationship("Stop", "Stop", "NEXT")
    
    query = """
    MATCH (a:Station { name: 'Denmark Hill' })<-[:CALLS_AT]-(d:Stop)
    ((:Stop)-[:NEXT]->(:Stop)){1,3}
    (a:Stop)-[:CALLS_AT]->(:Station { name: 'Clapham Junction' })
    RETURN d.departs AS departureTime, a.arrives AS arrivalTime
    """
    result = validate_cypher_with_schema(query, schema)
    assert result.is_ok()

def test_merge_clause():
    schema = DbSchema()
    # Add required properties to schema
    schema.add_label("Person")
    schema.add_node_property("Person", DbSchemaProperty(
        name="name",
        neo4j_type=PropertyType.STRING
    ))
    schema.add_node_property("Person", DbSchemaProperty(
        name="created",
        neo4j_type=PropertyType.BOOLEAN
    ))
    
    query = "MERGE (a:Person {name: 'Alice'}) ON CREATE SET a.created = true"
    result = validate_cypher_with_schema(query, schema)
    assert result.is_ok()

def test_path_variable_with_predicate():
    schema = DbSchema()
    # Add required properties
    schema.add_label("Station")
    schema.add_node_property("Station", DbSchemaProperty(
        name="name",
        neo4j_type=PropertyType.STRING
    ))
    schema.add_node_property("Station", DbSchemaProperty(
        name="location",
        neo4j_type=PropertyType.POINT
    ))
    schema.add_relationship_property("LINK", DbSchemaProperty(
        name="distance",
        neo4j_type=PropertyType.FLOAT
    ))
    # Add relationship definition
    schema.add_relationship("Station", "Station", "LINK")
    
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
    result = validate_cypher_with_schema(query, schema)
    assert result.is_ok() 