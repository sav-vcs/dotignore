use std::fs::File;
use std::io::Write;
use std::path::Path;
use tempfile::tempdir;
use ignore::{DotIgnore, Pattern, PatternGroup};

#[test]
fn test_load_and_parse_ignore_file() {
    let temp_dir = tempdir().unwrap();
    let file_path = temp_dir.path().join(".ignore");
    
    // Create a test .ignore file
    let content = r#"# Test .ignore file
*.tmp
*.log

[group1] {
    # Build artifacts
    *.o
    *.obj
    bin/
}

[group2:nested] {
    # Documentation
    docs/*.pdf
    docs/generated/
    !docs/README.md
}
"#;
    
    let mut file = File::create(&file_path).unwrap();
    file.write_all(content.as_bytes()).unwrap();
    
    // Load and parse
    let mut dotignore = DotIgnore::load_from_file(&file_path).unwrap();
    
    // Verify files are ignored correctly
    assert!(dotignore.is_ignored("test.tmp"));
    assert!(dotignore.is_ignored("logs/app.log"));
    assert!(dotignore.is_ignored("src/module.o"));
    assert!(dotignore.is_ignored("build/bin/app"));
    assert!(dotignore.is_ignored("docs/manual.pdf"));
    
    // Check negated patterns
    assert!(!dotignore.is_ignored("docs/README.md"));
    assert!(!dotignore.is_ignored("src/main.rs"));
}

#[test]
fn test_pattern_matching() {
    // Create a DotIgnore with standard patterns
    let mut dotignore = DotIgnore::new();
    
    // Add some basic patterns
    dotignore.add_pattern(Pattern::new("*.tmp"));
    dotignore.add_pattern(Pattern::new("*.log"));
    dotignore.add_pattern(Pattern::new("build/"));
    
    // Test pattern matching
    assert!(dotignore.is_ignored("test.tmp"));
    assert!(dotignore.is_ignored("logs/app.log"));
    assert!(dotignore.is_ignored("build/output.txt"));
    
    // Test non-matching patterns
    assert!(!dotignore.is_ignored("test.txt"));
    assert!(!dotignore.is_ignored("src/main.rs"));
}

#[test]
fn test_group_pattern_matching() {
    // Create a DotIgnore with grouped patterns
    let mut dotignore = DotIgnore::new();
    
    // Add a standard pattern
    dotignore.add_pattern(Pattern::new("*.tmp"));
    
    // Add a build group
    let mut build_group = PatternGroup::new("build");
    build_group.add_pattern(Pattern::new("*.o"));
    build_group.add_pattern(Pattern::new("*.obj"));
    build_group.add_pattern(Pattern::new("bin/"));
    dotignore.add_group(build_group);
    
    // Add a docs group
    let mut doc_group = PatternGroup::new("docs");
    doc_group.add_pattern(Pattern::new("*.pdf"));
    doc_group.add_pattern(Pattern::new("generated/"));
    dotignore.add_group(doc_group);
    
    // Test matching
    assert!(dotignore.is_ignored("test.tmp")); // Base pattern
    assert!(dotignore.is_ignored("build/main.o")); // build group
    assert!(dotignore.is_ignored("src/module.obj")); // build group
    assert!(dotignore.is_ignored("docs/manual.pdf")); // docs group
    
    // Test non-matching
    assert!(!dotignore.is_ignored("test.txt"));
    assert!(!dotignore.is_ignored("src/main.rs"));
}

#[test]
fn test_negated_patterns() {
    // Create a DotIgnore with negated patterns
    let mut dotignore = DotIgnore::new();
    
    // Add patterns with negation
    dotignore.add_pattern(Pattern::new("*.log"));
    dotignore.add_pattern(Pattern::new("!important.log")); // Negated pattern
    
    // Add a group with negated pattern
    let mut build_group = PatternGroup::new("build");
    build_group.add_pattern(Pattern::new("*"));
    build_group.add_pattern(Pattern::new("!keep/")); // Negated pattern
    build_group.add_pattern(Pattern::new("!keep/important.txt")); // Negated pattern
    dotignore.add_group(build_group);
    
    // Test negated pattern handling
    assert!(dotignore.is_ignored("debug.log"));
    assert!(!dotignore.is_ignored("important.log")); // Excluded by negation
    assert!(dotignore.is_ignored("build/temp.txt"));
    assert!(!dotignore.is_ignored("build/keep/important.txt")); // Excluded by negation
}

#[test]
fn test_convert_gitignore_to_ignore() {
    let temp_dir = tempdir().unwrap();
    let git_path = temp_dir.path().join(".gitignore");
    let dot_path = temp_dir.path().join(".ignore");
    
    // Create a test .gitignore file
    let gitignore_content = r#"# Test gitignore file
*.tmp
*.swp
*.bak

# Build artifacts
*.o
*.a
*.so
build/

# Logs
*.log
!important.log
"#;
    
    let mut file = File::create(&git_path).unwrap();
    file.write_all(gitignore_content.as_bytes()).unwrap();
    
    // Convert to .ignore
    let result = DotIgnore::convert_file(&git_path, Some(&dot_path)).unwrap();
    
    // Verify conversion results using accessor methods
    assert!(result.patterns_converted() > 0);
    assert!(result.groups_created() > 0);
    
    // Load the converted file and check
    let dotignore = DotIgnore::load_from_file(&dot_path).unwrap();
    
    // Verify the ignore rules work correctly
    assert!(dotignore.is_ignored("temp.tmp"));
    assert!(dotignore.is_ignored("backup.bak"));
    assert!(dotignore.is_ignored("build/output"));
    assert!(dotignore.is_ignored("main.o"));
    assert!(!dotignore.is_ignored("important.log"));

    // Check group information using getter methods
    let groups = dotignore.get_groups();
    assert!(!groups.is_empty(), "La conversión debe generar grupos de patrones");
    
    // Verificar que existen los grupos esperados
    assert!(groups.iter().any(|g| g.name == "temporary_files"), "Debe existir un grupo 'temporary_files'");
    assert!(groups.iter().any(|g| g.name == "build_artifacts"), "Debe existir un grupo 'build_artifacts'");
    
    // Verificar contenido de los grupos
    let temp_group = groups.iter().find(|g| g.name == "temporary_files").unwrap();
    assert!(temp_group.patterns.iter().any(|p| p.pattern.contains("*.tmp")), "El grupo temporal debe contener el patrón *.tmp");
    assert!(temp_group.patterns.iter().any(|p| p.pattern.contains("*.swp")), "El grupo temporal debe contener el patrón *.swp");
    
    let build_group = groups.iter().find(|g| g.name == "build_artifacts").unwrap();
    assert!(build_group.patterns.iter().any(|p| p.pattern.contains("*.o")), "El grupo build debe contener el patrón *.o");
    assert!(build_group.patterns.iter().any(|p| p.pattern.contains("build/")), "El grupo build debe contener el patrón build/");
}

#[test]
fn test_default_file_creation() {
    let temp_dir = tempdir().unwrap();
    let file_path = temp_dir.path().join(".ignore");
    
    // Create a default .ignore file
    let result = DotIgnore::create_default_file(&file_path);
    assert!(result.is_ok());
    assert!(file_path.exists());
    
    // Load and verify it contains expected content
    let dotignore = DotIgnore::load_from_file(&file_path).unwrap();
    
    // Verify the default patterns work
    assert!(dotignore.is_ignored("log/server.log"));
    assert!(dotignore.is_ignored("temp.tmp"));
    assert!(dotignore.is_ignored("build/output"));
    assert!(!dotignore.is_ignored("README.md"));
    
    // Check groups using getter method
    let groups = dotignore.get_groups();
    assert!(!groups.is_empty(), "El archivo default debe contener grupos");
    
    // Verificar que existen los grupos esperados
    assert!(groups.iter().any(|g| g.name == "common"), "Debe existir un grupo 'common'");
    assert!(groups.iter().any(|g| g.name == "system"), "Debe existir un grupo 'system'");
    assert!(groups.iter().any(|g| g.name == "development"), "Debe existir un grupo 'development'");
    
    // Verificar contenido de los grupos
    let system_group = groups.iter().find(|g| g.name == "system").unwrap();
    assert!(system_group.patterns.iter().any(|p| p.pattern.contains("*.log")), "El grupo system debe contener el patrón *.log");
}

#[test]
fn test_size_based_patterns() {
    // Create a DotIgnore with size-based patterns
    let mut dotignore = DotIgnore::new();
    
    // Add patterns with size conditions
    dotignore.add_pattern(Pattern::new("size:>10MB *.iso"));
    dotignore.add_pattern(Pattern::new("size:<500B *.txt"));
    
    // Test size-based matching
    assert!(dotignore.is_ignored_with_size_check("large.iso", 15 * 1024 * 1024));
    assert!(!dotignore.is_ignored_with_size_check("small.iso", 5 * 1024 * 1024));
    assert!(dotignore.is_ignored_with_size_check("tiny.txt", 100));
    assert!(!dotignore.is_ignored_with_size_check("medium.txt", 1000));
} 
