use warp::Filter;

// 引入必要的依赖库
use std::error::Error;
use serde::{Deserialize, Serialize};
use serde_json::json;
use warp::reject::Reject;
use warp::reply::Reply;

// 定义自定义的错误类型
#[derive(Debug)]
struct ValidationError;

impl Reject for ValidationError {}

// 定义消息通知请求的结构体
#[derive(Deserialize)]
struct NotificationRequest {
    message: String,
}

// 将请求结构体序列化为JSON
impl Reply for NotificationRequest {
    fn into_response(self) -> warp::reply::Response {
        json!({
            "status": "success",
            "message": self.message,
        })
        .into_response()
    }
}

// 定义通知服务的处理函数
async fn notify(notification: NotificationRequest) -> Result<impl Reply, ValidationError> {
    // 模拟消息发送
    println!("Sending notification: {}", notification.message);
    
    // 这里可以添加更多的逻辑来处理消息发送，例如发送到数据库或外部服务
    
    Ok(notification)
}

// 定义路由和过滤器
fn routes() -> impl Filter<Extract = impl Reply, Error = impl Reject> + Clone {
    warp::post()
        .and(warp::path("notify"))
        .and(warp::body::json())
        .and_then(notify)
        .recover(handle_rejection)
}

// 错误处理函数
async fn handle_rejection(err: warp::Rejection) -> Result<impl Reply, warp::Rejection> {
    let code = match err.find::<ValidationError>() {
        Some(_) => 400,
        None => 500,
    };
    
    Ok(warp::reply::with_status(
        json!({
            "status": "error",
            "message": format!("Internal server error: {:?}