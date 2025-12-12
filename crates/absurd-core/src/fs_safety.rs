//! Safe filesystem operations for ABSURDTTY.
//!
//! This module provides utilities for:
//! - XDG-compliant path resolution
//! - Atomic file writes (write to temp, then rename)
//! - Read-only file access with explicit safety guarantees
//! - Path validation and sanitization
//!
//! # Privacy Commitment
//!
//! ABSURDTTY is local-first. This module enforces that by:
//! - Never making network requests
//! - Only reading files the user explicitly allows
//! - Writing only to user-owned directories
//! - Providing clear audit trails for what files are accessed

use anyhow::{Context, Result};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

/// Standard paths used by ABSURDTTY.
pub struct Paths;

impl Paths {
    /// Get the ABSURDTTY data directory.
    ///
    /// Follows XDG Base Directory Specification:
    /// - `$XDG_DATA_HOME/absurdtty` if set
    /// - `~/.local/share/absurdtty` otherwise
    ///
    /// Creates the directory if it doesn't exist.
    pub fn data_dir() -> Result<PathBuf> {
        let base = dirs::data_dir()
            .context("Could not determine data directory")?;
        let path = base.join("absurdtty");

        if !path.exists() {
            fs::create_dir_all(&path)
                .with_context(|| format!("Failed to create data directory: {:?}", path))?;
        }

        Ok(path)
    }

    /// Get the ABSURDTTY config directory.
    ///
    /// Follows XDG Base Directory Specification:
    /// - `$XDG_CONFIG_HOME/absurdtty` if set
    /// - `~/.config/absurdtty` otherwise
    ///
    /// Does NOT create the directory (config is optional).
    pub fn config_dir() -> Result<PathBuf> {
        let base = dirs::config_dir()
            .context("Could not determine config directory")?;
        Ok(base.join("absurdtty"))
    }

    /// Get the default mood file path.
    ///
    /// Returns: `~/.local/share/absurdtty/mood.json`
    pub fn mood_file() -> Result<PathBuf> {
        Ok(Self::data_dir()?.join("mood.json"))
    }

    /// Get the path to the user's shell history.
    ///
    /// Attempts to detect the current shell and find its history file.
    /// Supports: bash, zsh, fish
    ///
    /// Returns the path if found, None if undetectable.
    pub fn shell_history() -> Option<PathBuf> {
        // Try to detect shell from SHELL environment variable
        let shell = std::env::var("SHELL").ok()?;

        let home = dirs::home_dir()?;

        if shell.contains("zsh") {
            let path = home.join(".zsh_history");
            if path.exists() {
                return Some(path);
            }
            // Try ZDOTDIR location
            if let Ok(zdotdir) = std::env::var("ZDOTDIR") {
                let path = PathBuf::from(zdotdir).join(".zsh_history");
                if path.exists() {
                    return Some(path);
                }
            }
        }

        if shell.contains("bash") {
            let path = home.join(".bash_history");
            if path.exists() {
                return Some(path);
            }
        }

        if shell.contains("fish") {
            let path = home.join(".local/share/fish/fish_history");
            if path.exists() {
                return Some(path);
            }
        }

        None
    }

    /// Resolve a path, expanding `~` to home directory.
    pub fn expand_tilde(path: &str) -> PathBuf {
        if let Some(stripped) = path.strip_prefix("~/") {
            if let Some(home) = dirs::home_dir() {
                return home.join(stripped);
            }
        }
        PathBuf::from(path)
    }
}

/// A read-only file handle with audit information.
///
/// This wrapper makes it explicit that we're only reading,
/// not modifying the underlying file.
pub struct ReadOnlyFile {
    path: PathBuf,
    content: String,
}

impl ReadOnlyFile {
    /// Open a file for read-only access.
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref().to_path_buf();
        let mut file = File::open(&path)
            .with_context(|| format!("Failed to open file for reading: {:?}", path))?;

        let mut content = String::new();
        file.read_to_string(&mut content)
            .with_context(|| format!("Failed to read file: {:?}", path))?;

        Ok(Self { path, content })
    }

    /// Get the file path.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Get the file content.
    pub fn content(&self) -> &str {
        &self.content
    }

    /// Get content as lines iterator.
    pub fn lines(&self) -> impl Iterator<Item = &str> {
        self.content.lines()
    }

    /// Get the number of bytes read.
    pub fn len(&self) -> usize {
        self.content.len()
    }

    /// Check if the file is empty.
    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }
}

/// Atomic file writer that writes to a temp file then renames.
///
/// This prevents partial writes from corrupting the target file.
pub struct AtomicWriter {
    target: PathBuf,
    temp_path: PathBuf,
    file: File,
}

impl AtomicWriter {
    /// Create a new atomic writer for the given target path.
    ///
    /// Creates a temporary file in the same directory as the target.
    pub fn new(target: impl AsRef<Path>) -> Result<Self> {
        let target = target.as_ref().to_path_buf();

        // Ensure parent directory exists
        if let Some(parent) = target.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)
                    .with_context(|| format!("Failed to create directory: {:?}", parent))?;
            }
        }

        // Create temp file in same directory (for atomic rename)
        let temp_path = target.with_extension(format!(
            "{}.tmp.{}",
            target.extension().unwrap_or_default().to_string_lossy(),
            std::process::id()
        ));

        let file = File::create(&temp_path)
            .with_context(|| format!("Failed to create temp file: {:?}", temp_path))?;

        Ok(Self {
            target,
            temp_path,
            file,
        })
    }

    /// Write content to the file.
    pub fn write_all(&mut self, content: &[u8]) -> Result<()> {
        self.file
            .write_all(content)
            .with_context(|| "Failed to write to temp file")?;
        Ok(())
    }

    /// Write string content to the file.
    pub fn write_str(&mut self, content: &str) -> Result<()> {
        self.write_all(content.as_bytes())
    }

    /// Finish writing and atomically move to target path.
    ///
    /// This is the commit operation. If this succeeds, the file
    /// is guaranteed to contain the complete content.
    pub fn finish(self) -> Result<PathBuf> {
        // Flush and sync
        self.file
            .sync_all()
            .with_context(|| "Failed to sync temp file")?;

        // Atomic rename
        fs::rename(&self.temp_path, &self.target)
            .with_context(|| format!("Failed to rename temp file to: {:?}", self.target))?;

        Ok(self.target.clone())
    }
}

impl Drop for AtomicWriter {
    fn drop(&mut self) {
        // Clean up temp file if we didn't finish
        let _ = fs::remove_file(&self.temp_path);
    }
}

/// Write content atomically to a file.
///
/// Convenience function that combines AtomicWriter creation and commit.
pub fn write_atomic(path: impl AsRef<Path>, content: &str) -> Result<()> {
    let mut writer = AtomicWriter::new(path)?;
    writer.write_str(content)?;
    writer.finish()?;
    Ok(())
}

/// Write JSON content atomically to a file.
///
/// Serializes the value and writes it atomically.
pub fn write_json_atomic<T: serde::Serialize>(path: impl AsRef<Path>, value: &T) -> Result<()> {
    let json = serde_json::to_string_pretty(value)
        .with_context(|| "Failed to serialize to JSON")?;
    write_atomic(path, &json)
}

/// Check if a path is safe to read.
///
/// Returns an error if:
/// - Path contains `..` components (directory traversal)
/// - Path is absolute and outside home directory
pub fn validate_read_path(path: impl AsRef<Path>) -> Result<PathBuf> {
    let path = path.as_ref();

    // Check for directory traversal
    for component in path.components() {
        if matches!(component, std::path::Component::ParentDir) {
            anyhow::bail!("Path contains directory traversal (..) which is not allowed: {:?}", path);
        }
    }

    // If absolute, must be under home directory or /tmp
    if path.is_absolute() {
        let home = dirs::home_dir()
            .context("Could not determine home directory")?;

        let is_under_home = path.starts_with(&home);
        let is_under_tmp = path.starts_with("/tmp");

        if !is_under_home && !is_under_tmp {
            anyhow::bail!(
                "Absolute paths must be under home directory or /tmp: {:?}",
                path
            );
        }
    }

    Ok(path.to_path_buf())
}

/// Check if a file exists and is readable.
pub fn is_readable(path: impl AsRef<Path>) -> bool {
    let path = path.as_ref();
    path.exists() && path.is_file() && File::open(path).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn expand_tilde_works() {
        let expanded = Paths::expand_tilde("~/test");
        assert!(!expanded.to_string_lossy().contains('~'));
        assert!(expanded.to_string_lossy().contains("test"));
    }

    #[test]
    fn expand_tilde_preserves_absolute() {
        let path = "/absolute/path";
        let expanded = Paths::expand_tilde(path);
        assert_eq!(expanded.to_string_lossy(), path);
    }

    #[test]
    fn validate_read_path_rejects_traversal() {
        let result = validate_read_path("../../../etc/passwd");
        assert!(result.is_err());
    }

    #[test]
    fn validate_read_path_accepts_normal() {
        let result = validate_read_path("some/normal/path");
        assert!(result.is_ok());
    }

    #[test]
    fn data_dir_is_under_home() {
        if let Ok(dir) = Paths::data_dir() {
            let home = dirs::home_dir().unwrap();
            assert!(dir.starts_with(&home) || dir.starts_with("/tmp"));
        }
    }

    #[test]
    fn shell_history_detection() {
        // This test just ensures the function doesn't panic
        let _ = Paths::shell_history();
    }

    #[test]
    fn atomic_write_creates_file() {
        let temp_dir = env::temp_dir();
        let test_file = temp_dir.join("absurdtty_test_atomic.txt");

        // Clean up from previous runs
        let _ = fs::remove_file(&test_file);

        write_atomic(&test_file, "test content").unwrap();

        assert!(test_file.exists());
        let content = fs::read_to_string(&test_file).unwrap();
        assert_eq!(content, "test content");

        // Clean up
        let _ = fs::remove_file(&test_file);
    }

    #[test]
    fn read_only_file_works() {
        let temp_dir = env::temp_dir();
        let test_file = temp_dir.join("absurdtty_test_readonly.txt");

        fs::write(&test_file, "line1\nline2\nline3").unwrap();

        let rof = ReadOnlyFile::open(&test_file).unwrap();
        assert_eq!(rof.lines().count(), 3);
        assert!(rof.content().contains("line2"));

        // Clean up
        let _ = fs::remove_file(&test_file);
    }
}

