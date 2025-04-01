use DotIgnore::{Pattern, PatternGroup, IgnoreFile};
use std::path::PathBuf;

#[cfg(test)]
mod ignore_file_basic_tests {
    use super::*;

    #[test]
    fn test_ignore_file_creation() {
        let ignore_file = IgnoreFile::new(PathBuf::from(".DotIgnore"));
        
        assert_eq!(ignore_file.path, PathBuf::from(".DotIgnore"));
        assert!(ignore_file.groups.is_empty());
        assert!(ignore_file.standard_patterns.is_empty());
    }
    
    #[test]
    fn test_adding_standard_patterns() {
        let mut ignore_file = IgnoreFile::new(PathBuf::from(".DotIgnore"));
        
        ignore_file.add_standard_pattern_str("*.log");
        ignore_file.add_standard_pattern_str("*.tmp");
        
        assert_eq!(ignore_file.standard_patterns.len(), 2);
        assert_eq!(ignore_file.standard_patterns[0].original, "*.log");
        assert_eq!(ignore_file.standard_patterns[1].original, "*.tmp");
    }
    
    #[test]
    fn test_adding_pattern_objects() {
        let mut ignore_file = IgnoreFile::new(PathBuf::from(".DotIgnore"));
        
        let pattern1 = Pattern::new("*.log");
        let pattern2 = Pattern::new("*.tmp");
        
        ignore_file.add_standard_pattern(pattern1);
        ignore_file.add_standard_pattern(pattern2);
        
        assert_eq!(ignore_file.standard_patterns.len(), 2);
        assert_eq!(ignore_file.standard_patterns[0].original, "*.log");
        assert_eq!(ignore_file.standard_patterns[1].original, "*.tmp");
    }
    
    #[test]
    fn test_adding_groups() {
        let mut ignore_file = IgnoreFile::new(PathBuf::from(".DotIgnore"));
        
        let mut group1 = PatternGroup::new("system");
        group1.add_pattern_str(".DS_Store");
        group1.add_pattern_str("Thumbs.db");
        
        let mut group2 = PatternGroup::new("build");
        group2.add_pattern_str("*.o");
        group2.add_pattern_str("build/");
        
        ignore_file.add_group(group1);
        ignore_file.add_group(group2);
        
        assert_eq!(ignore_file.groups.len(), 2);
        assert_eq!(ignore_file.groups[0].name, "system");
        assert_eq!(ignore_file.groups[1].name, "build");
        assert_eq!(ignore_file.groups[0].patterns.len(), 2);
        assert_eq!(ignore_file.groups[1].patterns.len(), 2);
    }
}

#[cfg(test)]
mod ignore_file_combined_tests {
    use super::*;

    #[test]
    fn test_complete_ignore_file() {
        // Crear un archivo completo con patrones estándar y grupos
        let mut ignore_file = IgnoreFile::new(PathBuf::from(".DotIgnore"));
        
        // Agregar patrones estándar
        ignore_file.add_standard_pattern_str("README.md");
        ignore_file.add_standard_pattern_str("LICENSE");
        
        // Crear y agregar grupos
        let mut system_group = PatternGroup::new("system");
        system_group.add_pattern_str(".DS_Store");
        system_group.add_pattern_str("Thumbs.db");
        
        let mut build_group = PatternGroup::new("build");
        build_group.add_pattern_str("*.o");
        build_group.add_pattern_str("build/");
        
        let mut temp_group = PatternGroup::new("temp");
        temp_group.add_pattern_str("*.tmp");
        temp_group.add_pattern_str("*.bak");
        temp_group.add_pattern_str("size:<1MB *.temp");
        
        ignore_file.add_group(system_group);
        ignore_file.add_group(build_group);
        ignore_file.add_group(temp_group);
        
        // Verificar la estructura completa
        assert_eq!(ignore_file.standard_patterns.len(), 2);
        assert_eq!(ignore_file.groups.len(), 3);
        
        // Verificar patrones estándar
        assert_eq!(ignore_file.standard_patterns[0].original, "README.md");
        assert_eq!(ignore_file.standard_patterns[1].original, "LICENSE");
        
        // Verificar grupos
        assert_eq!(ignore_file.groups[0].name, "system");
        assert_eq!(ignore_file.groups[1].name, "build");
        assert_eq!(ignore_file.groups[2].name, "temp");
        
        // Verificar patrones dentro de grupos
        assert_eq!(ignore_file.groups[0].patterns[0].original, ".DS_Store");
        assert_eq!(ignore_file.groups[1].patterns[1].original, "build/");
        assert_eq!(ignore_file.groups[2].patterns[2].original, "size:<1MB *.temp");
        assert!(ignore_file.groups[2].patterns[2].is_size_condition());
    }
    
    #[test]
    fn test_getters() {
        let mut ignore_file = IgnoreFile::new(PathBuf::from(".DotIgnore"));
        
        // Agregar patrones estándar
        ignore_file.add_standard_pattern_str("README.md");
        ignore_file.add_standard_pattern_str("LICENSE");
        
        // Crear y agregar grupos
        let mut system_group = PatternGroup::new("system");
        system_group.add_pattern_str(".DS_Store");
        
        let mut build_group = PatternGroup::new("build");
        build_group.add_pattern_str("*.o");
        
        ignore_file.add_group(system_group);
        ignore_file.add_group(build_group);
        
        // Verificar getter de patrones
        let patterns = ignore_file.patterns();
        assert_eq!(patterns.len(), 2);
        assert_eq!(patterns[0].original, "README.md");
        assert_eq!(patterns[1].original, "LICENSE");
        
        // Verificar getter de grupos
        let groups = ignore_file.groups();
        assert_eq!(groups.len(), 2);
        assert_eq!(groups[0].name, "system");
        assert_eq!(groups[1].name, "build");
    }
} 
