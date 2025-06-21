use std::fs;
use std::path::Path;

fn main() {
    let out_file = Path::new("static/gen/main.css");
    if !out_file.exists() {
        fs::create_dir_all(out_file.parent().unwrap()).unwrap();
        fs::write(&out_file, "").unwrap();
    }

    // Re-run build script if file is missing
    println!("cargo:rerun-if-changed=build.rs");
}