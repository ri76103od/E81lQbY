use warp::Filter;

// 定义一个简单的测试数据结构
#[derive(Debug, Clone)]
struct TestData {
    value: String,
}

// 创建一个测试数据生成器函数
fn generate_test_data() -> Vec<TestData> {
    vec![
        TestData { value: "test_value_1".to_string() },
        TestData { value: "test_value_2".to_string() },
        TestData { value: "test_value_3".to_string() },
    ]
}

// 创建一个路由，返回测试数据的JSON表示
fn create_test_data_route() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path("test_data"))
        .and_then(|| async move {
            // 调用测试数据生成器函数
            let test_data = generate_test_data();
            // 将测试数据序列化为JSON
            let json = serde_json::to_string(&test_data)?;
            // 返回JSON响应
            Ok(warp::reply::json(&json))
        })
}

// 主函数，启动WARP服务器
#[tokio::main]
async fn main() {
    // 创建并启动路由
    let routes = create_test_data_route();
    println!("Server running on http://127.0.0.1:3030"
);
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
