// environment_monitoring_system.rs

// 引入所需的库
use warp::Filter;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::sync::Arc;
use tokio::sync::Mutex;

// 定义环境监测系统的状态数据结构
#[derive(Clone, Deserialize, Serialize, Debug)]
struct EnvironmentData {
    // 温度
    temperature: f64,
    // 湿度
    humidity: f64,
    // 二氧化碳浓度
    co2_concentration: f64,
}

// 环境监测系统的模拟数据
lazy_static::lazy_static! {
    static ref ENVIRONMENT_DATA: Arc<Mutex<EnvironmentData>> = Arc::new(Mutex::new(EnvironmentData {
        temperature: 22.5,
        humidity: 45.0,
        co2_concentration: 400.0,
    }));
}

// 创建环境监测系统的API端点
fn create_environment_api() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // 获取环境数据的端点
    warp::path("environment")
        .and(warp::get())
        .and(with_environment_data())
        .and_then(get_environment_data)
}

// 获取环境数据的函数
async fn get_environment_data(data: EnvironmentData) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::json(&data))
}

// 用于获取环境数据的中间件
fn with_environment_data() -> impl Filter<Extract = EnvironmentData, Error = std::convert::Infallible> + Clone {
    warp::any().map(move || {
        // 从静态变量中获取环境数据
        let data = ENVIRONMENT_DATA.clone();
        let data = data.lock().unwrap();
        data.clone()
    })
}

#[tokio::main]
async fn main() {
    // 启动WARP服务器
    let api = create_environment_api();
    warp::serve(api)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

// 引入必要的库
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate lazy_static;
extern crate warp;
extern crate serde;
extern crate tokio;
extern crate tokio_sync;

// 启用必要的特性
#[cfg(test)]
mod tests {
    use super::*;
    use warp::http::StatusCode;
    use warp::test::request;
    use warp::Filter;

    #[tokio::test]
    async fn test_get_environment_data() {
        // 测试环境数据的获取
        let api = create_environment_api();
        let res = request()
            .path("/environment")
            .filter(&api)
            .await;
        assert_eq!(res.status(), StatusCode::OK);
    }
}
