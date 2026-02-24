use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::document::Document;

/// Global state for managing documents and indexes.
/// Documents are stored in a HashMap with their ID as the key.
/// Indexes will be implemented later.

pub type DocumentStore = Arc<Mutex<HashMap<String, Document>>>;

lazy_static::lazy_static! {
    /// Global storage for documents.
    pub static ref DOCUMENTS: DocumentStore = Arc::new(Mutex::new(HashMap::new()));

    /// Placeholder for indexes (to be implemented later).
    pub static ref INDEXES: Arc<Mutex<HashMap<String, Vec<String>>>> = Arc::new(Mutex::new(HashMap::new()));
}

/// Adds a document to the global document store.
pub fn add_document(document: Document) {
    let mut documents = DOCUMENTS.lock().unwrap();
    documents.insert(document.id.clone(), document);
}

/// Retrieves a document by its ID.
pub fn get_document(id: &str) -> Option<Document> {
    let documents = DOCUMENTS.lock().unwrap();
    documents.get(id).cloned()
}

/// Lists all document IDs in the store.
pub fn list_document_ids() -> Vec<String> {
    let documents = DOCUMENTS.lock().unwrap();
    documents.keys().cloned().collect()
}