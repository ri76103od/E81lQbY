use warp::Filter;

// 定义一个简单的用户结构体，用于存储用户信息
#[derive(Debug, Clone)]
struct User {
    username: String,
    password: String,
    // 可以添加更多用户信息字段
}

// 定义一个简单的认证服务
struct AuthService;

impl AuthService {
    // 模拟的验证用户登录的方法
    fn authenticate(&self, username: &str, password: &str) -> Result<bool, warp::Rejection> {
        // 这里只是一个示例，实际应用中应该使用数据库或其他数据存储
        let valid_user = User {
            username: String::from("admin"),
            password: String::from("password123"),
        };

        if username == valid_user.username && password == valid_user.password {
            Ok(true)
        } else {
            Err(warp::reject::custom(UnauthorizedError))
        }
    }
}

// 自定义错误类型
#[derive(Debug)]
struct UnauthorizedError;

impl warp::reject::Reject for UnauthorizedError {}

// 定义一个过滤器，用于处理登录认证
fn with_auth() -> impl Filter<Extract = (), Error = warp::Rejection, Future = impl Future<Output = ()>> {
    warp::header::<String>("Authorization")
        .and_then(|auth_header: String| async move {
            let parts: Vec<&str> = auth_header.split(" ").collect();
            if parts.len() != 2 || parts[0] != "Basic" {
                return Err(warp::reject::custom(UnauthorizedError));
            }

            let decoded = base64::decode(parts[1]).map_err(|_| warp::reject::custom(UnauthorizedError))?;
            let credentials = String::from_utf8(decoded).map_err(|_| warp::reject::custom(UnauthorizedError))?;
            let (username, password) = credentials.split_at(credentials.find(':').unwrap_or(0));

            AuthService.authenticate(username, password).await
                .map_err(|_| warp::reject::custom(UnauthorizedError))
        })
}

// 创建一个基本的路由，它要求访问者提供有效的认证信息
fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection, Future = impl Future<Output = Result<impl warp::Reply, warp::Rejection>>> {
    warp::path("secure")
        .and(warp::get())
        .and(with_auth())
        .map(|| warp::reply::json({"message": "Access granted"}))
}

#[tokio::main]
async fn main() {
    warp::serve(routes())
        .run(([127, 0, 0, 1], 3030))
        .await;
}

// 注意：这个示例代码假设你已经添加了warp和相关的依赖项到你的Cargo.toml文件中。
// 例如：
// warp = "0.3"
// tokio = { version = "1", features = ["full"] }
// base64 = "0.13"
