use warp::Filter;

// 定义一个简单的数据模型
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
struct User {
    id: u32,
    name: String,
    email: String,
}

// 一个简单的错误类型
#[derive(Debug)]
enum Error {
    ValidationError(&'static str),
}

// 将错误类型实现为 `std::fmt::Display`
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::ValidationError(ref err) => write!(f, "{}", err),
        },
    }
}

// 定义路由和处理函数
fn main() {
    // 创建一个用户数据的JSON过滤器
    let user_routes = warp::path("users")
        .and(warp::post())
        .and(warp::body::json::<User>())
        .map(handle_create_user);

    // 启动WARP服务器
    warp::serve(user_routes)
        .run(([127, 0, 0, 1], 3030)).await;
}

// 处理用户创建请求的函数
async fn handle_create_user(user: User) -> Result<impl warp::Reply, warp::Rejection> {
    // 简单的验证
    if user.name.is_empty() || user.email.is_empty() {
        return Err(warp::reject::custom(Error::ValidationError("Name and email cannot be empty")));
    }

    // 存储用户数据（这里只是打印出来，实际应用中可能是数据库操作）
    println!("Received user: {:?}