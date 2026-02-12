fn main() {
    // No asset generation or UI compilation here anymore.
    // Those tasks are moved to the 'app' crate.
    println!("cargo:rerun-if-changed=build.rs");
}
