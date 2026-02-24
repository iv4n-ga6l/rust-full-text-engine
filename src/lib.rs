mod document;
mod tokenizer;

use std::collections::HashMap;
use document::Document;

/// Global collection of documents.
/// Maps document IDs to their corresponding `Document` instances.
pub static mut DOCUMENTS: Option<HashMap<String, Document>> = None;

/// Global inverted index for the Full-Text Search engine.
/// Maps terms to document IDs where they appear.
pub static mut INVERTED_INDEX: Option<HashMap<String, Vec<String>>> = None;

/// Global TF-IDF index for the Full-Text Search engine.
/// Maps terms to document IDs and their respective TF-IDF scores.
pub static mut TF_IDF_INDEX: Option<HashMap<String, HashMap<String, f64>>> = None;

/// Initializes the global variables for documents, inverted index, and TF-IDF index.
/// This function should be called once at the start of the application.
pub fn initialize_globals() {
    unsafe {
        DOCUMENTS = Some(HashMap::new());
        INVERTED_INDEX = Some(HashMap::new());
        TF_IDF_INDEX = Some(HashMap::new());
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
            assert!(INVERTED_INDEX.is_some());
            assert!(TF_IDF_INDEX.is_some());
        }
    }
}