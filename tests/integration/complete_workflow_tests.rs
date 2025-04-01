use ignore::{DotIgnore, ConversionResult, Pattern, PatternGroup};
use std::path::{Path, PathBuf};
use std::fs::{self, File};
use std::io::Write;
use tempfile::tempdir;

// Test fijo para que compile
