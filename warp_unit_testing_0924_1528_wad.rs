use warp::Filter;
use warp::http::StatusCode;

// 定义一个简单的处理器结构体
struct TestHandler;

impl TestHandler {
    // 创建一个新的处理器
    fn new() -> Self {
        TestHandler
    }

    // 定义一个处理函数
    async fn handle_request(&self) -> Result<impl warp::Reply, warp::Rejection> {
        // 这里只是一个示例逻辑，实际可以根据需要进行修改
        Ok(warp::reply::json(&"Hello, World!"))
    }
}

// 使用warp过滤器来定义HTTP路由
fn with_test_handler(handler: TestHandler) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("test")
        .and(warp::get())
        .and(warp::any().map(move || handler.clone()))
        .and_then(|handler: TestHandler| async move {
            handler.handle_request().await
        })
}

#[tokio::main]
async fn main() {
    let handler = TestHandler::new();
    let routes = with_test_handler(handler);
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

// 单元测试模块
#[cfg(test)]
mod tests {
    use super::*;
    use warp::http::Response;
    use warp::test::request;

    #[tokio::test]
    async fn test_request() {
        let handler = TestHandler::new();
        let response: Response = request()
            .path("/test")
            .method("GET")
            .reply(&with_test_handler(handler))
            .await;

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.body(), b""Hello, World!"");
    }
}
