// 使用RUST和WARP框架防止SQL注入的示例程序
// 代码包含适当的错误处理和注释，遵循RUST最佳实践

use warp::Filter;
use sqlx::PgPool;
use sqlx::postgres::Pg;
use anyhow::Result;
use std::net::SocketAddr;

// 配置数据库连接池
#[tokio::main]
async fn main() -> Result<()> {
    let pool = PgPool::connect("postgres://username:password@localhost/dbname").await?;
    let addr = SocketAddr::from(([127, 0, 0, 1], 3030));
    let server = warp::service(
        get_users().with(warp::log("users"))
    ).run(addr);
    println!("Running on http://")
        .chain_err(|| "Failed to run server".to_string())?;
    server.await;
}

// 获取用户列表的端点
// 使用参数化查询防止SQL注入
fn get_users() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path("users"))
        .and(with_db_pool())
        .and_then(|pool: PgPool| async move {
            let users = sqlx::query("SELECT * FROM users")
                .fetch_all(&pool)
                .await;
            match users {
                Ok(users) => Ok(warp::reply::json(&users)),
                Err(e) => Err(warp::reject::custom(e)),
            }
        })
}

// 提供数据库连接池的中间件
fn with_db_pool() -> impl Filter<Extract = (PgPool,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || pool.clone())
}

// 定义全局数据库连接池
static POOL: PgPool = PgPool::connect("postgres://username:password@localhost/dbname")
    .await
    .expect("Failed to create pool");
let pool = POOL.clone();