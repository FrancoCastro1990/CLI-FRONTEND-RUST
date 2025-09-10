//! # Types Module
//!
//! Common types and type aliases used throughout the CLI Frontend Generator.
//! This module provides type safety and better code documentation through
//! strong typing.

use std::path::PathBuf;
use thiserror::Error;

/// Output path wrapper for type safety
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutputPath(PathBuf);

impl OutputPath {
    /// Create a new output path
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self(path.into())
    }

    /// Get the path as a PathBuf reference
    pub fn as_path(&self) -> &std::path::Path {
        &self.0
    }
}

impl From<PathBuf> for OutputPath {
    fn from(path: PathBuf) -> Self {
        Self(path)
    }
}

/// Template content wrapper
#[derive(Debug, Clone)]
pub struct TemplateContent(String);

impl TemplateContent {
    /// Create new template content
    pub fn new(content: impl Into<String>) -> Self {
        Self(content.into())
    }

    /// Get content as string slice
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for TemplateContent {
    fn from(content: String) -> Self {
        Self(content)
    }
}

/// File system operation errors
#[derive(Error, Debug)]
pub enum FileSystemError {
    #[error("File operation failed: {operation} on {path}: {source}")]
    OperationFailed {
        operation: String,
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Directory creation failed: {path}: {source}")]
    DirectoryCreationFailed {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("File not found: {path}")]
    FileNotFound { path: PathBuf },
}

/// Result type for file system operations
pub type FileSystemResult<T> = Result<T, FileSystemError>;
