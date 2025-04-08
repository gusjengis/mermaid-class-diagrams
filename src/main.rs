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

fn main() -> Result<(), Box<dyn Error>> {
    // Get command-line arguments and use the first argument as the directory.
    let args: Vec<String> = env::args().collect();
    // Use the provided directory or default to "src"
    let dir_to_scan = if args.len() > 1 { &args[1] } else { "src" };
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

    Ok(())
}
