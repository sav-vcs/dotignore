// Module: ignore/converter.rs
//
// This module implements conversions from other ignore file formats (.gitignore, .svnignore)
// to the VCS .DotIgnore format.

use anyhow::{Result, Context, anyhow};
use std::fs::{self, File};
use std::io::{BufReader, Write};
use std::path::{Path, PathBuf};
use chrono::Local;
use walkdir::WalkDir;
use std::collections::HashMap;
use regex::Regex;
use lazy_static::lazy_static;

use super::pattern::{Pattern, PatternGroup, IgnoreFile};

lazy_static! {
    static ref WINDOWS_PATTERN: Regex = Regex::new(r"(?i)thumbs\.db|desktop\.ini|\.lnk|\$RECYCLE\.BIN").unwrap();
    static ref MACOS_PATTERN: Regex = Regex::new(r"(?i)\.DS_Store|\.AppleDouble|\.LSOverride|\._|\.[Ss]potlight-V100|\.Trashes").unwrap();
    static ref LINUX_PATTERN: Regex = Regex::new(r"(?i)\*~|\.directory|\.Trash").unwrap();
}

/// Represents the result of a conversion operation
#[derive(Debug)]
pub struct ConversionResult {
    /// Source file that was converted
    pub source_file: PathBuf,
    /// Destination file where the converted content was written
    pub destination_file: PathBuf,
    /// Number of patterns that were converted
    pub pattern_count: usize,
    /// Number of standard patterns
    pub standard_patterns: usize,
    /// Number of platform-specific patterns
    pub platform_patterns: HashMap<String, usize>,
}

impl ConversionResult {
    /// Get the number of patterns converted (for backward compatibility)
    pub fn patterns_converted(&self) -> usize {
        self.pattern_count
    }
    
    /// Get the number of groups created (for backward compatibility)
    pub fn groups_created(&self) -> usize {
        self.platform_patterns.len()
    }
}

/// Converter for ignore files
pub struct IgnoreConverter {
    // Add converter state here if needed
}

impl IgnoreConverter {
    /// Create a new IgnoreConverter
    pub fn new() -> Self {
        Self {}
    }
    
    /// Convert a file from one ignore format to DotIgnore
    pub fn convert_file(&self, source_path: &Path, destination_path: Option<&Path>) -> Result<ConversionResult> {
        // Determine the source format
        let format = self.determine_format(source_path)?;
        
        // Read the content of the source file
        let content = std::fs::read_to_string(source_path)?;
        
        // Convert based on format
        let (converted_content, stats) = match format {
            IgnoreFormat::Git => self.convert_from_git(&content),
            IgnoreFormat::Svn => self.convert_from_svn(&content),
        };
        
        // Determine destination path
        let dest_path = if let Some(path) = destination_path {
            path.to_path_buf()
        } else {
            let parent = source_path.parent().unwrap_or(Path::new("."));
            parent.join(".ignore")
        };
        
        // Write to the destination file
        std::fs::write(&dest_path, converted_content)?;
        
        // Return conversion result
        Ok(ConversionResult {
            source_file: source_path.to_path_buf(),
            destination_file: dest_path,
            pattern_count: stats.total_patterns,
            standard_patterns: stats.standard_patterns,
            platform_patterns: stats.platform_patterns,
        })
    }
    
    /// Convert all files in a directory
    pub fn convert_directory(&self, directory_path: &Path, recursive: bool) -> Result<Vec<ConversionResult>> {
        if !directory_path.is_dir() {
            return Err(anyhow!("Not a directory: {:?}", directory_path));
        }

        let mut results = Vec::new();
        
        let walker = if recursive {
            WalkDir::new(directory_path)
        } else {
            WalkDir::new(directory_path).max_depth(1)
        };

        for entry in walker {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() {
                if let Some(file_name) = path.file_name() {
                    let file_name_str = file_name.to_string_lossy();
                    
                    if file_name_str == ".gitignore" || file_name_str == ".svnignore" {
                        match self.convert_file(path, None) {
                            Ok(result) => results.push(result),
                            Err(e) => eprintln!("Error converting {:?}: {}", path, e),
                        }
                    }
                }
            }
        }
        
        Ok(results)
    }
    
    /// Determine the format of an ignore file
    fn determine_format(&self, path: &Path) -> Result<IgnoreFormat> {
        let file_name = path.file_name()
            .ok_or_else(|| anyhow!("Invalid file path"))?
            .to_string_lossy()
            .to_lowercase();
        
        if file_name.contains("git") {
            Ok(IgnoreFormat::Git)
        } else if file_name.contains("svn") {
            Ok(IgnoreFormat::Svn)
        } else {
            Err(anyhow!("Unsupported ignore file format: {}", file_name))
        }
    }
    
    /// Convert content from .gitignore format
    fn convert_from_git(&self, content: &str) -> (String, PatternStatistics) {
        let mut lines = content.lines();
        let mut converted = String::new();
        let mut stats = PatternStatistics::new();
        
        // Add header
        converted.push_str("# Converted from .gitignore\n");
        converted.push_str("# Format version: 1.0\n\n");
        
        // Predefined groups based on common patterns
        let mut windows_files_group = Vec::new();
        let mut macos_files_group = Vec::new();
        let mut linux_files_group = Vec::new();
        let mut temp_files_group = Vec::new();
        let mut build_group = Vec::new();
        let mut logs_group = Vec::new();
        let mut docs_group = Vec::new();
        let mut ide_group = Vec::new();
        let mut default_group = Vec::new();
        
        // Process lines
        while let Some(line) = lines.next() {
            let trimmed = line.trim();
            
            // Skip empty lines
            if trimmed.is_empty() {
                converted.push('\n');
                continue;
            }
            
            // Process comments
            if trimmed.starts_with('#') {
                let comment = trimmed[1..].trim();
                converted.push_str(&format!("# {}\n", comment));
                continue;
            }
            
            // Process pattern
            let pattern = trimmed.to_string();
            stats.total_patterns += 1;
            
            // Classify patterns by type
            if WINDOWS_PATTERN.is_match(&pattern) {
                if !windows_files_group.contains(&pattern) {
                    windows_files_group.push(pattern.clone());
                }
            } else if MACOS_PATTERN.is_match(&pattern) {
                if !macos_files_group.contains(&pattern) {
                    macos_files_group.push(pattern.clone());
                }
            } else if LINUX_PATTERN.is_match(&pattern) {
                if !linux_files_group.contains(&pattern) {
                    linux_files_group.push(pattern.clone());
                }
            } else if pattern.contains(".tmp") || pattern.contains(".swp") || pattern.contains(".bak") || pattern.contains(".~") {
                if !temp_files_group.contains(&pattern) {
                    temp_files_group.push(pattern.clone());
                }
            } else if pattern.contains(".o") || pattern.contains(".obj") || pattern.contains(".exe") || 
                      pattern.contains(".dll") || pattern.contains(".class") || pattern.contains("build/") ||
                      pattern.contains("/bin/") || pattern.contains("/dist/") || pattern.contains("/out/") {
                if !build_group.contains(&pattern) {
                    build_group.push(pattern.clone());
                }
            } else if pattern.contains(".log") {
                if !logs_group.contains(&pattern) {
                    logs_group.push(pattern.clone());
                }
            } else if pattern.contains(".md") || pattern.contains(".pdf") || pattern.contains("docs/") {
                if !docs_group.contains(&pattern) {
                    docs_group.push(pattern.clone());
                }
            } else if pattern.contains(".vscode") || pattern.contains(".idea") || pattern.contains(".sublime") || 
                      pattern.contains(".project") {
                if !ide_group.contains(&pattern) {
                    ide_group.push(pattern.clone());
                }
            } else {
                if !default_group.contains(&pattern) {
                    default_group.push(pattern.clone());
                    stats.standard_patterns += 1;
                }
            }
        }
        
        // Add groups in a specific order
        if !default_group.is_empty() {
            converted.push_str("\n[default] {\n");
            converted.push_str("    # Standard patterns\n");
            for pattern in &default_group {
                converted.push_str(&format!("    {}\n", pattern));
            }
            converted.push_str("}\n");
        }
        
        if !windows_files_group.is_empty() {
            converted.push_str("\n[windows_files] {\n");
            converted.push_str("    # Windows specific files\n");
            for pattern in &windows_files_group {
                converted.push_str(&format!("    {}\n", pattern));
            }
            converted.push_str("}\n");
            stats.platform_patterns.insert("windows_files".to_string(), windows_files_group.len());
        }
        
        if !macos_files_group.is_empty() {
            converted.push_str("\n[macos_files] {\n");
            converted.push_str("    # macOS specific files\n");
            for pattern in &macos_files_group {
                converted.push_str(&format!("    {}\n", pattern));
            }
            converted.push_str("}\n");
            stats.platform_patterns.insert("macos_files".to_string(), macos_files_group.len());
        }
        
        if !linux_files_group.is_empty() {
            converted.push_str("\n[linux_files] {\n");
            converted.push_str("    # Linux specific files\n");
            for pattern in &linux_files_group {
                converted.push_str(&format!("    {}\n", pattern));
            }
            converted.push_str("}\n");
            stats.platform_patterns.insert("linux_files".to_string(), linux_files_group.len());
        }
        
        if !temp_files_group.is_empty() {
            converted.push_str("\n[temporary_files] {\n");
            converted.push_str("    # Temporary and cache files\n");
            for pattern in &temp_files_group {
                converted.push_str(&format!("    {}\n", pattern));
            }
            converted.push_str("}\n");
            stats.platform_patterns.insert("temporary_files".to_string(), temp_files_group.len());
        }
        
        if !build_group.is_empty() {
            converted.push_str("\n[build_artifacts] {\n");
            converted.push_str("    # Build output and artifacts\n");
            for pattern in &build_group {
                converted.push_str(&format!("    {}\n", pattern));
            }
            converted.push_str("}\n");
            stats.platform_patterns.insert("build_artifacts".to_string(), build_group.len());
        }
        
        if !logs_group.is_empty() {
            converted.push_str("\n[logs] {\n");
            converted.push_str("    # Log files\n");
            for pattern in &logs_group {
                converted.push_str(&format!("    {}\n", pattern));
            }
            converted.push_str("}\n");
            stats.platform_patterns.insert("logs".to_string(), logs_group.len());
        }
        
        if !docs_group.is_empty() {
            converted.push_str("\n[documentation] {\n");
            converted.push_str("    # Documentation files\n");
            for pattern in &docs_group {
                converted.push_str(&format!("    {}\n", pattern));
            }
            converted.push_str("}\n");
            stats.platform_patterns.insert("documentation".to_string(), docs_group.len());
        }
        
        if !ide_group.is_empty() {
            converted.push_str("\n[ide_files] {\n");
            converted.push_str("    # IDE specific files\n");
            for pattern in &ide_group {
                converted.push_str(&format!("    {}\n", pattern));
            }
            converted.push_str("}\n");
            stats.platform_patterns.insert("ide_files".to_string(), ide_group.len());
        }
        
        (converted, stats)
    }

    /// Convert content from .svnignore format to .DotIgnore
    fn convert_from_svn(&self, content: &str) -> (String, PatternStatistics) {
        let mut group_patterns: HashMap<String, Vec<String>> = HashMap::new();
        let mut current_group = String::from("default");
        group_patterns.insert(current_group.clone(), Vec::new());
        
        let mut statistics = PatternStatistics::new();
        let mut last_line_was_comment = false;
        let mut last_comment = String::new();
        
        // Process each line
        for line in content.lines() {
            let trimmed = line.trim();
            
            // Skip empty lines
            if trimmed.is_empty() {
                last_line_was_comment = false;
                continue;
            }
            
            // Handle comments - potential group names
            if trimmed.starts_with('#') {
                last_comment = trimmed.trim_start_matches('#').trim().to_string();
                last_line_was_comment = true;
                continue;
            }
            
            // If there was a comment before and this line is not a comment, consider creating new group
            if last_line_was_comment && !trimmed.starts_with('#') {
                if !last_comment.is_empty() {
                    current_group = self.slugify(&last_comment);
                    
                    // Create group if it doesn't exist
                    if !group_patterns.contains_key(&current_group) {
                        group_patterns.insert(current_group.clone(), Vec::new());
                    }
                }
            }
            
            // SVN patterns can be space-separated
            let line_patterns: Vec<&str> = line.split_whitespace().collect();
            for pattern in line_patterns {
                if !pattern.is_empty() {
                    // Add pattern to current group
                    if let Some(patterns) = group_patterns.get_mut(&current_group) {
                        patterns.push(pattern.to_string());
                        statistics.total_patterns += 1;
                        
                        if current_group == "default" {
                            statistics.standard_patterns += 1;
                        } else {
                            *statistics.platform_patterns.entry(current_group.clone()).or_insert(0) += 1;
                        }
                    }
                }
            }
            
            last_line_was_comment = false;
        }
        
        let mut result = String::new();
        
        // Add header
        result.push_str(&format!("# Archivo .DotIgnore convertido desde .svnignore\n"));
        result.push_str(&format!("# Fecha de conversión: {}\n\n", Local::now().format("%Y-%m-%d %H:%M:%S")));
        
        // Add each group
        for (group_name, patterns) in group_patterns.iter() {
            if patterns.is_empty() {
                continue;
            }
            
            // Ensure we always have at least a default group
            if group_name == "default" || !group_patterns.contains_key("default") || group_patterns["default"].is_empty() {
                result.push_str(&format!("[{}] {{\n", group_name));
                for pattern in patterns {
                    result.push_str(&format!("    {}\n", pattern));
                }
                result.push_str("}\n\n");
            } else {
                result.push_str(&format!("[{}] {{\n", group_name));
                for pattern in patterns {
                    result.push_str(&format!("    {}\n", pattern));
                }
                result.push_str("}\n\n");
            }
        }
        
        // If no default group was added, add an empty one
        if !result.contains("[default]") {
            result = format!("[default] {{\n    # Default patterns\n}}\n\n{}", result);
        }
        
        (result, statistics)
    }

    /// Convert a text to a slug format (for group names)
    fn slugify(&self, text: &str) -> String {
        let mut slug = String::new();
        
        // Remove non-alphanumeric characters and replace spaces with underscores
        for c in text.chars() {
            if c.is_alphanumeric() {
                slug.push(c.to_ascii_lowercase());
            } else if c.is_whitespace() {
                slug.push('_');
            }
        }
        
        // If empty, use "default"
        if slug.is_empty() {
            return String::from("default");
        }
        
        slug
    }
}

impl Default for IgnoreConverter {
    fn default() -> Self {
        Self::new()
    }
}

/// Format of an ignore file
#[derive(Debug, PartialEq, Eq)]
enum IgnoreFormat {
    Git,
    Svn,
}

/// Statistics about patterns in a conversion
struct PatternStatistics {
    total_patterns: usize,
    standard_patterns: usize,
    platform_patterns: HashMap<String, usize>,
}

impl PatternStatistics {
    fn new() -> Self {
        Self {
            total_patterns: 0,
            standard_patterns: 0,
            platform_patterns: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_convert_gitignore() {
        let converter = IgnoreConverter::new();
        let temp_dir = tempdir().unwrap();
        
        // Create a test .gitignore file
        let git_ignore_path = temp_dir.path().join(".gitignore");
        let mut file = File::create(&git_ignore_path).unwrap();
        writeln!(file, "# This is a comment").unwrap();
        writeln!(file, "*.txt").unwrap();
        writeln!(file, "build/").unwrap();
        writeln!(file, "# Windows files").unwrap();
        writeln!(file, "Thumbs.db").unwrap();
        writeln!(file, "# macOS files").unwrap();
        writeln!(file, ".DS_Store").unwrap();
        
        // Convert the file
        let dest_path = temp_dir.path().join(".ignore");
        let result = converter.convert_file(&git_ignore_path, Some(&dest_path)).unwrap();
        
        // Check the conversion result
        assert_eq!(result.pattern_count, 4);
        assert_eq!(result.groups_created(), 3);
        
        // Check the destination file exists
        assert!(dest_path.exists());
        
        // Check the content
        let content = fs::read_to_string(&dest_path).unwrap();
        assert!(content.contains("[default]"));
        assert!(content.contains("[windows_files]"));
        assert!(content.contains("[macos_files]"));
        assert!(content.contains("*.txt"));
        assert!(content.contains("Thumbs.db"));
        assert!(content.contains(".DS_Store"));
    }

    #[test]
    fn test_convert_svnignore() {
        let converter = IgnoreConverter::new();
        let temp_dir = tempdir().unwrap();
        
        // Create a test .svnignore file
        let svn_ignore_path = temp_dir.path().join(".svnignore");
        let mut file = File::create(&svn_ignore_path).unwrap();
        writeln!(file, "# This is a comment").unwrap();
        writeln!(file, "*.obj *.bin *.exe").unwrap();
        writeln!(file, "# Build files").unwrap();
        writeln!(file, "build").unwrap();
        
        // Convert the file
        let dest_path = temp_dir.path().join(".ignore");
        let result = converter.convert_file(&svn_ignore_path, Some(&dest_path)).unwrap();
        
        // Check the conversion result
        assert_eq!(result.pattern_count, 4);
        
        // Check the destination file exists
        assert!(dest_path.exists());
        
        // Check the content
        let content = fs::read_to_string(&dest_path).unwrap();
        assert!(content.contains("[default]"));
        assert!(content.contains("[build_files]"));
        assert!(content.contains("*.obj"));
        assert!(content.contains("*.bin"));
        assert!(content.contains("*.exe"));
        assert!(content.contains("build"));
    }
} 
