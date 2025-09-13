fn main() {
    use cypher_guard::parser::clauses::where_clause;
    let test_input = "WHERE a.name = \"Alice\"";
    match where_clause(test_input) {
        Ok((rest, result)) => println!("SUCCESS: {:?}, remaining: \"{}\"", result, rest),
        Err(e) => println!("FAILED: {:?}", e),
    }
}
