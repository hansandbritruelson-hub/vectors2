use std::env;
use std::fs;
use std::path::Path;
use std::io::Write;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("generated_assets.rs");
    let mut f = fs::File::create(&dest_path).unwrap();

    let assets_dir = Path::new("assets");
    
    // Check if assets dir exists (it should, but just in case)
    if !assets_dir.exists() {
         writeln!(f, "pub fn get_asset(_path: &str) -> Option<&'static [u8]> {{ None }}").unwrap();
         return;
    }

    writeln!(f, "pub fn get_asset(path: &str) -> Option<&'static [u8]> {{").unwrap();
    writeln!(f, "    match path {{").unwrap();

    for entry in fs::read_dir(assets_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            let file_name = path.file_name().unwrap().to_str().unwrap();
            // Use forward slashes for keys to be platform independent in the app logic
            // But include_bytes needs a path relative to the crate root or absolute?
            // include_bytes! is relative to the file it's in. 
            // So if we put this in OUT_DIR, we need to be careful.
            // Actually, include_bytes! macro resolves relative to the file. 
            // BUT generated_assets.rs is in OUT_DIR. 
            // So include_bytes!("assets/foo.svg") might not work if it thinks it's strictly relative.
            // However, we can use absolute paths or paths from CARGO_MANIFEST_DIR.
            
            let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
            let full_path = Path::new(&manifest_dir).join("assets").join(file_name);
            let full_path_str = full_path.to_str().unwrap().replace("\\", "/");

            writeln!(f, "        \"{}\" => Some(include_bytes!(\"{}\")),", file_name, full_path_str).unwrap();
        }
    }

    writeln!(f, "        _ => None,").unwrap();
    writeln!(f, "    }}").unwrap();
    writeln!(f, "}}").unwrap();
    
    // Rerun if assets change
    println!("cargo:rerun-if-changed=assets");

    // --- UI Compiler Automation ---
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let root_dir = Path::new(&manifest_dir).parent().unwrap();
    let template_path = root_dir.join("templates").join("App.vue");
    let output_path = Path::new(&manifest_dir).join("src").join("generated_ui").join("app.rs");
    
    // Check if ui-compiler exists and is built. 
    // Usually we'd want to build it if it's not there, but for a build.rs, 
    // we can just try to run it.
    let compiler_path = root_dir.join("target").join("debug").join("ui-compiler");
    
    if template_path.exists() && compiler_path.exists() {
        let status = std::process::Command::new(compiler_path)
            .arg(&template_path)
            .arg("-o")
            .arg(&output_path)
            .status()
            .expect("Failed to execute ui-compiler");
        
        if status.success() {
            println!("cargo:warning=Successfully recompiled UI template: {}", template_path.display());
        } else {
            println!("cargo:warning=UI Compiler failed for template: {}", template_path.display());
        }
    }

    println!("cargo:rerun-if-changed={}", template_path.display());
}
