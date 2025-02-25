use clap::{Arg, Command};
use jqr::*;
use std::fs;
use std::io::{self, Read};

fn main() {
    let matches = Command::new("jqr")
        .version("0.1.0")
        .author("Author <Daniel Morlim>")
        .about("Pretty-print and query JSON data")
        .arg(Arg::new("file").help("Path to JSON file. If omitted, reads from stdin."))
        .arg(Arg::new("query").help("JSONPath query (e.g., '$.user.name')"))
        .arg(
            Arg::new("to-yaml")
                .long("to-yaml")
                .help("Convert JSON to YAML"),
        )
        .arg(
            Arg::new("to-json")
                .long("to-json")
                .help("Convert YAML to JSON"),
        )
        .get_matches();

    // If no arguments are provided, display help message
    if !matches.args_present() {
        let mut cmd = Command::new("jqr");
        cmd.print_long_help().unwrap();
        return;
    }

    let file_path = matches.get_one::<String>("file");
    let query = matches.get_one::<String>("query");

    let content = if let Some(path) = file_path {
        match fs::read_to_string(path) {
            Ok(data) => data,
            Err(e) => {
                eprintln!("Error reading file: {}", e);
                return;
            }
        }
    } else {
        let mut buffer = String::new();
        if io::stdin().read_to_string(&mut buffer).is_ok() {
            buffer
        } else {
            eprintln!("Failed to read from stdin");
            return;
        }
    };

    if matches.contains_id("to-yaml") {
        if let Err(e) = convert_to_yaml(&content) {
            eprintln!("Error converting to YAML: {}", e);
            return;
        }
    } else if matches.contains_id("to-json") {
        convert_to_json(&content)
    } else {

        let result = pretty_print_json(&content, query);
        match result {
            Ok(output) => println!("{}", output),
            Err(e) => eprintln!("Error processing JSON: {}", e),
        }

        return;

    }
}
