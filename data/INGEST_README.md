# Neo4j Data Ingestion Script

This script ingests the CSV test data into a Neo4j database according to the schema defined in `eval_schema.json`.

## Prerequisites

1. **Neo4j Database**: You need a running Neo4j instance (local or remote)
2. **Python Dependencies**: Install the required packages

```bash
pip install -r requirements.txt
```

## Environment Variables

Set the following environment variables for Neo4j connection:

```bash
# Required
export NEO4J_PASSWORD="your_password_here"

# Optional (with defaults)
export NEO4J_URI="bolt://localhost:7687"
export NEO4J_USER="neo4j"
export NEO4J_DATABASE="neo4j"
```

### Alternative: Using a .env file

Create a `.env` file in the data directory:

```bash
NEO4J_URI=bolt://localhost:7687
NEO4J_USER=neo4j
NEO4J_PASSWORD=your_password_here
NEO4J_DATABASE=neo4j
```

Then load it before running the script:
```bash
source .env
```

## Usage

### Basic Usage (Clear database and load data)

```bash
cd data
python ingest.py
```

### Keep Existing Data

```bash
python ingest.py --keep-data
```

## What the Script Does

1. **Connects to Neo4j** using environment variables
2. **Clears the database** (unless `--keep-data` is used)
3. **Creates node key constraints and indexes**:
   - Node key constraint on Person (firstName, lastName)
   - Node key constraint on Company (companyName)
   - Node key constraint on Location (city, country)
   - Additional indexes on email, age, industry, and coordinates
4. **Merges nodes** from CSV files using key properties:
   - 15 Person nodes (keyed by firstName + lastName)
   - 10 Company nodes (keyed by companyName)
   - 8 Location nodes (keyed by city + country)
5. **Merges relationships** from CSV files:
   - 15 WORKS_FOR relationships
   - 10 LOCATED_IN relationships
   - 30 KNOWS relationships
6. **Shows statistics** of ingested data

### MERGE Pattern Benefits
- **Idempotent**: Safe to run multiple times without creating duplicates
- **Upsert behavior**: Creates nodes/relationships if they don't exist, updates if they do
- **Key-based matching**: Uses meaningful business key properties as identifiers

### Node Key Constraints
Node key constraints ensure that the combination of properties is unique and NOT NULL across all nodes of that label:
- **Person**: firstName + lastName (composite key)
- **Company**: companyName (single key)
- **Location**: city + country (composite key)

These constraints automatically create indexes on the key properties for optimal performance.

### Relationship Matching
All relationships now use node key properties for matching instead of artificial IDs:
- **WORKS_FOR**: Matches Person by (firstName, lastName) and Company by (companyName)
- **LOCATED_IN**: Matches Company by (companyName) and Location by (city, country)
- **KNOWS**: Matches both Person nodes by (firstName, lastName)

## Data Types

The script properly converts CSV string values to appropriate Neo4j types:

- **INTEGER**: age, foundedYear, employeeCount, sinceYear, etc.
- **FLOAT**: salary
- **BOOLEAN**: active
- **DATE_TIME**: startDate, since (converted to Neo4j datetime)
- **POINT**: coordinates (parsed and converted to Neo4j point)
- **STRING**: names, email, position, etc.

## Schema Compliance

The ingested data follows the constraints defined in `eval_schema.json`:

- Property names are in camelCase
- Enum values match allowed values
- Numeric values are within specified ranges
- All required properties are included

## Example Output

```
🚀 Starting Neo4j data ingestion...
📁 CSV directory: /path/to/data/csv
✅ Connected to Neo4j at bolt://localhost:7687
🗑️  Database cleared
✅ Created constraint: person_node_key
✅ Created constraint: company_node_key
✅ Created constraint: location_node_key
✅ Created index: person_email_index
✅ Created index: person_age_index
✅ Created index: company_industry_index
✅ Created index: location_coordinates_index

📥 Ingesting nodes...
📖 Read 15 records from person.csv
✅ Merged 15 Person nodes
📖 Read 10 records from company.csv
✅ Merged 10 Company nodes
📖 Read 8 records from location.csv
✅ Merged 8 Location nodes

🔗 Ingesting relationships...
📖 Read 15 records from works_for.csv
✅ Merged 15 WORKS_FOR relationships
📖 Read 10 records from located_in.csv
✅ Merged 10 LOCATED_IN relationships
📖 Read 30 records from knows.csv
✅ Merged 30 KNOWS relationships

📊 Database Statistics:
   Nodes: 33
   - Person: 15
   - Company: 10
   - Location: 8
   Relationships: 55
   - WORKS_FOR: 15
   - LOCATED_IN: 10
   - KNOWS: 30

✅ Data ingestion completed successfully!
🔌 Connection closed
```

## Error Handling

The script includes comprehensive error handling for:

- Missing environment variables
- Neo4j connection failures
- Authentication errors
- Missing CSV files
- Data type conversion errors

## Testing Queries

After ingestion, you can test the data with example Cypher queries:

```cypher
// Find all people who work for Technology companies
MATCH (p:Person)-[:WORKS_FOR]->(c:Company {industry: "Technology"})
RETURN p.firstName, p.lastName, c.companyName

// Find companies located in the USA
MATCH (c:Company)-[:LOCATED_IN]->(l:Location {country: "USA"})
RETURN c.companyName, l.city

// Find friend relationships
MATCH (p1:Person)-[:KNOWS {relationshipType: "friend"}]->(p2:Person)
RETURN p1.firstName, p2.firstName

// Find a specific person by name (using node key)
MATCH (p:Person {firstName: "John", lastName: "Smith"})
RETURN p

// Find all companies in a specific city
MATCH (c:Company)-[:LOCATED_IN]->(l:Location {city: "New York", country: "USA"})
RETURN c.companyName, c.industry
``` 