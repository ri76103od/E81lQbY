use warp::Filter;

// 定义用户结构体
#[derive(Debug, Clone)]
struct User {
    username: String,
    password: String,
}

// 登录请求结构体
#[derive(Debug, Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

// 登录响应结构体
#[derive(Debug, Serialize)]
struct LoginResponse {
    success: bool,
    message: String,
}

// 模拟的用户数据库
fn get_user_database() -> Vec<User> {
    let mut users = Vec::new();
    users.push(User {
        username: "admin".to_string(),
        password: "password123".to_string(),
    });
    users
}

// 验证登录函数
fn validate_login(user: &LoginRequest, users: &[User]) -> Result<LoginResponse, warp::Rejection> {
    for registered_user in users {
        if registered_user.username == user.username && registered_user.password == user.password {
            return Ok(LoginResponse {
                success: true,
                message: "Login successful".to_string(),
            });
        }
    }
    Err(warp::reject::custom(LoginResponse {
        success: false,
        message: "Invalid credentials".to_string(),
    }))
}

// 创建登录路由
fn create_login_route() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post()
        .and(warp::path("login"))
        .and(warp::body::json())
        .and(with_user_database())
        .and_then(|request: LoginRequest, users: Vec<User>| async move {
            match validate_login(&request, &users) {
                Ok(response) => warp::reply::json(&response),
                Err(e) => warp::reply::json(&e),
            }
        })
}

// 提供用户数据库的闭包
fn with_user_database() -> impl Filter<Extract = Vec<User>, Error = warp::Rejection> + Clone {
    warp::any().map(|| get_user_database())
}

#[tokio::main]
async fn main() {
    let routes = create_login_route();
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

// 这里是额外的注释和文档
//
// 这个程序是一个简单的用户登录验证系统，使用RUST和WARP框架创建。
//
// 它定义了三个主要的结构体：User、LoginRequest和LoginResponse，分别用于存储用户信息、登录请求和响应。
//
// `get_user_database`函数模拟了一个简单的用户数据库。
//
// `validate_login`函数检查提供的登录请求是否与数据库中的用户匹配。
//
// `create_login_route`函数创建了一个登录路由，它接收JSON格式的登录请求，验证凭据，然后返回相应的响应。
//
// `with_user_database`函数是一个闭包，它提供了用户数据库。
//
// 主函数`main`设置了WARP服务器，并定义了登录路由。
