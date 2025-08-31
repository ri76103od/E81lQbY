use warp::Filter;

/// 定义请求体结构体，用于处理支付请求
#[derive(Debug, serde::Deserialize)]
struct PaymentRequest {
    amount: f64,
    currency: String,
}

/// 定义响应结构体，用于返回支付结果
#[derive(Debug, serde::Serialize)]
struct PaymentResponse {
    status: String,
    message: String,
}

/// 处理支付逻辑的函数
async fn process_payment(request: PaymentRequest) -> Result<impl warp::Reply, warp::Rejection> {
    match validate_request(&request) {
        Ok(_) => {
            // 这里添加实际的支付处理逻辑
            // 例如调用支付服务API，数据库操作等
            // 目前仅返回一个模拟的成功响应
            Ok(warp::reply::json(&PaymentResponse {
                status: "success".to_string(),
                message: "Payment processed successfully".to_string(),
            }))
        }
        Err(e) => {
            // 返回错误响应
            Ok(warp::reply::json(&PaymentResponse {
                status: "error".to_string(),
                message: e.to_string(),
            }))
        }
    }
}

/// 验证支付请求的函数
fn validate_request(request: &PaymentRequest) -> Result<(), String> {
    if request.amount <= 0.0 {
        Err("Amount must be greater than zero".to_string())
    } else if request.currency.is_empty() {
        Err("Currency cannot be empty".to_string())
    } else {
        Ok(())
    }
}

/// 创建支付处理路由
fn create_payment_route() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post()
        .and(warp::path("payment"))
        .and(warp::body::json())
        .and_then(process_payment)
}

#[tokio::main]
async fn main() {
    // 启动服务器，监听端口3030
    warp::serve(create_payment_route())
        .run(([127, 0, 0, 1], 3030))
        .await;
}