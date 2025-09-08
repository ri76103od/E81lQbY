// error_log_collector.rs
// 这个程序使用RUST和WARP框架实现一个简单的错误日志收集器。
// 它监听HTTP请求并记录错误日志。

use warp::Filter;

#[tokio::main]
async fn main() {
    // 设置日志记录器
    setup_logger();

    // 定义路由
    let routes = error_log_route();

    // 启动服务器
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

// 设置日志记录器
fn setup_logger() {
    // 使用env_logger初始化日志记录器
    env_logger::init();
}

// 定义一个用于记录错误日志的路由
fn error_log_route() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("log")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handle_error_log)
}

// 处理错误日志的函数
async fn handle_error_log(error: warp::json::Json<ErrorLog>) -> Result<impl warp::Reply, warp::Rejection> {
    // 检查错误日志数据
    if error.message.is_empty() {
        return Err(warp::reject::custom(InvalidErrorLog));
    }

    // 记录错误日志
    log::error!("Error: {}", error.message);

    // 返回成功响应
    Ok(warp::reply::json(&"Error logged successfully"))
}

// 定义错误日志结构体
#[derive(warp::serde::Serialize, serde::Deserialize, Debug)]
struct ErrorLog {
    message: String,
}

// 定义自定义错误类型
#[derive(Debug)]
struct InvalidErrorLog;

// 实现自定义错误类型的warp::reject::Reject
impl warp::reject::Reject for InvalidErrorLog {}
