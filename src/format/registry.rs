//! Handles registration of format readers and writers.
//!
//! This module makes it possible to integrate readers and writers from foreign crates
//! with multimesh.

/// Returns a [FormatRegistry] containing all the available within crate formats.
///
/// If you want additional formats, you have to add them manually.
pub fn default_registry() -> FormatRegistry {
    unimplemented!()
}

/// Manages a set of available formats.
pub struct FormatRegistry {
    formats: Vec<FormatEntry>,
}

pub struct FormatEntry {
    /// Name of the format.
    name: String,

    read_handler: Option<Box<dyn ReadHandler>>,
    write_handler: Option<Box<dyn WriteHandler>>,
}

pub trait ReadHandler {}

pub trait WriteHandler {}

impl FormatRegistry {
    /// Create an empty registry.
    pub fn empty() -> FormatRegistry {
        FormatRegistry {
            formats: Vec::new(),
        }
    }

    pub fn formats(&self) -> &[FormatEntry] {
        &self.formats
    }

    pub fn register(&mut self, entry: FormatEntry) {
        self.formats.push(entry);
    }
}
