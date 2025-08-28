// restful_api_warp.rs
// 这是一个使用RUST和WARP框架实现的RESTful API接口程序

use warp::Filter;

// 定义一个简单的响应体结构体
#[derive(Debug, serde::Serialize)]
struct ApiResponse {
    message: String,
}

// 创建一个GET请求的端点，返回一个简单的响应
fn get_endpoint() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("hello") // 设置路径
        .and(warp::get()) // 限制为GET请求
        .map(|| { // 处理函数
            let response = ApiResponse {
                message: "Hello, World!".to_string(),
            };
            warp::reply::json(&response) // 将响应体序列化为JSON
        }).recover(handle_rejection) // 错误处理
}

// 错误处理函数
fn handle_rejection(err: warp::Rejection) -> Result<impl warp::Reply, warp::Rejection> {
    if err.is_not_found() {
        Ok(warp::reply::with_status(
            "Not Found",
            warp::http::StatusCode::NOT_FOUND,
        ))
    } else {
        Err(err)
    }
}

// 主函数，启动WARP服务器
#[tokio::main]
async fn main() {
    let api = get_endpoint();
    warp::serve(api)
        .run(([127, 0, 0, 1], 3030)).await; // 在本地的3030端口运行
}
