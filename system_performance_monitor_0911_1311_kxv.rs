use warp::Filter;

// 定义一个结构体来存放系统性能监控的数据
#[derive(Debug, Deserialize)]
struct PerformanceMetrics {
    cpu_usage: f32,
    memory_usage: f32,
    disk_usage: f32,
}

// 创建一个异步函数来获取系统性能监控数据
// 这里我们使用一个模拟的数据，实际应用中你需要替换为真实的系统调用
async fn get_performance_metrics() -> Result<PerformanceMetrics, warp::Rejection> {
    let cpu_usage = 0.75; // 模拟CPU使用率
    let memory_usage = 0.60; // 模拟内存使用率
    let disk_usage = 0.80; // 模拟磁盘使用率

    Ok(PerformanceMetrics {
        cpu_usage,
        memory_usage,
        disk_usage,
    })
}

// 创建一个路由来返回系统性能监控数据
fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("performance") // 定义路由路径
        .and(warp::get()) // 定义HTTP方法为GET
        .map(|| async { // 使用异步块来处理请求
            match get_performance_metrics().await {
                Ok(metrics) => {
                    warp::reply::json(&metrics) // 使用WARP的JSON回复功能
                }
                Err(_) => {
                    warp::reject::not_found() // 处理错误情况
                }
            }
        })
}

#[tokio::main]
async fn main() {
    let routes = routes();
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await; // 启动WARP服务器
}

// 以下是宏定义和模块导入，需要放在文件的最顶部
#[macro_use]
extern crate serde_derive;
extern crate warp;
extern crate tokio;
