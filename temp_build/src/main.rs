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
    let cli = Cli::parse();

    match &cli.command {
        Commands::ConvertIgnore { format, source, destination } => {
            println!("Converting from {:?} to {:?}", source, destination);
            
            // Realizar la conversión
            let result = match format.to_lowercase().as_str() {
                "git" => {
                    let default_path = PathBuf::from(".ignore");
                    let dest_path = destination.as_ref().unwrap_or(&default_path);
                    DotIgnore::convert_file(source, Some(dest_path))
                },
                "svn" => {
                    let default_path = PathBuf::from(".ignore");
                    let dest_path = destination.as_ref().unwrap_or(&default_path);
                    DotIgnore::convert_file(source, Some(dest_path))
                },
                _ => {
                    eprintln!("Unsupported format: {}", format);
                    process::exit(1);
                }
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
        },
        Commands::Info => {
            println!("DotIgnore - A unified ignore file format");
            println!("Version: {}", env!("CARGO_PKG_VERSION"));
            println!("\nSupported formats:");
            println!("  - git (.gitignore)");
            println!("  - svn (.svnignore)");
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
