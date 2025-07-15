use clap::{Arg, Command};
use colored::*;
use cypher_guard::{validate_cypher_with_schema, DbSchema};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Deserialize, Serialize)]
struct QueryFile {
    name: String,
    description: String,
    category: String,
    queries: Vec<Query>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Query {
    name: String,
    description: String,
    cypher: String,
}

#[derive(Debug)]
struct ValidationResult {
    file_path: PathBuf,
    query_name: String,
    query_description: String,
    cypher: String,
    is_valid: bool,
    error_message: Option<String>,
    expected_valid: bool,
}

#[derive(Debug)]
struct EvalStats {
    total_files: usize,
    total_queries: usize,
    successful_validations: usize,
    failed_validations: usize,
    parsing_errors: usize,
    #[allow(dead_code)]
    schema_validation_errors: usize,
}

impl EvalStats {
    fn new() -> Self {
        Self {
            total_files: 0,
            total_queries: 0,
            successful_validations: 0,
            failed_validations: 0,
            parsing_errors: 0,
            schema_validation_errors: 0,
        }
    }

    fn accuracy(&self) -> f64 {
        if self.total_queries == 0 {
            return 0.0;
        }
        (self.successful_validations as f64) / (self.total_queries as f64) * 100.0
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("cypher-guard-eval")
        .version("0.1.0")
        .author("Cypher Guard Team")
        .about("Evaluates Cypher Guard against test query datasets")
        .arg(
            Arg::new("schema")
                .short('s')
                .long("schema")
                .value_name("FILE")
                .help("Path to the schema JSON file")
                .default_value("../../data/schema/eval_schema.json"),
        )
        .arg(
            Arg::new("queries")
                .short('q')
                .long("queries")
                .value_name("DIR")
                .help("Directory containing query YAML files")
                .default_value("../../data/queries"),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Enable verbose output")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("detailed")
                .short('d')
                .long("detailed")
                .help("Show detailed results for each query")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    let schema_path = matches.get_one::<String>("schema").unwrap();
    let queries_dir = matches.get_one::<String>("queries").unwrap();
    let verbose = matches.get_flag("verbose");
    let detailed = matches.get_flag("detailed");

    println!("{}", "🚀 Cypher Guard Evaluation Tool".bright_blue().bold());
    println!("Schema: {}", schema_path.bright_yellow());
    println!("Queries: {}", queries_dir.bright_yellow());
    println!();

    // Load schema
    if verbose {
        println!("{}", "📄 Loading schema...".bright_green());
    }

    let schema = match DbSchema::from_json_file(schema_path) {
        Ok(schema) => {
            if verbose {
                println!("✅ Schema loaded successfully");
                println!("   - Node labels: {}", schema.node_props.len());
                println!("   - Relationship types: {}", schema.rel_props.len());
                println!("   - Relationships: {}", schema.relationships.len());
            }
            schema
        }
        Err(e) => {
            eprintln!("{} {}", "❌ Failed to load schema:".bright_red(), e);
            return Err(e.into());
        }
    };

    // Discover and validate query files
    let mut results = Vec::new();
    let mut stats = EvalStats::new();

    if verbose {
        println!("{}", "🔍 Discovering query files...".bright_green());
    }

    let query_files = discover_query_files(queries_dir)?;
    stats.total_files = query_files.len();

    if verbose {
        println!("   Found {} query files", query_files.len());
    }

    for (file_path, expected_valid) in query_files {
        if verbose {
            println!(
                "📂 Processing: {}",
                file_path.display().to_string().bright_cyan()
            );
        }

        match process_query_file(&file_path, &schema, expected_valid, verbose) {
            Ok(mut file_results) => {
                stats.total_queries += file_results.len();
                for result in &file_results {
                    if result.is_valid == result.expected_valid {
                        stats.successful_validations += 1;
                    } else {
                        stats.failed_validations += 1;
                    }
                }
                results.append(&mut file_results);
            }
            Err(e) => {
                eprintln!("   {} {}", "❌ Error processing file:".bright_red(), e);
                stats.parsing_errors += 1;
            }
        }
    }

    // Generate report
    println!();
    print_summary(&stats);

    if detailed {
        println!();
        print_detailed_results(&results);
    }

    // Print failures if any
    let failures: Vec<_> = results
        .iter()
        .filter(|r| r.is_valid != r.expected_valid)
        .collect();
    if !failures.is_empty() {
        println!();
        print_failures(&failures);
    }

    Ok(())
}

fn discover_query_files(
    queries_dir: &str,
) -> Result<Vec<(PathBuf, bool)>, Box<dyn std::error::Error>> {
    let mut query_files = Vec::new();

    for entry in WalkDir::new(queries_dir) {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("yml")
            || path.extension().and_then(|s| s.to_str()) == Some("yaml")
        {
            // Determine if queries should be valid based on directory structure
            let expected_valid = path.to_string_lossy().contains("valid");
            query_files.push((path.to_path_buf(), expected_valid));
        }
    }

    // Sort for consistent ordering
    query_files.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(query_files)
}

fn process_query_file(
    file_path: &Path,
    schema: &DbSchema,
    expected_valid: bool,
    verbose: bool,
) -> Result<Vec<ValidationResult>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;
    let query_file: QueryFile = serde_yaml::from_str(&content)?;

    if verbose {
        println!(
            "   📋 {}: {} queries",
            query_file.name,
            query_file.queries.len()
        );
    }

    let mut results = Vec::new();

    for query in query_file.queries {
        if verbose {
            println!("      🔍 Validating: {}", query.name);
        }

        let validation_result = match validate_cypher_with_schema(&query.cypher, schema) {
            Ok(is_valid) => ValidationResult {
                file_path: file_path.to_path_buf(),
                query_name: query.name.clone(),
                query_description: query.description.clone(),
                cypher: query.cypher.clone(),
                is_valid,
                error_message: None,
                expected_valid,
            },
            Err(e) => ValidationResult {
                file_path: file_path.to_path_buf(),
                query_name: query.name.clone(),
                query_description: query.description.clone(),
                cypher: query.cypher.clone(),
                is_valid: false,
                error_message: Some(e.to_string()),
                expected_valid,
            },
        };

        if verbose {
            let status = if validation_result.is_valid == expected_valid {
                "✅ PASS".bright_green()
            } else {
                "❌ FAIL".bright_red()
            };
            println!("         {}", status);
        }

        results.push(validation_result);
    }

    Ok(results)
}

fn print_summary(stats: &EvalStats) {
    println!("{}", "📊 EVALUATION SUMMARY".bright_blue().bold());
    println!("═══════════════════════════════════════");

    println!(
        "Files processed: {}",
        stats.total_files.to_string().bright_cyan()
    );
    println!(
        "Total queries: {}",
        stats.total_queries.to_string().bright_cyan()
    );
    println!();

    println!("Results:");
    println!(
        "  ✅ Correct validations: {}",
        stats.successful_validations.to_string().bright_green()
    );
    println!(
        "  ❌ Incorrect validations: {}",
        stats.failed_validations.to_string().bright_red()
    );
    println!(
        "  🚫 Parsing errors: {}",
        stats.parsing_errors.to_string().bright_yellow()
    );
    println!();

    let accuracy = stats.accuracy();
    let accuracy_color = if accuracy >= 90.0 {
        accuracy.to_string().bright_green()
    } else if accuracy >= 70.0 {
        accuracy.to_string().bright_yellow()
    } else {
        accuracy.to_string().bright_red()
    };

    println!("🎯 Accuracy: {}%", accuracy_color.bold());
}

fn print_detailed_results(results: &[ValidationResult]) {
    println!("{}", "📋 DETAILED RESULTS".bright_blue().bold());
    println!("═══════════════════════════════════════");

    let mut by_file: HashMap<PathBuf, Vec<&ValidationResult>> = HashMap::new();
    for result in results {
        by_file
            .entry(result.file_path.clone())
            .or_default()
            .push(result);
    }

    for (file_path, file_results) in by_file {
        println!();
        println!(
            "📂 {}",
            file_path.display().to_string().bright_cyan().bold()
        );

        for result in file_results {
            let status = if result.is_valid == result.expected_valid {
                "✅ PASS".bright_green()
            } else {
                "❌ FAIL".bright_red()
            };

            println!("   {} {}", status, result.query_name.bright_white());
            println!("      📝 {}", result.query_description.dimmed());

            if let Some(error) = &result.error_message {
                println!("      🚫 Error: {}", error.bright_red());
            }
        }
    }
}

fn print_failures(failures: &[&ValidationResult]) {
    println!("{}", "❌ FAILED VALIDATIONS".bright_red().bold());
    println!("═══════════════════════════════════════");

    for failure in failures {
        println!();
        println!(
            "📂 File: {}",
            failure.file_path.display().to_string().bright_cyan()
        );
        println!("📝 Query: {}", failure.query_name.bright_white().bold());
        println!("📋 Description: {}", failure.query_description);
        println!(
            "🎯 Expected: {}",
            if failure.expected_valid {
                "VALID".bright_green()
            } else {
                "INVALID".bright_red()
            }
        );
        println!(
            "📊 Got: {}",
            if failure.is_valid {
                "VALID".bright_green()
            } else {
                "INVALID".bright_red()
            }
        );

        if let Some(error) = &failure.error_message {
            println!("🚫 Error: {}", error.bright_yellow());
        }

        println!("🔍 Cypher:");
        println!("{}", failure.cypher.bright_white().on_black());
        println!("{}", "─".repeat(50).dimmed());
    }
}
