use std::fs;
use syn::File;
use walkdir::WalkDir;

pub fn get_rust_files(dir: &str) -> Vec<String> {
    WalkDir::new(dir)
        .into_iter()
        .filter_map(|entry| {
            let entry = entry.ok()?;
            if entry.path().extension()?.to_str()? == "rs" {
                Some(entry.path().display().to_string())
            } else {
                None
            }
        })
        .collect()
}

pub fn parse_rust_file(file_path: &str) -> Result<File, Box<dyn std::error::Error>> {
    let code = fs::read_to_string(file_path)?;
    let syntax = syn::parse_file(&code)?;
    Ok(syntax)
}
