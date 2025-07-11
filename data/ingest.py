#!/usr/bin/env python3
"""
Neo4j Data Ingestion Script

This script reads CSV files and creates nodes and relationships in Neo4j
according to the eval_schema.json structure.

Environment Variables:
- NEO4J_URI: Neo4j connection URI (default: bolt://localhost:7687)
- NEO4J_USER: Neo4j username (default: neo4j)
- NEO4J_PASSWORD: Neo4j password (required)
- NEO4J_DATABASE: Neo4j database name (default: neo4j)
"""

import os
import csv
import sys
from datetime import datetime
from typing import Dict, List, Any
from neo4j import GraphDatabase
from neo4j.exceptions import ServiceUnavailable, AuthError
from dotenv import load_dotenv

# Load environment variables from .env file
load_dotenv()


class Neo4jIngestor:
    def __init__(self):
        self.uri = os.getenv('NEO4J_URI', 'bolt://localhost:7687')
        self.user = os.getenv('NEO4J_USER', 'neo4j')
        self.password = os.getenv('NEO4J_PASSWORD')
        self.database = os.getenv('NEO4J_DATABASE', 'neo4j')
        
        if not self.password:
            raise ValueError("NEO4J_PASSWORD environment variable is required")
        
        self.driver = None
        self.csv_dir = os.path.join(os.path.dirname(__file__), 'csv')
    
    def connect(self):
        """Connect to Neo4j database"""
        try:
            self.driver = GraphDatabase.driver(self.uri, auth=(self.user, self.password))
            # Test connection
            with self.driver.session(database=self.database) as session:
                result = session.run("RETURN 1 as test")
                result.single()
            print(f"✅ Connected to Neo4j at {self.uri}")
        except AuthError:
            raise Exception("Authentication failed. Check NEO4J_USER and NEO4J_PASSWORD")
        except ServiceUnavailable:
            raise Exception(f"Could not connect to Neo4j at {self.uri}")
    
    def close(self):
        """Close Neo4j connection"""
        if self.driver:
            self.driver.close()
            print("🔌 Connection closed")
    
    def clear_database(self):
        """Clear all nodes and relationships from the database"""
        with self.driver.session(database=self.database) as session:
            session.run("MATCH (n) DETACH DELETE n")
            print("🗑️  Database cleared")
    
    def create_constraints(self):
        """Create node key constraints for unique identification"""
        constraints = [
            "CREATE CONSTRAINT person_node_key IF NOT EXISTS FOR (p:Person) REQUIRE (p.firstName, p.lastName) IS NODE KEY",
            "CREATE CONSTRAINT company_node_key IF NOT EXISTS FOR (c:Company) REQUIRE c.companyName IS NODE KEY",
            "CREATE CONSTRAINT location_node_key IF NOT EXISTS FOR (l:Location) REQUIRE (l.city, l.country) IS NODE KEY"
        ]
        
        with self.driver.session(database=self.database) as session:
            for constraint in constraints:
                try:
                    session.run(constraint)
                    constraint_name = constraint.split('CONSTRAINT')[1].split('IF NOT EXISTS')[0].strip()
                    print(f"✅ Created constraint: {constraint_name}")
                except Exception as e:
                    print(f"⚠️  Constraint may already exist: {e}")
    
    def create_indexes(self):
        """Create additional indexes for performance"""
        indexes = [
            "CREATE INDEX person_email_index IF NOT EXISTS FOR (p:Person) ON (p.email)",
            "CREATE INDEX person_age_index IF NOT EXISTS FOR (p:Person) ON (p.age)",
            "CREATE INDEX company_industry_index IF NOT EXISTS FOR (c:Company) ON (c.industry)",
            "CREATE INDEX location_coordinates_index IF NOT EXISTS FOR (l:Location) ON (l.coordinates)"
        ]
        
        with self.driver.session(database=self.database) as session:
            for index in indexes:
                try:
                    session.run(index)
                    index_name = index.split('INDEX')[1].split('IF NOT EXISTS')[0].strip()
                    print(f"✅ Created index: {index_name}")
                except Exception as e:
                    print(f"⚠️  Index may already exist: {e}")
    
    def read_csv(self, filename: str) -> List[Dict[str, Any]]:
        """Read CSV file and return list of dictionaries"""
        filepath = os.path.join(self.csv_dir, filename)
        if not os.path.exists(filepath):
            raise FileNotFoundError(f"CSV file not found: {filepath}")
        
        data = []
        with open(filepath, 'r', newline='', encoding='utf-8') as file:
            reader = csv.DictReader(file)
            for row in reader:
                data.append(row)
        
        print(f"📖 Read {len(data)} records from {filename}")
        return data
    
    def convert_value(self, value: str, data_type: str) -> Any:
        """Convert string values to appropriate data types"""
        if value == '' or value is None:
            return None
        
        try:
            if data_type == 'INTEGER':
                return int(value)
            elif data_type == 'FLOAT':
                return float(value)
            elif data_type == 'BOOLEAN':
                return value.lower() in ('true', '1', '1.0')
            elif data_type == 'DATE_TIME':
                return value  # Keep as string for Neo4j
            elif data_type == 'POINT':
                return value  # Keep as string for Neo4j
            else:  # STRING
                return value
        except (ValueError, TypeError):
            print(f"⚠️  Could not convert '{value}' to {data_type}, using string")
            return value
    
    def ingest_persons(self):
        """Ingest Person nodes with employment type labels"""
        persons = self.read_csv('person.csv')
        
        # Convert data types
        for person in persons:
            person['age'] = self.convert_value(person['age'], 'INTEGER')
            person['active'] = self.convert_value(person['active'], 'BOOLEAN')
        
        with self.driver.session(database=self.database) as session:
            # First, create/merge the base Person nodes
            base_query = """
            UNWIND $persons AS person
            MERGE (p:Person {firstName: person.firstName, lastName: person.lastName})
            SET p.id = person.id,
                p.email = person.email,
                p.age = person.age,
                p.active = person.active,
                p.employmentType = person.employmentType
            """
            session.run(base_query, persons=persons)
            
            # Add Employee label for employees
            employee_query = """
            UNWIND $persons AS person
            MATCH (p:Person {firstName: person.firstName, lastName: person.lastName})
            WHERE person.employmentType = 'Employee'
            SET p:Employee
            """
            session.run(employee_query, persons=persons)
            
            # Add Contractor label for contractors
            contractor_query = """
            UNWIND $persons AS person
            MATCH (p:Person {firstName: person.firstName, lastName: person.lastName})
            WHERE person.employmentType = 'Contractor'
            SET p:Contractor
            """
            session.run(contractor_query, persons=persons)
            
            print(f"✅ Merged {len(persons)} Person nodes with employment type labels")
    
    def ingest_companies(self):
        """Ingest Company nodes"""
        companies = self.read_csv('company.csv')
        
        query = """
        UNWIND $companies AS company
        MERGE (c:Company {companyName: company.companyName})
        SET c.id = company.id,
            c.foundedYear = company.foundedYear,
            c.industry = company.industry,
            c.employeeCount = company.employeeCount
        """
        
        # Convert data types
        for company in companies:
            company['foundedYear'] = self.convert_value(company['foundedYear'], 'INTEGER')
            company['industry'] = self.convert_value(company['industry'], 'STRING')
            company['employeeCount'] = self.convert_value(company['employeeCount'], 'INTEGER')
        
        with self.driver.session(database=self.database) as session:
            session.run(query, companies=companies)
            print(f"✅ Merged {len(companies)} Company nodes")
    
    def ingest_locations(self):
        """Ingest Location nodes"""
        locations = self.read_csv('location.csv')
        
        query = """
        UNWIND $locations AS location
        MERGE (l:Location {city: location.city, country: location.country})
        SET l.coordinates = point({x: toFloat(location.longitude), y: toFloat(location.latitude)})
        """
        
        with self.driver.session(database=self.database) as session:
            session.run(query, locations=locations)
            print(f"✅ Merged {len(locations)} Location nodes")
    
    def ingest_works_for(self):
        """Ingest WORKS_FOR relationships"""
        works_for = self.read_csv('works_for.csv')

        query = """
        UNWIND $works_for AS work
        MATCH (p:Person {firstName: work.personFirstName, lastName: work.personLastName})
        MATCH (c:Company {companyName: work.companyName})
        MERGE (p)-[r:WORKS_FOR]->(c)
        SET r.startDate = datetime(work.startDate),
            r.position = work.position,
            r.salary = toFloat(work.salary)
        """
        
        # Convert data types
        for work in works_for:
            work['salary'] = self.convert_value(work['salary'], 'FLOAT')
        
        with self.driver.session(database=self.database) as session:
            session.run(query, works_for=works_for)
            print(f"✅ Merged {len(works_for)} WORKS_FOR relationships")
    
    def ingest_located_in(self):
        """Ingest LOCATED_IN relationships"""
        located_in = self.read_csv('located_in.csv')
        print(located_in)
        query = """
        UNWIND $located_in AS location
        MATCH (c:Company {companyName: location.companyName})
        MATCH (l:Location {city: location.city, country: location.country})
        MERGE (c)-[r:LOCATED_IN]->(l)
        SET r.since = datetime(location.since)
        """
        
        # No data type conversion needed for string properties
        
        with self.driver.session(database=self.database) as session:
            session.run(query, located_in=located_in)
            print(f"✅ Merged {len(located_in)} LOCATED_IN relationships")
    
    def ingest_knows(self):
        """Ingest KNOWS relationships"""
        knows = self.read_csv('knows.csv')
        
        query = """
        UNWIND $knows AS know
        MATCH (p1:Person {firstName: know.person1FirstName, lastName: know.person1LastName})
        MATCH (p2:Person {firstName: know.person2FirstName, lastName: know.person2LastName})
        MERGE (p1)-[r:KNOWS]->(p2)
        SET r.relationshipType = know.relationshipType,
            r.sinceYear = know.sinceYear
        """
        
        # Convert data types
        for know in knows:
            know['sinceYear'] = self.convert_value(know['sinceYear'], 'INTEGER')
        
        with self.driver.session(database=self.database) as session:
            session.run(query, knows=knows)
            print(f"✅ Merged {len(knows)} KNOWS relationships")
    
    def get_stats(self):
        """Get database statistics"""
        with self.driver.session(database=self.database) as session:
            # Node counts
            person_count = session.run("MATCH (p:Person) RETURN count(p) as count").single()['count']
            company_count = session.run("MATCH (c:Company) RETURN count(c) as count").single()['count']
            location_count = session.run("MATCH (l:Location) RETURN count(l) as count").single()['count']
            
            # Relationship counts
            works_for_count = session.run("MATCH ()-[r:WORKS_FOR]->() RETURN count(r) as count").single()['count']
            located_in_count = session.run("MATCH ()-[r:LOCATED_IN]->() RETURN count(r) as count").single()['count']
            knows_count = session.run("MATCH ()-[r:KNOWS]->() RETURN count(r) as count").single()['count']
            
            print("\n📊 Database Statistics:")
            print(f"   Nodes: {person_count + company_count + location_count}")
            print(f"   - Person: {person_count}")
            print(f"   - Company: {company_count}")
            print(f"   - Location: {location_count}")
            print(f"   Relationships: {works_for_count + located_in_count + knows_count}")
            print(f"   - WORKS_FOR: {works_for_count}")
            print(f"   - LOCATED_IN: {located_in_count}")
            print(f"   - KNOWS: {knows_count}")
    
    def run_ingestion(self, clear_db: bool = True):
        """Run the complete ingestion process"""
        print("🚀 Starting Neo4j data ingestion...")
        print(f"📁 CSV directory: {self.csv_dir}")
        
        try:
            self.connect()
            
            if clear_db:
                self.clear_database()
            
            # Create constraints and indexes
            self.create_constraints()
            self.create_indexes()
            
            # Ingest nodes
            print("\n📥 Ingesting nodes...")
            self.ingest_persons()
            self.ingest_companies()
            self.ingest_locations()
            
            # Ingest relationships
            print("\n🔗 Ingesting relationships...")
            self.ingest_works_for()
            self.ingest_located_in()
            self.ingest_knows()
            
            # Show statistics
            self.get_stats()
            
            print("\n✅ Data ingestion completed successfully!")
            
        except Exception as e:
            print(f"❌ Error during ingestion: {e}")
            raise
        finally:
            self.close()


def main():
    """Main function"""
    import argparse
    
    parser = argparse.ArgumentParser(description='Ingest CSV data into Neo4j')
    parser.add_argument('--keep-data', action='store_true', 
                       help='Keep existing data (do not clear database)')
    args = parser.parse_args()
    
    try:
        ingestor = Neo4jIngestor()
        ingestor.run_ingestion(clear_db=not args.keep_data)
    except Exception as e:
        print(f"❌ Failed to run ingestion: {e}")
        sys.exit(1)


if __name__ == "__main__":
    main() 