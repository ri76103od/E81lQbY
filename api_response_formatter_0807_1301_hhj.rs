use warp::Filter;

/// 定义 API 响应体结构
#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct ApiResponse<T> {
    /// 状态码
    status: i32,
    /// 消息
    message: String,
    /// 响应数据
    data: T,
}

/// 创建基本的 API 响应体
fn create_response<T>(data: T, status: i32, message: &str) -> ApiResponse<T> {
    ApiResponse {
        status,
        message: message.to_string(),
        data,
    }
}

/// 定义 API 响应过滤器
fn with_response<T>(data: T) -> warp::reply::Json<ApiResponse<T>> {
    warp::reply::json(&create_response(data, 200, "success"))
}

/// 创建一个 API 响应格式化的路由
fn create_api_route() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("api")
        .and(warp::get())
        .and(warp::any().map(move || Ok(42))) // 示例数据
        .and_then(with_response)
}

/// 主函数，启动服务器
#[tokio::main]
async fn main() {
    let api_route = create_api_route();
    warp::serve(api_route)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
