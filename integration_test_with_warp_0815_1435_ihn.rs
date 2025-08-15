use warp::Filter;

// 定义一个简单的GET请求处理器
fn hello_world() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path::end()
        .and(warp::get())
        .map(|| "Hello, World!")
}

// 定义集成测试
#[cfg(test)]
mod tests {
    use super::*;
    use warp::http::StatusCode;
    use warp::test::request;

    #[test]
    async fn test_hello_world() {
        let hello_world = hello_world();
        let response = request()
            .method("GET")
            .path("/")
            .reply(&hello_world)
            .await;

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.body(), "Hello, World!");
    }
}

// 运行WARP服务器
#[tokio::main]
async fn main() {
    let hello_world = hello_world();
    warp::serve(hello_world)
        .run(([127, 0, 0, 1], 3030))
        .await;
}