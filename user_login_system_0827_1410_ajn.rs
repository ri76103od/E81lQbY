use warp::Filter;

// 定义用户结构体
#[derive(Debug, Clone)]
struct User {
    username: String,
    password: String,
}

// 定义登录请求和响应的数据结构
#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(serde::Serialize, Debug)]
struct LoginResponse {
    success: bool,
    message: String,
}

// 登录验证逻辑
async fn login(user: User) -> Result<LoginResponse, warp::Rejection> {
    // 这里可以根据实际情况对用户进行验证，例如查询数据库
    if user.username == "admin" && user.password == "password123" {
        Ok(LoginResponse {
            success: true,
            message: "Login successful".to_string(),
        })
    } else {
        Err(warp::reject::custom(LoginError { message: "Invalid username or password".to_string() }))
    }
}

// 自定义错误类型
#[derive(Debug)]
struct LoginError {
    message: String,
}

impl warp::reject::Reject for LoginError {}

// 定义登录路由
fn login_route() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post()
        .and(warp::path("login"))
        .and(warp::body::json())
        .and_then(|login_request: LoginRequest| async move {
            let user = User {
                username: login_request.username,
                password: login_request.password,
            };
            match login(user).await {
                Ok(response) => Ok(warp::reply::json(&response)),
                Err(_) => Err(warp::reject::custom(LoginError { message: "Invalid username or password".to_string() })),
            }
        })
}

// 运行服务器
#[tokio::main]
async fn main() {
    let routes = login_route();
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

// 实现LoginError的Display和Debug方法，用于日志记录和错误处理
impl std::fmt::Display for LoginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl warp::reject::Reject for LoginError {}
