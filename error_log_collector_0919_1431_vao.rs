use warp::Filter;

// 定义日志收集器结构体
struct ErrorLogCollector;

impl ErrorLogCollector {
    // 创建一个新的错误日志收集器实例
    pub fn new() -> Self {
        ErrorLogCollector
    }

    // 定义一个处理错误日志的函数
    pub fn log_error(&self, error: String) {
        // 这里可以使用实际的日志记录框架或库来记录错误信息
        // 例如使用 `log` crate 或者将错误写入文件
        println!("Error: {}", error);
    }
}

// 使用WARP框架创建一个HTTP服务
#[tokio::main]
async fn main() {
    // 实例化错误日志收集器
    let error_logger = ErrorLogCollector::new();

    // 定义一个POST路由，用于接收错误日志
    let log_error_route = warp::post()
        .and(warp::path("log_error"))
        .and(warp::body::json())
        .and(with_error_logger(error_logger))
        .map(move |error: String| {
            // 调用错误日志收集器的log_error方法
            error_logger.log_error(error);

            // 返回响应
            warp::reply::json(&"Error logged successfully")
        });

    // 启动服务
    warp::serve(log_error_route)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

// 定义一个闭包，用于将错误日志收集器传递给路由处理器
fn with_error_logger(error_logger: ErrorLogCollector) -> impl Filter<Extract = (ErrorLogCollector,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || error_logger.clone())
}