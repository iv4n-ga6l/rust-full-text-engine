/// Main library file for the Full-Text Search engine.

// Modules
pub mod document;
pub mod global_state;

// Re-export commonly used items for convenience.
pub use document::Document;
pub use global_state::{add_document, get_document, list_document_ids};