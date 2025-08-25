// text_file_analyzer.rs
// A simple Rust program using the Warp framework to analyze the content of text files.

use std::fs::File;
use std::io::{self, BufReader, BufRead};
use warp::Filter;

// Main function to start the server.
#[tokio::main]
async fn main() -> io::Result<()> {
    // Define route for the text analysis.
    let analyze = warp::path("analyze")
        .and(warp::post())
        .and(warp::fs::file_upload_limit(1024 * 1024)) // Limit file size to 1MB.
        .and_then(analyze_text_file);

    // Start the server on localhost port 3030.
    warp::serve(analyze).run(([127, 0, 0, 1], 3030)).await;
}

// Handler function to analyze text files.
async fn analyze_text_file(mut payload: warp::fs::Payload) -> Result<impl warp::Reply, warp::Rejection> {
    let mut file = match warp::fs::write_file("./uploaded.txt", &mut payload).await {
        Ok(f) => f,
        Err(e) => return Err(warp::reject::custom(e)),
    };

    // Read the file and perform analysis.
    let reader = BufReader::new(File::open(file).unwrap());
    let mut lines = reader.lines();
    let mut analysis_results = vec![];
    while let Some(line) = lines.next() {
        let line = line.unwrap();
        analysis_results.push(analyze_line(&line));
    }

    // Return the analysis results as JSON.
    let json = warp::reply::json(&analysis_results);
    Ok(json)
}

// Function to analyze a single line of text.
fn analyze_line(line: &str) -> AnalysisResult {
    // Example analysis: count the number of characters.
    let character_count = line.chars().count();
    AnalysisResult { character_count }
}

// Structure to hold analysis results.
#[derive(serde::Serialize)]
struct AnalysisResult {
    character_count: usize,
}

// Define the error type for custom rejections.
#[derive(Debug, Clone)]
struct UploadError(pub String);

impl warp::reject::Reject for UploadError {}

// Implement a custom rejection handler for the UploadError.
impl warp::reject::Reject for io::Error {}

// Implement a custom response for the UploadError.
fn reject_upload_error(err: UploadError, _reply: warp::reply::Reply) -> warp::reply::Reply {
    warp::reply::with_status(warp::reply::json(&err.0), warp::http::StatusCode::BAD_REQUEST)
}
