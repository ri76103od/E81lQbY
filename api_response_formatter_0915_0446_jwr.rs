use warp::Filter;

// 定义API响应结构体
#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct ApiResponse {
    code: i32,
    message: String,
    data: Option<serde_json::Value>,
}

// 创建一个简单的API响应格式化工具
#[tokio::main]
async fn main() {
    // 使用Warp框架定义路由和处理函数
    let api = warp::path("api")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handle_api);

    warp::serve(api).run(([127, 0, 0, 1], 3030)).await;
}

// 处理API请求的函数
async fn handle_api(data: serde_json::Value) -> Result<impl warp::Reply, warp::Rejection> {
    // 尝试解析请求数据
    let request_data = match serde_json::from_value::<ApiResponse>(data) {
        Ok(api_response) => api_response,
        Err(_) => return Ok(warp::reply::json(&ApiResponse {
            code: -1,
            message: String::from("Invalid request data"),
            data: None,
        })),
    };

    // 根据请求数据生成响应
    let response = ApiResponse {
        code: 200,
        message: String::from("Success"),
        data: Some(serde_json::json!({
            "received": request_data,
        })),
    };

    // 返回JSON格式的响应
    Ok(warp::reply::json(&response))
}
