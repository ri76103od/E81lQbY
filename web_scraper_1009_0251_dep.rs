use warp::Filter;
use reqwest;
use std::error::Error;
use serde::Deserialize;
use tokio::runtime;

// 定义配置结构体，用于存储网页内容抓取的URL
#[derive(Deserialize)]
struct ScraperConfig {
    url: String,
}

// 异步函数，用于抓取网页内容
async fn fetch_web_content(config: ScraperConfig) -> Result<String, Box<dyn Error>> {
    let response = reqwest::get(&config.url).await?;
    let status = response.status();
    if status.is_success() {
        Ok(response.text().await?)
    } else {
        Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("Failed to fetch web content, status: {}", status))))
    }
}

// 定义Warp过滤器，用于处理POST请求
fn create_scraper_filter() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post()
        .and(warp::body::json())
        .and_then(|config: ScraperConfig| async move {
            match fetch_web_content(config).await {
                Ok(content) => Ok(warp::reply::json(&content)),
                Err(e) => Err(warp::reject::custom(e)),
            }
        })
}

#[tokio::main]
async fn main() {
    let scraper_filter = create_scraper_filter();
    warp::serve(scraper_filter)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

// 错误处理
#[derive(Debug)]
enum ScraperError {
    IoError(std::io::Error),
    ReqwestError(reqwest::Error),
}

impl From<std::io::Error> for ScraperError {
    fn from(err: std::io::Error) -> Self {
        ScraperError::IoError(err)
    }
}

impl From<reqwest::Error> for ScraperError {
    fn from(err: reqwest::Error) -> Self {
        ScraperError::ReqwestError(err)
    }
}

impl warp::reject::Reject for ScraperError {}
