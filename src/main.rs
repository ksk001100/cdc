use koral::prelude::*;
use std::collections::HashMap;

#[derive(Flag)]
#[flag(
    name = "header",
    short = 'h',
    required = true,
    value_name = "HEADER_NAME",
    help = "Specify the header name to check for duplicates"
)]
struct HeaderFlag(#[allow(dead_code)] String);

#[derive(Flag)]
#[flag(
    name = "input",
    short = 'i',
    value_name = "FILE_PATH",
    required = true,
    help = "Specify the input CSV file path"
)]
struct InputFlag(#[allow(dead_code)] String);

#[derive(App, Default)]
#[app(name = "cdc", version = env!("CARGO_PKG_VERSION"), action = run)]
#[app(flags(HeaderFlag, InputFlag))]
struct CdcApp;

fn main() {
    let mut app = CdcApp;
    if let Err(e) = app.run(std::env::args().collect()) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run(header: FlagArg<HeaderFlag>, input: FlagArg<InputFlag>) -> KoralResult<()> {
    process_csv(&*input, &*header).koral_err()?;
    Ok(())
}

fn process_csv(file_path: &str, target_header: &str) -> anyhow::Result<()> {
    let mut reader = csv::Reader::from_path(file_path)?;

    // Get headers
    let headers = reader.headers()?.clone();

    // Find target column index
    let target_index = headers
        .iter()
        .position(|h| h == target_header)
        .ok_or_else(|| anyhow::anyhow!("Header '{}' not found in CSV file", target_header))?;

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
