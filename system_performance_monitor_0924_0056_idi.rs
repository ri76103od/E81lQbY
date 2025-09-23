use warp::http::StatusCode;
use warp::{Filter, Reply, Rejection};
use sysinfo::{System, SystemExt};
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::http::Response;

// 创建一个系统性能监控工具的模块
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // 创建一个系统信息的实例
    let sys = System::new_all();
    let sys = Arc::new(Mutex::new(sys));

    // 定义路由
    let health_check = warp::path("health")
        .and_then(|| async move {
            Ok(Response::builder()
                .status(StatusCode::OK)
                .body("This service is up and running.")?)
        });

    let system_info = warp::path("system")
        .and_then(|| async move {
            let sys = sys.lock().await;
            Ok(format!("Cpu: {:.2}%", sys.cpu_usage()))
        });

    // 启动服务器
    warp::serve(health_check.or(system_info))
        .run(([127, 0, 0, 1], 3030))
        .await;
}

// 错误处理
impl warp::reject::Reject for std::io::Error {}

// 处理GET请求的函数
async fn get_system_info() -> Result<impl Reply, Rejection> {
    let sys = sys.lock().await;
    let mut info = String::new();
    info.push_str(&format!("Cpu: {:.2}%", sys.cpu_usage()));
    info.push_str("
");
    info.push_str(&format!("Load: {:.2}%", sys.load_average()));
    Ok(warp::reply::json(&info))
}
