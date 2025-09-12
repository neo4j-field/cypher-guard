from testcontainers.neo4j import Neo4jContainer
import pytest
import os
from neo4j import Driver
neo4j = (
    Neo4jContainer("neo4j:latest")
    .with_env("NEO4J_apoc_export_file_enabled", "true")
    .with_env("NEO4J_apoc_import_file_enabled", "true")
    .with_env("NEO4J_apoc_import_file_use__neo4j__config", "true")
    .with_env("NEO4J_PLUGINS", '["apoc"]')
)
neo4j.with_exposed_ports(7687, 7474)

@pytest.fixture(scope="module", autouse=True)
def setup(request):
    neo4j.start()

    def remove_container():
        neo4j.get_driver().close()
        neo4j.stop()

    request.addfinalizer(remove_container)

    yield neo4j


@pytest.fixture(scope="function")
def neo4j_driver(setup: Neo4jContainer):
    return setup.get_driver()


@pytest.fixture(scope="function")
def init_data(neo4j_driver: Driver, clear_data: None):
    "This uses the driver from testcontainers to create data in the database."
    with neo4j_driver.session(database="neo4j") as session:
        session.run("CREATE CONSTRAINT person_name IF NOT EXISTS FOR (n:Person) REQUIRE n.name IS UNIQUE")
        session.run("CREATE (a:Person {name: 'Alice', age: 30})")
        session.run("CREATE (b:Person {name: 'Bob', age: 25})")
        session.run("CREATE (c:Person {name: 'Charlie', age: 35})")
        session.run("MATCH (a:Person {name: 'Alice'}), (b:Person {name: 'Bob'}) CREATE (a)-[:FRIEND {since: datetime()}]->(b)")
        session.run("MATCH (b:Person {name: 'Bob'}), (c:Person {name: 'Charlie'}) CREATE (b)-[:FRIEND {since: datetime()}]->(c)")


@pytest.fixture(scope="function")
def clear_data(setup: Neo4jContainer):
    "This uses the driver from testcontainers to clear the data in the database."
    with setup.get_driver().session(database="neo4j") as session:
        session.run("MATCH (n) DETACH DELETE n")