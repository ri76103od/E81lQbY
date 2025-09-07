// database_pool_manager.rs
// 该模块提供了创建和管理数据库连接池的功能。

use std::sync::Arc;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use diesel::r2d2::{self, ConnectionManager, Pool, PooledConnection};

// 定义数据库连接池的配置
struct DatabaseConfig {
    database_url: String,
}

impl DatabaseConfig {
    // 从环境变量中加载配置
    fn new() -> Self {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        DatabaseConfig { database_url }
    }
}

// 创建数据库连接池
fn create_pool() -> Pool<ConnectionManager<PgConnection>> {
    let config = DatabaseConfig::new();
    let manager = ConnectionManager::<PgConnection>::new(config.database_url);
    r2d2::Pool::builder()
        .max_size(5) // 设置连接池的最大连接数
        .build(manager)
        .expect("Failed to create pool.")
}

// 获取数据库连接
async fn get_db_connection(pool: Arc<Pool<ConnectionManager<PgConnection>>>) -> Result<PooledConnection<ConnectionManager<PgConnection>>, diesel::r2d2::Error> {
    Ok(pool.get().expect("Failed to get database connection from pool."))
}

// 一个简单的示例函数，展示如何使用数据库连接池
async fn use_database(pool: Arc<Pool<ConnectionManager<PgConnection>>>) -> Result<(), diesel::r2d2::Error> {
    let conn = get_db_connection(pool).await?;
    // 这里可以执行数据库操作，例如查询或者更新等
    // 示例：查询总数
    use schema::users::dsl;
    let count = users.count().eq(dsl::id).load::<i32>(&conn)?;
    println!("User count: {}", count);
    Ok(())
}

// main函数用于启动程序
#[tokio::main]
async fn main() {
    let pool = Arc::new(create_pool());
    if let Err(e) = use_database(pool).await {
        eprintln!("Error using database: {}", e);
    }
}
