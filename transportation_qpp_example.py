# Cell 1: Install and Import
!pip install cypher-guard

from cypher_guard import (
    DbSchema, DbSchemaProperty, PropertyType, 
    DbSchemaRelationshipPattern, DbSchemaMetadata,
    validate_cypher, get_validation_errors
)
import json

print("Cypher Guard imported successfully!")

# Cell 2: Create Transportation Schema
print("=== Creating Transportation Network Schema ===")

# Create properties for Station nodes
station_props = [
    DbSchemaProperty("name", "STRING"),
    DbSchemaProperty("location", "STRING")
]

# Create properties for Stop nodes (train stops at stations)
stop_props = [
    DbSchemaProperty("arrives", "STRING"),
    DbSchemaProperty("departs", "STRING")
]

# Create properties for relationships
next_rel_props = [
    DbSchemaProperty("distance", "FLOAT")
]

link_rel_props = [
    DbSchemaProperty("distance", "FLOAT")
]

# Create relationships
relationships = [
    DbSchemaRelationshipPattern("Stop", "Station", "CALLS_AT"),
    DbSchemaRelationshipPattern("Stop", "Stop", "NEXT"),
    DbSchemaRelationshipPattern("Station", "Station", "LINK")
]

# Create metadata
metadata = DbSchemaMetadata(constraint=[], index=[])

# Create schema
schema = DbSchema(
    node_props={
        "Station": station_props,
        "Stop": stop_props
    },
    rel_props={
        "CALLS_AT": [],
        "NEXT": next_rel_props,
        "LINK": link_rel_props
    },
    relationships=relationships,
    metadata=metadata
)

print("Transportation schema created successfully!")
print(f"Node types: {list(schema.node_props.keys())}")
print(f"Relationship types: {[r.rel_type for r in schema.relationships]}")

# Convert schema to JSON string for validation
schema_dict = schema.to_dict()
schema_json = json.dumps(schema_dict)
print("Schema converted to JSON for validation")

# Cell 3: Test Basic QPP - Simple Variable Length Path
print("\n=== Test 1: Basic QPP - Simple Variable Length Path ===")

# Let's start with a simple QPP between stations
simple_qpp = """
MATCH (start:Station {name: 'London Blackfriars'})
      ((s:Station)-[:LINK]->(e:Station)){1,3}
      (end:Station {name: 'North Dulwich'})
RETURN start, end
"""

print("Testing simple QPP between stations...")
result = validate_cypher(simple_qpp, schema_json)
print(f"Result: {result}")

# Cell 4: Test QPP with Stops and NEXT Relationships
print("\n=== Test 2: QPP with Stops and NEXT Relationships ===")

# Test QPP with stops connected by NEXT relationships
stops_qpp = """
MATCH (:Station {name: 'Denmark Hill'})<-[:CALLS_AT]-(first:Stop)
      ((s:Stop)-[:NEXT]->(e:Stop)){1,3}
      (last:Stop)-[:CALLS_AT]->(:Station {name: 'Clapham Junction'})
RETURN first, last
"""

print("Testing QPP with stops and NEXT relationships...")
result = validate_cypher(stops_qpp, schema_json)
print(f"Result: {result}") 