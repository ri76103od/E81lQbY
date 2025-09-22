use warp::Filter;

#[tokio::main]
async fn main() {
    // 定义用户登录的路由
    let login_route = warp::path!("login")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handle_login);

    // 启动服务
    warp::serve(login_route)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

// 用户登录请求的处理函数
async fn handle_login(user: User) -> Result<impl warp::Reply, warp::Rejection> {
    // 检查用户名和密码是否正确
    if user.username != "admin" || user.password != "password" {
        // 返回401 Unauthorized错误
        return Err(warp::reject::custom(Unauthorized));
    }

    // 如果验证成功，返回200 OK的响应
    Ok(warp::reply::json(&LoginResponse {
        message: "Login successful".to_string(),
    }))
}

// 用户请求的结构体
#[derive(serde::Deserialize)]
struct User {
    username: String,
    password: String,
}

// 用户登录响应的结构体
#[derive(serde::Serialize)]
struct LoginResponse {
    message: String,
}

// 定义401 Unauthorized错误
#[derive(Debug)]
struct Unauthorized;

impl warp::reject::Reject for Unauthorized {}

impl warp::reply::Reply for Unauthorized {
    fn into_response(self) -> warp::reply::Response {
        warp::reply::with_status(warp::reply::json(&{"error": "Unauthorized"}), warp::http::StatusCode::UNAUTHORIZED)
    }
}