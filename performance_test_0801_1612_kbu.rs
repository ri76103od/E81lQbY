// 使用WARP框架创建性能测试脚本
use warp::Filter;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::{Duration, Instant};
use futures::executor::block_on;

/// 共享请求计数器
struct SharedCounter {
    counter: Mutex<usize>,
}

impl SharedCounter {
    fn new() -> Arc<Self> {
        Arc::new(Self {
            counter: Mutex::new(0),
        })
    }

    fn increment(&self) {
        let mut counter = self.counter.lock().unwrap();
        *counter += 1;
    }
}

/// 创建性能测试的端点
fn create_performance_test_endpoint(counter: Arc<SharedCounter>) -> impl Filter<Extract = (), Error = warp::Rejection, Future = impl Future<Output = Result<&'static str, warp::Rejection>>> {
    warp::path("performance")
        .and(warp::get())
        .and(with_counter(counter))
        .map(|| "Request handled")
}

/// 使用共享计数器的提取器
fn with_counter(counter: Arc<SharedCounter>) -> impl Filter<Extract = (), Error = std::convert::Infallible, Future = impl Future<Output = Result<(), std::convert::Infallible>>> {
    warp::any().map(move || {
        counter.increment();
        ()
    }).untuple_one()
}

#[tokio::main]
async fn main() {
    // 创建共享计数器
    let counter = SharedCounter::new();

    // 创建性能测试端点
    let route = create_performance_test_endpoint(counter);

    // 启动WARP服务器
    let addr = "127.0.0.1:3030".parse().unwrap();
    println!("Running on http://{}", addr);
    warp::serve(route).run(addr).await;
}

/// 性能测试函数
fn performance_test() {
    let start_time = Instant::now();
    let url = "http://127.0.0.1:3030/performance";
    let client = reqwest::blocking::Client::new();
    for _ in 0..10000 {
        let _ = client.get(url).send();
    }
    let duration = start_time.elapsed();
    println!("Completed 10,000 requests in {:?}, average: {:.2?}