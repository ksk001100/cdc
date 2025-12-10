use seahorse::{App, Context};
use std::collections::HashMap;
use std::error::Error;

fn main() {
    let app = App::new("cdc")
        .description("Check for duplicate values in CSV file by specified header")
        .version(env!("CARGO_PKG_VERSION"))
        .usage("cdc <header_name> <file_path>")
        .action(check_duplicates);

    if let Err(e) = app.run(std::env::args().collect()) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn check_duplicates(c: &Context) -> Result<(), Box<dyn Error>> {
    if c.args.len() < 2 {
        return Err("Usage: cdc <header_name> <file_path>".into());
    }

    let header = &c.args[0];
    let file_path = &c.args[1];

    process_csv(file_path, header)?;
    Ok(())
}

fn process_csv(file_path: &str, target_header: &str) -> Result<(), Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(file_path)?;

    // Get headers
    let headers = reader.headers()?.clone();

    // Find target column index
    let target_index = headers
        .iter()
        .position(|h| h == target_header)
        .ok_or_else(|| format!("Header '{}' not found in CSV file", target_header))?;

    // Track values and their line numbers
    let mut value_lines: HashMap<String, Vec<usize>> = HashMap::new();

    for (line_num, result) in reader.records().enumerate() {
        let record = result?;
        if let Some(value) = record.get(target_index) {
            value_lines
                .entry(value.to_string())
                .or_default()
                .push(line_num + 2); // +2 because CSV line numbers start at 1 and header is line 1
        }
    }

    // Find duplicates
    let mut duplicates: Vec<(String, Vec<usize>)> = value_lines
        .into_iter()
        .filter(|(_, lines)| lines.len() > 1)
        .collect();

    if duplicates.is_empty() {
        println!("No duplicates found in column '{}'", target_header);
        return Ok(());
    }

    // Sort by first occurrence line number
    duplicates.sort_by_key(|(_, lines)| lines[0]);

    println!("Duplicates found in column '{}':\n", target_header);
    for (value, lines) in duplicates {
        println!("Value: '{}'", value);
        println!("  Lines: {:?}", lines);
        println!("  Count: {}\n", lines.len());
    }

    Ok(())
}
