#!/usr/bin/env python3

import sys
import os

# Add the current directory to Python path to find the module
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

try:
    import cypher_guard
    print("✅ Successfully imported cypher_guard module")
except ImportError as e:
    print(f"❌ Failed to import cypher_guard: {e}")
    sys.exit(1)

def test_parse_query_success():
    """Test successful query parsing"""
    try:
        result = cypher_guard.parse_query_py("MATCH (n) RETURN n")
        print("✅ parse_query_py with valid query succeeded")
        return True
    except Exception as e:
        print(f"❌ parse_query_py with valid query failed: {e}")
        return False

def test_parse_query_failure():
    """Test query parsing with custom error handling"""
    try:
        result = cypher_guard.parse_query_py("INVALID QUERY SYNTAX")  # Completely invalid query
        print("❌ parse_query_py with invalid query should have failed")
        return False
    except Exception as e:
        error_type = type(e).__name__
        error_msg = str(e)
        print(f"✅ parse_query_py with invalid query correctly raised {error_type}: {error_msg}")
        
        # Check that we get a meaningful error message
        if "Expected" in error_msg or "Invalid" in error_msg or "Unexpected" in error_msg or "Nom parsing error" in error_msg:
            print("✅ Error message contains meaningful parsing information")
            return True
        else:
            print("❌ Error message doesn't contain expected parsing information")
            return False

def test_validation_functions():
    """Test validation functions with custom error handling"""
    # Simple schema for testing - using the correct format
    schema_json = '''
    {
        "node_props": {
            "Person": [
                {"name": "name", "neo4j_type": "STRING"},
                {"name": "age", "neo4j_type": "INTEGER"}
            ]
        },
        "rel_props": {
            "KNOWS": [
                {"name": "since", "neo4j_type": "STRING"}
            ]
        },
        "relationships": [
            {"start": "Person", "end": "Person", "rel_type": "KNOWS"}
        ],
        "metadata": {
            "index": [],
            "constraint": []
        }
    }
    '''
    
    # Test valid query
    try:
        result = cypher_guard.validate_cypher("MATCH (p:Person) RETURN p.name", schema_json)
        print("✅ validate_cypher with valid query succeeded")
    except Exception as e:
        print(f"❌ validate_cypher with valid query failed: {e}")
        return False
    
    # Test invalid query (property doesn't exist)
    try:
        result = cypher_guard.validate_cypher("MATCH (p:Person) RETURN p.invalid_prop", schema_json)
        print("❌ validate_cypher with invalid query should have failed")
        return False
    except Exception as e:
        error_type = type(e).__name__
        error_msg = str(e)
        print(f"✅ validate_cypher with invalid query correctly raised {error_type}: {error_msg}")
        return True

if __name__ == "__main__":
    print("🧪 Testing Cypher Guard Python Bindings with Custom Error Handling")
    print("=" * 70)
    
    success_count = 0
    total_tests = 3
    
    if test_parse_query_success():
        success_count += 1
    
    if test_parse_query_failure():
        success_count += 1
    
    if test_validation_functions():
        success_count += 1
    
    print("=" * 70)
    print(f"📊 Test Results: {success_count}/{total_tests} tests passed")
    
    if success_count == total_tests:
        print("🎉 All tests passed! Custom error handling is working correctly.")
        sys.exit(0)
    else:
        print("❌ Some tests failed. Check the output above for details.")
        sys.exit(1) 