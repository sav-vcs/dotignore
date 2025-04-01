use ignore::Pattern;

#[test]
fn test_pattern_creation() {
    let pattern = Pattern::new("*.txt");
    assert_eq!(pattern.original, "*.txt");
    assert!(!pattern.is_negated());
    assert!(!pattern.is_dir_only());
    assert!(!pattern.is_size_condition());
}

#[test]
fn test_pattern_matching() {
    let pattern = Pattern::new("*.txt");
    assert!(pattern.matches("file.txt"));
    assert!(pattern.matches("path/to/file.txt"));
    assert!(!pattern.matches("file.rs"));
    assert!(!pattern.matches("text.doc"));
}

#[test]
fn test_negated_pattern() {
    let pattern = Pattern::new("!*.txt");
    assert!(pattern.is_negated());
    assert!(pattern.matches("file.txt"));
    assert!(!pattern.matches("file.rs"));
}

#[test]
fn test_directory_pattern() {
    let pattern = Pattern::new("bin/");
    assert!(pattern.is_dir_only());
    assert!(pattern.matches("bin/file"));
    assert!(pattern.matches("path/to/bin/file"));
    assert!(!pattern.matches("binary"));
}

#[test]
fn test_size_condition_pattern() {
    let pattern = Pattern::new("size:>10KB *.iso");
    assert!(pattern.is_size_condition());
    assert!(pattern.matches_with_size("file.iso", 15 * 1024));
    assert!(!pattern.matches_with_size("file.iso", 5 * 1024));
    assert!(!pattern.matches_with_size("file.txt", 15 * 1024));
    
    let pattern = Pattern::new("size:<500B *.log");
    assert!(pattern.is_size_condition());
    assert!(pattern.matches_with_size("app.log", 300));
    assert!(!pattern.matches_with_size("app.log", 1000));
}
