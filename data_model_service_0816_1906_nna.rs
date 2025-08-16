// data_model_service.rs

// 引入所需的库
use warp::Filter;
use serde::{Deserialize, Serialize};
use warp::reject::Reject;
use std::fmt;

// 定义数据模型
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserModel {
    pub id: i32,
    pub username: String,
    pub email: String,
}

// 自定义错误类型
#[derive(Debug)]
pub enum AppError {
    NotFound,
    InvalidInput,
    InternalError(String),
}

// 实现错误转换特性，将AppError转换为warp::Rejection
impl Reject for AppError {}

// 错误格式化实现
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::NotFound => write!(f, "Not found"),
            AppError::InvalidInput => write!(f, "Invalid input"),
            AppError::InternalError(msg) => write!(f, "{}", msg),
        }
    }
}

// 创建一个简单的GET路由，返回UserModel实例
pub fn get_user() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("user")
        .and(warp::get())
        .and_then(|_| async move {
            Ok(warp::reply::json(&UserModel {
                id: 1,
                username: "JohnDoe".to_string(),
                email: "johndoe@example.com".to_string(),
            }))
        })
        .recover(handle_rejection)
}

// 统一错误处理函数
fn handle_rejection(err: warp::Rejection) -> Result<impl warp::Reply, warp::Rejection> {
    if err.is_not_found() {
        // 处理404错误
        Ok(warp::reply::with_status(
            warp::reply::json(&AppError::NotFound),
            warp::http::StatusCode::NOT_FOUND,
        ))
    } else if let Some(AppError::InvalidInput) = err.find::<AppError>() {
        // 处理自定义的InvalidInput错误
        Ok(warp::reply::with_status(
            warp::reply::json(&AppError::InvalidInput),
            warp::http::StatusCode::BAD_REQUEST,
        ))
    } else {
        // 处理其他所有错误
        Ok(warp::reply::with_status(
            warp::reply::json(&AppError::InternalError("An internal error occurred".to_string())),
            warp::http::StatusCode::INTERNAL_SERVER_ERROR,
        ))
    }
}

// 定义main函数，启动服务器
#[tokio::main]
pub async fn main() {
    let routes = get_user();
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
