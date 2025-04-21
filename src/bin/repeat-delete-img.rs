use std::collections::HashSet;
use std::fs;
use std::io::Read;
use std::path::Path;
use walkdir::WalkDir;

fn main() {
    let root_path = "C:\\Users\\carlo\\Downloads\\New folder";
    remove_global_duplicate_images(root_path).unwrap();
}

fn remove_global_duplicate_images(root: &str) -> std::io::Result<()> {
    let mut hashes: HashSet<String> = HashSet::new();

    for entry in WalkDir::new(root)
        .min_depth(1)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.path().is_file() && is_image(e.path()))
    {
        let file_path = entry.path();

        if let Ok(hash) = hash_file(file_path) {
            if !hashes.insert(hash) {
                println!("🗑️ Eliminando duplicado global: {}", file_path.display());
                fs::remove_file(file_path)?; // o mover al basurero
            }
        }
    }

    Ok(())
}

fn hash_file<P: AsRef<Path>>(path: P) -> std::io::Result<String> {
    let mut file = fs::File::open(&path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(format!("{:x}", md5::compute(buffer)))
}

fn is_image(path: &Path) -> bool {
    if let Some(ext) = path.extension() {
        matches!(
            ext.to_str().unwrap_or("").to_lowercase().as_str(),
            "jpg" | "jpeg" | "png" | "gif" | "bmp" | "webp"
        )
    } else {
        false
    }
}
