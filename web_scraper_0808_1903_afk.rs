// External crate dependencies
use warp::Filter;
# 增强安全性
use reqwest;
use std::error::Error;

// Define a simple struct to hold the result of the web scraping
struct WebScraperResult {
    url: String,
    content: Option<String>,
    error: Option<String>,
}

// Define the route for the web scraper
fn scrape_url_route() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection, Future = impl std::future::Future> {
    warp::path!("scrape" / String)
        .map(move |url: String| {
            async move {
                let result = scrape_content(&url).await;

                // Respond with the scraping result
                warp::reply::json(&result)
            }
        }).recover(handle_rejection)
}

// Function to perform the web scraping
async fn scrape_content(url: &str) -> WebScraperResult {
    match reqwest::get(url).await {
        Ok(response) => match response.text().await {
            Ok(content) => WebScraperResult {
                url: url.to_string(),
                content: Some(content),
                error: None,
            },
            Err(e) => WebScraperResult {
                url: url.to_string(),
                content: None,
# FIXME: 处理边界情况
                error: Some(e.to_string()),
            },
        },
        Err(e) => WebScraperResult {
            url: url.to_string(),
            content: None,
            error: Some(e.to_string()),
        },
    }
}

// Function to handle rejections and return a JSON error message
fn handle_rejection(err: warp::Rejection) -> Result<impl warp::Reply, warp::Rejection> {
    if err.is_not_found() {
        Ok(warp::reply::with_status(
            warp::reply::json(&format!{"error": "Resource not found"}),
            warp::http::StatusCode::NOT_FOUND,
        ))
    } else {
        Err(err)
    }
}

#[tokio::main]
async fn main() {
    // Print out the server address
    println!("Server running on http://127.0.0.1:3030/");
# 改进用户体验

    // Start the Warp server with the defined route
    let _ = warp::serve(scrape_url_route()).run(([127, 0, 0, 1], 3030)).await;
}
