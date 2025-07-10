# Test Data CSV Files

This directory contains test data CSV files that correspond to the schema defined in `eval_schema.json`.

## Node Files

### `person.csv`
- **15 Person nodes** with properties: id, firstName, lastName, age, email, active
- Age range: 24-52 (within schema constraint 0-120)
- Mix of active/inactive users
- Unique email addresses

### `company.csv`
- **10 Company nodes** with properties: id, companyName, foundedYear, industry, employeeCount
- Founded years: 1976-2018 (within schema constraint 1800-2024)
- Industries: Technology, Healthcare, Education, Finance, Manufacturing (from schema enum)
- Employee counts: 75-800 (minimum 1 as per schema)

### `location.csv`
- **8 Location nodes** with properties: id, city, country, coordinates
- Major cities across different countries
- Coordinates in POINT format as specified in schema

## Relationship Files

### `works_for.csv`
- **15 WORKS_FOR relationships** between Person and Company nodes
- Properties: personFirstName, personLastName, companyName, startDate, position, salary
- Uses node key properties for matching (firstName+lastName for Person, companyName for Company)
- Salaries: $60,000-$120,000 (within schema constraint 30,000-500,000)
- Start dates: 2017-2021 in ISO 8601 format
- Realistic job positions for each industry

### `located_in.csv`
- **10 LOCATED_IN relationships** between Company and Location nodes
- Properties: companyName, city, country, since
- Uses node key properties for matching (companyName for Company, city+country for Location)
- Since dates: 2017-2021 in ISO 8601 format
- Connects companies to their office locations

### `knows.csv`
- **30 KNOWS relationships** between Person nodes
- Properties: person1FirstName, person1LastName, person2FirstName, person2LastName, relationshipType, sinceYear
- Uses node key properties for matching (firstName+lastName for both Person nodes)
- Relationship types: colleague, friend, acquaintance, family (from schema enum)
- Since years: 2017-2021 (within schema constraint 1950-2024)
- Creates a connected network of people who know each other

## Data Relationships

The test data creates a realistic network where:
- People work for companies (WORKS_FOR)
- Companies are located in cities (LOCATED_IN)
- People know each other through various relationships (KNOWS)

This provides a comprehensive dataset for testing graph queries and validating the CypherGuard schema. 