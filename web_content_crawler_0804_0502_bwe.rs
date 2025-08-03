use warp::http::StatusCode;
use warp::Filter;
use reqwest;
use std::error::Error;
use serde::Serialize;
use serde_json::json;
use warp::reply::Response;

// 定义一个用于存储网页内容的结构体
#[derive(Serialize)]
struct WebpageContent {
    status: String,
    content: String,
}

// 定义一个错误枚举
#[derive(Debug, Serialize)]
enum WebCrawlerError {
    ReqwestError(reqwest::Error),
    ResponseError(String),
}

// 实现错误枚举的`Error` trait
impl Error for WebCrawlerError {}

// 实现`From` trait，以便能够将`reqwest::Error`转换为`WebCrawlerError`
impl From<reqwest::Error> for WebCrawlerError {
    fn from(err: reqwest::Error) -> WebCrawlerError {
        WebCrawlerError::ReqwestError(err)
    }
}

// 定义一个函数，用于抓取网页内容
async fn crawl_webpage(url: String) -> Result<WebpageContent, WebCrawlerError> {
    // 使用reqwest库发送HTTP请求
    let response = reqwest::get(&url).await?;

    // 检查HTTP响应状态码
    if response.status().is_success() {
        // 将网页内容转换为字符串
        let content = response.text().await?;

        // 返回网页内容结构体
        Ok(WebpageContent {
            status: "success".to_string(),
            content,
        })
    } else {
        // 如果状态码不是`200 OK`，则返回错误
        Err(WebCrawlerError::ResponseError(
            format!("Failed to fetch webpage: HTTP Status {}", response.status()),
        ))
    }
}

// 定义一个路由，用于处理网页内容抓取请求
fn web_crawler_route() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("crawl")
        .and(warp::post())
        .and(warp::body::content())
        .and_then(|body: String| async move {
            // 尝试抓取网页内容
            match crawl_webpage(body).await {
                Ok(content) => {
                    // 返回网页内容JSON
                    Ok(warp::reply::json(&content))
                },
                Err(e) => {
                    // 返回错误信息JSON
                    let error_message = match e {
                        WebCrawlerError::ReqwestError(_) => "Reqwest error".to_string(),
                        WebCrawlerError::ResponseError(msg) => msg,
                    };
                    Ok(warp::reply::json(&json!({
                        "status": "error",
                        "message": error_message,
                    })))
                },
            }
        })
        .with(warp::log("web_crawler"))
}

// 定义`main`函数
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 设置WARP过滤器
    let routes = web_crawler_route();

    // 启动WARP服务器
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;

    Ok(())
}
