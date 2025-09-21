// file_backup_and_sync.rs
// A simple file backup and sync tool using Rust and Warp framework.

use std::fs;
use std::io;
use std::path::Path;
use warp::Filter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define the source and destination paths.
    // These should be replaced with actual paths.
    let source_path = "./source/";
    let destination_path = "./destination/";

    // Ensure the destination directory exists.
    fs::create_dir_all(destination_path)?;

    // Define a warp filter to handle HTTP requests.
    let backup_route = warp::path("backup")
        .and(warp::post())
        .and(with_source_path(source_path))
        .and(with_destination_path(destination_path))
        .and_then(backup_files);

    // Start the warp server.
    warp::serve(backup_route)
        .run(([127, 0, 0, 1], 3030))
        .await;

    Ok(())
}

// A function to handle file backup requests.
async fn backup_files(source_path: String, destination_path: String) -> Result<impl warp::Reply, warp::Rejection> {
    let source = Path::new(&source_path);
    let destination = Path::new(&destination_path);

    // Check if the source directory exists.
    if !source.is_dir() {
        return Err(warp::reject::not_found());
    }

    // Iterate through all files in the source directory and copy them to the destination directory.
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let path = entry.path();

        // Skip directories, only copy files.
        if path.is_dir() {
            continue;
        }

        // Copy the file to the destination directory.
        let destination_path = destination.join(path.file_name().ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::Other, "Failed to get file name")
        })?);

        // Ensure the destination path's directory exists.
        let parent = destination_path.parent().ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::Other, "Failed to get parent directory")
        })?;
        fs::create_dir_all(parent)?;

        // Copy the file.
        fs::copy(&path, &destination_path)?;
    }

    Ok(warp::reply::json(&{"status": "backup completed successfully"}))
}

// Helper function to extract the source path from the request.
fn with_source_path(path: &'static str) -> impl Filter<Extract = (String,), Error = std::convert::Infallible> + Clone {
    warp::path::end().map(move || path.to_string())
}

// Helper function to extract the destination path from the request.
fn with_destination_path(path: &'static str) -> impl Filter<Extract = (String,), Error = std::convert::Infallible> + Clone {
    warp::path::end().map(move || path.to_string())
}