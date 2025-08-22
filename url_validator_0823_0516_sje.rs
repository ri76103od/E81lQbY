use warp::Filter;
use std::net::IpAddr;
use std::str::FromStr;

// 定义一个错误类型，用于处理URL验证过程中的错误
#[derive(Debug)]
struct ValidationError(String);

// 异步函数，用于验证URL的有效性
async fn validate_url(url_filter: impl Filter<Extract = String, Error = warp::Rejection> + Clone + Send + 'static) -> Result<impl warp::Reply, warp::Rejection> {
    let url = url_filter.clone().extract().await;
    if let Ok(url) = url {
        // 尝试解析URL，如果失败则返回ValidationError
        if let Err(_) = warp::http::Uri::from_str(&url) {
            return Err(warp::reject::custom(ValidationError("Invalid URL".to_string()))
        }
        // 如果URL有效，返回成功的响应
        Ok(warp::reply::with_status("URL is valid".to_string(), warp::http::StatusCode::OK))
    } else {
        // 如果URL提取失败，返回ValidationError
        Err(warp::reject::custom(ValidationError("URL extraction failed".to_string())))
    }
}

// 为ValidationError实现warp::reject::Reject，以便它可以用作Rejection类型
impl warp::reject::Reject for ValidationError {}

#[tokio::main]
async fn main() {
    // 设置WARP过滤器，用于从请求中提取URL
    let url_filter = warp::path::end()
        .and(warp::get())
        .map(move || warp::path("validate_url").and(warp::path::param()));

    // 创建一个路由，当访问`/validate_url/<url>`时，调用validate_url函数
    let routes = url_filter.clone().map(move |url: String| validate_url(url.clone()));

    // 启动WARP服务器，监听3000端口
    warp::serve(routes).run(([127, 0, 0, 1], 3000)).await;
}
