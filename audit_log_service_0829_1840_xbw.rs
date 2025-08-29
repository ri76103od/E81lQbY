It is designed to be clear, maintainable, and extensible.
*/

use warp::Filter;
use serde_json::json;
use std::fs::OpenOptions;
use std::io::{self, Write};

// Define the AuditLog struct to store the log details
struct AuditLog {
    event: String,
    timestamp: String,
    user_id: Option<String>,
    action: String,
}

// Function to create a new AuditLog instance
impl AuditLog {
    pub fn new(event: String, user_id: Option<String>, action: String) -> Self {
        let timestamp = chrono::Utc::now().to_rfc3339();
        AuditLog {
            event,
            timestamp,
            user_id,
            action,
        }
    }

    // Function to write the log to a file
    pub fn write_log(&self) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("audit_log.txt")?;
        writeln!(file, "{} - {} - {} - {}", self.timestamp, self.event, self.user_id.unwrap_or("".to_string()), self.action)?;
        Ok(())
    }
}

// Define the route that will handle the audit log requests
fn audit_log_route() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post()
        .and(warp::path("audit"))
        .and(warp::body::json())
        .and_then(handle_audit_log)
}

// The handler function for audit log requests
async fn handle_audit_log(body: AuditLog) -> Result<impl warp::Reply, warp::Rejection> {
    match body.write_log() {
        Ok(_) => Ok(warp::reply::json(&json!({"status": "success"}))),
        Err(e) => Ok(warp::reply::json(&json!({"status": "error", "message": e.to_string()}))),
    }
}

#[tokio::main]
async fn main() {
    let route = audit_log_route();
    warp::serve(route)
        .run(([127, 0, 0, 1], 3030))
        .await;
    println!("Audit Log Service running on http://127.0.0.1:3030");
}
