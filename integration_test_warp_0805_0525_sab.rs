// integration_test_warp.rs
// 这是一个使用RUST和WARP框架的集成测试工具

use warp::Filter;
use warp::http::StatusCode;
use warp::test::WarpTest;
use warp::Rejection;
use warp::Reply;
use std::task::{Context, Poll};
use futures::future::Future;

// 定义一个简单的API端点，返回'Hello World'
fn hello_world() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("hello" / "world")
        .map(|| "Hello World")
        .with(warp::reply::json())
}

// 定义一个模拟的API端点，返回特定的响应
fn mock_api() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("mock" / "api")
        .map(|| {
            serde_json::json!({
                "message": "This is a mock API response"
            })
        }).with(warp::reply::json())
}

// 定义测试函数，使用WarpTest来模拟请求和断言响应
fn test_hello_world() {
    let hello_world = hello_world();
    let warp_test = WarpTest::new();

    // 模拟对'/hello/world'的GET请求
    let response = warp_test.run(&hello_world, warp::test::request()).expect("Failed to run warp test");

    // 断言响应状态码为200
    assert_eq!(response.status(), StatusCode::OK);

    // 断言响应体为'Hello World'
    assert_eq!(response.body(), b"Hello World");
}

// 定义测试函数，使用WarpTest来模拟请求和断言响应
fn test_mock_api() {
    let mock_api = mock_api();
    let warp_test = WarpTest::new();

    // 模拟对'/mock/api'的GET请求
    let response = warp_test.run(&mock_api, warp::test::request()).expect("Failed to run warp test");

    // 断言响应状态码为200
    assert_eq!(response.status(), StatusCode::OK);

    // 断言响应体为特定的JSON对象
    assert_eq!(response.body(), b"{"message":"This is a mock API response"}");
}

// 入口函数，运行测试
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello_world_request() {
        test_hello_world();
    }

    #[test]
    fn test_mock_api_request() {
        test_mock_api();
    }
}
