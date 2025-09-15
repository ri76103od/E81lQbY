use warp::Filter;
use std::sync::Arc;
use std::process::Command;
use tokio::process::Command as TokioCommand;
use serde::Serialize;
use serde_json::json;

// 定义一个结构体来存储内存使用情况
#[derive(Serialize)]
struct MemoryUsage {
    total: String,
    free: String,
    used: String,
    used_percent: String,
    available: String,
}

// 创建一个获取内存使用情况的异步函数
async fn get_memory_usage() -> Result<MemoryUsage, warp::Rejection> {
    let output = TokioCommand::new("free")
        .arg("-m")
        .output()
        .await;
    
    if let Ok(output) = output {
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let lines: Vec<_> = stdout.lines().collect();
            
            let mem_info: Vec<_> = lines[1].split_whitespace().collect();
            
            // 根据free命令输出解析内存使用情况
            Ok(MemoryUsage {
                total: mem_info.get(1).unwrap_or(&"N/A").to_string(),
                free: mem_info.get(3).unwrap_or(&"N/A").to_string(),
                used: mem_info.get(2).unwrap_or(&"N/A").to_string(),
                used_percent: mem_info.get(4).unwrap_or(&"N/A").to_string(),
                available: mem_info.get(6).unwrap_or(&"N/A").to_string(),
            })
        } else {
            Err(warp::reject::not_found())
        }
    } else {
        Err(warp::reject::reject())
    }
}

// 创建一个路由来处理内存分析请求
fn memory_analysis_route() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("memory")
        .and(warp::get())
        .and_then(|| async {
            match get_memory_usage().await {
                Ok(mem_usage) => Ok(warp::reply::json(&mem_usage)),
                Err(_) => Err(warp::reject::not_found()),
            }
        })
}

#[tokio::main]
async fn main() {
    // 启动WARP服务器
    let addr = warp::addr::tokio::Tokio::bind("0.0.0.0:3030");
    warp::serve(memory_analysis_route())
        .run(addr)
        .await;
}
