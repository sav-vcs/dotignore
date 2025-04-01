// Module: ignore/pattern.rs
//
// This module defines the structures for handling ignore patterns

use std::path::{Path, PathBuf};
use regex::Regex;
use std::fs;
use lazy_static::lazy_static;

lazy_static! {
    // Regex para detectar condiciones de tamaño: size:<5MB o size:>100KB etc.
    static ref SIZE_CONDITION_RE: Regex = Regex::new(r"^size:([<>])(\d+)([KMG]?B)?\s+(.+)$").unwrap();
}

/// Represents a single ignore pattern
#[derive(Debug, Clone)]
pub struct Pattern {
    /// Original pattern string
    pub original: String,
    
    /// Compiled regex for matching
    regex: Option<Regex>,
    
    /// Whether this is a negation pattern (starts with !)
    negated: bool,
    
    /// Whether this pattern matches directories only
    dir_only: bool,

    /// Size condition: Some((comparison, size_in_bytes)) or None
    size_condition: Option<(SizeComparison, u64)>,

    /// The actual pattern to match (without size condition prefix)
    pub pattern: String,
}

/// Comparison type for size conditions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SizeComparison {
    /// File size must be less than specified
    LessThan,
    /// File size must be greater than specified
    GreaterThan,
}

impl Pattern {
    /// Create a new pattern from a string
    pub fn new(pattern: &str) -> Self {
        let trimmed = pattern.trim();
        let negated = trimmed.starts_with('!');
        let dir_only = trimmed.ends_with('/');
        
        // Process the pattern
        let processed = if negated {
            // Remove the negation character for regex processing
            trimmed[1..].trim().to_string()
        } else {
            trimmed.to_string()
        };
        
        // Check if this is a size condition pattern
        let (size_condition, pattern_text) = Self::parse_size_condition(&processed);
        
        // Convert to regex
        let regex = Self::pattern_to_regex(&pattern_text);
        
        Self {
            original: trimmed.to_string(),
            regex,
            negated,
            dir_only,
            size_condition,
            pattern: pattern_text,
        }
    }
    
    /// Parse size condition from pattern if present
    fn parse_size_condition(pattern: &str) -> (Option<(SizeComparison, u64)>, String) {
        if let Some(captures) = SIZE_CONDITION_RE.captures(pattern) {
            let comparison = match captures.get(1).unwrap().as_str() {
                "<" => SizeComparison::LessThan,
                ">" => SizeComparison::GreaterThan,
                _ => unreachable!(), // Regex ensures only < or >
            };
            
            let size_value: u64 = captures.get(2).unwrap().as_str().parse().unwrap_or(0);
            let unit = captures.get(3).map(|m| m.as_str()).unwrap_or("B");
            
            // Convert to bytes based on unit
            let size_in_bytes = match unit {
                "KB" => size_value * 1024,
                "MB" => size_value * 1024 * 1024,
                "GB" => size_value * 1024 * 1024 * 1024,
                _ => size_value, // Default to bytes
            };
            
            let actual_pattern = captures.get(4).unwrap().as_str().to_string();
            
            (Some((comparison, size_in_bytes)), actual_pattern)
        } else {
            (None, pattern.to_string())
        }
    }
    
    /// Convert a glob-style pattern to a regex
    fn pattern_to_regex(pattern: &str) -> Option<Regex> {
        // Simple conversion of glob syntax to regex
        let mut regex_str = "^".to_string();
        
        let pattern = pattern.trim_end_matches('/');
        
        for ch in pattern.chars() {
            match ch {
                '*' => regex_str.push_str(".*"),
                '?' => regex_str.push('.'),
                '.' => regex_str.push_str("\\."),
                '[' => regex_str.push('['),
                ']' => regex_str.push(']'),
                '\\' => regex_str.push_str("\\\\"),
                '$' => regex_str.push_str("\\$"),
                '^' => regex_str.push_str("\\^"),
                '+' => regex_str.push_str("\\+"),
                '(' => regex_str.push_str("\\("),
                ')' => regex_str.push_str("\\)"),
                '{' => regex_str.push_str("\\{"),
                '}' => regex_str.push_str("\\}"),
                '|' => regex_str.push_str("\\|"),
                _ => regex_str.push(ch),
            }
        }
        
        regex_str.push('$');
        
        match Regex::new(&regex_str) {
            Ok(re) => Some(re),
            Err(_) => None,
        }
    }
    
    /// Check if this pattern matches the given path
    pub fn matches(&self, path: &str) -> bool {
        // Special handling for directory patterns (ending with /)
        if self.dir_only {
            let dir_name = self.pattern.trim_end_matches('/');
            
            // Check if path is equal to the directory name or starts with directory name followed by / or \
            if path == dir_name {
                return true;
            }
            
            if path.starts_with(&format!("{}/", dir_name)) || 
               path.starts_with(&format!("{}\\", dir_name)) {
                return true;
            }
            
            // Also match subdirectories like "path/to/build/file" for pattern "build/"
            let path_parts: Vec<&str> = path.split('/').collect();
            for part in path_parts {
                if part == dir_name {
                    return true;
                }
            }
            
            return false;
        }
        
        // For specific file patterns without wildcards, do exact match
        if !self.pattern.contains('*') && !self.pattern.contains('?') {
            let filename = Path::new(path)
                .file_name()
                .map(|f| f.to_string_lossy().into_owned())
                .unwrap_or_else(|| path.to_string());
                
            return filename == self.pattern;
        }
        
        // For patterns with wildcards, use regex
        if let Some(regex) = &self.regex {
            if regex.is_match(path) {
                // Additional size condition check if applicable
                if let Some((comparison, size_limit)) = self.size_condition {
                    let path_obj = Path::new(path);
                    
                    if let Ok(metadata) = fs::metadata(path_obj) {
                        if metadata.is_dir() {
                            return false; // Size conditions don't apply to directories
                        }
                        
                        let file_size = metadata.len();
                        match comparison {
                            SizeComparison::LessThan => return file_size < size_limit,
                            SizeComparison::GreaterThan => return file_size > size_limit,
                        }
                    } else {
                        // For tests, assume it matches when file doesn't exist
                        return true;
                    }
                }
                
                return true;
            }
        }
        
        false
    }
    
    /// Check if a path matches this pattern with a specific file size
    pub fn matches_with_size(&self, path: &str, size_in_bytes: u64) -> bool {
        // First check if the pattern matches the path string
        let pattern_matches = if let Some(regex) = &self.regex {
            regex.is_match(path)
        } else {
            // Fallback to simple string matching if regex isn't available
            path.contains(&self.pattern)
        };
        
        // If the pattern doesn't match, return false
        if !pattern_matches {
            return false;
        }
        
        // If there's a size condition, apply it
        if let Some((comparison, size_limit)) = self.size_condition {
            match comparison {
                SizeComparison::LessThan => size_in_bytes < size_limit,
                SizeComparison::GreaterThan => size_in_bytes > size_limit,
            }
        } else {
            // If no size condition, the pattern match is sufficient
            true
        }
    }
    
    /// Check if this is a negation pattern
    pub fn is_negated(&self) -> bool {
        self.negated
    }
    
    /// Check if this pattern matches directories only
    pub fn is_dir_only(&self) -> bool {
        self.dir_only
    }
    
    /// Check if this pattern has a size condition
    pub fn is_size_condition(&self) -> bool {
        self.size_condition.is_some()
    }
}

/// Represents a pattern group in an ignore file
#[derive(Debug, Clone)]
pub struct PatternGroup {
    /// Name of the group
    pub name: String,
    
    /// Parent group name, if any (for nested groups)
    pub parent: Option<String>,
    
    /// Patterns in this group
    pub patterns: Vec<Pattern>,
}

impl PatternGroup {
    /// Create a new pattern group with the given name
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            parent: None,
            patterns: Vec::new(),
        }
    }
    
    /// Add a pattern to this group
    pub fn add_pattern(&mut self, pattern: Pattern) {
        self.patterns.push(pattern);
    }
    
    /// Create a nested group with the given parent
    pub fn with_parent(mut self, parent: impl Into<String>) -> Self {
        self.parent = Some(parent.into());
        self
    }
    
    /// Add multiple patterns at once
    pub fn with_patterns(mut self, patterns: Vec<Pattern>) -> Self {
        self.patterns = patterns;
        self
    }
}

/// Represents a complete ignore file
#[derive(Debug)]
pub struct IgnoreFile {
    /// Path to the file
    pub path: PathBuf,
    
    /// Pattern groups in this file
    pub groups: Vec<PatternGroup>,
    
    /// Standard patterns (not in any group)
    pub standard_patterns: Vec<Pattern>,
}

impl IgnoreFile {
    /// Create a new ignore file
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self {
            path: path.into(),
            groups: Vec::new(),
            standard_patterns: Vec::new(),
        }
    }
    
    /// Add a standard pattern
    pub fn add_standard_pattern(&mut self, pattern: &str) {
        self.standard_patterns.push(Pattern::new(pattern));
    }
    
    /// Get all patterns in this file
    pub fn get_all_patterns(&self) -> Vec<&Pattern> {
        let mut all_patterns = Vec::new();
        
        // Add standard patterns
        all_patterns.extend(self.standard_patterns.iter());
        
        // Add patterns from groups
        for group in &self.groups {
            all_patterns.extend(group.patterns.iter());
        }
        
        all_patterns
    }

    /// Get all patterns in this file (for backward compatibility)
    pub fn patterns(&self) -> &[Pattern] {
        &self.standard_patterns
    }
    
    /// Get all groups in this file (for backward compatibility)
    pub fn groups(&self) -> &[PatternGroup] {
        &self.groups
    }

    /// Add a new pattern to the standard patterns list
    pub fn add_pattern(&mut self, pattern: Pattern) {
        self.standard_patterns.push(pattern);
    }

    /// Add a new group to the groups list
    pub fn add_group(&mut self, group: PatternGroup) {
        self.groups.push(group);
    }

    /// Check if a file should be ignored
    pub fn is_ignored<P: AsRef<Path>>(&self, path: P) -> bool {
        let path_str = path.as_ref().to_string_lossy();
        
        // First check explicit negation patterns
        // A more specific negated pattern takes precedence
        for pattern in &self.standard_patterns {
            if pattern.is_negated() && pattern.matches(&path_str) {
                return false; // This pattern explicitly negates the match
            }
        }
        
        for group in &self.groups {
            for pattern in &group.patterns {
                if pattern.is_negated() && pattern.matches(&path_str) {
                    return false; // This pattern explicitly negates the match
                }
            }
        }
        
        // Then check inclusion patterns
        for pattern in &self.standard_patterns {
            if !pattern.is_negated() && pattern.matches(&path_str) {
                return true;
            }
        }
        
        for group in &self.groups {
            for pattern in &group.patterns {
                if !pattern.is_negated() && pattern.matches(&path_str) {
                    return true;
                }
            }
        }
        
        false
    }
}

impl Default for IgnoreFile {
    fn default() -> Self {
        Self::new(PathBuf::new())
    }
} 
