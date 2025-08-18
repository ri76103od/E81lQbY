use warp::{Filter, Rejection, Reply, Reply as WarpReply};
use warp::http::Response;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::error::Error;
use std::fmt::{self, Formatter};
use std::result::Result as StdResult;
use serde_json::json;

// Define an error type for the application
#[derive(Debug)]
enum AppError {
    FileError(String),
    SerializationError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            AppError::FileError(ref err) => write!(f, "File error: {}", err),
            AppError::SerializationError(ref err) => write!(f, "Serialization error: {}", err),
        }
    }
}

impl Error for AppError {}

// Define a TestResult struct to hold test report data
#[derive(Serialize, Deserialize)]
struct TestResult {
    test_name: String,
    test_result: bool,
    test_message: String,
}

// Generate a test report
async fn generate_report(results: Vec<TestResult>) -> StdResult<String, AppError> {
    // Serialize the test results to JSON
    let report = serde_json::to_string(&results).map_err(|e| AppError::SerializationError(e.to_string()))?;

    // Save the report to a file
    let mut file = File::create("test_report.json").map_err(|e| AppError::FileError(e.to_string()))?;
    file.write_all(report.as_bytes()).map_err(|e| AppError::FileError(e.to_string()))?;

    // Return the report as a JSON string
    Ok(report)
}

// Warp route to handle generating a test report
fn generate_report_route() -> impl Filter<Extract = impl WarpReply, Error = Rejection> + Clone {
    warp::post()
        .and(warp::path("generate-report"))
        .and(warp::body::json::<Vec<TestResult>>())
        .and_then(|results: Vec<TestResult>| async move {
            generate_report(results).await.map_err(|e| warp::reject::custom(e))
        })
        .map(warp::reply::json)
}

fn main() {
    // Start the Warp server on port 3030
    let generate_report_route_filter = generate_report_route();
    warp::serve(generate_report_route_filter)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
