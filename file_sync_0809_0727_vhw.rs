use std::fs;
use std::io;
use std::path::Path;
use warp::Filter;

// Define error types for file operations.
#[derive(Debug)]
enum FileError {
    NotFound,
    PermissionDenied,
    IoError(io::Error),
}

// Implement From trait to convert IoError to FileError.
impl From<io::Error> for FileError {
    fn from(err: io::Error) -> Self {
        FileError::IoError(err)
    }
}

// Function to copy a file from source to destination.
async fn copy_file(source: String, destination: String) -> Result<(), FileError> {
    let src = Path::new(&source);
    let dst = Path::new(&destination);

    if !src.is_file() {
        return Err(FileError::NotFound);
    }

    fs::copy(src, dst).await.map_err(FileError::from)
}

// Warp route to handle file copy requests.
fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post()
        .and(warp::path("copy"))
        .and(warp::body::json())
        .and_then(handle_copy)
}

// Handler for the file copy route.
async fn handle_copy(body: CopyRequest) -> Result<impl warp::Reply, warp::Rejection> {
    match copy_file(body.source, body.destination).await {
        Ok(_) => Ok(warp::reply::json(&"File copied successfully.".to_string())),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

// Request body structure for copying files.
#[derive(warp::serde::Serialize, serde::Deserialize)]
struct CopyRequest {
    source: String,
    destination: String,
}

#[tokio::main]
async fn main() {
    println!("Starting file sync server...");

    let addr = "127.0.0.1:3030".parse().unwrap();
    warp::serve(routes()).run(addr).await;
}
