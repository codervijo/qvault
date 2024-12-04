use reqwest::{blocking::get, header, Error};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use dotenv::dotenv;

use crate::qvault_log;
use crate::qvault_log::log_info;

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchResult {
    error: Option<String>,
    status: Option<u16>,
    items: Option<Vec<SearchItem>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct SearchItem {
    title: String,
    url: String,
}

/*
#[derive(Serialize, Deserialize, Debug)]
pub struct SearchResult {
    title: String,
    url: String,
    snippet: String,
}
*/

impl SearchResult {
    pub fn title(&self) -> &str {
        if let Some(items) = &self.items {
            return &items[0].title;
        }
        ""
    }

    /*
    pub fn url(&self) -> &str {
        &self.url
    }
    */

    pub fn snippet(&self) -> &str {
        if let Some(items) = &self.items {
            return &items[0].title;
        }
        "This is a snippet from the search result"
    }
}

#[derive(Serialize, Debug)]
pub struct SearchErrorResponse {
    error: String,
}

impl SearchErrorResponse {
    pub fn error(&self) -> &str {
        &self.error
    }
}

/*
// Mocking the Brave search function; replace with actual implementation
pub fn search_brave(query: &str) -> Result<SearchResult, String> {
    if query.is_empty() {
        return Err("Query parameter 'q' is required.".to_string());
    }

    Ok(
        SearchResult {
            title: "Example Result".to_string(),
            url: "https://example.com".to_string(),
            snippet: "This is a snippet from the search result.".to_string(),
        },
    )
}
*/

/*
 * Command equivalent
 *  curl -s --compressed "https://api.search.brave.com/res/v1/web/search?q=brave+search"\
    -H "Accept: application/json"\
    -H "Accept-Encoding: gzip"\
	-H "X-Subscription-Token: BSA4als3AeUnLwszMAQalx8N2azrl1S")
 */
pub fn search_brave(query: &str) -> Result<SearchResult, String> {
    // Check if query is empty and return an error in JSON
    if query.trim().is_empty() {
        return Ok(SearchResult {
            error: Some("Search query cannot be empty.".to_string()),
            status: Some(400),
            items: None,
        });
    }

    qvault_log::log_info(&format!("Doing brave search for query {}", query).to_string());
    let mut api_key = env::var("BRAVE_SEARCH_API_KEY").ok();

    if api_key.is_none() {
        // Check if qvault.env exists and load it
        if fs::metadata("qvault.env").is_ok() {
            dotenv::from_path("qvault.env").ok();  // Load the .env file
            api_key = env::var("BRAVE_SEARCH_API_KEY").ok();  // Retry fetching the API key
        }
    }

    if api_key.is_none() {
        return Ok(SearchResult {
            error: Some("API key not found. Please set the 'BRAVE_SEARCH_API_KEY' environment variable or provide it in qvault.env.".to_string()),
            status: Some(401),
            items: None,
        });
    }

    let url = "https://api.search.brave.com/res/v1/web/search";
    let params = [("q", query)];

    let client = reqwest::blocking::Client::new();
    let response = client
        .get(url)
        .header("Accept", "application/json")
        .header("X-Subscription-Token", api_key.unwrap())
        .query(&params)
        .send();

        match response {
            Ok(resp) => {
                if resp.status().is_success() {
                    match resp.text() {
                        Ok(body) => {
                            /*
                            println!("Raw Body with Escapes: {:?}", body);
                            // Print as Hexadecimal for Debugging Unprintable Characters
                            for byte in body.as_bytes() {
                                print!("{:02X} ", byte);
                            }
                            println!();
                            */

                            // Attempt to parse as JSON
                            match serde_json::from_str::<serde_json::Value>(&body) {
                                Ok(json) => println!("Parsed JSON: {:#?}", json),
                                Err(err) => eprintln!("Failed to parse JSON: {}", err),
                            }

                            // Attempt to Parse JSON
                            match serde_json::from_str::<SearchResult>(&body) {
                                Ok(result) => {
                                    println!("Parsed SearchResult: {:#?}", result);
                                    Ok(result)
                                }
                                Err(json_err) => {
                                    eprintln!("Failed to parse JSON into SearchResult: {}", json_err);

                                    // Attempt to parse as generic JSON for further debugging
                                    match serde_json::from_str::<serde_json::Value>(&body) {
                                        Ok(generic_json) => {
                                            println!("Parsed as generic JSON for inspection: {:#?}", generic_json);
                                        }
                                        Err(generic_err) => {
                                            eprintln!("Failed to parse as generic JSON: {}", generic_err);
                                        }
                                    }

                                    Err("Failed to parse response body.".to_string())
                                }
                            }
                        }
                        Err(read_err) => {
                            eprintln!("Failed to read response body: {}", read_err);
                            Err("Failed to read response body.".to_string())
                        }
                    }
                } else {
                    eprintln!("HTTP Error: {}", resp.status());
                    Err(format!("HTTP Error: {}", resp.status()))
                }
            }
            Err(e) => {
                eprintln!("Request Error: {}", e);
                Err(format!("Request Error: {}", e))
            }
        }

}
