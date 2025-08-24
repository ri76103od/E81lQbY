// math_service.rs
// 这是一个使用RUST和WARP框架创建的数学计算工具集

use warp::Filter;
use std::error::Error;
use warp::http::StatusCode;
use serde::{Deserialize, Serialize};

// 定义请求和响应的数据结构
#[derive(Deserialize, Serialize, Debug, Clone)]
struct AddRequest {
    a: f64,
    b: f64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct AddResponse {
    sum: f64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct MultiplyRequest {
    a: f64,
    b: f64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct MultiplyResponse {
    product: f64,
}

// 实现加法操作
async fn add(request: AddRequest) -> Result<impl warp::Reply, warp::Rejection> {
    let sum = request.a + request.b;
    Ok(warp::reply::json(&AddResponse { sum }))
}

// 实现乘法操作
async fn multiply(request: MultiplyRequest) -> Result<impl warp::Reply, warp::Rejection> {
    let product = request.a * request.b;
    Ok(warp::reply::json(&MultiplyResponse { product }))
}

// 设置路由和过滤器
fn init_routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let add_route = warp::path!("math" / "add")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(add);

    let multiply_route = warp::path!("math" / "multiply")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(multiply);

    add_route.or(multiply_route)
}

// 启动服务
#[tokio::main]
async fn main() {
    let routes = init_routes();
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}