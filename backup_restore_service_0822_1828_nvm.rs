use warp::http::StatusCode;
use warp::Filter;
use std::fs;
use std::path::Path;
use std::result::Result;
use serde::{Deserialize, Serialize};
use serde_json::json;
use warp::reject;

// 定义请求体结构
#[derive(Deserialize, Serialize)]
struct BackupRequest {
    data: String,
}

// 备份文件
async fn backup_file(path: &str, data: &str) -> Result<impl warp::Reply, warp::Rejection> {
    let file_path = Path::new(path);
    if file_path.exists() {
        // 如果文件已存在，返回错误
        return Err(reject::custom(BackupError::FileExists));
    }
    fs::write(file_path, data).map_err(|_| reject::custom(BackupError::WriteError))?;
    Ok(warp::reply::json(&json!({
        "status": "success",
        "message": "File backed up successfully",
    })))
}

// 恢复文件
async fn restore_file(path: &str) -> Result<impl warp::Reply, warp::Rejection> {
    let file_path = Path::new(path);
    if !file_path.exists() {
        // 如果文件不存在，返回错误
        return Err(reject::custom(BackupError::FileNotFound));
    }
    let data = fs::read_to_string(file_path).map_err(|_| reject::custom(BackupError::ReadError))?;
    Ok(warp::reply::json(&json!({
        "status": "success",
        "message": "File restored successfully",
        "data": data,
    })))
}

// 定义错误类型
#[derive(Debug)]
enum BackupError {
    FileExists,
    WriteError,
    FileNotFound,
    ReadError,
}

// 实现自定义错误类型的reject方法
impl warp::reject::Reject for BackupError {}

// 创建备份文件的路由
fn backup_route() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post()
        .and(warp::path("backup"))
        .and(warp::body::content_length_limit(1024 * 32)) // 限制请求体大小为32KB
        .and(warp::body::json())
        .and(warp::path::param::<String>())
        .and_then(|backup_request: BackupRequest, path: String| async move {
            backup_file(&path, &backup_request.data).await
        })
}

// 创建恢复文件的路由
fn restore_route() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post()
        .and(warp::path("restore"))
        .and(warp::path::param::<String>())
        .and_then(|path: String| async move {
            restore_file(&path).await
        })
}

// 主函数
#[tokio::main]
async fn main() {
    let backup_routes = backup_route().or(restore_route());
    warp::serve(backup_routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
