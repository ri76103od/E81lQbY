use warp::Filter;
use std::fs;
use std::path::Path;
use std::io::prelude::*;
use std::error::Error;
use serde::Deserialize;
use serde::Serialize;
use warp::http::StatusCode;
use warp::reject::Reject;
use warp::reply::{Response, Json};
use warp::Rejection;
use warp:: Reply;
use serde_json::json;

// 定义配置文件的内容结构
#[derive(Serialize, Deserialize, Debug)]
struct Config {
    database_url: String,
    api_key: String,
}

// 定义自定义错误类型
#[derive(Debug)]
enum ConfigError {
    FileNotFound,
    ReadError(std::io::Error),
    ParseError(serde_json::Error),
}

// 实现 Reject trait，以便我们的自定义错误可以被 Warp 处理
impl Reject for ConfigError {}

// 定义获取配置的端点
fn get_config() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("config")
    .and_then(|| handle_get_config())
}

// 处理 GET 请求以获取配置文件
async fn handle_get_config() -> Result<impl Reply, Rejection> {
    let config_path = "config.json";
    let config = read_config(config_path).await?;
    Ok(config.into_response())
}

// 读取配置文件
async fn read_config(path: &str) -> Result<Config, ConfigError> {
    let content = fs::read_to_string(path).await.map_err(ConfigError::ReadError)?;
    let config: Config = serde_json::from_str(&content).map_err(ConfigError::ParseError)?;
    Ok(config)
}

impl Config {
    // 将配置转换为 Response 对象
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

// 启动 Warp 服务器
#[tokio::main]
async fn main() {
    let config_route = get_config();
    warp::serve(config_route)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

// 用于处理自定义错误的函数
async fn handle_rejection(err: Rejection) -> Result<impl Reply, std::convert::Infallible> {
    if err.is_not_found() {
        Ok(warp::reply::with_status("Config not found", StatusCode::NOT_FOUND))
    } else {
        Err(err)
    }
}