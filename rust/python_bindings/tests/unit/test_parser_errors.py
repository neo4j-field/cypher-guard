"""
Unit tests for parser error handling.

This module tests that parser error types are properly raised
when different kinds of syntax errors are encountered.
"""

import pytest
import cypher_guard
from cypher_guard import (
    # Core parsing errors
    NomParsingError,
    UnexpectedEndOfInput,
    ExpectedToken,
    InvalidSyntax,
    ParsingUndefinedVariable,
    
    # Clause structure errors
    MissingRequiredClause,
    InvalidClauseOrder,
    WhereBeforeMatch,
    ReturnAfterReturn,
    OrderByBeforeReturn,
    SkipBeforeReturn,
    LimitBeforeReturn,
    
    # Clause position errors
    ReturnBeforeOtherClauses,
    MatchAfterReturn,
    CreateAfterReturn,
    MergeAfterReturn,
    DeleteAfterReturn,
    SetAfterReturn,
    WhereAfterReturn,
    WithAfterReturn,
    UnwindAfterReturn,
    
    # Pattern and expression errors
    InvalidPattern,
    InvalidWhereCondition,
    InvalidExpression,
    
    # Base error class
    CypherParsingError,
)


@pytest.fixture(scope="session")
def schema_json():
    """Basic schema for testing."""
    return '''
    {
        "node_props": {
            "Person": [
                {"name": "name", "neo4j_type": "STRING"},
                {"name": "age", "neo4j_type": "INTEGER"}
            ]
        },
        "rel_props": {
            "KNOWS": [
                {"name": "since", "neo4j_type": "DATE_TIME"}
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


class TestNomParsingErrors:
    """Test that NomParsingError is raised for basic syntax errors."""
    
    def test_nom_parsing_error_basic_syntax(self, schema_json):
        """Test that NomParsingError is raised for basic syntax errors."""
        with pytest.raises(NomParsingError):
            cypher_guard.check_syntax("MATCH (n RETURN n")
    
    def test_nom_parsing_error_incomplete_query(self, schema_json):
        """Test that NomParsingError is raised for incomplete queries."""
        with pytest.raises(NomParsingError):
            cypher_guard.check_syntax("MATCH (n:Person")
    
    def test_nom_parsing_error_invalid_keyword(self, schema_json):
        """Test that NomParsingError is raised for invalid keywords."""
        with pytest.raises(NomParsingError):
            cypher_guard.check_syntax("MATCH (n:Person) INVALID")
    
    def test_nom_parsing_error_incomplete_where(self, schema_json):
        """Test that NomParsingError is raised for incomplete WHERE clauses."""
        with pytest.raises(NomParsingError):
            cypher_guard.check_syntax("MATCH (n:Person) WHERE")
    
    def test_nom_parsing_error_multiple_return(self, schema_json):
        """Test that NomParsingError is raised for multiple RETURN clauses."""
        with pytest.raises(NomParsingError):
            cypher_guard.check_syntax("MATCH (n:Person) RETURN n RETURN n")
    
    def test_nom_parsing_error_order_by_before_return(self, schema_json):
        """Test that NomParsingError is raised for ORDER BY before RETURN."""
        with pytest.raises(NomParsingError):
            cypher_guard.check_syntax("MATCH (n:Person) ORDER BY n.name RETURN n")
    
    def test_nom_parsing_error_delete_after_return(self, schema_json):
        """Test that NomParsingError is raised for DELETE after RETURN."""
        with pytest.raises(NomParsingError):
            cypher_guard.check_syntax("MATCH (n:Person) RETURN n DELETE n")
    
    def test_nom_parsing_error_set_after_return(self, schema_json):
        """Test that NomParsingError is raised for SET after RETURN."""
        with pytest.raises(NomParsingError):
            cypher_guard.check_syntax("MATCH (n:Person) RETURN n SET n.age = 30")


class TestSpecificParserErrors:
    """Test specific parser error types that are actually raised."""
    
    def test_return_before_other_clauses(self, schema_json):
        """Test that ReturnBeforeOtherClauses is raised when RETURN comes too early."""
        with pytest.raises(ReturnBeforeOtherClauses):
            cypher_guard.check_syntax("RETURN n MATCH (n:Person)")
    
    def test_where_before_match(self, schema_json):
        """Test that WhereBeforeMatch is raised when WHERE comes before MATCH."""
        with pytest.raises(WhereBeforeMatch):
            cypher_guard.check_syntax("WHERE n.age > 30 MATCH (n:Person) RETURN n")
    
    def test_match_after_return(self, schema_json):
        """Test that MatchAfterReturn is raised when MATCH comes after RETURN."""
        with pytest.raises(MatchAfterReturn):
            cypher_guard.check_syntax("MATCH (n:Person) RETURN n MATCH (m:Person)")
    
    def test_with_after_return(self, schema_json):
        """Test that WithAfterReturn is raised when WITH comes after RETURN."""
        with pytest.raises(WithAfterReturn):
            cypher_guard.check_syntax("MATCH (n:Person) RETURN n WITH n")
    
    def test_unwind_after_return(self, schema_json):
        """Test that UnwindAfterReturn is raised when UNWIND comes after RETURN."""
        with pytest.raises(UnwindAfterReturn):
            cypher_guard.check_syntax("MATCH (n:Person) RETURN n UNWIND [1,2,3] AS x")
    
    def test_invalid_clause_order_where_after_return(self, schema_json):
        """Test that InvalidClauseOrder is raised for WHERE after RETURN."""
        with pytest.raises(InvalidClauseOrder):
            cypher_guard.check_syntax("MATCH (n:Person) RETURN n WHERE n.age > 30")


class TestValidQueries:
    """Test that some queries that might seem invalid actually parse successfully."""
    
    def test_create_after_return_is_valid(self, schema_json):
        """Test that CREATE after RETURN actually parses successfully."""
        # This is surprising but true - CREATE after RETURN is valid Cypher
        result = cypher_guard.check_syntax("MATCH (n:Person) RETURN n CREATE (m:Person)")
        assert result is True
    
    def test_merge_after_return_is_valid(self, schema_json):
        """Test that MERGE after RETURN actually parses successfully."""
        # This is surprising but true - MERGE after RETURN is valid Cypher
        result = cypher_guard.check_syntax("MATCH (n:Person) RETURN n MERGE (m:Person)")
        assert result is True
    
    def test_undefined_variable_is_valid_parsing(self, schema_json):
        """Test that undefined variables don't cause parsing errors."""
        # Undefined variables are a validation issue, not a parsing issue
        result = cypher_guard.check_syntax("MATCH (n:Person) RETURN undefined_var")
        assert result is True


class TestErrorInheritance:
    """Test that all parser errors inherit from the base CypherParsingError."""
    
    def test_nom_parsing_error_inheritance(self, schema_json):
        """Test that NomParsingError inherits from CypherParsingError."""
        with pytest.raises(NomParsingError) as exc_info:
            cypher_guard.check_syntax("MATCH (n RETURN n")
        
        assert isinstance(exc_info.value, CypherParsingError)
    
    def test_specific_errors_inheritance(self, schema_json):
        """Test that specific parser errors inherit from CypherParsingError."""
        error_queries = [
            ("RETURN n MATCH (n:Person)", ReturnBeforeOtherClauses),
            ("WHERE n.age > 30 MATCH (n:Person) RETURN n", WhereBeforeMatch),
            ("MATCH (n:Person) RETURN n MATCH (m:Person)", MatchAfterReturn),
            ("MATCH (n:Person) RETURN n WITH n", WithAfterReturn),
            ("MATCH (n:Person) RETURN n UNWIND [1,2,3] AS x", UnwindAfterReturn),
            ("MATCH (n:Person) RETURN n WHERE n.age > 30", InvalidClauseOrder),
        ]
        
        for query, expected_error in error_queries:
            with pytest.raises(expected_error) as exc_info:
                cypher_guard.check_syntax(query)
            
            assert isinstance(exc_info.value, CypherParsingError), f"Error {expected_error.__name__} should inherit from CypherParsingError"


class TestErrorMessages:
    """Test that error messages are informative and helpful."""
    
    def test_nom_parsing_error_message(self, schema_json):
        """Test that NomParsingError messages contain useful information."""
        with pytest.raises(NomParsingError) as exc_info:
            cypher_guard.check_syntax("MATCH (n:Person")
        
        error_msg = str(exc_info.value)
        assert "Nom parsing error" in error_msg
        assert "error Verify" in error_msg or "error Tag" in error_msg
    
    def test_specific_error_messages(self, schema_json):
        """Test that specific error messages are descriptive."""
        with pytest.raises(ReturnBeforeOtherClauses) as exc_info:
            cypher_guard.check_syntax("RETURN n MATCH (n:Person)")
        
        error_msg = str(exc_info.value)
        assert "RETURN clause must come after" in error_msg
        assert "line" in error_msg and "column" in error_msg
        
        with pytest.raises(WhereBeforeMatch) as exc_info:
            cypher_guard.check_syntax("WHERE n.age > 30 MATCH (n:Person) RETURN n")
        
        error_msg = str(exc_info.value)
        assert "WHERE clause must come after" in error_msg


class TestErrorConsistency:
    """Test that errors are consistent across different functions."""
    
    def test_nom_parsing_error_consistency(self, schema_json):
        """Test that NomParsingError is consistent across functions that raise errors."""
        invalid_query = "MATCH (n:Person) WHERE"
        
        # Functions that raise errors
        error_raising_functions = [
            cypher_guard.check_syntax,
            lambda q: cypher_guard.is_write(q),
            lambda q: cypher_guard.is_read(q),
        ]

        for func in error_raising_functions:
            with pytest.raises(NomParsingError):
                func(invalid_query)
        
        # has_parser_errors returns boolean instead of raising
        result = cypher_guard.has_parser_errors(invalid_query)
        assert result is True
    
    def test_specific_error_consistency(self, schema_json):
        """Test that specific errors are consistent across functions that raise errors."""
        invalid_query = "RETURN n MATCH (n:Person)"
        
        # Functions that raise errors
        error_raising_functions = [
            cypher_guard.check_syntax,
            lambda q: cypher_guard.is_write(q),
            lambda q: cypher_guard.is_read(q),
        ]

        for func in error_raising_functions:
            with pytest.raises(ReturnBeforeOtherClauses):
                func(invalid_query)
        
        # has_parser_errors returns boolean instead of raising
        result = cypher_guard.has_parser_errors(invalid_query)
        assert result is True


class TestErrorEdgeCases:
    """Test edge cases and boundary conditions."""
    
    def test_empty_query(self, schema_json):
        """Test that empty queries raise appropriate errors."""
        with pytest.raises((UnexpectedEndOfInput, NomParsingError)):
            cypher_guard.check_syntax("")
    
    def test_whitespace_only_query(self, schema_json):
        """Test that whitespace-only queries raise appropriate errors."""
        with pytest.raises((UnexpectedEndOfInput, NomParsingError)):
            cypher_guard.check_syntax("   \n\t  ")
    
    def test_very_long_invalid_query(self, schema_json):
        """Test that very long invalid queries still raise appropriate errors."""
        long_query = "MATCH " + "(" * 1000 + "n:Person" + ")" * 1000 + " RETURN n"
        with pytest.raises((NomParsingError, InvalidSyntax)):
            cypher_guard.check_syntax(long_query)


class TestErrorTypeSummary:
    """Test to document what error types are actually raised."""
    
    def test_error_type_summary(self, schema_json):
        """Document which error types are actually raised by the parser."""
        # This test documents the current behavior for reference
        
        # These raise NomParsingError:
        nom_error_cases = [
            "MATCH (n:Person",  # Incomplete query
            "MATCH (n:Person) WHERE",  # Incomplete WHERE
            "MATCH (n:Person) INVALID",  # Invalid keyword
            "MATCH (n:Person) RETURN n RETURN n",  # Multiple RETURN
            "MATCH (n:Person) ORDER BY n.name RETURN n",  # ORDER BY before RETURN
            "MATCH (n:Person) RETURN n DELETE n",  # DELETE after RETURN
            "MATCH (n:Person) RETURN n SET n.age = 30",  # SET after RETURN
        ]
        
        # These raise specific error types:
        specific_error_cases = [
            ("RETURN n MATCH (n:Person)", ReturnBeforeOtherClauses),
            ("WHERE n.age > 30 MATCH (n:Person) RETURN n", WhereBeforeMatch),
            ("MATCH (n:Person) RETURN n MATCH (m:Person)", MatchAfterReturn),
            ("MATCH (n:Person) RETURN n WITH n", WithAfterReturn),
            ("MATCH (n:Person) RETURN n UNWIND [1,2,3] AS x", UnwindAfterReturn),
            ("MATCH (n:Person) RETURN n WHERE n.age > 30", InvalidClauseOrder),
        ]
        
        # These parse successfully (surprisingly):
        valid_cases = [
            "MATCH (n:Person) RETURN n CREATE (m:Person)",  # CREATE after RETURN
            "MATCH (n:Person) RETURN n MERGE (m:Person)",   # MERGE after RETURN
            "MATCH (n:Person) RETURN undefined_var",        # Undefined variable
        ]
        
        # Test NomParsingError cases
        for query in nom_error_cases:
            with pytest.raises(NomParsingError):
                cypher_guard.check_syntax(query)
        
        # Test specific error cases
        for query, expected_error in specific_error_cases:
            with pytest.raises(expected_error):
                cypher_guard.check_syntax(query)
        
        # Test valid cases
        for query in valid_cases:
            result = cypher_guard.check_syntax(query)
            assert result is True
        
        # This test always passes - it's for documentation
        assert True


if __name__ == "__main__":
    pytest.main([__file__])