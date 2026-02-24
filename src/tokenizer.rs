/// Provides functionality for tokenizing and normalizing text.

/// Tokenizes the input text into words and normalizes them to lowercase.
///
/// # Arguments
/// * `text` - A string slice containing the text to be tokenized.
///
/// # Returns
/// A `Vec<String>` containing the normalized tokens.
///
/// # Example
/// ```
/// use tokenizer::tokenize_and_normalize;
/// let tokens = tokenize_and_normalize("Hello World!");
/// assert_eq!(tokens, vec!["hello", "world!"]);
/// ```
pub fn tokenize_and_normalize(text: &str) -> Vec<String> {
    text.split_whitespace() // Split text by whitespace
        .map(|word| word.to_lowercase()) // Normalize each word to lowercase
        .collect() // Collect into a vector
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_and_normalize() {
        let input = "Hello World! This is a Test.";
        let expected = vec!["hello", "world!", "this", "is", "a", "test."];
        assert_eq!(tokenize_and_normalize(input), expected);
    }

    #[test]
    fn test_tokenize_and_normalize_empty_string() {
        let input = "";
        let expected: Vec<String> = vec![];
        assert_eq!(tokenize_and_normalize(input), expected);
    }

    #[test]
    fn test_tokenize_and_normalize_mixed_case() {
        let input = "Rust is AWESOME!";
        let expected = vec!["rust", "is", "awesome!"];
        assert_eq!(tokenize_and_normalize(input), expected);
    }
}