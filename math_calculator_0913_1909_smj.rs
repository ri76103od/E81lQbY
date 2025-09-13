use warp::Filter;

/// 计算两个数字的和
#[tokio::main]
async fn main() {
    let sum_route = warp::path!("sum" / u32 / u32)
        .map(|a, b| warp::reply::json(&SumResponse { sum: a + b }));

    let routes = warp::service(sum_route);
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

/// 定义一个响应结构体，用于返回计算结果
#[derive(serde::Serialize)]
struct SumResponse {
    sum: u32,
}

/// 定义一个错误响应结构体，用于返回错误信息
#[derive(serde::Serialize)]
struct ErrorResponse {
    error: String,
}

/// 定义一个错误类型
#[derive(Debug)]
enum CalculatorError {
    InvalidInput,
# 优化算法效率
}

/// 实现错误转换为JSON响应
impl warp::reject::Reject for CalculatorError {}

/// 实现错误为JSON的转换
impl warp::reply::Reply for CalculatorError {
    fn into_response(self) -> warp::reply::Response {
        match self {
            CalculatorError::InvalidInput => warp::reply::json(&ErrorResponse {
                error: "Invalid input".to_string(),
            }),
        }
    }
}