use warp::Filter;
use std::collections::HashMap;

// 定义请求结构体
#[derive(Debug, serde::Deserialize)]
struct MathRequest {
    operation: String,
    number1: f64,
    number2: f64,
}

// 定义响应结构体
#[derive(Debug, serde::Serialize)]
struct MathResponse {
    result: f64,
}

// 实现数学计算函数
fn calculate_math(req: MathRequest) -> Result<MathResponse, warp::Rejection> {
    let result = match req.operation.as_str() {
        "add" => req.number1 + req.number2,
        "subtract" => req.number1 - req.number2,
        "multiply" => req.number1 * req.number2,
        "divide" => {
            if req.number2 == 0.0 {
                return Err(warp::reject::custom("math divide error: division by zero"));
            }
            req.number1 / req.number2
        },
        _ => return Err(warp::reject::custom("invalid operation")),
    };

    Ok(MathResponse { result })
}

// 设置路由和启动服务器
#[tokio::main]
async fn main() {
    let math_routes = warp::path("math")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(|req: MathRequest| async {
            let result = calculate_math(req);
            match result {
                Ok(response) => Ok(warp::reply::json(&response)),
                Err(_) => Err(warp::reject::custom("internal server error")),
            }
        });

    warp::serve(math_routes).run(([127, 0, 0, 1], 3030)).await;
}

// 错误处理
impl warp::reject::Reject for MathError {}

// 自定义错误类型
#[derive(Debug, Clone)]
enum MathError {
    MathDivideError(String),
    InvalidOperation(String),
}

// 实现错误消息函数
impl fmt::Display for MathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MathError::MathDivideError(msg) => write!(f, "{}", msg),
            MathError::InvalidOperation(msg) => write!(f, "{}", msg),
        }
    }
}

// 实现错误原因函数
impl std::error::Error for MathError {}
