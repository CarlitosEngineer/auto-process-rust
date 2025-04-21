use std::collections::HashSet;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

fn main() {
    let root_path = "";
    remove_duplicate_videos(root_path).unwrap();
}

fn remove_duplicate_videos(root: &str) -> std::io::Result<()> {
    let valid_extensions = ["mp4", "mkv", "avi", "mov", "webm"];
    let mut hashes: HashSet<String> = HashSet::new();

    for entry in WalkDir::new(root)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.path().is_file())
    {
        let file_path = entry.path();

        if let Some(ext) = file_path.extension().and_then(|e| e.to_str()) {
            if valid_extensions.contains(&ext.to_lowercase().as_str()) {
                if let Ok(hash) = hash_file(file_path) {
                    if !hashes.insert(hash) {
                        println!("🗑️ Eliminando video duplicado: {}", file_path.display());
                        fs::remove_file(file_path)?;
                    }
                }
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
