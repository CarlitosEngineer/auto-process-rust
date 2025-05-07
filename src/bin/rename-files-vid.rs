use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

const VIDEO_EXTENSIONS: [&str; 7] = ["mp4", "avi", "mov", "mkv", "flv", "wmv", "webm"];

fn es_extension_valida(path: &Path) -> Option<String> {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext_str| ext_str.to_ascii_lowercase())
        .filter(|ext_str| VIDEO_EXTENSIONS.contains(&ext_str.as_str()))
}

fn construir_nuevo_nombre(
    nombre_carpeta: &str,
    extension_mayus: &str,
    contador: usize,
    extension_original: &str,
) -> String {
    format!(
        "{}_{}_({}).{}",
        nombre_carpeta, extension_mayus, contador, extension_original
    )
}

fn main() {
    use std::io::{self, Write};

    print!("Introduce la ruta de la carpeta: ");
    io::stdout().flush().unwrap();

    let mut ruta = String::new();
    io::stdin().read_line(&mut ruta).unwrap();
    let ruta = ruta.trim();

    let mut contador_por_carpeta_extension: HashMap<(String, String), usize> = HashMap::new();

    for entry in WalkDir::new(ruta)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();

        if let Some(ext) = es_extension_valida(path) {
            let extension_mayus = ext.to_ascii_uppercase();
            let extension_original = ext; // en minúscula

            let carpeta = path
                .parent()
                .and_then(|p| p.file_name())
                .and_then(|n| n.to_str())
                .unwrap_or("SIN_NOMBRE")
                .to_string();

            let key = (carpeta.clone(), extension_mayus.clone());
            let contador = contador_por_carpeta_extension
                .entry(key)
                .and_modify(|c| *c += 1)
                .or_insert(1);
            let contador_actual = *contador;

            let nuevo_nombre = construir_nuevo_nombre(
                &carpeta,
                &extension_mayus,
                contador_actual,
                &extension_original,
            );

            let nuevo_path = path.with_file_name(&nuevo_nombre);

            match fs::rename(&path, &nuevo_path) {
                Ok(_) => println!(
                    "✅ Renombrado:\n    {}  →  {}",
                    path.display(),
                    nuevo_path.display()
                ),
                Err(e) => eprintln!("❗ Error al renombrar {}: {}", path.display(), e),
            }
        }
    }

    println!("🏁 Renombrado completado.");
}

// cargo build --release --bin rename-files-vid
// cargo run --release --bin rename-files-vid
