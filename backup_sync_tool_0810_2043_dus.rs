// backup_sync_tool.rs
// 一个使用RUST和WARP框架实现的文件备份和同步工具

use warp::Filter;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::io::Error;
use std::fs::Metadata;
use std::time::SystemTime;
use std::collections::HashMap;
use serde::Serialize;
use serde_json::json;
use std::sync::Arc;

// 定义文件的元数据结构体
#[derive(Serialize)]
struct FileInfo {
    path: String,
    size: u64,
    mtime: u64,
}

// 创建文件备份和同步的API路由
fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let backup_file = warp::post()
        .and(warp::path("backup"))
        .and(warp::path::param())
        .and(with_file_system_state())
        .and_then(|path: String, file_system_state: Arc<HashMap<String, FileInfo>>| {
            let path = Path::new(&path);
            let metadata = match path.metadata() {
                Ok(metadata) => metadata,
                Err(e) => return Err(warp::reject::custom(e)),
            };
            
            let file_info = FileInfo {
                path: path.to_str().unwrap().to_string(),
                size: metadata.len(),
                mtime: metadata.modified().unwrap().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs(),
            };
            
            file_system_state.insert(path.to_str().unwrap().to_string(), file_info);
            Ok(warp::reply::json(&json!({
                "message": "File backed up successfully",
                "file_info": file_info,
            })))
        });
    
    let sync_files = warp::post()
        .and(warp::path("sync"))
        .and(with_file_system_state())
        .and_then(|file_system_state: Arc<HashMap<String, FileInfo>>| {
            let mut changes: Vec<&FileInfo> = Vec::new();
            for (path, file_info) in file_system_state.iter() {
                let current_metadata = match fs::metadata(path) {
                    Ok(metadata) => metadata,
                    Err(_) => continue,
                };
                
                if current_metadata.len() != file_info.size ||
                    current_metadata.modified().unwrap().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() != file_info.mtime {
                    changes.push(file_info);
                }
            }
            Ok(warp::reply::json(&json!({
                "message": "Sync completed",
                "changes": changes,
            })))
        });
    
    backup_file.or(sync_files)
}

// 辅助函数，用于获取文件系统状态
fn with_file_system_state() -> impl Filter<Extract = Arc<HashMap<String, FileInfo>>, Error = std::convert::Infallible> + Clone {
    warp::any().map(move || {
        Arc::new(HashMap::new())
    }).recover(|_| {
        // 如果出现错误，返回一个空的文件系统状态
        Arc::new(HashMap::new())
    }).untuple_one()
}

// 主函数，启动WARP服务器
#[tokio::main]
async fn main() {
    let routes = routes();
    println!("Server started at http://127.0.0.1:3030");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}