mod document;

use document::Document;
use std::collections::HashMap;
use std::sync::Mutex;

/// Global storage for documents.
/// Using a Mutex to ensure thread-safe access.
pub static DOCUMENTS: Mutex<HashMap<String, Document>> = Mutex::new(HashMap::new());

/// Global storage for indexes.
/// Placeholder for future index implementation.
pub static INDEXES: Mutex<HashMap<String, Vec<String>>> = Mutex::new(HashMap::new());

/// Adds a document to the global storage.
///
/// # Arguments
/// * `document` - The document to add.
pub fn add_document(document: Document) {
    let mut docs = DOCUMENTS.lock().unwrap();
    docs.insert(document.id.clone(), document);
}

/// Retrieves a document by its ID.
///
/// # Arguments
/// * `id` - The ID of the document to retrieve.
///
/// # Returns
/// * `Option<Document>` - The document if found, otherwise `None`.
pub fn get_document(id: &str) -> Option<Document> {
    let docs = DOCUMENTS.lock().unwrap();
    docs.get(id).cloned()
}