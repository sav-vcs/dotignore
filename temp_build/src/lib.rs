// DotIgnore Library
//
// Main library file for DotIgnore format support and plugin interface

pub mod ignore;
pub mod converter;

// Re-export key items from modules for standard usage
pub use ignore::{DotIgnore, IgnoreConverter, ConversionResult, IgnoreParser, Pattern, PatternGroup};

// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

// Library information
pub fn library_info() -> String {
    format!(
        "DotIgnore v{}\nImplemented in Rust\nSupports Git, SVN, and Dot ignore formats",
        VERSION
    )
}

// Definiciones de errores comunes
#[derive(Debug, thiserror::Error)]
pub enum IgnoreError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Parsing error at line {line}: {message}")]
    Parse { line: usize, message: String },
    
    #[error("Unsupported format: {0}")]
    UnsupportedFormat(String),
    
    #[error("File not found: {0}")]
    FileNotFound(std::path::PathBuf),
    
    #[error("Other error: {0}")]
    Other(#[from] anyhow::Error),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IgnoreFormat {
    Git,
    Svn,
    Vcs,
}

// Plugin support (conditionally compiled)
#[cfg(feature = "plugin")]
pub mod plugin {
    use std::path::{Path, PathBuf};
    use std::collections::HashMap;
    use crate::ignore::{DotIgnore, Pattern};
    use crate::IgnoreError;

    // Data structure to hold ignore rules
    #[derive(Debug, Clone, Default)]
    pub struct IgnoreRules {
        // Core patterns
        pub patterns: Vec<Pattern>,
        
        // Group-based patterns
        pub groups: HashMap<String, Vec<Pattern>>,
    }

    impl IgnoreRules {
        // Create new empty rules
        pub fn new() -> Self {
            Self {
                patterns: Vec::new(),
                groups: HashMap::new(),
            }
        }
        
        // Create from DotIgnore instance
        pub fn from_DotIgnore(vcs_ignore: &DotIgnore) -> Self {
            let mut rules = Self::new();
            
            // Add patterns
            for pattern in &vcs_ignore.patterns {
                rules.patterns.push(pattern.clone());
            }
            
            // Add patterns from groups
            for group in &vcs_ignore.groups {
                let group_patterns: Vec<Pattern> = group.patterns.clone();
                rules.groups.insert(group.name.clone(), group_patterns);
            }
            
            rules
        }
    }

    // Plugin interface traits
    pub trait Plugin: Send + Sync {
        fn name(&self) -> &str;
        fn version(&self) -> &str;
        fn description(&self) -> &str;
    }

    // Ignore plugin interface
    pub trait IgnorePlugin: Plugin {
        fn parse_ignore_file(&self, path: &Path) -> Result<IgnoreRules, IgnoreError>;
        fn is_file_ignored(&self, file_path: &Path, rules: &IgnoreRules) -> bool;
        fn convert_ignore_file(&self, from_path: &Path, to_path: &Path, format: &str) -> Result<(), IgnoreError>;
        fn create_default_ignore_file(&self, path: &Path) -> Result<(), IgnoreError>;
    }

    // DotIgnore plugin implementation
    pub struct DotIgnorePlugin {
        name: String,
        version: String,
        description: String,
    }

    impl DotIgnorePlugin {
        pub fn new() -> Self {
            Self {
                name: "dotignore".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
                description: "DotIgnore Plugin".to_string(),
            }
        }
    }

    impl Plugin for DotIgnorePlugin {
        fn name(&self) -> &str {
            &self.name
        }
        
        fn version(&self) -> &str {
            &self.version
        }
        
        fn description(&self) -> &str {
            &self.description
        }
    }

    impl IgnorePlugin for DotIgnorePlugin {
        fn parse_ignore_file(&self, path: &Path) -> Result<IgnoreRules, IgnoreError> {
            if !path.exists() {
                return Err(IgnoreError::FileNotFound(path.to_path_buf()));
            }
            
            let vcs_ignore = DotIgnore::load_from_file(path)
                .map_err(|e| IgnoreError::Other(e))?;
                
            Ok(IgnoreRules::from_DotIgnore(&vcs_ignore))
        }
        
        fn is_file_ignored(&self, file_path: &Path, rules: &IgnoreRules) -> bool {
            // Convertir a string para comparación
            let path_str = file_path.to_string_lossy();
            
            // Verificar si es un directorio
            let is_dir = file_path.is_dir();
            
            // Primero verificar patrones estándar
            for pattern in &rules.patterns {
                // Si el patrón es solo para directorios pero el archivo no es un directorio, saltamos
                if pattern.is_dir_only() && !is_dir {
                    continue;
                }
                
                if pattern.matches(&path_str) {
                    return !pattern.is_negated();
                }
            }
            
            // Luego verificar patrones en grupos
            for (_, patterns) in &rules.groups {
                for pattern in patterns {
                    // Si el patrón es solo para directorios pero el archivo no es un directorio, saltamos
                    if pattern.is_dir_only() && !is_dir {
                        continue;
                    }
                    
                    if pattern.matches(&path_str) {
                        return !pattern.is_negated();
                    }
                }
            }
            
            false
        }
        
        fn convert_ignore_file(&self, from_path: &Path, to_path: &Path, format: &str) -> Result<(), IgnoreError> {
            match format.to_lowercase().as_str() {
                "git" => {
                    // Convertir de .gitignore a .ignore
                    let _ = DotIgnore::convert_file(from_path, Some(to_path))
                        .map_err(|e| IgnoreError::Other(e))?;
                    Ok(())
                },
                "svn" => {
                    // Convertir de .svnignore a .ignore
                    let _ = DotIgnore::convert_file(from_path, Some(to_path))
                        .map_err(|e| IgnoreError::Other(e))?;
                    Ok(())
                },
                "vcs" => {
                    // Si ya es .ignore, simplemente copiar
                    std::fs::copy(from_path, to_path)
                        .map_err(|e| IgnoreError::Io(e))?;
                    Ok(())
                },
                _ => Err(IgnoreError::UnsupportedFormat(format.to_string())),
            }
        }
        
        fn create_default_ignore_file(&self, path: &Path) -> Result<(), IgnoreError> {
            // Crear un archivo .ignore predeterminado
            let default_content = r#"# Default .ignore file automatically created by DotIgnore Plugin

[standard_patterns] {
    # Common files to ignore
    *.log
    *.tmp
    *.bak
    *~
}

[platform:windows] {
    # Windows-specific files
    Thumbs.db
    desktop.ini
    *.lnk
}

[platform:macos] {
    # macOS-specific files
    .DS_Store
    ._*
    .Spotlight-V100
    .Trashes
}

[platform:linux] {
    # Linux-specific files
    .directory
}

[development] {
    # Common development files/directories
    .env
    .env.local
    .env.development
    node_modules/
    vendor/
    .cache/
}

[build] {
    # Build artifacts
    build/
    dist/
    out/
    target/
    *.o
    *.obj
}
"#;
            
            std::fs::write(path, default_content)
                .map_err(|e| IgnoreError::Io(e))?;
            
            Ok(())
        }
    }

    // Create a new plugin instance
    #[no_mangle]
    pub extern "C" fn create_plugin() -> Box<dyn Plugin> {
        Box::new(DotIgnorePlugin::new())
    }

    // Get plugin information
    #[no_mangle]
    pub extern "C" fn plugin_info() -> String {
        format!(
            "DotIgnore Plugin v{}\nImplemented in Rust\nSupports Git, SVN, and Dot ignore formats",
            env!("CARGO_PKG_VERSION")
        )
    }

    // For testing in other contexts
    pub fn get_plugin() -> Box<dyn IgnorePlugin> {
        Box::new(DotIgnorePlugin::new())
    }

    // Export
    pub use self::{Plugin, IgnorePlugin, DotIgnorePlugin, IgnoreRules};

    // Unit tests
    #[cfg(test)]
    mod tests {
        use super::*;
        use std::io::Write;
        use tempfile::tempdir;

        #[test]
        fn test_plugin_info() {
            let info = plugin_info();
            assert!(info.contains("DotIgnore Plugin v"));
            assert!(info.contains("Implemented in Rust"));
        }

        #[test]
        fn test_parse_ignore_file() {
            let temp_dir = tempdir().unwrap();
            let ignore_path = temp_dir.path().join(".ignore");
            
            let content = r#"*.txt
!important.txt

[group1] {
    *.tmp
    *.log
}
"#;
            
            let mut file = std::fs::File::create(&ignore_path).unwrap();
            file.write_all(content.as_bytes()).unwrap();
            
            let plugin = DotIgnorePlugin::new();
            let rules = plugin.parse_ignore_file(&ignore_path).unwrap();
            
            assert_eq!(rules.patterns.len(), 2);
            assert_eq!(rules.groups.len(), 1);
            assert_eq!(rules.groups["group1"].len(), 2);
        }
    }
}

// Re-exportar módulos de plugin cuando está habilitada la feature
#[cfg(feature = "plugin")]
pub use plugin::*;

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;
    use tempfile::tempdir;

    #[test]
    fn test_library_info() {
        let info = library_info();
        assert!(info.contains("DotIgnore v"));
        assert!(info.contains(VERSION));
    }
    
    #[test]
    fn test_basic_ignore_patterns() {
        let temp_dir = tempdir().unwrap();
        let ignore_path = temp_dir.path().join(".ignore");
        
        let content = r#"
[standard_patterns] {
    *.txt
    !important.txt
    temp/
}
"#;
        
        let mut file = File::create(&ignore_path).unwrap();
        file.write_all(content.as_bytes()).unwrap();
        
        let vcs_ignore = DotIgnore::load_from_file(&ignore_path).unwrap();
        
        assert!(vcs_ignore.is_ignored("file.txt"));
        assert!(!vcs_ignore.is_ignored("important.txt"));
        assert!(vcs_ignore.is_ignored("temp/file"));
        assert!(!vcs_ignore.is_ignored("file.rs"));
    }
} 
