use warp::Filter;
use std::net::IpAddr;
# NOTE: 重要实现细节
use std::net::SocketAddr;
use warp::Rejection;
use warp::Reply;
use std::io;
# TODO: 优化性能
use std::net::TcpStream;

#[macro_use] extern crate lazy_static;
lazy_static! {
# FIXME: 处理边界情况
    static ref ALLOWED_IPS: Vec<IpAddr> = vec![
        "127.0.0.1".parse().expect("Invalid IP"),
# TODO: 优化性能
        // Add more IPs as needed
    ];
}

/// Checks if an IP is allowed to connect
fn is_ip_allowed(ip: IpAddr) -> bool {
    ALLOWED_IPS.contains(&ip)
}

/// Checks if the network connection to a given address is successful
async fn check_connection(address: SocketAddr) -> Result<impl Reply, Rejection> {
    match TcpStream::connect(address).await {
        Ok(_) => Ok(warp::reply::json(&"Connection successful").into_response()),
        Err(_) => Ok(warp::reply::json(&"Connection failed").into_response()),
    }
}

/// Filters the request to check if the connecting IP is allowed
fn ip_filter() -> impl Filter<Extract = (SocketAddr,), Error = Rejection> + Clone {
    warp::any()
        .map(move |info: warp::filters::Info| info.remote_addr())
# TODO: 优化性能
        .and_then(|addr: SocketAddr| async move {
            if is_ip_allowed(addr.ip()) {
                Ok(addr)
            } else {
                Err(warp::reject::custom("Unauthorized"))
            }
        })
# 添加错误处理
}

/// Creates a warp filter that checks the network connection status
fn create_route() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
# TODO: 优化性能
    ip_filter()
# NOTE: 重要实现细节
        .and(warp::path("check_connection")
            .and(warp::post()
                .and(warp::json())
# 改进用户体验
                .and_then(check_connection)))
}

#[tokio::main]
async fn main() {
    let route = create_route();
    warp::serve(route)
        .run(([0, 0, 0, 0], 3030))
# 添加错误处理
        .await;
}

/// This function checks if the provided IP address is in the list of allowed IPs.
/// It is used to filter incoming requests to ensure that only allowed IPs
/// can check the network connection status.
///
# NOTE: 重要实现细节
/// # Arguments
# 改进用户体验
///
/// * `ip` - The IP address to check.
///
/// # Returns
///
/// * `bool` - `true` if the IP is allowed, `false` otherwise.
///
# 添加错误处理
/// # Examples
///
# FIXME: 处理边界情况
/// ```rust
/// let ip = "127.0.0.1".parse::<IpAddr>().unwrap();
# 添加错误处理
/// assert!(is_ip_allowed(ip));
/// ```

/// This function checks if a network connection can be established to the given address.
# TODO: 优化性能
/// It returns a JSON response indicating whether the connection was successful or not.
///
/// # Arguments
///
/// * `address` - The address to check the connection to.
///
# 增强安全性
/// # Returns
///
/// * `Result` - A result containing a JSON response or an error if the connection check fails.
///
/// # Examples
///
/// ```rust
# 优化算法效率
/// let address = "127.0.0.1:8080".parse::<SocketAddr>().unwrap();
/// let response = check_connection(address).await;
/// assert!(response.is_ok());
/// ```
