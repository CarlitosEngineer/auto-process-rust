use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{BufReader, Read};
use std::path::Path;
use walkdir::WalkDir;

const VIDEO_EXTENSIONS: [&str; 7] = ["mp4", "avi", "mov", "mkv", "flv", "wmv", "webm"];
const BLOCK_SIZE: usize = 65536; // 64KB

fn calcular_hash_video(ruta_archivo: &Path) -> Option<String> {
    let file = File::open(ruta_archivo);
    let mut file = match file {
        Ok(f) => f,
        Err(e) => {
            eprintln!(
                "❗ Error al abrir archivo {}: {}",
                ruta_archivo.display(),
                e
            );
            return None;
        }
    };

    let mut reader = BufReader::new(file);
    let mut buffer = [0u8; BLOCK_SIZE];
    let mut contexto = md5::Context::new();

    loop {
        let bytes_read = match reader.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => n,
            Err(e) => {
                eprintln!("❗ Error al leer archivo {}: {}", ruta_archivo.display(), e);
                return None;
            }
        };
        contexto.consume(&buffer[..bytes_read]);
    }

    let digest = contexto.compute();
    Some(format!("{:x}", digest))
}

fn es_extension_valida(path: &Path) -> bool {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some(ext) => VIDEO_EXTENSIONS
            .iter()
            .any(|&valid_ext| ext.eq_ignore_ascii_case(valid_ext)),
        None => false,
    }
}

fn buscar_y_eliminar_duplicados(ruta_carpeta: &str) {
    let mut hashes: HashMap<String, String> = HashMap::new();

    for entry in WalkDir::new(ruta_carpeta)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();

        if es_extension_valida(path) {
            println!("📂 Procesando: {}", path.display());

            if let Some(hash) = calcular_hash_video(path) {
                if let Some(original) = hashes.get(&hash) {
                    println!(
                        "⚠️ Duplicado detectado: \n    {}\n    duplicado de\n    {}",
                        path.display(),
                        original
                    );
                    match fs::remove_file(path) {
                        Ok(_) => println!("✅ Se eliminó duplicado: {}", path.display()),
                        Err(e) => eprintln!("❗ Error al eliminar {}: {}", path.display(), e),
                    }
                } else {
                    hashes.insert(hash, path.display().to_string());
                }
            }
        }
    }

    println!("🏁 Análisis completado.");
}

fn main() {
    use std::io::{self, Write};

    print!("Introduce la ruta de la carpeta: ");
    io::stdout().flush().unwrap();

    let mut ruta = String::new();
    io::stdin().read_line(&mut ruta).unwrap();
    let ruta = ruta.trim();

    buscar_y_eliminar_duplicados(ruta);
}

// cargo build --release --bin repeat-delete-vid
// cargo run --release --bin repeat-delete-vid
