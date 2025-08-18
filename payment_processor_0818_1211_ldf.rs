use warp::Filter;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::error::Error;
use std::fmt;
use std::result::Result;

// Error types for the application
#[derive(Debug, Clone)]
enum PaymentError {
    InvalidInput(String),
    PaymentFailed(String),
}

// Implementing Display for PaymentError to provide user-friendly error messages
impl fmt::Display for PaymentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PaymentError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            PaymentError::PaymentFailed(msg) => write!(f, "Payment failed: {}", msg),
        }
    }
}

// Implementing Error trait for PaymentError
impl Error for PaymentError {}

// Data structure for the payment request
#[derive(Deserialize)]
struct PaymentRequest {
    amount: f64,
    currency: String,
    customer_id: u32,
}

// Handlers for the payment process
async fn process_payment(req: PaymentRequest) -> Result<impl warp::Reply, PaymentError> {
    // Example payment logic (to be replaced with actual payment processing)
    if req.amount <= 0.0 {
        return Err(PaymentError::InvalidInput("Amount must be greater than zero".to_string()));
    }

    // Simulate payment processing
    println!("Processing payment of {} {} for customer id {}", req.amount, req.currency, req.customer_id);
    // If payment processing fails, return an error
    // For demonstration, always succeed
    Ok(warp::reply::json(&json!({
        "status": "success",
        "message": format!("Payment of {} {} processed successfully for customer id {}", req.amount, req.currency, req.customer_id),
    })))
}

// Setting up the Warp filter
fn payment_route() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post()
        .and(warp::path("payment"))
        .and(warp::body::json())
        .and_then(process_payment)
}

// Main function to start the Warp server
#[tokio::main]
async fn main() {
    println!("Starting payment processor...");
    let payment_route = payment_route();
    warp::serve(payment_route).run(([127, 0, 0, 1], 3030)).await;
}
