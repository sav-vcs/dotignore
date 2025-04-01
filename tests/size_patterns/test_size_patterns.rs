#[cfg(test)]
mod size_pattern_tests {
    use crate::ignore::Pattern;
    use std::fs::{self, File};
    use std::io::Write;
    use std::path::Path;
    use tempfile::tempdir;

    #[test]
    fn test_size_pattern_parsing() {
        // Patrón con tamaño menor que 1MB
        let pattern = Pattern::new("size:<1MB *.log");
        assert!(pattern.is_size_condition());
        
        // Patrón con tamaño mayor que 100KB
        let pattern = Pattern::new("size:>100KB temp.*");
        assert!(pattern.is_size_condition());
        
        // Patrón normal sin condición de tamaño
        let pattern = Pattern::new("*.tmp");
        assert!(!pattern.is_size_condition());
    }

    #[test]
    fn test_size_comparison_less_than() {
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("small.tmp");
        let large_file_path = temp_dir.path().join("large.tmp");
        
        // Crear un archivo pequeño de 10KB
        {
            let mut file = File::create(&file_path).unwrap();
            let data = [0u8; 10 * 1024]; // 10 KB
            file.write_all(&data).unwrap();
        }
        
        // Crear un archivo grande de 2MB
        {
            let mut file = File::create(&large_file_path).unwrap();
            let data = [0u8; 2 * 1024 * 1024]; // 2 MB
            file.write_all(&data).unwrap();
        }
        
        // Probar el patrón "size:<1MB *.tmp"
        let pattern = Pattern::new("size:<1MB *.tmp");
        
        // El archivo pequeño debe coincidir
        assert!(pattern.matches(&file_path.to_string_lossy()));
        
        // El archivo grande no debe coincidir
        assert!(!pattern.matches(&large_file_path.to_string_lossy()));
    }

    #[test]
    fn test_size_comparison_greater_than() {
        let temp_dir = tempdir().unwrap();
        let small_file_path = temp_dir.path().join("small.log");
        let large_file_path = temp_dir.path().join("large.log");
        
        // Crear un archivo pequeño de 50KB
        {
            let mut file = File::create(&small_file_path).unwrap();
            let data = [0u8; 50 * 1024]; // 50 KB
            file.write_all(&data).unwrap();
        }
        
        // Crear un archivo grande de 1.5MB
        {
            let mut file = File::create(&large_file_path).unwrap();
            let data = [0u8; 1536 * 1024]; // 1.5 MB
            file.write_all(&data).unwrap();
        }
        
        // Probar el patrón "size:>1MB *.log"
        let pattern = Pattern::new("size:>1MB *.log");
        
        // El archivo pequeño no debe coincidir
        assert!(!pattern.matches(&small_file_path.to_string_lossy()));
        
        // El archivo grande debe coincidir
        assert!(pattern.matches(&large_file_path.to_string_lossy()));
    }

    #[test]
    fn test_pattern_matching_with_size() {
        let temp_dir = tempdir().unwrap();
        
        // Crear archivos con diferentes extensiones y tamaños
        let txt_small = temp_dir.path().join("doc.txt");
        let txt_large = temp_dir.path().join("large.txt");
        let bin_small = temp_dir.path().join("small.bin");
        let bin_large = temp_dir.path().join("data.bin");
        
        // Crear archivos
        create_file(&txt_small, 10 * 1024); // 10 KB
        create_file(&txt_large, 2 * 1024 * 1024); // 2 MB
        create_file(&bin_small, 50 * 1024); // 50 KB
        create_file(&bin_large, 5 * 1024 * 1024); // 5 MB
        
        // Probar patrones
        // Solo archivos .txt pequeños
        let pattern = Pattern::new("size:<1MB *.txt");
        assert!(pattern.matches(&txt_small.to_string_lossy()));
        assert!(!pattern.matches(&txt_large.to_string_lossy()));
        assert!(!pattern.matches(&bin_small.to_string_lossy())); // Extensión diferente
        assert!(!pattern.matches(&bin_large.to_string_lossy())); // Extensión diferente
        
        // Solo archivos .bin grandes
        let pattern = Pattern::new("size:>1MB *.bin");
        assert!(!pattern.matches(&txt_small.to_string_lossy())); // Extensión diferente
        assert!(!pattern.matches(&txt_large.to_string_lossy())); // Extensión diferente
        assert!(!pattern.matches(&bin_small.to_string_lossy())); // Tamaño pequeño
        assert!(pattern.matches(&bin_large.to_string_lossy()));
    }

    // Función auxiliar para crear un archivo con un tamaño específico
    fn create_file(path: &Path, size: usize) {
        let mut file = File::create(path).unwrap();
        let chunk_size = 4096; // 4 KB
        let mut remaining = size;
        
        let chunk = vec![0u8; chunk_size.min(remaining)];
        
        while remaining > 0 {
            let write_size = chunk_size.min(remaining);
            file.write_all(&chunk[0..write_size]).unwrap();
            remaining -= write_size;
        }
    }
} 
