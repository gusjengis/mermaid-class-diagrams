mod get_rust_files;
mod parse_rust_file;
mod process_file;

use crate::get_rust_files::*;
use crate::parse_rust_file::*;
use crate::process_file::*;
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

    for file_path in rust_files {
        let syntax = parse_rust_file(&file_path)?;
        process_file(&syntax, &mut diagram);
    }

    // Write the output to a file, e.g. diagram.mmd
    fs::write("diagram.mmd", diagram)?;
    println!("Mermaid diagram generated: diagram.mmd");

    Ok(())
}
