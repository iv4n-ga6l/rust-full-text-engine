/// Represents a document in the Full-Text Search engine.
#[derive(Debug, Clone)]
pub struct Document {
    pub id: String, // Unique identifier for the document
    pub content: String, // The textual content of the document
}

impl Document {
    /// Creates a new document with the given ID and content.
    pub fn new(id: &str, content: &str) -> Self {
        Self {
            id: id.to_string(),
            content: content.to_string(),
        }
    }
}