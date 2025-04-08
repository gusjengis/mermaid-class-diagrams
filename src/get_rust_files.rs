use std::fs;
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
