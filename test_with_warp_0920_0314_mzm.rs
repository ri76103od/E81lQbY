use warp::Filter;
use warp::http::StatusCode;
use warp::test::Request;
use warp::Filter as WarpFilter;
use warp::Rejection;
use warp::Reply;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::result::Result;

// Custom Error Type
#[derive(Debug)]
struct MyError;

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Custom error")
    }
}

// Implement the Error trait
impl Error for MyError {}

// Define a type that can be returned in case of an error
type Result<T> = std::result::Result<T, MyError>;

// Example handler function that returns a greeting
async fn greet() -> Result<impl Reply> {
    Ok("Hello, Warp!")
}

// Warp route filter
fn greet_route() -> WarpFilter<impl Reply> {
    warp::path("hello")
        .and(warp::get())
        .and_then(greet)
        .recover(handle_rejection)
}

// Function to handle rejections
async fn handle_rejection(err: Rejection) -> Result<impl Reply> {
    if err.is_not_found() {
        Ok(warp::reply::with_status("Not Found", StatusCode::NOT_FOUND))
    } else {
        Err(MyError)
    }
}

// Define tests for greeting route
#[tokio::test]
async fn test_greet_route() {
    let app = warp::Filter::new().and(greet_route());
    let res = warp::test::request()
        .path("/hello")
        .reply(&app)
        .await;

    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(res.body(), "Hello, Warp!");
}

// Define tests for unknown route
#[tokio::test]
async fn test_unknown_route() {
    let app = warp::Filter::new().and(greet_route());
    let res = warp::test::request()
        .path("/unknown")
        .reply(&app)
        .await;

    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}
