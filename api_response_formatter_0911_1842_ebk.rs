use warp::Filter;

#[tokio::main]
async fn main() {
    // Define the API endpoint for the formatter
    let formatter_route = warp::path("api")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(api_response_formatter);

    // Start the Warp server on localhost:3030
    warp::serve(formatter_route)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

// Handler function for the /api endpoint
async fn api_response_formatter(data: serde_json::Value) -> Result<impl warp::Reply, warp::Rejection> {
    // Attempt to format the input data and return a response
    match format_response(&data) {
        Ok(formatted_response) => Ok(warp::reply::json(&formatted_response)),
        Err(error) => Err(warp::reject::custom(error)),
    }
}

// Function to format API responses
fn format_response(data: &serde_json::Value) -> Result<serde_json::Value, ErrorResponse> {
    // Example of formatting logic, to be replaced with actual formatting implementation
    // This is a simple example, real-world scenarios might require more complex handling
    if data.is_object() {
        let mut formatted = serde_json::Map::new();
        for (key, value) in data.as_object().unwrap().iter() {
            // Here we just capitalize the keys for demonstration purposes
            formatted.insert(key.to_owned(), value.clone());
        }
        Ok(serde_json::Value::Object(formatted))
    } else {
        Err(ErrorResponse { message: "Invalid input data".to_string() })
    }
}

// Custom error response structure
#[derive(Debug, serde::Serialize)]
struct ErrorResponse {
    message: String,
}

// Implement Display trait for ErrorResponse to return a human-readable error message
impl std::fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

// Implement Warp's Reject trait to be able to use ErrorResponse in rejections
impl warp::reject::Reject for ErrorResponse {}
