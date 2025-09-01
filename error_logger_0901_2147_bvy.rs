// error_logger.rs
// 这是一个使用RUST和WARP框架实现的错误日志收集器程序。

use warp::Filter;
use std::sync::Mutex;
use std::collections::HashMap;
use std::sync::Arc;
use log::info;
use serde::Serialize;

// 定义错误日志数据结构
#[derive(Serialize)]
struct ErrorLog {
    id: u32,
    message: String,
    timestamp: String,
}

// 定义全局错误日志存储
lazy_static::lazy_static! {
    static ref ERROR_LOGS: Mutex<HashMap<u32, ErrorLog>> = Mutex::new(HashMap::new());
}

// 生成下一个错误日志ID
fn next_id() -> u32 {
    let mut logs = ERROR_LOGS.lock().unwrap();
    let max_id = logs.keys().max().unwrap_or(&0).clone();
    max_id + 1
}

// 创建一个新的错误日志
fn create_error_log(message: String) -> u32 {
    let id = next_id();
    let error_log = ErrorLog {
        id,
        message,
        timestamp: chrono::Utc::now().to_rfc3339(),
    };
    let mut logs = ERROR_LOGS.lock().unwrap();
    logs.insert(id, error_log);
    id
}

// 实现WARP路由
fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post()
        .and(warp::path("log_error"))
        .and(warp::body::json())
        .and_then(|message: String| async move {
            let id = create_error_log(message);
            Ok::<_, warp::Rejection>(warp::reply::json(&id))
        })
}

#[tokio::main]
async fn main() {
    let log_filter = warp::log::custom(|info| {
        match info {
            warp::log::Info::Log(ref log) => {
                info!("{}", log);
            },
            _ => (),
        }
    });

    warp::serve(routes().with(log_filter)).run(([127, 0, 0, 1], 3030)).await;
}
