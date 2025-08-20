use warp::Filter;
use std::process::{Command, Output};
use warp::http::StatusCode;
use warp::reply::Response;
use serde::{Serialize, Deserialize};
use serde_json::json;
use std::error::Error;

// Response structure for process management
#[derive(Serialize, Deserialize, Debug)]
struct ProcessInfo {
    name: String,
    status: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Define the route for listing processes
    let processes_route = warp::path("processes")
        .and_then(|| list_processes());

    // Define the route for starting a process
    let start_process_route = warp::path("start")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(start_process);

    // Define the route for stopping a process
    let stop_process_route = warp::path("stop\)
        .and(warp::post())
        .and(warp::body::json())
        .and_then(stop_process);

    // Combine routes and start the server
    let routes = processes_route.or(start_process_route).or(stop_process_route);
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

// Function to list all active processes
async fn list_processes() -> Result<impl warp::Reply, warp::Rejection> {
    let output = Command::new("ps").args(&["-aux"]).output()?;
    if !output.status.success() {
        return Ok(warp::reply::json(&json!({
            "error": "Failed to list processes",
        })));
    }
    let processes = String::from_utf8_lossy(&output.stdout).to_string();
    Ok(warp::reply::json(&json!({
        "processes": processes,
    })));
}

// Function to start a new process
async fn start_process(process_info: ProcessInfo) -> Result<impl warp::Reply, warp::Rejection> {
    let output = Command::new(&process_info.name)
        .status()
        .map_err(|e| warp::reject::custom(e))?;
    if output.code() == Some(0) {
        Ok(warp::reply::json(&json!({
            "message": "Process started successfully",
        })));
    } else {
        Ok(warp::reply::json(&json!({
            "error": "Failed to start process",
        })));
    }
}

// Function to stop an existing process
async fn stop_process(process_info: ProcessInfo) -> Result<impl warp::Reply, warp::Rejection> {
    let output = Command::new("kill")
        .arg(&process_info.name)
        .status()
        .map_err(|e| warp::reject::custom(e))?;
    if output.code() == Some(0) {
        Ok(warp::reply::json(&json!({
            "message": "Process stopped successfully",
        })));
    } else {
        Ok(warp::reply::json(&json!({
            "error": "Failed to stop process",
        })));
    }
}
