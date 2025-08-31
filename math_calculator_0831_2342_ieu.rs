// math_calculator.rs
// 这是一个使用RUST和WARP框架实现的数学计算工具集

use warp::Filter;
use std::collections::HashMap;
use std::str::FromStr;
use num_traits::FromPrimitive;
use thiserror::Error;

// 定义可能的错误类型
#[derive(Error, Debug)]
pub enum MathError {
    #[error("invalid request: {0}")]
    InvalidRequest(String),
    #[error("unsupported operation: {0}")]
    UnsupportedOperation(String),
    #[error("invalid number: {0}")]
    InvalidNumber(String),
    #[error("arithmetic error: {0}")]
    ArithmeticError(String),
}

// 定义请求参数的结构体
#[derive(Debug, serde::Deserialize)]
pub struct MathRequest {
    operation: String,
    a: f64,
    b: f64,
}

// 创建数学计算逻辑
pub async fn calculate_math(req: MathRequest) -> Result<String, MathError> {
    match req.operation.as_str() {
        "add" => Ok((req.a + req.b).to_string()),
        "subtract" => Ok((req.a - req.b).to_string()),
        "multiply" => Ok((req.a * req.b).to_string()),
        "divide" => {
            if req.b == 0.0 {
                Err(MathError::ArithmeticError("cannot divide by zero".to_string()))
            } else {
                Ok((req.a / req.b).to_string())
            }
        },
        _ => Err(MathError::UnsupportedOperation(req.operation)),
    }
}

// 创建WARP路由
pub fn create_routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let math_route = warp::post()
        .and(warp::path("calculate"))
        .and(warp::body::json())
        .and_then(handle_calculate);

    math_route
}

// 处理数学计算请求的函数
pub async fn handle_calculate(req: MathRequest) -> Result<impl warp::Reply, warp::Rejection> {
    match calculate_math(req).await {
        Ok(result) => Ok(warp::reply::json(&result)),
        Err(e) => Ok(warp::reply::json(&e)),
    }
}

// 程序入口点
#[tokio::main]
async fn main() {
    println!("Math calculator service is running...");
    let routes = create_routes();
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
