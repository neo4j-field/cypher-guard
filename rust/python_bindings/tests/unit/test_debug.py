#!/usr/bin/env python3

import sys
import os
sys.path.insert(0, os.path.join(os.path.dirname(__file__), 'src'))

from cypher_guard import validate_cypher

# Schema from the test
schema_json = """
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
"""

def test_simple_qpp():
    """Test a simple QPP pattern without complex functions"""
    query = "MATCH ((a)-[:LINK]-(b:Station))+ RETURN a.name"
    print(f"Testing simple QPP: {query}")
    try:
        result = validate_cypher(query, schema_json)
        print(f"Result: {result}")
    except Exception as e:
        print(f"Error: {e}")
        import traceback
        traceback.print_exc()

def test_qpp_with_where():
    """Test QPP with WHERE clause but no complex functions"""
    query = "MATCH ((a)-[:LINK]-(b:Station) WHERE a.name = 'test')+ RETURN a.name"
    print(f"Testing QPP with WHERE: {query}")
    try:
        result = validate_cypher(query, schema_json)
        print(f"Result: {result}")
    except Exception as e:
        print(f"Error: {e}")
        import traceback
        traceback.print_exc()

def test_simple_pattern():
    """Test a simple pattern without QPP"""
    query = "MATCH (a:Station)-[:LINK]-(b:Station) RETURN a.name"
    print(f"Testing simple pattern: {query}")
    try:
        result = validate_cypher(query, schema_json)
        print(f"Result: {result}")
    except Exception as e:
        print(f"Error: {e}")
        import traceback
        traceback.print_exc()

if __name__ == "__main__":
    print("Testing simplified patterns...")
    test_simple_pattern()
    print("\n" + "="*50 + "\n")
    test_simple_qpp()
    print("\n" + "="*50 + "\n")
    test_qpp_with_where() 