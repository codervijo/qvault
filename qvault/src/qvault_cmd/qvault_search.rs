use reqwest::{blocking::get, header, Error};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use dotenv::dotenv;
use std::fmt;

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

// Implement Display for SearchResult
impl fmt::Display for SearchResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Format the error and status if present
        if let Some(ref err) = self.error {
            write!(f, "Error: {}\n", err)?;
        }

        if let Some(status_code) = self.status {
            write!(f, "Status: {}\n", status_code)?;
        }

        // Format the items if present
        if let Some(ref items) = self.items {
            for item in items {
                write!(f, "{}\n", item)?;
            }
        }

        Ok(())
    }
}

// Implement Display for SearchItem
impl fmt::Display for SearchItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Title: {}, URL: {}", self.title, self.url)
    }
}

impl SearchResult {
    pub fn title(&self) -> &str {
        if let Some(items) = &self.items {
            return &items[0].title;
        }
        ""
    }

    pub fn url(&self) -> &str {
        if let Some(items) = &self.items {
            return &items[0].url;
        }
        &"https://google.com"
    }

    pub fn snippet(&self) -> &str {
        if let Some(items) = &self.items {
            return &items[0].title;
        }
        "This is a snippet from the search result"
    }

    pub fn count(&self) -> usize {
        if let Some(items) = &self.items {
            return items.len();
        }
        0
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

    log_info("Doing brave search for query >>", format_args!("{}", query));
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
                                Ok(json) => {
                                    log_info("Parsed JSON: ",format_args!("abc {}", json));

                                    // Navigate to the "web.results" array
                                    let results = json["web"]["results"].as_array().ok_or("Invalid results format")?;
                                    log_info("VCVC Found results in JSON number:", format_args!("{}", results.len()));
                                    log_info("VCVC Title of first result ", format_args!("{}", results[0]["title"]));

                                    // Perform operations to extract the required data from `json`.
                                    let search_result = SearchResult {
                                        error: None, // Populate fields appropriately
                                        status: Some(200), // Example data
                                        items: Some(vec![SearchItem {
                                            title: results[0]["title"].to_string(),
                                            url: results[0]["url"].to_string(),
                                        }]),
                                    };

                                    Ok(search_result) // Return the constructed `SearchResult`

                                }
                                Err(err) => {
                                    eprintln!("Failed to parse JSON: {}", err);
                                    Err(format!("JSON parsing error: {}", err)) // Return an error wrapped in `Err`
                                }
                            }

/*
                            // Attempt to Parse JSON
                            match serde_json::from_str::<SearchResult>(&body) {
                                Ok(result) => {
                                    //println!("Parsed SearchResult: {:#?}", result);
                                    //log_info("Parsed SearchResult:", &[result]);
                                    log_info("Parsed SearchResult: {}", format_args!("def {}", result));
                                    Ok(result)
                                }
                                Err(json_err) => {
                                    eprintln!("Failed to parse JSON into SearchResult: {}", json_err);

                                    // Attempt to parse as generic JSON for further debugging
                                    match serde_json::from_str::<serde_json::Value>(&body) {
                                        Ok(generic_json) => {
                                            //println!("Parsed as generic JSON for inspection: {:#?}", generic_json);
                                            log_info("Parsed as generic JSON for inspection", format_args!("{}", generic_json));
                                        }
                                        Err(generic_err) => {
                                            eprintln!("Failed to parse as generic JSON: {}", generic_err);
                                        }
                                    }

                                    Err("Failed to parse response body.".to_string())
                                }
                            }
*/
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
