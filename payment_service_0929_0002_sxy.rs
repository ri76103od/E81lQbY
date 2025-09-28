// payment_service.rs
// This file contains a simple payment processing service using Rust and Warp.

use warp::Filter;
use warp::http::StatusCode;
use serde::Deserialize;
use serde_json::json;
use std::error::Error;

// Define a struct to represent the payment request data.
#[derive(Debug, Deserialize)]
struct PaymentRequest {
    amount: f64,
    currency: String,
    description: String,
}

// Define a struct to represent the payment response data.
#[derive(Debug, Serialize)]
struct PaymentResponse {
    transaction_id: String,
    status: String,
    amount: f64,
    currency: String,
}

// Function to simulate processing a payment.
// In a real-world scenario, this would interact with a payment gateway.
async fn process_payment(req: PaymentRequest) -> Result<PaymentResponse, Box<dyn Error>> {
    // Simulate some processing logic here.
    // For example, you might validate the request, check for fraud, etc.
    // For now, we'll just return a success response.
    Ok(PaymentResponse {
        transaction_id: "TXN123456".to_string(),
        status: "success".to_string(),
        amount: req.amount,
        currency: req.currency,
    })
}

// Define the route for the payment processing endpoint.
fn create_payment_route() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post()
        .and(warp::path("process_payment"))
        .and(warp::body::json())
        .and_then(handle_process_payment)
}

// The handler function for the payment processing endpoint.
async fn handle_process_payment(req: PaymentRequest) -> Result<impl warp::Reply, warp::Rejection> {
    match process_payment(req).await {
        Ok(response) => Ok(warp::reply::json(&response)),
        Err(e) => {
            // Handle errors appropriately.
            // Log the error, and return a 500 Internal Server Error response.
            eprintln!("Error processing payment: {}", e);
            Ok(warp::reply::with_status(json!({"error": e.to_string()}), StatusCode::INTERNAL_SERVER_ERROR))
        }
    }
}

// Main function to run the Warp server.
#[tokio::main]
async fn main() {
    // Define the route and start the server.
    let route = create_payment_route();
    println!("Starting payment service on port 3030");
    warp::serve(route)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
