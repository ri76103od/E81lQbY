//! 配置文件管理器
//!
//! 这个程序使用 Rust 和 Warp 框架来实现一个简单的配置文件管理器。
//! 它提供了读取和更新配置文件的功能。

use warp::http::StatusCode;
use warp::{Filter, Rejection, Reply};
use std::fs;
use std::io::Read;
use std::path::Path;
use serde::{Deserialize, Serialize};
use serde_json::Result as SerdeResult;
use thiserror::Error;

// 错误定义
#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("File not found: {0}")]
    FileNotFound(String),
    #[error("Could not read file: {0}")]
    IoError(String),
    #[error("Could not parse config: {0}")]
    ParseError(String),
    #[error("Invalid config update: {0}")]
    InvalidUpdate(String),
}

// 配置结构体定义
#[derive(Serialize, Deserialize, Debug)]
struct Config {
    // 可以在这里添加更多的配置项
    setting1: String,
    setting2: u32,
}

// 读取配置文件
async fn read_config(path: String) -> Result<impl Reply, Rejection> {
    let path = Path::new(&path);
    let mut file = match fs::File::open(path) {
        Ok(file) => file,
        Err(e) => return Err(warp::reject::custom(ConfigError::FileNotFound(e.to_string()))),
    };
    let mut contents = String::new();
    if let Err(e) = file.read_to_string(&mut contents) {
        return Err(warp::reject::custom(ConfigError::IoError(e.to_string())));
    }
    let config: Config = match serde_json::from_str(&contents) {
        Ok(config) => config,
        Err(e) => return Err(warp::reject::custom(ConfigError::ParseError(e.to_string()))),
    };
    Ok(warp::reply::json(&config))
}

// 更新配置文件
async fn update_config(path: String, config: Config) -> Result<impl Reply, Rejection> {
    let path = Path::new(&path);
    let contents = serde_json::to_string(&config).map_err(|e| warp::reject::custom(ConfigError::InvalidUpdate(e.to_string())))?;
    if let Err(e) = fs::write(path, contents) {
        return Err(warp::reject::custom(ConfigError::IoError(e.to_string())));
    }
    Ok(warp::reply::with_status("Config updated", StatusCode::OK))
}

// 创建 Warp 路由
fn config_routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let read = warp::path("config")
        .and(warp::path::param())
        .and_then(read_config);
    let update = warp::path("config")
        .and(warp::path::param())
        .and(warp::post())
        .and(warp::body::json())
        .and_then(|config: Config| update_config(config.0, config.1));
    read.or(update)
}

#[tokio::main]
async fn main() {
    // 启动 Warp 服务器
    let routes = config_routes();
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
