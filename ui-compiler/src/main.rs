mod codegen;
mod css;
mod parser;

use clap::Parser;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(help = "Path to the root .vue template file")]
    input: PathBuf,

    #[arg(
        short,
        long,
        help = "Path to the output structure (usually generated_ui/app.rs)"
    )]
    output: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let mut visited = HashSet::new();
    compile_recursive(&args.input, &args.output, &mut visited)?;

    Ok(())
}

fn compile_recursive(
    input_path: &Path,
    output_path: &Path,
    visited: &mut HashSet<PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    if visited.contains(input_path) {
        return Ok(());
    }
    visited.insert(input_path.to_path_buf());

    println!(
        "Compiling {} -> {}",
        input_path.display(),
        output_path.display()
    );

    let content = fs::read_to_string(input_path)
        .map_err(|e| format!("Failed to read {}: {}", input_path.display(), e))?;

    let (_, template) = parser::parse_template(&content)
        .map_err(|e| format!("Failed to parse template {}: {:?}", input_path.display(), e))?;

    // recursing for imports
    let input_dir = input_path.parent().unwrap_or(Path::new("."));
    let output_dir = output_path.parent().unwrap_or(Path::new("."));

    // Ensure output directory exists (for recursive calls)
    fs::create_dir_all(output_dir)?;

    let file_stem = output_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("app");

    for import in &template.imports {
        if import.is_component {
            // "Header" -> "Header.vue"
            let child_input = input_dir.join(format!("{}.vue", import.module_name));

            // Rust modules inside "app.rs" look for files in "app/".
            let child_output_dir = output_dir.join(file_stem);
            fs::create_dir_all(&child_output_dir)?;
            let child_output = child_output_dir.join(format!("{}.rs", import.module_name));

            if child_input.exists() {
                compile_recursive(&child_input, &child_output, visited)?;
            } else {
                println!(
                    "Warning: Component {} imports module '{}' but {}.vue not found.",
                    input_path.display(),
                    import.module_name,
                    import.module_name
                );
            }
        }
    }

    let rust_code = codegen::generate_rust(&template);
    fs::write(output_path, &rust_code)?;

    // Run rustfmt
    let format_status = Command::new("rustfmt").arg(output_path).status();

    if let Ok(status) = format_status {
        if !status.success() {
            println!("Warning: rustfmt failed for {}", output_path.display());
        }
    } else {
        println!("Warning: rustfmt not found. generated code might be messy.");
    }

    Ok(())
}
