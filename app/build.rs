use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::io::Write;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("generated_assets.rs");
    let mut f = fs::File::create(&dest_path).unwrap();

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let root_dir = Path::new(&manifest_dir).parent().unwrap();
    let assets_dir = root_dir.join("assets");
    
    let mut asset_keys = Vec::new();

    // Check if assets dir exists
    if !assets_dir.exists() {
         writeln!(f, "pub fn get_asset(_path: &str) -> Option<&'static [u8]> {{ None }}").unwrap();
         writeln!(f, "pub const ASSET_KEYS: &[&str] = &[];").unwrap();
    } else {
        writeln!(f, "pub fn get_asset(path: &str) -> Option<&'static [u8]> {{").unwrap();
        writeln!(f, "    match path {{").unwrap();

        visit_dirs(&assets_dir, &assets_dir, &mut f, &mut asset_keys);

        writeln!(f, "        _ => None,").unwrap();
        writeln!(f, "    }}").unwrap();
        writeln!(f, "}}").unwrap();
        
        writeln!(f, "pub const ASSET_KEYS: &[&str] = &[").unwrap();
        for key in asset_keys {
            writeln!(f, "    \"{}\",", key).unwrap();
        }
        writeln!(f, "];").unwrap();

        // Rerun if assets change
        println!("cargo:rerun-if-changed={}", assets_dir.display());
    }

    // --- UI Compiler Automation ---
    let template_path = root_dir.join("templates").join("App.vue");
    let output_path = Path::new(&manifest_dir).join("src").join("generated_ui").join("app.rs");
    
    // Check if ui-compiler exists and is built. 
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

    // Watch all .vue files in templates dir
    let templates_dir = root_dir.join("templates");
    if let Ok(entries) = fs::read_dir(templates_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if let Some(ext) = path.extension() {
                    if ext == "vue" {
                        println!("cargo:rerun-if-changed={}", path.display());
                    }
                }
            }
        }
    }
}

fn visit_dirs(dir: &Path, base_dir: &Path, f: &mut fs::File, keys: &mut Vec<String>) {
    if dir.is_dir() {
        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, base_dir, f, keys);
            } else {
                let rel_path = path.strip_prefix(base_dir).unwrap();
                let rel_path_str = rel_path.to_str().unwrap().replace("\\", "/");
                let full_path_str = path.to_str().unwrap().replace("\\", "/");
                writeln!(f, "        \"{}\" => Some(include_bytes!(\"{}\")),", rel_path_str, full_path_str).unwrap();
                keys.push(rel_path_str);
            }
        }
    }
}
