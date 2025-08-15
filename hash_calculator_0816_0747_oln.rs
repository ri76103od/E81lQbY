use warp::http::StatusCode;
use warp::Filter;
use sha2::{Sha224, Sha256, Sha384, Sha512};
use std::hash::{Hash, Hasher};
use std::io::Write;

// Define the hash calculation function
fn calculate_hash<T: Hash + ?Sized>(data: &T) -> String {
    let mut hasher = Sha256::new();
    data.hash(&mut hasher);
    format!("%x", hasher.finalize())
}

// Define the route for calculating SHA-256 hash
fn sha256_route() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("hash" / "sha256")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(|data: serde_json::Value| async move {
            let result = calculate_hash(&data);
            Ok::<_, warp::Rejection>(warp::reply::json(&result))
        })
}

// Define the main function
#[tokio::main]
async fn main() {
    let routes = sha256_route();
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

// Add unit tests to validate the hash calculation function
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn test_calculating_sha256_hash() {
        let data = json!({ "message": "Hello, World!" });
        let result = calculate_hash(&data);
        assert_eq!(result.len(), 64); // SHA-256 hash length
        assert!(result.starts_with("f7f77d3a5e9e2a5e