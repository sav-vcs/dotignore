// Module: ignore/parser.rs
//
// This module implements the parser for .DotIgnore files.

use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};
use anyhow::{Result, Context, anyhow};
use regex::Regex;
use lazy_static::lazy_static;

use super::{DotIgnore, Pattern, PatternGroup, IgnoreFile};

lazy_static! {
    // Regex to match the start of a group: [group_name] {
    static ref GROUP_START_RE: Regex = Regex::new(r"^\s*\[([a-zA-Z0-9_:.-]+)\]\s*\{\s*$").unwrap();
    
    // Regex to match the end of a group: } # end group_name
    static ref GROUP_END_RE: Regex = Regex::new(r"^\s*\}\s*(?:#\s*end\s+([a-zA-Z0-9_:.-]+))?\s*$").unwrap();
}

/// Parser for .DotIgnore files
pub struct IgnoreParser {
    // Add parser state here if needed
}

impl IgnoreParser {
    /// Create a new parser
    pub fn new() -> Self {
        Self {}
    }
    
    /// Parse a .DotIgnore file and return a DotIgnore instance
    pub fn parse_file(&self, path: &Path) -> Result<DotIgnore> {
        if !path.exists() {
            return Err(anyhow!("File does not exist: {:?}", path));
        }
        
        let file = File::open(path).with_context(|| format!("Failed to open file: {:?}", path))?;
        let reader = BufReader::new(file);
        
        let mut ignore_file = IgnoreFile::new(path);
        
        // State variables for parsing
        let mut current_group: Option<PatternGroup> = None;
        let mut line_number = 0;
        
        for line in reader.lines() {
            line_number += 1;
            let line = line.with_context(|| format!("Error reading line {} from {:?}", line_number, path))?;
            
            // Skip empty lines
            if line.trim().is_empty() {
                continue;
            }
            
            // Skip comment lines (but not comments at end of line)
            if line.trim_start().starts_with('#') {
                continue;
            }
            
            // Check for group start
            if let Some(captures) = GROUP_START_RE.captures(&line) {
                // If we're already in a group, add it to the file
                if let Some(group) = current_group.take() {
                    ignore_file.add_group(group);
                }
                
                // Start a new group
                let group_name = captures.get(1).unwrap().as_str();
                current_group = Some(PatternGroup::new(group_name));
                continue;
            }
            
            // Check for group end
            if GROUP_END_RE.is_match(&line) {
                // Add the current group to the file
                if let Some(group) = current_group.take() {
                    ignore_file.add_group(group);
                } else {
                    return Err(anyhow!("Unexpected group end at line {}", line_number));
                }
                continue;
            }
            
            // Parse pattern
            let pattern = Pattern::new(&line);
            
            // Add pattern to current group or directly to file
            if let Some(group) = &mut current_group {
                group.add_pattern(pattern);
            } else {
                ignore_file.add_pattern(pattern);
            }
        }
        
        // Add any remaining group
        if let Some(group) = current_group.take() {
            ignore_file.add_group(group);
        }
        
        // Create DotIgnore from IgnoreFile
        let mut vcs_ignore = DotIgnore::new();
        for pattern in ignore_file.patterns() {
            vcs_ignore.add_pattern(pattern.clone());
        }
        
        for group in ignore_file.groups() {
            vcs_ignore.add_group(group.clone());
        }
        
        Ok(vcs_ignore)
    }
    
    /// Parse a string as a .DotIgnore file content
    pub fn parse_string(&self, content: &str) -> Result<DotIgnore> {
        let mut ignore_file = IgnoreFile::new(Path::new(".DotIgnore"));
        
        // State variables for parsing
        let mut current_group: Option<PatternGroup> = None;
        let mut line_number = 0;
        
        for line in content.lines() {
            line_number += 1;
            
            // Skip empty lines
            if line.trim().is_empty() {
                continue;
            }
            
            // Skip comment lines (but not comments at end of line)
            if line.trim_start().starts_with('#') {
                continue;
            }
            
            // Check for group start
            if let Some(captures) = GROUP_START_RE.captures(line) {
                // If we're already in a group, add it to the file
                if let Some(group) = current_group.take() {
                    ignore_file.add_group(group);
                }
                
                // Start a new group
                let group_name = captures.get(1).unwrap().as_str();
                current_group = Some(PatternGroup::new(group_name));
                continue;
            }
            
            // Check for group end
            if GROUP_END_RE.is_match(line) {
                // Add the current group to the file
                if let Some(group) = current_group.take() {
                    ignore_file.add_group(group);
                } else {
                    return Err(anyhow!("Unexpected group end at line {}", line_number));
                }
                continue;
            }
            
            // Parse pattern
            let pattern = Pattern::new(line);
            
            // Add pattern to current group or directly to file
            if let Some(group) = &mut current_group {
                group.add_pattern(pattern);
            } else {
                ignore_file.add_pattern(pattern);
            }
        }
        
        // Add any remaining group
        if let Some(group) = current_group.take() {
            ignore_file.add_group(group);
        }
        
        // Create DotIgnore from IgnoreFile
        let mut vcs_ignore = DotIgnore::new();
        for pattern in ignore_file.patterns() {
            vcs_ignore.add_pattern(pattern.clone());
        }
        
        for group in ignore_file.groups() {
            vcs_ignore.add_group(group.clone());
        }
        
        Ok(vcs_ignore)
    }
}

impl Default for IgnoreParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;
    
    #[test]
    fn test_parse_simple_file() {
        let content = r#"
# This is a comment
[standard_patterns] {
    *.txt
    !important.txt
    temp/
}

[platform:windows] {
    Thumbs.db
    Desktop.ini
}
"#;
        
        let mut file = NamedTempFile::new().unwrap();
        write!(file, "{}", content).unwrap();
        
        let parser = IgnoreParser::new();
        let vcs_ignore = parser.parse_file(file.path()).unwrap();
        
        // Check patterns and groups
        assert_eq!(vcs_ignore.patterns.len(), 0);
        assert_eq!(vcs_ignore.groups.len(), 2);
        
        // Ensure groups have the correct patterns
        let standard_group = vcs_ignore.groups.iter().find(|g| g.name == "standard_patterns").unwrap();
        assert_eq!(standard_group.patterns.len(), 3);

        let windows_group = vcs_ignore.groups.iter().find(|g| g.name == "platform:windows").unwrap();
        assert_eq!(windows_group.patterns.len(), 2);
        
        // Check ignore functionality
        assert!(vcs_ignore.is_ignored("file.txt"));
        assert!(!vcs_ignore.is_ignored("important.txt"));
        assert!(vcs_ignore.is_ignored("temp/file"));
        assert!(vcs_ignore.is_ignored("Thumbs.db"));
        assert!(!vcs_ignore.is_ignored("file.rs"));
    }
    
    #[test]
    fn test_parse_string() {
        let content = r#"
# This is a comment
[standard_patterns] {
    *.txt
    !important.txt
}
"#;
        
        let parser = IgnoreParser::new();
        let vcs_ignore = parser.parse_string(content).unwrap();
        
        // Check we have expected patterns and groups
        assert_eq!(vcs_ignore.patterns.len(), 0);
        assert_eq!(vcs_ignore.groups.len(), 1);
        
        // Ensure group has the correct patterns
        let standard_group = vcs_ignore.groups.iter().find(|g| g.name == "standard_patterns").unwrap();
        assert_eq!(standard_group.patterns.len(), 2);
        
        // Check ignore functionality
        assert!(vcs_ignore.is_ignored("file.txt"));
        assert!(!vcs_ignore.is_ignored("important.txt"));
        assert!(!vcs_ignore.is_ignored("file.rs"));
    }
    
    #[test]
    fn test_invalid_file() {
        let parser = IgnoreParser::new();
        let result = parser.parse_file(Path::new("/non/existent/file"));
        assert!(result.is_err());
    }

    #[test]
    fn test_mismatched_groups() {
        let content = r#"
# This is a comment
[standard_patterns] {
    *.txt
}

# Missing closing bracket for this group
[unclosed_group] {
    *.obj
"#;
        
        let parser = IgnoreParser::new();
        let vcs_ignore = parser.parse_string(content).unwrap();
        
        // Both groups should be present in the result
        assert_eq!(vcs_ignore.groups.len(), 2);
        
        // Check that both groups exist
        assert!(vcs_ignore.groups.iter().any(|g| g.name == "standard_patterns"));
        assert!(vcs_ignore.groups.iter().any(|g| g.name == "unclosed_group"));
        
        // Verify the unclosed group has patterns
        let unclosed = vcs_ignore.groups.iter().find(|g| g.name == "unclosed_group").unwrap();
        assert_eq!(unclosed.patterns.len(), 1);
        assert_eq!(unclosed.patterns[0].pattern, "*.obj");
    }
} 
