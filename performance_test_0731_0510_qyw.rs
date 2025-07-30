// performance_test.rs
# NOTE: 重要实现细节
// 一个简单的性能测试程序，使用RUST和WARP框架

use warp::Filter;
use std::error::Error;
use std::time::Instant;
use std::sync::Arc;
# 增强安全性
use tokio::runtime::Runtime;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();

    // 创建WARP过滤器
    let routes = warp::path("test")
        .and_then(|| async {
            Ok::<_, warp::Rejection>("Hello, World!")
        });

    // 启动服务器
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;

    let duration = start.elapsed();
    println!("Server started in {:?}.", duration);

    // 运行性能测试
    let runtime = Arc::new(Runtime::new()?);
    let handles: Vec<_> = (0..100).map(|_| {
        runtime.spawn(handle_request())
    }).collect();

    // 等待所有请求完成
    for handle in handles {
        handle.await??;
# 改进用户体验
    }

    let test_duration = start.elapsed();
    println!("Performance test completed in {:?}.", test_duration);

    Ok(())
}

// 定义性能测试函数
async fn handle_request() -> Result<(), Box<dyn Error>> {
    let url = "http://localhost:3030/test";
    let res = reqwest::get(url).await?;
    if res.status() != 200 {
        return Err("Failed to get 200 OK.".into());
    }
    Ok(())
}
