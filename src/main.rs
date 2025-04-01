// DotIgnore CLI
//
// Command-line interface for DotIgnore system

use anyhow::anyhow;
use clap::{Parser, Subcommand, ArgAction, Command, arg};
use std::path::PathBuf;
use std::path::Path;
use std::process;
use std::fs;
use ignore::DotIgnore;

#[derive(Parser)]
#[command(name = "ignore")]
#[command(about = "DotIgnore - A unified ignore file format for all version control systems", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Convert ignore files from other VCS formats to .ignore
    #[command(name = "convert-ignore")]
    ConvertIgnore {
        /// Source format (git, svn)
        #[arg(short = 'f', long = "format")]
        format: String,

        /// Source file path
        #[arg(short = 's', long = "source")]
        source: PathBuf,

        /// Destination file path (optional)
        #[arg(short = 'd', long = "destination")]
        destination: Option<PathBuf>,
    },

    /// Show information about DotIgnore
    #[command(name = "info")]
    Info,
}

fn main() {
    let matches = Command::new("ignore")
        .version(env!("CARGO_PKG_VERSION"))
        .about("DotIgnore - A unified ignore file format for all version control systems")
        .arg(arg!(-i --input <FILE> "Input .gitignore or .svnignore file"))
        .arg(arg!(-o --output <FILE> "Output .ignore file"))
        .arg(arg!(-c --convert "Convert .ignore to other formats").action(ArgAction::SetTrue))
        .arg(arg!(-f --format <FORMAT> "Target format (git, svn)"))
        .arg(arg!(-v --validate "Validate a .ignore file").action(ArgAction::SetTrue))
        .arg(arg!(-n --create "Create a new .ignore file").action(ArgAction::SetTrue))
        .get_matches();

    // Crear un nuevo archivo .ignore
    if matches.get_flag("create") {
        let output_path = matches.get_one::<String>("output")
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from(".ignore"));
        
        println!("Creating new .ignore file at {:?}", output_path);
        // Usando example_content en lugar de create_default_file
        match fs::write(&output_path, DotIgnore::example_content()) {
            Ok(_) => println!("Successfully created default .ignore file"),
            Err(e) => {
                eprintln!("Error creating default .ignore: {}", e);
                process::exit(1);
            }
        }
        return;
    }

    // Validar un archivo .ignore existente
    if matches.get_flag("validate") {
        let input_path = matches.get_one::<String>("input")
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from(".ignore"));
            
        println!("Validating .ignore file at {:?}", input_path);
        
        match DotIgnore::load_from_file(&input_path) {
            Ok(_) => {
                println!("The .ignore file is valid!");
                return;
            },
            Err(e) => {
                eprintln!("Error validating .ignore: {}", e);
                process::exit(1);
            }
        }
    }

    // Convertir entre formatos
    let input_path = matches.get_one::<String>("input").map(PathBuf::from);
    let output_path = matches.get_one::<String>("output").map(PathBuf::from);
    
    // Determinar las rutas de origen y destino
    let (src_path, dest_path) = if matches.get_flag("convert") {
        // Modo de conversión de .ignore a otros formatos
        let src = input_path.unwrap_or_else(|| PathBuf::from(".ignore"));
        let dest = output_path.unwrap_or_else(|| {
            // Determinar formato destino
            let default_format = String::from("git");
            let format = matches.get_one::<String>("format").unwrap_or(&default_format);
            match format.to_lowercase().as_str() {
                "git" => PathBuf::from(".gitignore"),
                "svn" => PathBuf::from(".svnignore"),
                _ => {
                    eprintln!("Unsupported target format: {}", format);
                    process::exit(1);
                }
            }
        });
        (src, dest)
    } else {
        // Modo de conversión a .ignore
        let src = if let Some(path) = input_path {
            path
        } else {
            // Auto-detectar formatos comunes
            if Path::new(".gitignore").exists() {
                PathBuf::from(".gitignore")
            } else if Path::new(".svnignore").exists() {
                PathBuf::from(".svnignore")
            } else {
                eprintln!("No input file specified and couldn't find .gitignore or .svnignore");
                process::exit(1);
            }
        };
        
        let dest = output_path.unwrap_or_else(|| PathBuf::from(".ignore"));
        (src, dest)
    };
    
    println!("Converting from {:?} to {:?}", src_path, dest_path);
    
    // Realizar la conversión
    let result = if matches.get_flag("convert") {
        // Usar la función convert_file para cualquier conversión
        DotIgnore::convert_file(&src_path, Some(&dest_path))
    } else {
        // Convertir a .ignore
        DotIgnore::convert_file(&src_path, Some(&dest_path))
    };
    
    match result {
        Ok(result) => {
            println!("Conversion successful!");
            println!("Converted patterns: {}", result.pattern_count);
        },
        Err(e) => {
            eprintln!("Error during conversion: {}", e);
            process::exit(1);
        }
    }
}

fn check_file(file_path: &str) {
    let path = Path::new(file_path);
    
    if !path.exists() {
        println!("El archivo no existe: {}", file_path);
        return;
    }
    
    // Intentar cargar el archivo .DotIgnore
    match DotIgnore::load_from_file(path) {
        Ok(ignore) => {
            println!("Archivo válido: {}", file_path);
            // No podemos acceder a los grupos directamente ya que son privados
            println!("Archivo .DotIgnore válido");
            
            // Comprobar si algunos archivos comunes serían ignorados
            let test_files = [
                "file.txt", "build/output.log", "node_modules/package.json",
                "tmp/cache.dat", ".DS_Store", "Thumbs.db"
            ];
            
            println!("Comprobando patrones con archivos de ejemplo:");
            for test_file in test_files {
                let status = if ignore.is_ignored(test_file) { "ignorado" } else { "no ignorado" };
                println!("  - {}: {}", test_file, status);
            }
        },
        Err(err) => {
            println!("Error al cargar el archivo: {}", err);
        }
    }
}

fn convert_file(file_path: &str) {
    let path = Path::new(file_path);
    
    if !path.exists() {
        println!("El archivo no existe: {}", file_path);
        process::exit(1);
    }
    
    match DotIgnore::convert_file(path, None::<&Path>) {
        Ok(result) => {
            println!("Conversión exitosa:");
            println!("  Archivo fuente: {}", result.source_file.display());
            println!("  Archivo destino: {}", result.destination_file.display());
            println!("  Patrones convertidos: {}", result.pattern_count);
        }
        Err(e) => {
            println!("Error al convertir archivo: {}", e);
            process::exit(1);
        }
    }
}

fn find_ignore_file(start_path: &Path) -> Option<std::path::PathBuf> {
    let mut current_dir = if start_path.is_dir() {
        start_path.to_path_buf()
    } else {
        start_path.parent()?.to_path_buf()
    };
    
    loop {
        let ignore_path = current_dir.join(".DotIgnore");
        
        if ignore_path.exists() {
            return Some(ignore_path);
        }
        
        match current_dir.parent() {
            Some(parent) => current_dir = parent.to_path_buf(),
            None => break,
        }
    }
    
    None
} 
