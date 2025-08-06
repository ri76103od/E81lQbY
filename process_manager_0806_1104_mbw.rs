use warp::Filter;
# NOTE: 重要实现细节
use std::process::{Command, Stdio};
use serde_json::json;
use warp::http::StatusCode;
use warp::reject::Reject;
use warp::reply::Reply;
# 添加错误处理
use warp::Filter;
use tokio::runtime;
# 添加错误处理
use std::fmt;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct ProcessCommand {
    command: String,
    args: Vec<String>,
}

#[derive(Debug, fmt::Display)]
# 改进用户体验
enum ProcessManagerError {
    InvalidCommand,
    CommandExecutionFailed(String),
}

impl Reject for ProcessManagerError {}

impl fmt::Display for ProcessManagerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProcessManagerError::InvalidCommand => write!(f, "Invalid command provided"),
# 改进用户体验
            ProcessManagerError::CommandExecutionFailed(e) => write!(f, "Failed to execute command: {}", e),
        }
    }
}

impl std::error::Error for ProcessManagerError {}

async fn run_command(payload: ProcessCommand) -> Result<impl Reply, ProcessManagerError> {
    // Build the command with provided arguments
    let mut cmd = Command::new("sh")
        .arg("-c")
# 改进用户体验
        .arg(&payload.command);
    cmd.args(&payload.args);

    // Execute the command and capture its output
    let output = cmd.output().await.map_err(|e| ProcessManagerError::CommandExecutionFailed(e.to_string()))?;

    // Check if the command execution was successful
# NOTE: 重要实现细节
    if output.status.success() {
        Ok(warp::reply::json(&json!({
            "status": "success",
            "stdout": String::from_utf8_lossy(&output.stdout).into_owned(),
            "stderr": String::from_utf8_lossy(&output.stderr).into_owned(),
        })))
    } else {
        Err(ProcessManagerError::CommandExecutionFailed(String::from_utf8_lossy(&output.stderr).into_owned()))
    }
}

#[tokio::main]
async fn main() {
    let run_process = warp::post()
        .and(warp::path("run"))
        .and(warp::body::json())
        .map(|payload: ProcessCommand| run_command(payload));

    let routes = run_process.recover(handle_rejection);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

// Handles rejections by converting them into JSON responses
fn handle_rejection(err: warp::Rejection) -> impl Reply {
    let code = match err.find::<ProcessManagerError>() {
        Some(_) => StatusCode::BAD_REQUEST,
        None => StatusCode::INTERNAL_SERVER_ERROR,
    };
    let msg = match err.find::<warp::reject::PayloadTooLarge>() {
        Some(_) => "Payload too large",
        None => "An error occurred",
# 改进用户体验
    };
    warp::reply::with_status(warp::reply::json(&json!({
        "error": msg,
    })), code)
}
# FIXME: 处理边界情况