// sql_injection_prevention.rs
// 使用RUST和WARP框架实现防止SQL注入的程序示例。

use warp::http::StatusCode;
use warp::Filter;
use serde::Deserialize;
use serde_json::json;
use sqlx::postgres::PgPoolOptions;
use sqlx::Row;
use thiserror::Error;
use warp::reject::Reject;
use warp::Reply;

// 定义一个错误类型，以处理与SQL注入相关的错误。
#[derive(Debug, Error)]
pub enum SqlInjectionError {
    #[error("invalid input: {0}")]
    InvalidInput(String),
}

// 实现 Reject trait，使得 SqlInjectionError 可以被warp::reject使用。
impl Reject for SqlInjectionError {}

// 新建一个结构体来存储数据库连接池。
#[derive(Clone)]
struct Pool(sqlx::Pool<sqlx::Postgres>);

// 实现 Filter trait，使得 Pool 可以被Warp使用。
impl Filter<Extract = (Pool,), Error = warp::Rejection> for Pool {   
    fn filter(&self) -> warp::filters::Filter<Self> {
        warp::any().map(move || self.clone())
    }
}

// 创建一个 POST 请求处理器，防止SQL注入。
fn create_user(pool: Pool, user_data: NewUser) -> Result<impl Reply, warp::Rejection> {
    // 确保输入数据是有效的，例如，用户名和邮箱不包含SQL注入风险的字符。
    if user_data.username.contains('\') || user_data.email.contains('\') {
        return Err(SqlInjectionError::InvalidInput("Username or email contains invalid characters.".to_string()).into());
    }

    // 使用参数化查询来防止SQL注入。
    let sql = "INSERT INTO users (username, email) VALUES ($1, $2)";
    let mut conn = pool.0.acquire().map_err(|e| warp::reject::custom(SqlInjectionError::InvalidInput(e.to_string())))?;
    match sqlx::query(sql)
        .bind(&user_data.username)
        .bind(&user_data.email)
        .execute(&mut *conn) {
        Ok(_) => Ok(warp::reply::json(&json!({"status": "success"}))),
        Err(e) => Ok(warp::reply::json(&json!({"status": "error", "message": e.to_string()}))),
    }
}

// 创建一个 Warp 过滤器来处理请求。
fn user_routes(pool: Pool) -> impl Filter<Extract = impl Reply, Error = warp::Rejection> + Clone {
    let create_user_route = warp::post()
        .and(warp::path("user"))
        .and(warp::body::json::<Value>())
        .and(pool.clone())
        .map(|value: Value, pool: Pool| -> Result<impl Reply, warp::Rejection> {
            let user_data: NewUser = serde_json::from_value(value).map_err(|_| warp::reject::custom(SqlInjectionError::InvalidInput("Invalid JSON format".to_string())))?;
            create_user(pool, user_data)
        });

    create_user_route
}

// 定义一个结构体来解析JSON请求体中的新用户数据。
#[derive(Deserialize)]
struct NewUser {
    username: String,
    email: String,
}

// 定义一个结构体来存储JSON响应体中的数据。
#[derive(Serialize)]
struct Value {
    status: String,
}

#[tokio::main]
async fn main() {
    // 创建一个 PostgreSQL 数据库连接池。
    let database_url = "postgres://username:password@localhost/dbname";
    let pool = PgPoolOptions::new().connect(database_url).await.unwrap();
    let pool = Pool(pool);

    // 定义 Warp 过滤器并启动服务器。
    let routes = user_routes(pool);
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
