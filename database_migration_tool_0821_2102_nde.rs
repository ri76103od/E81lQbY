use warp::Filter;
use std::error::Error;
use diesel::prelude::*;
use diesel_migrations::RunMigrations;
use warp::http::StatusCode;
use warp::reply::{self, Reply};
use warp::Filter;
use serde::Deserialize;

// 数据库连接配置
#[derive(Clone, Debug)]
pub struct DatabaseConfig {
    pub url: String,
}

// 迁移请求结构
#[derive(Deserialize)]
pub struct MigrationRequest {
    pub direction: String,
}

// 定义数据库迁移工具的模块
pub mod db_migration_tool {
    use super::*;
    use warp::reject::Reject;
    use warp::{self, Rejection, Reply};
    use diesel::migrations::MigrationError;
    use diesel::r2d2::ConnectionManager;
    use diesel::r2d2_diesel_demo::DieselDemoMigrator;
    use diesel::result::QueryResult;

    // 数据库连接池配置
    pub fn db_pool() -> r2d2::Pool<ConnectionManager<diesel::pg::PgConnection>> {
        let manager = ConnectionManager::<diesel::pg::PgConnection>::new(
            &std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
        );
        r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.")
    }

    // 运行数据库迁移
    pub async fn run_migration<E: Error + Send + Sync + 'static>(
        pool: &r2d2::Pool<ConnectionManager<diesel::pg::PgConnection>>,
        direction: String,
    ) -> Result<impl Reply, E> {
        let connection = pool
            .get()
            .map_err(|e| warp::reject::custom(MigrationError::new(e)))?;
        match direction.as_str() {
            