use warp::Filter;

// 定义一个简单的GET请求处理函数
async fn get_request() -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::json(&"Hello, World!"))
}

// 定义路由并返回一个Filter
fn main() {
    // 创建一个GET请求的路由，路径为"/"，处理函数为get_request
    let route = warp::path::end().map(|| get_request());

    // 启动服务器监听在127.0.0.1的3030端口
    warp::serve(route)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

// 集成测试模块
#[cfg(test)]
mod tests {
    use super::*;
    use warp::test::request;

    #[tokio::test]
    async fn test_get_request() {
        // 创建一个GET请求
        let response = request()
            .method("GET")
            .path("")
            .send()
            .await;

        // 检查状态码是否为200 OK
        assert_eq!(response.status(), 200);

        // 检查响应体是否为预期的JSON字符串
        let body = response.body();
        let expected_body = r#"{