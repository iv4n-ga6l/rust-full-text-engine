use std::collections::HashMap;
use crate::document::Document;
use crate::tokenizer::tokenize_and_normalize;

/// Calculates the term frequency (TF) for tokens in a document.
/// Returns a HashMap mapping terms to their frequency in the document.
pub fn calculate_term_frequency(content: &str) -> HashMap<String, usize> {
    let tokens = tokenize_and_normalize(content);
    let mut term_frequency = HashMap::new();

    for token in tokens {
        *term_frequency.entry(token).or_insert(0) += 1;
    }

    term_frequency
}

/// Builds an inverted index mapping terms to document IDs.
/// Returns a HashMap where keys are terms and values are vectors of document IDs.
pub fn build_inverted_index(documents: &HashMap<String, Document>) -> HashMap<String, Vec<String>> {
    let mut inverted_index = HashMap::new();

    for (doc_id, document) in documents {
        let tokens = tokenize_and_normalize(&document.content);
        for token in tokens {
            inverted_index.entry(token).or_insert_with(Vec::new).push(doc_id.clone());
        }
    }

    inverted_index
}

/// Calculates the inverse document frequency (IDF) for each term.
/// Returns a HashMap mapping terms to their IDF scores.
pub fn calculate_inverse_document_frequency(inverted_index: &HashMap<String, Vec<String>>, total_documents: usize) -> HashMap<String, f64> {
    let mut idf_scores = HashMap::new();

    for (term, doc_ids) in inverted_index {
        let doc_count = doc_ids.len();
        let idf = (total_documents as f64 / doc_count as f64).ln();
        idf_scores.insert(term.clone(), idf);
    }

    idf_scores
}

/// Builds the TF-IDF index by combining TF and IDF scores.
/// Returns a HashMap mapping terms to document IDs and their respective TF-IDF scores.
pub fn build_tf_idf_index(documents: &HashMap<String, Document>, idf_scores: &HashMap<String, f64>) -> HashMap<String, HashMap<String, f64>> {
    let mut tf_idf_index = HashMap::new();

    for (doc_id, document) in documents {
        let term_frequency = calculate_term_frequency(&document.content);

        for (term, tf) in term_frequency {
            let idf = idf_scores.get(&term).unwrap_or(&0.0);
            let tf_idf_score = tf as f64 * idf;

            tf_idf_index
                .entry(term.clone())
                .or_insert_with(HashMap::new)
                .insert(doc_id.clone(), tf_idf_score);
        }
    }

    tf_idf_index
}

/// Performs a TF-IDF search on the query and returns a map of document IDs to their TF-IDF scores.
pub fn perform_tf_idf_search(query: &str, tf_idf_index: &HashMap<String, HashMap<String, f64>>) -> HashMap<String, f64> {
    let query_tokens = tokenize_and_normalize(query);
    let mut scores = HashMap::new();

    for token in query_tokens {
        if let Some(doc_scores) = tf_idf_index.get(&token) {
            for (doc_id, score) in doc_scores {
                *scores.entry(doc_id.clone()).or_insert(0.0) += score;
            }
        }
    }

    scores
}

/// Performs a letter-by-letter search for suggestions based on the query.
/// Returns a vector of terms that match the query prefix.
pub fn perform_letter_by_letter_search(query: &str, inverted_index: &HashMap<String, Vec<String>>) -> Vec<String> {
    let query_lower = query.to_lowercase();
    inverted_index
        .keys()
        .filter(|term| term.starts_with(&query_lower))
        .cloned()
        .collect()
}

/// Ranks search results based on their TF-IDF scores.
/// Returns a sorted vector of tuples containing document IDs and their scores.
pub fn rank_search_results(scores: HashMap<String, f64>) -> Vec<(String, f64)> {
    let mut results: Vec<(String, f64)> = scores.into_iter().collect();
    results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    results
}

/// Handles search requests by combining results from TF-IDF and letter-by-letter searches.
/// Returns a tuple containing ranked TF-IDF results and letter-by-letter suggestions.
pub fn handle_search_request(query: &str, tf_idf_index: &HashMap<String, HashMap<String, f64>>, inverted_index: &HashMap<String, Vec<String>>) -> (Vec<(String, f64)>, Vec<String>) {
    let tf_idf_scores = perform_tf_idf_search(query, tf_idf_index);
    let ranked_results = rank_search_results(tf_idf_scores);
    let suggestions = perform_letter_by_letter_search(query, inverted_index);

    (ranked_results, suggestions)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::document::Document;

    #[test]
    fn test_calculate_term_frequency() {
        let content = "hello world hello";
        let tf = calculate_term_frequency(content);
        assert_eq!(tf.get("hello"), Some(&2));
        assert_eq!(tf.get("world"), Some(&1));
    }

    #[test]
    fn test_build_inverted_index() {
        let mut documents = HashMap::new();
        documents.insert("doc1".to_string(), Document::new("doc1", "hello world"));
        documents.insert("doc2".to_string(), Document::new("doc2", "world hello"));

        let inverted_index = build_inverted_index(&documents);
        assert_eq!(inverted_index.get("hello"), Some(&vec!["doc1".to_string(), "doc2".to_string()]));
        assert_eq!(inverted_index.get("world"), Some(&vec!["doc1".to_string(), "doc2".to_string()]));
    }

    #[test]
    fn test_calculate_inverse_document_frequency() {
        let mut inverted_index = HashMap::new();
        inverted_index.insert("hello".to_string(), vec!["doc1".to_string(), "doc2".to_string()]);
        inverted_index.insert("world".to_string(), vec!["doc1".to_string()]);

        let idf_scores = calculate_inverse_document_frequency(&inverted_index, 2);
        assert!(idf_scores.get("hello").unwrap() > idf_scores.get("world").unwrap());
    }

    #[test]
    fn test_rank_search_results() {
        let scores = HashMap::from([
            ("doc1".to_string(), 0.5),
            ("doc2".to_string(), 1.0),
        ]);

        let ranked = rank_search_results(scores);
        assert_eq!(ranked, vec![("doc2".to_string(), 1.0), ("doc1".to_string(), 0.5)]);
    }
}