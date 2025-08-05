use warp::Filter;
use std::fs;
use std::io::Read;
use std::path::Path;
use serde::Deserialize;
use serde_json;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::http::StatusCode;
use warp::reply::Reply;

// 配置数据结构
#[derive(Deserialize, Debug)]
struct BackupConfig {
    data_path: String,
    backup_path: String,
    restore_path: Option<String>,
}

// 异步全局数据存储
struct DataStore {
    data: Arc<Mutex<Vec<u8>>>,
}

// 创建备份文件
async fn backup_data(config: BackupConfig) -> Result<impl Reply, warp::Rejection> {
    let data = fs::read(&config.data_path).map_err(|e| warp::reject::custom(e))?;
    fs::write(&config.backup_path, data).map_err(|e| warp::reject::custom(e))?;
    
    Ok(warp::reply::json(&{"message": "Backup successful"}
        )
        .into_response())
}

// 恢复备份文件
async fn restore_data(config: BackupConfig) -> Result<impl Reply, warp::Rejection> {
    let backup_data = fs::read(&config.backup_path).map_err(|e| warp::reject::custom(e))?;
    if let Some(restore_path) = config.restore_path.as_ref() {
        fs::write(restore_path, backup_data).map_err(|e| warp::reject::custom(e))?;
    } else {
        return Err(warp::reject::custom("Restore path is required".to_string()));
    }
    
    Ok(warp::reply::json(&{"message": "Restore successful"}
        )
        .into_response())
}

// 数据备份和恢复的配置端点
fn config_filter() -> impl Filter<Extract = impl Reply, Error = warp::Rejection> + Clone {
    warp::post()
        .and(warp::path("backup"))
        .and(warp::path::end())
        .and(warp::body::json())
        .and(with_data_store())
        .and_then(backup_data)
}

fn restore_filter() -> impl Filter<Extract = impl Reply, Error = warp::Rejection> + Clone {
    warp::post()
        .and(warp::path("restore"))
        .and(warp::path::end())
        .and(warp::body::json())
        .and(with_data_store())
        .and_then(restore_data)
}

// 配置全局数据存储
fn with_data_store() -> impl Filter<Extract = DataStore, Error = std::convert::Infallible> + Clone {
    warp::any().map(move || DataStore {
        data: Arc::new(Mutex::new(Vec::new())),
    })
}

#[tokio::main]
async fn main() {
    let config_filter = config_filter();
    let restore_filter = restore_filter();
    
    let routes = config_filter.or(restore_filter);
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

// 错误处理
impl warp::reject::Reject for std::io::Error {}

// 将IO错误转换为WARP拒绝
fn custom_rejection(err: std::io::Error) -> warp::Rejection {
    warp::reject::custom(err)
}