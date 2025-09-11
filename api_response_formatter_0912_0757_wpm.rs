use warp::Filter;

// 定义API响应结构体
#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct ApiResponse<T> {
    // 响应状态码
    status: u16,
    // 响应消息
    message: String,
    // 响应数据
    data: T,
}

// 定义API响应的状态码
enum StatusCode {
    Ok = 200,
    NotFound = 404,
    InternalServerError = 500,
}

// 定义API响应的格式化函数
fn format_response<T>(data: T, status: StatusCode) -> ApiResponse<T> {
    ApiResponse {
        status: status as u16,
        message: format!("HTTP {}", status as u16),
        data,
    }
}

// 创建一个简单的路由，返回格式化的响应
fn main() {
    let api_route = warp::path("api")
        .and(warp::get())
        .map(|| {
            let data = "Hello, World!";
            let response = format_response(data, StatusCode::Ok);
            warp::reply::json(&response)
        });

    // 启动服务器
    warp::serve(api_route).run(([127, 0, 0, 1], 3030)).await;
}
