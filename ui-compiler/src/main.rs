mod parser;
mod codegen;
mod css;

use clap::Parser;
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(help = "Path to the .vue template file")]
    input: PathBuf,

    #[arg(short, long, help = "Path to the output .rs file")]
    output: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let content = fs::read_to_string(&args.input)?;
    
    let (_, mut template) = parser::parse_template(&content)
        .map_err(|e| format!("Failed to parse template: {:?}", e))?;

    // Parse CSS if any
    for style_content in &template.styles {
        if let Ok((_, rules)) = css::parse_css(style_content) {
            // In a real compiler, we'd apply these rules to the AST elements.
            // For now, let's just print them or skip.
            println!("Found {} CSS rules", rules.len());
        }
    }

    let rust_code = codegen::generate_rust(&template);

    fs::write(&args.output, rust_code)?;

    println!("Successfully compiled {} to {}", args.input.display(), args.output.display());

    Ok(())
}
