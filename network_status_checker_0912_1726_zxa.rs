// network_status_checker.rs
// 这个程序实现了一个网络连接状态检查器，用于检测网络连接状态。

use warp::Filter;
use std::net::SocketAddr;
use tokio::net::TcpStream;
use std::time::Duration;
use std::str::FromStr;
use warp::http::Response;
use warp::http::StatusCode;
use warp::reply;
use serde_json::json;

#[tokio::main]
async fn main() {
    // 设置WARP过滤器，检查网络连接状态
    let status_route = warp::path("status")
        .and(warp::get())
        .and(with_ip_address())
        .and_then(check_network_status);

    // 启动WARP服务器
    warp::serve(status_route)
        .run(([0, 0, 0, 0], 3030))
        .await;
}

// 从请求中提取IP地址的函数
fn with_ip_address() -> impl Filter<Extract = (String,), Error = std::convert::Infallible> + Clone {
    warp::header::<String>"X-Real-IP".or_else(|_| async {
        Ok(warp::remote_addr().0.to_string())
    })
}

// 检查网络连接状态的函数
async fn check_network_status(ip: String) -> Result<impl reply::Reply, warp::Rejection> {
    // 设置超时时间
    let timeout_duration = Duration::from_secs(5);

    // 尝试建立TCP连接
    match TcpStream::connect_timeout(&ip, timeout_duration).await {
        Ok(_) => Ok(Response::builder()
            .status(StatusCode::OK)
            .body(reply::json(&json!({
                "status": "connected",
                "ip_address": ip
            })))?),
        Err(_) => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(reply::json(&json!({
                "status": "disconnected",
                "ip_address": ip
            })))?),
    }
}
