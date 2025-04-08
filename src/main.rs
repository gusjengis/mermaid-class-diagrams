mod class;
mod discover_types;
mod get_rust_files;
mod parse_rust_file;
mod process_file;

use crate::discover_types::*;
use crate::get_rust_files::*;
use crate::parse_rust_file::*;
use crate::process_file::*;
use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::fs;
use std::process::Command;

fn main() -> Result<(), Box<dyn Error>> {
    // Get command-line arguments
    let args: Vec<String> = env::args().collect();

    // Parse arguments
    let mut dir_to_scan = "src";
    let mut generate_png = true;

    // Simple argument parsing
    for i in 1..args.len() {
        if args[i] == "--no-png" {
            generate_png = false;
        } else if !args[i].starts_with("--") {
            dir_to_scan = &args[i];
        }
    }
    let rust_files = get_rust_files(dir_to_scan);
    let mut diagram = String::from("classDiagram\n\n");
    let mut defined_types = vec![];
    for file_path in &rust_files {
        let syntax = parse_rust_file(file_path)?;
        defined_types.extend_from_slice(discover_types(&syntax).as_slice());
    }
    let type_set: HashSet<&String> =
        HashSet::from_iter(defined_types.iter().take(defined_types.len()));
    for file_path in &rust_files {
        let syntax = parse_rust_file(file_path)?;
        process_file(&syntax, &mut diagram, &type_set);
    }

    // Write the output to a file, e.g. diagram.mmd
    fs::write("diagram.mmd", diagram)?;
    println!("Mermaid diagram generated: diagram.mmd");

    // Generate PNG using mmdc (Mermaid CLI)
    println!("Generating PNG from diagram...");
    let sudo_user = env::var("SUDO_USER").unwrap_or_else(|_| "your_username".to_string());

    let output = Command::new("sudo")
        .arg("-u")
        .arg(&sudo_user)
        .arg("mmdc")
        .arg("-i")
        .arg("diagram.mmd")
        .arg("-o")
        .arg("diagram.png")
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                println!("PNG diagram generated successfully: diagram.png");
            } else {
                eprintln!(
                    "Failed to generate PNG: {}",
                    String::from_utf8_lossy(&output.stderr)
                );
                eprintln!(
                    "Make sure you have mermaid-cli installed (npm install -g @mermaid-js/mermaid-cli)"
                );
            }
        }
        Err(e) => {
            eprintln!("Error executing mmdc command: {}", e);
            eprintln!(
                "Make sure you have mermaid-cli installed (npm install -g @mermaid-js/mermaid-cli)"
            );
        }
    }

    Ok(())
}
