use std::fs;
// use std::path::{Path, PathBuf};
// use walkdir::WalkDir;
use std::collections::HashMap;

fn main() {
    let root_path = ""; 
    rename_images_in_subfolders(root_path).unwrap();
}

fn rename_images_in_subfolders(root: &str) -> std::io::Result<()> {
    let mut counters: HashMap<String, u32> = HashMap::new();

    for entry in fs::read_dir(root)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            let subfolder_name = path.file_name().unwrap().to_string_lossy().to_string();

            for file in fs::read_dir(&path)? {
                let file = file?;
                let file_path = file.path();

                if file_path.is_file() {
                    if let Some(ext) = file_path.extension().and_then(|e| e.to_str()) {
                        let ext_upper = ext.to_uppercase();
                        let counter = counters.entry(subfolder_name.clone()).or_insert(1);

                        let new_filename = format!(
                            // "{}_img-{}{}.{}",
                            "{} {}{}.{}",
                            subfolder_name,
                            ext_upper,
                            counter,
                            ext
                        );

                        let new_path = file_path.with_file_name(new_filename);
                        fs::rename(&file_path, &new_path)?;
                        *counter += 1;
                    }
                }
            }
        }
    }

    Ok(())
}
