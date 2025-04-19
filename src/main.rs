mod capabilities;
mod class_diagram_model;
mod drop;
mod file_util;
mod find_relationships;
mod init_classes;
mod lsp_servers;
mod mermaid_diagram;
mod settings;

use drop::drop;
use lsp_servers::start_lsp_servers;

use crate::file_util::{get_rust_files, parse_rust_file};
use crate::find_relationships::find_relationships;
use crate::init_classes::init_classes;
use crate::mermaid_diagram::MermaidDiagram;
use crate::settings::Settings;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs;
use std::process::Command;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let settings = Settings::defaults();
    // Get command-line arguments
    let args: Vec<String> = env::args().collect();

    // Parse arguments
    let mut dir_to_scan = "src";
    let mut _generate_png = true;

    // Simple argument parsing
    for i in 1..args.len() {
        if args[i] == "--png" {
            _generate_png = false;
        } else if !args[i].starts_with("--") {
            dir_to_scan = &args[i];
        }
    }

    let mut servers = start_lsp_servers(dir_to_scan).await;

    let rust_files = get_rust_files(dir_to_scan);
    let mut diagram = String::from("classDiagram\n\n");
    let mut class_names = vec![];
    let mut classes = vec![];
    for file_path in &rust_files {
        let syntax = parse_rust_file(file_path)?;
        let (a, b) = init_classes(&syntax);
        class_names.extend_from_slice(a.as_slice());
        classes.extend_from_slice(b.as_slice());
    }

    let mut class_map = HashMap::<&str, usize>::new();
    for i in 0..class_names.len() {
        class_map.insert(class_names[i].as_str(), i);
    }

    for file_path in &rust_files {
        let syntax = parse_rust_file(file_path)?;
        find_relationships(&syntax, &class_map, &mut classes);
    }

    // construct output
    for class in classes {
        diagram.push_str(class.to_diagram_syntax(&settings.diagram_settings).as_str());
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
        .arg("--scale")
        .arg(settings.image_settings.scale.to_string().as_str())
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

    for server in &mut servers {
        drop(server).await;
    }

    Ok(())
}
