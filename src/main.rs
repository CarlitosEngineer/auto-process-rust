use std::fs;
use std::collections::HashMap;
use std::io;

// Lista de extensiones de video válidas
const VIDEO_EXTENSIONS: [&str; 5] = ["mp4", "mkv", "avi", "mov", "webm"];

// D:\\vid-anim\\packa01

fn main() -> io::Result<()> {
    let root_path = "D:\\vid-anim\\packa530"; // Ruta raíz
    rename_videos_in_subfolders(root_path)
}

// Función que recorre subcarpetas y renombra archivos de video
fn rename_videos_in_subfolders(root: &str) -> io::Result<()> {
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
                    // Verifica si la extensión es de video
                    if let Some(ext) = file_path.extension().and_then(|e| e.to_str()) {
                        let ext_lower = ext.to_lowercase();
                        if VIDEO_EXTENSIONS.contains(&ext_lower.as_str()) {
                            let counter = counters.entry(subfolder_name.clone()).or_insert(1);

                            // Formato: <Subcarpeta>_VID-001.<ext>
                            let new_filename = format!(
                                // "{}_VID-{:03}.{}",
                                "{}_{}.{}",
                                subfolder_name,
                                counter,
                                ext_lower
                            );

                            let new_path = file_path.with_file_name(new_filename);
                            fs::rename(&file_path, &new_path)?;
                            *counter += 1;
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
