use warp::Filter;
use std::fs;
use std::io::Error;
use std::path::PathBuf;
use serde::Deserialize;
use serde_json;
use warp::http::StatusCode;
use warp::reject::{reject, Reject};

// 定义请求体结构
#[derive(Deserialize)]
struct UpdateRequest {
    firmware: String,
}

// 自定义错误类型
#[derive(Debug)]
enum FirmwareUpdateError {
    FileError(Error),
    InvalidFirmware,
}

// 实现自定义错误类型到 Reject 的转换
impl Reject for FirmwareUpdateError {}

// 更新固件的函数
async fn update_firmware(path: PathBuf, update_request: UpdateRequest) -> Result<impl warp::Reply, FirmwareUpdateError> {
    // 检查固件是否有效，这里简单地检查长度
    if update_request.firmware.len() < 10 {
        return Err(FirmwareUpdateError::InvalidFirmware);
    }

    // 尝试将固件写入文件
    fs::write(path, update_request.firmware).map_err(FirmwareUpdateError::FileError)
}

// 设置路由和过滤器
fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post()
        .and(warp::path("update"))
        .and(warp::path::tail())
        .and(warp::body::content_length_limit(1024 * 1024)) // 限制请求体大小为1MB
        .and(warp::body::json())
        .and_then(|path: PathBuf, update_request: UpdateRequest| {
            async move {
                match update_firmware(path, update_request).await {
                    Ok(_) => Ok(warp::reply::json(&"Firmware updated successfully")
                        .with_status(StatusCode::OK)),
                    Err(e) => Err(warp::reject::custom(e)),
                }
            }
        })
}

#[tokio::main]
async fn main() {
    let routes = routes();
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

// 实现 FirmwareUpdateError 的 Display 特性，用于错误信息显示
impl std::fmt::Display for FirmwareUpdateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FirmwareUpdateError::FileError(e) => write!(f, "File error: {}", e),
            FirmwareUpdateError::InvalidFirmware => write!(f, "Invalid firmware data"),
        }
    }
}
