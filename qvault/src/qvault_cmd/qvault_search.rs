use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct SearchResult {
    title: String,
    url: String,
    snippet: String,
}

#[derive(Serialize, Debug)]
struct ErrorResponse {
    error: String,
}

// Mocking the Brave search function; replace with actual implementation
pub fn search_brave(query: &str) -> Result<Vec<SearchResult>, String> {
    if query.is_empty() {
        return Err("Query parameter 'q' is required.".to_string());
    }

    Ok(vec![
        SearchResult {
            title: "Example Result".to_string(),
            url: "https://example.com".to_string(),
            snippet: "This is a snippet from the search result.".to_string(),
        },
    ])
}