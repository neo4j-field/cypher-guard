# Cypher Guard

![Cypher Guard Banner](assets/cypher-guard-banner.png)

A Rust library for parsing and validating [Cypher](https://neo4j.com/developer/cypher/) queries with Python and JavaScript bindings. Provides robust, schema-aware validation for graph queries with high performance and safety.

---

## Quick Start

### Python

```bash
# Install from PyPI
pip install cypher-guard
```

```python
from cypher_guard import validate_cypher, DbSchema

# Create schema from Neo4j GraphRAG format
schema = DbSchema.from_dict({
    "node_props": {
        "Person": [{"property": "name", "type": "STRING"}]
    },
    "relationships": [
        {"start": "Person", "type": "KNOWS", "end": "Person"}
    ]
})

# Validate query
errors = validate_cypher("MATCH (p:Person) RETURN p.name", schema)
if not errors:
    print("Query is valid!")
```


### JavaScript/TypeScript

```bash
# Install from npm
npm install cypher-guard
```

```typescript
import { validateCypher, DbSchema } from "cypher-guard";

const schema = DbSchema.fromDict({
    node_props: {
        Person: [{ property: "name", type: "STRING" }]
    },
    relationships: [
        { start: "Person", type: "KNOWS", end: "Person" }
    ]
});

const errors = validateCypher("MATCH (p:Person) RETURN p.name", schema);
```

### Rust

```bash
# Add to Cargo.toml
cargo add cypher-guard
```

```rust
use cypher_guard::{validate_cypher_with_schema, DbSchema};

let schema = DbSchema::from_json_string(schema_json)?;
let errors = validate_cypher_with_schema(query, &schema)?;
```

---

## Core API

### Python Functions

- **`validate_cypher(query, schema)`** - Returns list of validation errors
- **`check_syntax(query)`** - Check syntax only (no schema needed)
- **`is_write(query)`** - Check if query modifies data
- **`has_parser_errors(query)`** - Check if query has syntax errors

### Schema Classes

- **`DbSchema`** - Main schema container
- **`DbSchemaProperty`** - Property definitions
- **`DbSchemaRelationshipPattern`** - Relationship patterns

### Error Handling

Specific exception types for different validation failures:
- `CypherValidationError` - Base validation error
- `InvalidNodeLabel`, `InvalidRelationshipType` - Schema errors
- `UndefinedVariable`, `TypeMismatch` - Query errors
- `NomParsingError` - Syntax errors

---

## Schema Format

Cypher Guard uses the Neo4j GraphRAG schema format:

```json
{
    "node_props": {
        "Person": [
            {"property": "name", "type": "STRING"},
            {"property": "age", "type": "INTEGER"}
        ]
    },
    "rel_props": {
        "KNOWS": [
            {"property": "since", "type": "DATE"}
        ]
    },
    "relationships": [
        {"start": "Person", "type": "KNOWS", "end": "Person"}
    ],
    "metadata": {
        "constraint": [],
        "index": []
    }
}
```

### Integration with Neo4j GraphRAG

```python
from neo4j_graphrag.schema import get_structured_schema
from cypher_guard import DbSchema

# Get schema from Neo4j
schema = get_structured_schema(driver, is_enhanced=True)

# Convert to DbSchema
db_schema = DbSchema.from_dict(schema)

# Use for validation
errors = validate_cypher(query, db_schema)
```

---

## Development

For local development, building from source, and contributing:

**[Development Guide](docs/DEVELOPMENT.md)** - Complete setup, build, and testing instructions

---

## Documentation

- **[API Reference](docs/API.md)** - Complete API documentation
- **[Schema Format](docs/SCHEMA.md)** - Schema definition guide
- **[Development Guide](docs/DEVELOPMENT.md)** - Local development and building from source
- **[Parser Internals](docs/PARSER_INTERNALS.md)** - How the parser works
- **[Releases](docs/RELEASES.md)** - Download latest releases
- **[Versioning](docs/VERSIONING.md)** - Release management

---

## Iterative Query Fixing with LLMs

Here's a practical example of how to iteratively fix Cypher queries using LangChain, Neo4j GraphRAG, and cypher-guard:

```python
import neo4j
from neo4j_graphrag.retrievers import Text2CypherRetriever
from neo4j_graphrag.llm.openai_llm import OpenAILLM
from neo4j_graphrag.schema import get_structured_schema
from neo4j_graphrag.exceptions import Text2CypherRetrievalError, SearchValidationError
from cypher_guard import check_syntax, validate_cypher, DbSchema
from langchain_core.messages import HumanMessage, AIMessage
from langchain_openai import ChatOpenAI

# Initialize Neo4j connection
driver = neo4j.GraphDatabase.driver("neo4j://localhost:7687", auth=("neo4j", "password"))

# Get schema from Neo4j GraphRAG
schema_dict = get_structured_schema(driver)
schema = DbSchema.from_dict(schema_dict)

# Initialize LLM and retriever
llm = OpenAILLM(model_name="gpt-4o")
text2cypher_retriever = Text2CypherRetriever(driver, llm, neo4j_schema=schema_dict)

# Initialize LangChain chat model for query fixing
chat_model = ChatOpenAI(model="gpt-4o", temperature=0)

def fix_natural_language_query_iteratively(user_query: str, max_attempts: int = 5) -> str:
    """
    Convert natural language to Cypher and iteratively fix any validation issues.
    """
    conversation_history = []
    current_natural_query = user_query
    attempt = 0
    
    while attempt < max_attempts:
        attempt += 1
        print(f"\n--- Attempt {attempt} ---")
        print(f"Natural language: {current_natural_query}")
        
        try:
            # Step 1: Convert natural language to Cypher using Text2CypherRetriever
            result = text2cypher_retriever.search(current_natural_query)
            
            # Extract the generated Cypher from metadata
            generated_cypher = result.metadata.get('cypher_query', '') if result.metadata else ''
            print(f"Generated Cypher: {generated_cypher}")
            
            if not generated_cypher:
                print("❌ No Cypher query generated")
                break
            
            # Step 2: Check syntax first (fail-fast parser)
            try:
                syntax_valid = check_syntax(generated_cypher)
                if not syntax_valid:
                    print("❌ Syntax error detected in generated Cypher")
                    # Ask LLM to improve the natural language query
                    fix_prompt = f"""
                    The generated Cypher query has syntax errors:
                    
                    Generated Cypher: {generated_cypher}
                    Original natural language: {current_natural_query}
                    
                    Please provide an improved natural language query that would generate better Cypher.
                    Be more specific about the query structure and avoid ambiguous terms.
                    """
                    
                    response = chat_model.invoke([HumanMessage(content=fix_prompt)])
                    current_natural_query = response.content.strip()
                    conversation_history.append(HumanMessage(content="Fix syntax errors"))
                    conversation_history.append(AIMessage(content=current_natural_query))
                    continue
            except Exception as e:
                print(f"❌ Syntax error: {e}")
                # Ask LLM to improve the natural language query
                fix_prompt = f"""
                The generated Cypher query has a syntax error: {e}
                
                Generated Cypher: {generated_cypher}
                Original natural language: {current_natural_query}
                
                Please provide an improved natural language query that would generate better Cypher.
                Be more specific about the query structure and avoid ambiguous terms.
                """
                
                response = chat_model.invoke([HumanMessage(content=fix_prompt)])
                current_natural_query = response.content.strip()
                conversation_history.append(HumanMessage(content=f"Fix syntax error: {e}"))
                conversation_history.append(AIMessage(content=current_natural_query))
                continue
            
            # Step 3: Check validation against schema
            validation_errors = validate_cypher(generated_cypher, schema)
            if validation_errors:
                print(f"❌ Validation errors: {len(validation_errors)} found")
                for error in validation_errors:
                    print(f"  - {error}")
                
                # Ask LLM to improve the natural language query
                errors_text = "\n".join([f"- {error}" for error in validation_errors])
                fix_prompt = f"""
                The generated Cypher query has validation errors against the schema:
                
                Generated Cypher: {generated_cypher}
                Original natural language: {current_natural_query}
                
                Schema: {schema_dict}
                
                Validation errors:
                {errors_text}
                
                Please provide an improved natural language query that would generate better Cypher.
                Be more specific about node labels, relationship types, and properties that exist in the schema.
                """
                
                response = chat_model.invoke([HumanMessage(content=fix_prompt)])
                current_natural_query = response.content.strip()
                conversation_history.append(HumanMessage(content=f"Fix validation errors: {errors_text}"))
                conversation_history.append(AIMessage(content=current_natural_query))
                continue
            
            # Step 4: Check if query returns results
            if not result.records:
                print("⚠️ Generated Cypher returns no results")
                # Ask LLM to improve the natural language query
                improve_prompt = f"""
                The generated Cypher query returns no results:
                
                Generated Cypher: {generated_cypher}
                Original natural language: {current_natural_query}
                
                Please provide an improved natural language query that might return results.
                Consider being more specific about what you're looking for or using different terms.
                """
                
                response = chat_model.invoke([HumanMessage(content=improve_prompt)])
                current_natural_query = response.content.strip()
                conversation_history.append(HumanMessage(content="Query returns no results, improve it"))
                conversation_history.append(AIMessage(content=current_natural_query))
                continue
            
            # Step 5: Query is syntactically and semantically valid
            print("✅ Generated Cypher is valid and returns results!")
            return generated_cypher
                
        except Text2CypherRetrievalError as e:
            print(f"❌ Text2Cypher retrieval error: {e}")
            # Ask LLM to improve the natural language query
            fix_prompt = f"""
            The Text2CypherRetriever failed to generate a valid Cypher query: {e}
            
            Original natural language: {current_natural_query}
            
            Please provide an improved natural language query that would be easier to convert to Cypher.
            Be more specific about the query structure and avoid ambiguous terms.
            """
            
            response = chat_model.invoke([HumanMessage(content=fix_prompt)])
            current_natural_query = response.content.strip()
            conversation_history.append(HumanMessage(content=f"Fix Text2Cypher error: {e}"))
            conversation_history.append(AIMessage(content=current_natural_query))
            continue
        except SearchValidationError as e:
            print(f"❌ Search validation error: {e}")
            # Ask LLM to improve the natural language query
            fix_prompt = f"""
            The search validation failed: {e}
            
            Original natural language: {current_natural_query}
            
            Please provide an improved natural language query that would pass validation.
            """
            
            response = chat_model.invoke([HumanMessage(content=fix_prompt)])
            current_natural_query = response.content.strip()
            conversation_history.append(HumanMessage(content=f"Fix validation error: {e}"))
            conversation_history.append(AIMessage(content=current_natural_query))
            continue
        except Exception as e:
            print(f"❌ Unexpected error: {str(e)}")
            break
    
    print(f"❌ Failed to generate valid Cypher after {max_attempts} attempts")
    return ""

# Example usage
natural_query = "show me the top 5 people related to Ted"
generated_cypher = fix_natural_language_query_iteratively(natural_query)
print(f"\nFinal Cypher: {generated_cypher}")

# Close the driver
driver.close()
```

This example demonstrates:

1. **Fail-fast parsing**: The parser stops at the first syntax error, which we catch and fix iteratively
2. **Schema validation**: After syntax is correct, we validate against the actual database schema
3. **Query execution testing**: We test if the query actually returns results
4. **LLM-powered fixing**: Each error type gets specific prompts to the LLM for correction
5. **Iterative improvement**: The process continues until we have a working query or reach the maximum attempts

The key insight is that cypher-guard's fail-fast behavior is actually beneficial for iterative fixing - you fix one error at a time, making the process more manageable and giving the LLM focused feedback.

---

## License

MIT