#!/usr/bin/env python3

import sys
import os
sys.path.insert(0, os.path.join(os.path.dirname(__file__), 'src'))

from cypher_guard import validate_cypher, get_validation_errors

# Schema from the test
schema_json = '''
    {
        "node_props": {
        "Person": [
            {"name": "name", "neo4j_type": "STRING"},
            {"name": "age", "neo4j_type": "INTEGER"},
            {"name": "created", "neo4j_type": "BOOLEAN"}
        ],
        "Movie": [
            {"name": "title", "neo4j_type": "STRING"},
            {"name": "year", "neo4j_type": "INTEGER"}
        ],
        "Station": [
            {"name": "name", "neo4j_type": "STRING"},
            {"name": "location", "neo4j_type": "POINT"}
        ],
        "Stop": [
            {"name": "departs", "neo4j_type": "STRING"},
            {"name": "arrives", "neo4j_type": "STRING"}
        ]
    },
    "rel_props": {
        "KNOWS": [
            {"name": "since", "neo4j_type": "DATE_TIME"}
        ],
        "ACTED_IN": [
            {"name": "role", "neo4j_type": "STRING"}
        ],
        "CALLS_AT": [],
        "NEXT": [],
        "LINK": [
            {"name": "distance", "neo4j_type": "FLOAT"}
        ]
    },
    "relationships": [
        {"start": "Person", "end": "Person", "rel_type": "KNOWS"},
        {"start": "Person", "end": "Movie", "rel_type": "ACTED_IN"},
        {"start": "Stop", "end": "Station", "rel_type": "CALLS_AT"},
        {"start": "Stop", "end": "Stop", "rel_type": "NEXT"},
        {"start": "Station", "end": "Station", "rel_type": "LINK"}
    ],
    "metadata": {
        "index": [],
        "constraint": []
    }
}
'''

def test_validation():
    print("Testing validation logic...")
    
    # Test the failing pattern
    query = """
    MATCH (a:Station)<-[:CALLS_AT]-(d:Stop)
    ((:Stop)-[:NEXT]->(:Stop)){1,3}
    (final_stop:Stop)-[:CALLS_AT]->(:Station)
    RETURN d.departs, final_stop.arrives
    """
    
    print(f"Testing query: {query}")
    
    try:
        result = validate_cypher(query, schema_json)
        print(f"✅ Validation result: {result}")
    except Exception as e:
        print(f"❌ Validation error: {e}")
        print(f"Error type: {type(e)}")
    
    # Test a simpler pattern to see if the issue is with QPPs
    simple_query = """
    MATCH (a:Station)<-[:CALLS_AT]-(d:Stop)
    (d:Stop)-[:NEXT]->(e:Stop)
    (e:Stop)-[:CALLS_AT]->(f:Station)
    RETURN d.departs, e.arrives
    """
    
    print(f"\nTesting simple query: {simple_query}")
    
    try:
        result = validate_cypher(simple_query, schema_json)
        print(f"✅ Simple validation result: {result}")
    except Exception as e:
        print(f"❌ Simple validation error: {e}")
        print(f"Error type: {type(e)}")

if __name__ == "__main__":
    test_validation() 