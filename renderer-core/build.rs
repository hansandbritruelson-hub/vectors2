use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=style_defs.toml");

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let toml_path = Path::new(&manifest_dir).join("style_defs.toml");
    let out_dir = env::var("OUT_DIR").unwrap();

    let toml_str = fs::read_to_string(&toml_path).expect("Failed to read style_defs.toml");
    let toml_val: toml::Value = toml_str.parse().expect("Failed to parse style_defs.toml");

    // Collect properties, controls, units into sorted maps
    let properties = extract_table(&toml_val, "properties");
    let controls = extract_table(&toml_val, "controls");
    let units = extract_table(&toml_val, "units");

    // --- Generate Rust constants ---
    let rust_path = Path::new(&out_dir).join("style_constants.rs");
    let mut rust_file = fs::File::create(&rust_path).unwrap();
    writeln!(rust_file, "// THIS FILE IS GENERATED — DO NOT EDIT").unwrap();
    writeln!(rust_file, "// Source: renderer-core/style_defs.toml").unwrap();
    writeln!(rust_file, "").unwrap();
    writeln!(rust_file, "#[allow(dead_code)]").unwrap();
    writeln!(rust_file, "").unwrap();

    writeln!(rust_file, "// --- CSS Property IDs ---").unwrap();
    for (name, id) in &properties {
        writeln!(
            rust_file,
            "pub const PROP_{}: u32 = {};",
            name.to_uppercase(),
            id
        )
        .unwrap();
    }

    writeln!(rust_file, "").unwrap();
    writeln!(rust_file, "// --- Control Markers ---").unwrap();
    for (name, id) in &controls {
        writeln!(
            rust_file,
            "pub const CTRL_{}: u32 = {};",
            name.to_uppercase(),
            id
        )
        .unwrap();
    }

    writeln!(rust_file, "").unwrap();
    writeln!(rust_file, "// --- Unit IDs ---").unwrap();
    for (name, id) in &units {
        writeln!(
            rust_file,
            "pub const UNIT_{}: u32 = {};",
            name.to_uppercase(),
            id
        )
        .unwrap();
    }

    // --- Generate WGSL constants ---
    let wgsl_path = Path::new(&out_dir).join("style_constants.wgsl");
    let mut wgsl_file = fs::File::create(&wgsl_path).unwrap();
    writeln!(wgsl_file, "// THIS FILE IS GENERATED — DO NOT EDIT").unwrap();
    writeln!(wgsl_file, "// Source: renderer-core/style_defs.toml").unwrap();
    writeln!(wgsl_file, "").unwrap();

    writeln!(wgsl_file, "// --- CSS Property IDs ---").unwrap();
    for (name, id) in &properties {
        writeln!(
            wgsl_file,
            "const PROP_{}: u32 = {}u;",
            name.to_uppercase(),
            id
        )
        .unwrap();
    }

    writeln!(wgsl_file, "").unwrap();
    writeln!(wgsl_file, "// --- Control Markers ---").unwrap();
    for (name, id) in &controls {
        writeln!(
            wgsl_file,
            "const CTRL_{}: u32 = {}u;",
            name.to_uppercase(),
            id
        )
        .unwrap();
    }

    writeln!(wgsl_file, "").unwrap();
    writeln!(wgsl_file, "// --- Unit IDs ---").unwrap();
    for (name, id) in &units {
        writeln!(
            wgsl_file,
            "const UNIT_{}: u32 = {}u;",
            name.to_uppercase(),
            id
        )
        .unwrap();
    }
}

fn extract_table(val: &toml::Value, section: &str) -> BTreeMap<String, u64> {
    let mut result = BTreeMap::new();
    if let Some(table) = val.get(section).and_then(|v| v.as_table()) {
        for (key, value) in table {
            if let Some(id) = value.as_integer() {
                result.insert(key.clone(), id as u64);
            }
        }
    }
    result
}
