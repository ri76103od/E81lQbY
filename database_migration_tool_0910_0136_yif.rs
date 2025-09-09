use warp::Filter;
use warp::http::StatusCode;
use std::sync::Arc;
use diesel::prelude::*;
use diesel::migration::Migrator;
use std::env;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

// 配置数据库连接池
#[derive(Clone)]
struct DbConn(
    Arc<Pool<ConnectionManager<PgConnection>>>
);

impl DbConn {
    fn new() -> DbConn {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder().build(manager).expect("Failed to create pool.");
        DbConn(Arc::new(pool))
    }
}

// 异步获取数据库连接
async fn db_conn() -> DbConn {
    DbConn::new()
}

// 定义迁移路由
fn migrations_route(db_conn: DbConn) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post()
        .and(warp::path("migrate"))
        .and(with_db_conn(db_conn))
        .and_then(handle_migrate)
}

// 处理迁移请求
async fn handle_migrate(conn: DbConn) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = conn.0.get().expect("Failed to get DB connection");
    diesel::migrations::run_pending_migrations(&conn)
        .map_err(|e| warp::reject::custom(e))
        .map(|_| warp::reply::json(&"Migrations successful"))
}

// 创建数据库连接池
fn with_db_conn(db_conn: DbConn) -> impl Filter<Extract = (DbConn,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db_conn.clone())
}

#[tokio::main]
async fn main() {
    let db_conn = DbConn::new();
    let routes = migrations_route(db_conn);
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

// 错误处理
#[derive(Debug)]
pub struct MigrationError(String);

impl std::fmt::Display for MigrationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl warp::reject::Reject for MigrationError {}
