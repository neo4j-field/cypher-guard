use cypher_guard::parse_query;

fn main() {
    let query = r#"
    MATCH (bfr:Station {name: 'Blackfriars'})
    MATCH p = (bfr)
    ((a)-[:LINK]-(b:Station)
    WHERE point.distance(a.location, ndl.location) >
    point.distance(b.location, ndl.location))+ (ndl)
    RETURN reduce(acc = 0, r in relationships(p) | round(acc + r.distance, 2))
    AS distance
    "#;
    
    println!("Testing complex query parsing...");
    match parse_query(query) {
        Ok(ast) => {
            println!("✅ Query parsed successfully!");
            println!("Match clauses: {}", ast.match_clauses.len());
            println!("Return clauses: {}", ast.return_clauses.len());
        }
        Err(e) => {
            println!("❌ Query parsing failed: {}", e);
        }
    }
} 