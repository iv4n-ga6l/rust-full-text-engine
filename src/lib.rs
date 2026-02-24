mod document;

use std::collections::HashMap;
use document::Document;

/// Global collection of documents.
/// Maps document IDs to their corresponding `Document` instances.
pub static mut DOCUMENTS: Option<HashMap<String, Document>> = None;

/// Global index for the Full-Text Search engine.
/// This will be populated with the necessary data structures for indexing.
pub static mut INDEX: Option<HashMap<String, Vec<String>>> = None;

/// Initializes the global variables for documents and indexes.
/// This function should be called once at the start of the application.
pub fn initialize_globals() {
    unsafe {
        DOCUMENTS = Some(HashMap::new());
        INDEX = Some(HashMap::new());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_globals() {
        initialize_globals();
        unsafe {
            assert!(DOCUMENTS.is_some());
            assert!(INDEX.is_some());
        }
    }
}