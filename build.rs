use std::{fs, io};
use std::path::Path;

fn main() -> io::Result<()> {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let target_dir = Path::new(&out_dir).ancestors().nth(3).unwrap();

    let source = Path::new("assets");
    let destination = target_dir.join("debug").join("assets");

    if let Err(e) = fs::create_dir_all(&destination) {
        eprintln!("Failed to create destination directory: {:?}", e);
    }

    if let Err(e) = copy_dir_all(source, &destination) {
        eprintln!("Failed to copy assets: {:?}", e);
    }

    println!("cargo:rerun-if-changed=assets");

    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_icon("hso.ico");
        res.compile()?;
    }
    Ok(())
}

fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(&entry.path(), &dst.join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.join(entry.file_name()))?;
        }
    }
    Ok(())
}