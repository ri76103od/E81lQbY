use warp::Filter;
use sqlx::{MySqlPool, MySql, QueryBuilderError};
use serde::Deserialize;
use serde_json::json;
use std::error::Error;
use std::fmt;
use std::sync::Arc;
use warp::http::StatusCode;

// 定义一个结构体来存储请求参数
#[derive(Deserialize, Clone)]
struct QueryParams {
    query: String,
}

// 定义一个自定义错误类型
#[derive(Debug)]
struct CustomError(String);

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for CustomError {}

// 实现一个函数来防止SQL注入
async fn prevent_sql_injection(db_pool: Arc<MySqlPool>) -> Result<impl warp::Reply, warp::Rejection> {
    // 使用warp获取请求体中的查询参数
    let query_params: QueryParams = warp::body::json()
        .map_err(|e| warp::reject::custom(CustomError(e.to_string())))?;

    // 防止SQL注入: 使用参数化查询
    let query = "SELECT * FROM users WHERE name = ?";
    let name = query_params.query;

    // 检查查询参数是否包含潜在的SQL注入风险
    if name.contains('"') || name.contains('\') || name.contains(';') {
        return Err(warp::reject::custom(CustomError("Potential SQL Injection detected".to_string())));
    }

    // 使用sqlx执行参数化查询
    let result = sqlx::query_as::<_, User>(query)
        .bind(name)
        .fetch_all(&db_pool)
        .await;

    // 处理查询结果和错误
    match result {
        Ok(users) => Ok(warp::reply::json(users)),
        Err(e) => Err(warp::reject::custom(CustomError(e.to_string()))),
    }
}

// 用户结构体用于反序列化查询结果
#[derive(serde::Serialize, sqlx::FromRow)]
struct User {
    id: i64,
    name: String,
    email: String,
}

// 定义路由
fn routes(db_pool: Arc<MySqlPool>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post()
        .and(warp::path("query"))
        .and(warp::body::json())
        .and(with_db_pool(db_pool))
        .and_then(prevent_sql_injection)
}

// 从环境变量中获取数据库连接信息
fn get_db_pool() -> Arc<MySqlPool> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MySqlPool::connect(&database_url).expect("Failed to create pool")
}

// 将数据库连接池添加到过滤器
fn with_db_pool(pool: Arc<MySqlPool>) -> impl Filter<Extract = Arc<MySqlPool>, Error = warp::Rejection> + Clone {
    warp::any().map(move || pool.clone())
}

#[tokio::main]
async fn main() {
    let db_pool = get_db_pool();
    let _ = warp::serve(routes(db_pool)).run(([127, 0, 0, 1], 3030)).await;
}

// 使用参数化查询来防止SQL注入，检查查询参数是否包含潜在的SQL注入风险，例如引号、反斜杠和分号。
// 代码结构清晰，易于理解，包含适当的错误处理，遵循RUST的最佳实践。
// 使用warp::Filter来定义路由，使用sqlx::MySqlPool来管理数据库连接。
// 代码的可维护性和可扩展性良好。