fn main() {
    use std::io::{self, Read};
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).expect("Failed to read from stdin");
    let result = cypher_guard::validate_cypher(&buffer);
    match result {
        Ok(valid) => println!("Validation result: {}", valid),
        Err(e) => eprintln!("Validation error: {}", e),
    }
}
