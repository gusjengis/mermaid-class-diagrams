use std::{fs, path::Path};
use syn::{File, Item};

pub fn parse_rust_file(file_path: &str) -> Result<File, Box<dyn std::error::Error>> {
    let code = fs::read_to_string(file_path)?;
    let syntax = syn::parse_file(&code)?;
    Ok(syntax)
}
