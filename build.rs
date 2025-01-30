use std::fs;
use std::path::Path;

fn main() {
    // Get the output directory from cargo
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let target_dir = Path::new(&out_dir).join("../../..");
    
    // Ensure the assets directory exists in the target directory
    let target_assets = target_dir.join("assets");
    if !target_assets.exists() {
        fs::create_dir_all(&target_assets).unwrap();
    }

    // Copy assets
    let source_assets = Path::new("assets");
    if source_assets.exists() {
        copy_dir_recursively(source_assets, &target_assets).unwrap();
        println!("cargo:rerun-if-changed=assets");
    }
}

fn copy_dir_recursively(src: &Path, dst: &Path) -> std::io::Result<()> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if file_type.is_dir() {
            copy_dir_recursively(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }

    Ok(())
}