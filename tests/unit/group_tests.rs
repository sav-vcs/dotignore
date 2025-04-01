use DotIgnore::{Pattern, PatternGroup};

#[cfg(test)]
mod pattern_group_tests {
    use super::*;

    #[test]
    fn test_group_creation() {
        let group = PatternGroup::new("test_group");
        
        assert_eq!(group.name, "test_group");
        assert!(group.patterns.is_empty());
    }
    
    #[test]
    fn test_adding_patterns() {
        let mut group = PatternGroup::new("build");
        
        group.add_pattern_str("*.o");
        group.add_pattern_str("*.obj");
        group.add_pattern_str("build/");
        
        assert_eq!(group.patterns.len(), 3);
        assert_eq!(group.patterns[0].original, "*.o");
        assert_eq!(group.patterns[1].original, "*.obj");
        assert_eq!(group.patterns[2].original, "build/");
    }
    
    #[test]
    fn test_adding_pattern_objects() {
        let mut group = PatternGroup::new("logs");
        
        let pattern1 = Pattern::new("*.log");
        let pattern2 = Pattern::new("logs/");
        
        group.add_pattern(pattern1);
        group.add_pattern(pattern2);
        
        assert_eq!(group.patterns.len(), 2);
        assert_eq!(group.patterns[0].original, "*.log");
        assert_eq!(group.patterns[1].original, "logs/");
    }
    
    #[test]
    fn test_group_with_mixed_pattern_types() {
        let mut group = PatternGroup::new("mixed");
        
        // Patrón normal
        group.add_pattern_str("*.txt");
        
        // Patrón negado
        group.add_pattern_str("!important.txt");
        
        // Patrón de directorio
        group.add_pattern_str("temp/");
        
        // Patrón de tamaño
        group.add_pattern_str("size:>1MB *.bin");
        
        assert_eq!(group.patterns.len(), 4);
        assert!(!group.patterns[0].is_negated());
        assert!(group.patterns[1].is_negated());
        assert!(group.patterns[2].is_dir_only());
        assert!(group.patterns[3].is_size_condition());
    }
}

#[cfg(test)]
mod multiple_groups_tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_multiple_independent_groups() {
        // Simulamos tener múltiples grupos como en un archivo .DotIgnore
        let mut groups = HashMap::new();
        
        // Grupo para archivos del sistema
        let mut system_group = PatternGroup::new("system");
        system_group.add_pattern_str(".DS_Store");
        system_group.add_pattern_str("Thumbs.db");
        
        // Grupo para archivos de construcción
        let mut build_group = PatternGroup::new("build");
        build_group.add_pattern_str("*.o");
        build_group.add_pattern_str("build/");
        
        // Grupo para archivos temporales
        let mut temp_group = PatternGroup::new("temp");
        temp_group.add_pattern_str("*.tmp");
        temp_group.add_pattern_str("*.bak");
        
        // Agregar todos los grupos al mapa
        groups.insert(system_group.name.clone(), system_group);
        groups.insert(build_group.name.clone(), build_group);
        groups.insert(temp_group.name.clone(), temp_group);
        
        // Verificar que los grupos se mantienen independientes
        assert_eq!(groups.len(), 3);
        assert_eq!(groups["system"].patterns.len(), 2);
        assert_eq!(groups["build"].patterns.len(), 2);
        assert_eq!(groups["temp"].patterns.len(), 2);
        
        // Verificar que los patrones son correctos
        assert_eq!(groups["system"].patterns[0].original, ".DS_Store");
        assert_eq!(groups["build"].patterns[0].original, "*.o");
        assert_eq!(groups["temp"].patterns[0].original, "*.tmp");
    }
    
    #[test]
    fn test_groups_with_hierarchical_names() {
        // Grupo principal y subgrupos con nombres jerárquicos
        let mut parent_group = PatternGroup::new("env");
        let mut dev_group = PatternGroup::new("env:dev");
        let mut prod_group = PatternGroup::new("env:prod");
        
        parent_group.add_pattern_str(".env");
        dev_group.add_pattern_str(".env.dev");
        dev_group.add_pattern_str(".env.local");
        prod_group.add_pattern_str(".env.prod");
        
        assert_eq!(parent_group.name, "env");
        assert_eq!(dev_group.name, "env:dev");
        assert_eq!(prod_group.name, "env:prod");
        
        assert_eq!(parent_group.patterns.len(), 1);
        assert_eq!(dev_group.patterns.len(), 2);
 
