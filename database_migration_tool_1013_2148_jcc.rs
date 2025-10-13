use warp::Filter;
use warp::http::StatusCode;
use warp::reject;
use std::sync::Arc;
use diesel::prelude::*;
use diesel_migrations::run_all;
use std::io;
use std::path::PathBuf;
use std::env;
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use r2d2_diesel::DieselConnection;
use r2d2::Pool;
use tokio::runtime::Runtime;
use warp::filters::BoxedFilter;
use diesel::MysqlConnection;
use diesel::SqliteConnection;
use diesel::SqliteConnection;
use diesel::MigrationHarnessStorage;
use diesel_migrations::MysqlMigrationHarnessStorage;
use diesel_migrations::PgMigrationHarnessStorage;
use diesel_migrations::SqliteMigrationHarnessStorage;
use diesel_migrations::MigrationHarness;

// Define the database connection pool
pub struct DatabaseConnectionPool;

impl DatabaseConnectionPool {
    // Create a new connection pool
    pub fn new(url: String) -> Pool<ConnectionManager<PgConnection>> {
        let manager = ConnectionManager::<PgConnection>::new(&url);
        Pool::builder().build(manager).expect("Failed to create pool.")
    }
}

// Define the main function
#[tokio::main]
async fn main() {
    // Get the database URL from the environment variable
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = DatabaseConnectionPool::new(database_url);

    // Define the migration filter
    let migration_filter = warp::any()
        .map(move || {
            // Run the migrations
            run_all(&pool.get().expect("Failed to get connection from pool")).expect("Failed to run migrations");
            Ok("Migrations applied successfully")
        }).then(handle_result);

    // Start the Warp server
    let migration_route = warp::path("migrate")
        .and(warp::post())
        .and(migration_filter);
    warp::serve(migration_route)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

// Handle the result of the migration
async fn handle_result(result: Result<&str, Box<dyn std::error::Error>>) -> Result<impl warp:: Reply, warp::Rejection> {
    match result {
        Ok(message) => Ok(warp::reply::with_status(message, StatusCode::OK)),
        Err(e) => Err(warp::reject::custom(MigrationError { message: e.to_string() })),
    }
}

// Define a custom error type for migration errors
#[derive(Debug)]
pub struct MigrationError {
    pub message: String,
}

impl warp::reject::Reject for MigrationError {}
