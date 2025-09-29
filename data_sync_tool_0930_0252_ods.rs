use warp::Filter;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::io::{self, Read};
use warp::http::StatusCode;
use serde::{Serialize, Deserialize};
use tokio::sync::Mutex as AsyncMutex;
use tokio::sync::RwLock;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use futures::stream::StreamExt;
use futures::future::join_all;
use serde_json::json;
use warp::http::{Response, Request, Uri};

// 定义全局共享数据
struct SharedData {
    pub data: Arc<AsyncMutex<HashMap<String, String>>>
}

// 定义同步数据请求的结构体
#[derive(Deserialize)]
struct SyncDataRequest {
    key: String,
    value: String
}

// 定义同步数据响应的结构体
#[derive(Serialize)]
struct SyncDataResponse {
    status: String,
    message: String
}

// 启动数据同步工具
#[tokio::main]
async fn main() {
    let shared_data = SharedData {
        data: Arc::new(AsyncMutex::new(HashMap::new()))
    };

    // 设置路由
    let sync_route = warp::path("sync")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_shared_data(shared_data.clone()))
        .then(sync_data_handler);

    // 启动服务
    warp::serve(sync_route).run(([0, 0, 0, 0], 3030)).await;
}

// 同步数据处理函数
async fn sync_data_handler(body: SyncDataRequest, shared_data: SharedData) -> Result<impl warp::Reply, warp::Rejection> {
    // 尝试获取共享数据
    let mut data = match shared_data.data.lock().await {
        Ok(data) => data,
        Err(_) => return Ok(warp::reply::json(&SyncDataResponse {
            status: "error".to_string(),
            message: "Failed to acquire lock on shared data".to_string()
        })).into_response()
    };

    // 更新共享数据
    data.insert(body.key, body.value);

    // 返回成功响应
    Ok(warp::reply::json(&SyncDataResponse {
        status: "success".to_string(),
        message: "Data synced successfully