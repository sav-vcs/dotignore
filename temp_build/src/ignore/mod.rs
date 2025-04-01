// Module: ignore
//
// This module handles the Dot ignore system, including parsing .DotIgnore files
// and converting from other formats like .gitignore and .svnignore.

mod parser;
mod converter;
mod pattern;

pub use parser::IgnoreParser;
pub use converter::{IgnoreConverter, ConversionResult};
pub use pattern::{Pattern, PatternGroup, IgnoreFile};

use anyhow::Result;
use std::path::Path;

/// Core functionality for the Dot ignore system
pub struct DotIgnore {
    pub patterns: Vec<Pattern>,
    pub groups: Vec<PatternGroup>,
}

impl DotIgnore {
    /// Create a new empty DotIgnore instance
    pub fn new() -> Self {
        Self {
            patterns: Vec::new(),
            groups: Vec::new(),
        }
    }

    /// Load a .DotIgnore file from the given path
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let parser = IgnoreParser::new();
        parser.parse_file(path.as_ref())
    }

    /// Check if a file should be ignored
    pub fn is_ignored<P: AsRef<Path>>(&self, path: P) -> bool {
        let path_str = path.as_ref().to_string_lossy();
        
        // Primero verificar patrones de negación explícita
        // Un patrón negado con mayor especificidad tiene prioridad
        for pattern in &self.patterns {
            if pattern.is_negated() && pattern.matches(&path_str) {
                return false; // Este patrón niega explícitamente la coincidencia
            }
        }
        
        // Verificar patrones en todos los grupos
        for group in &self.groups {
            for pattern in &group.patterns {
                if pattern.is_negated() && pattern.matches(&path_str) {
                    return false; // Este patrón niega explícitamente la coincidencia
                }
            }
        }
        
        // Verificar patrones normales en todos los grupos
        for group in &self.groups {
            for pattern in &group.patterns {
                if !pattern.is_negated() && pattern.matches(&path_str) {
                    return true; // Coincidencia encontrada
                }
            }
        }
        
        // Verificar patrones normales en la raíz
        for pattern in &self.patterns {
            if !pattern.is_negated() && pattern.matches(&path_str) {
                return true; // Coincidencia encontrada
            }
        }
        
        false // No se encontraron coincidencias
    }

    /// Add a new pattern to the global patterns list
    pub fn add_pattern(&mut self, pattern: Pattern) {
        self.patterns.push(pattern);
    }

    /// Add a new group to the groups list
    pub fn add_group(&mut self, group: PatternGroup) {
        self.groups.push(group);
    }

    /// Get all patterns in this DotIgnore
    pub fn get_patterns(&self) -> &[Pattern] {
        &self.patterns
    }

    /// Get all groups in this DotIgnore
    pub fn get_groups(&self) -> &[PatternGroup] {
        &self.groups
    }

    /// Check if a file should be ignored with size consideration
    pub fn is_ignored_with_size_check<P: AsRef<Path>>(&self, path: P, size_in_bytes: u64) -> bool {
        let path_str = path.as_ref().to_string_lossy();
        let is_dir = path.as_ref().is_dir();
        
        // Check global patterns
        for pattern in &self.patterns {
            if pattern.is_dir_only() && !is_dir {
                continue;
            }
            
            if pattern.matches_with_size(&path_str, size_in_bytes) {
                return !pattern.is_negated();
            }
        }
        
        // Check group patterns
        for group in &self.groups {
            for pattern in &group.patterns {
                if pattern.is_dir_only() && !is_dir {
                    continue;
                }
                
                if pattern.matches_with_size(&path_str, size_in_bytes) {
                    return !pattern.is_negated();
                }
            }
        }
        
        false
    }

    /// Convert a file from another format to .DotIgnore
    pub fn convert_file<P: AsRef<Path>, Q: AsRef<Path>>(source_path: P, destination_path: Option<Q>) -> Result<ConversionResult> {
        let converter = IgnoreConverter::new();
        let destination = destination_path.as_ref().map(|p| p.as_ref());
        converter.convert_file(source_path.as_ref(), destination)
    }

    /// Convert all ignore files in a directory
    pub fn convert_directory<P: AsRef<Path>>(directory_path: P, recursive: bool) -> Result<Vec<ConversionResult>> {
        let converter = IgnoreConverter::new();
        converter.convert_directory(directory_path.as_ref(), recursive)
    }

    /// Create a default .DotIgnore file at the specified path
    pub fn create_default_file<P: AsRef<Path>>(path: P) -> Result<()> {
        let content = Self::example_content();
        std::fs::write(path, content)?;
        Ok(())
    }

    pub fn example_content() -> String {
        let mut content = String::new();
        content.push_str("# Archivo .ignore de ejemplo generado automáticamente\n");
        content.push_str("# Para más información, visite https://dotignore.dev\n\n");
        
        // Grupo común de patrones
        content.push_str("[common] {\n");
        content.push_str("    # Archivos y directorios comunes que suelen ignorarse\n");
        content.push_str("    .DS_Store\n");
        content.push_str("    Thumbs.db\n");
        content.push_str("    .vscode/\n");
        content.push_str("    .idea/\n");
        content.push_str("}\n\n");
        
        // Grupo de sistema
        content.push_str("[system] {\n");
        content.push_str("    # Archivos de sistema y logs\n");
        content.push_str("    *.log\n");
        content.push_str("    *.tmp\n");
        content.push_str("    *.bak\n");
        content.push_str("    core\n");
        content.push_str("}\n\n");
        
        // Grupo de desarrollo
        content.push_str("[development] {\n");
        content.push_str("    # Directorios y archivos de desarrollo\n");
        content.push_str("    build/\n");
        content.push_str("    dist/\n");
        content.push_str("    node_modules/\n");
        content.push_str("    target/\n");
        content.push_str("    *.o\n");
        content.push_str("    *.a\n");
        content.push_str("    *.so\n");
        content.push_str("    *.class\n");
        content.push_str("}\n");
        
        content
    }
}

impl Default for DotIgnore {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_basic_ignore_patterns() {
        let temp_dir = tempdir().unwrap();
        let ignore_path = temp_dir.path().join(".DotIgnore");
        
        let content = r#"
[standard_patterns] {
    *.txt
    !important.txt
    temp/
} # end standard_patterns
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
