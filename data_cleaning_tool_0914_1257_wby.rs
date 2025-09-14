use warp::Filter;
use serde::Deserialize;
use serde_json::json;
use std::fs::File;
use std::io::{self, prelude::*};
use std::path::Path;
use regex::Regex;

// Define a struct to deserialize incoming JSON data
#[derive(Deserialize, Debug)]
struct Data {
    text: String,
}

// Define function to clean and preprocess data
fn clean_and_preprocess(text: &str) -> Result<String, String> {
    // Example of a simple regex pattern to clean text (replace non-alphanumeric characters)
    let re = Regex::new(r"[^a-zA-Z0-9\s]").unwrap();
    let cleaned = re.replace_all(text, "").to_string();

    // Further preprocessing can be added here
    // For example, convert to lowercase, remove extra spaces, etc.
    let preprocessed = cleaned.to_lowercase().replace(" ", "");

    Ok(preprocessed)
}

// Define a route that accepts POST requests with JSON data
fn data_cleaning_route() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post()
        .and(warp::path("clean"))
        .and(warp::json::json())
        .and_then(|data: Data| async move {
            match clean_and_preprocess(&data.text) {
                Ok(cleaned) => {
                    Ok(warp::reply::json(&json!({
                        "original": data.text,
                        "cleaned": cleaned
                    })));
                },
                Err(e) => {
                    Err(warp::reject::custom(e))
                },
            }
        })
}

#[tokio::main]
async fn main() {
    let route = data_cleaning_route();
    warp::serve(route)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
