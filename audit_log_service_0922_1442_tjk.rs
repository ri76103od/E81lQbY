use warp::Filter;
use std::time::SystemTime;
use chrono::prelude::*;
use serde::Serialize;
use log::{info, error};
use warp::http::Response;
use warp::reject::Reject;

// Define an error type for handling common errors
#[derive(Debug, Clone)]
struct Error;

impl Reject for Error {}

// Define AuditLog struct to store log entry details
#[derive(Serialize)]
struct AuditLog {
    timestamp: String,
    event: String,
    details: String,
}

// Function to create a new audit log entry
fn create_audit_log(event: &str, details: &str) -> Result<AuditLog, Error> {
    let now = Utc::now();
    let timestamp = now.to_rfc3339();
    Ok(AuditLog {
        timestamp,
        event: event.to_string(),
        details: details.to_string(),
    })
}

// Warp filter to handle incoming requests and produce audit logs
fn audit_log_filter() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::any()
        .map(move || {
            let event = "HTTP Request Received";
            let details = format!("Request at: {}", Utc::now().to_rfc3339());
            match create_audit_log(event, &details) {
                Ok(log) => {
                    // Log the successful creation of an audit log
                    info!("Audit Log: {:#?}", log);
                    Ok::<_, Error>(Response::new(warp::reply::json(&log).into_response()))
                },
                Err(_) => {
                    // Log the error in case of audit log creation failure
                    error!("Failed to create audit log");
                    Err(warp::reject::custom(Error))
                },
            }
        })
        .recover(handle_rejection)
}

// Function to handle rejections and produce a meaningful error response
fn handle_rejection(err: warp::Rejection) -> Result<impl warp::Reply, warp::Rejection> {
    if err.is_not_found() {
        Ok(warp::reply::with_status(
            "Not Found",
            warp::http::StatusCode::NOT_FOUND,
        ))
    } else if let Some(custom_err) = err.find::<Error>() {
        Ok(warp::reply::with_status(
            "Internal Server Error",
            warp::http::StatusCode::INTERNAL_SERVER_ERROR,
        ))
    } else {
        Err(err)
    }
}

// Main function to start the Warp server
#[tokio::main]
async fn main() {
    let route = audit_log_filter();
    warp::serve(route)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
