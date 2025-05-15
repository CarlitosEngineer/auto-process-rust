use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{BufReader, Read};
use std::path::Path;
use walkdir::WalkDir;

const IMAGE_EXTENSIONS: [&str; 7] = ["jpg", "jpeg", "png", "bmp", "gif", "tiff", "webp"];
const BLOCK_SIZE: usize = 65536; // 64KB

fn es_extension_valida(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext_str| IMAGE_EXTENSIONS.contains(&ext_str.to_ascii_lowercase().as_str()))
        .unwrap_or(false)
}

fn calcular_hash_imagen(ruta_archivo: &Path) -> Option<String> {
    let file = File::open(ruta_archivo).ok()?;
    let mut reader = BufReader::new(file);
    let mut buffer = [0u8; BLOCK_SIZE];
    let mut contexto = md5::Context::new();

    loop {
        let bytes_read = match reader.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => n,
            Err(e) => {
                eprintln!("❗ Error al leer {}: {}", ruta_archivo.display(), e);
                return None;
            }
        };
        contexto.consume(&buffer[..bytes_read]);
    }

    let digest = contexto.compute();
    Some(format!("{:x}", digest))
}

fn main() {
    use std::io::{self, Write};

    print!("Introduce la ruta de la carpeta: ");
    io::stdout().flush().unwrap();

    let mut ruta = String::new();
    io::stdin().read_line(&mut ruta).unwrap();
    let ruta = ruta.trim();

    let mut hashes: HashMap<String, String> = HashMap::new();

    for entry in WalkDir::new(ruta)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();

        if es_extension_valida(path) {
            println!("📂 Procesando: {}", path.display());

            if let Some(hash) = calcular_hash_imagen(path) {
                if let Some(original) = hashes.get(&hash) {
                    println!(
                        "⚠️ Imagen duplicada encontrada:\n    {}\n    duplicado de\n    {}",
                        path.display(),
                        original
                    );
                    match fs::remove_file(path) {
                        Ok(_) => println!("✅ Eliminado duplicado: {}", path.display()),
                        Err(e) => eprintln!("❗ Error al eliminar {}: {}", path.display(), e),
                    }
                } else {
                    hashes.insert(hash, path.display().to_string());
                }
            }
        }
    }

    println!("🏁 Proceso completado.");
}

/*

cargo build --release --bin repeat-delete-img

cargo run --release --bin repeat-delete-img

*/
