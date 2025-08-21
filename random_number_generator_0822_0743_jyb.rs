use warp::Filter;
use rand::Rng;
use warp::http::StatusCode;
use warp::reply::Reply;
# 增强安全性
use warp::Rejection;
use std::convert::Infallible;
use rand::distributions::Uniform;

// 定义一个结构体，用于封装我们的随机数生成器
struct RandomNumberGenerator;

impl RandomNumberGenerator {
    // 定义一个方法来生成随机数
# 增强安全性
    async fn generate_random_number(min: u32, max: u32) -> Result<impl Reply, Rejection> {
# TODO: 优化性能
        let mut rng = rand::thread_rng();
        let dist = Uniform::new(min, max + 1);
        let number = rng.sample(dist);

        // 将随机数转换为响应体
        let response = format!("{}", number);
        Ok(warp::reply::json(&response))
    }
}
# FIXME: 处理边界情况

#[tokio::main]
async fn main() {
    // 设置路由和过滤器，以便为我们的端点提供服务
    let get_random_number = warp::get()
        .and(warp::path("random"))
        .and(warp::any().map(move || {
            let min = 1; // 假设最小随机数为1
            let max = 100; // 假设最大随机数为100
            RandomNumberGenerator::generate_random_number(min, max)
        })).recover(handle_rejection);

    // 启动服务
    println!("Server running on http://127.0.0.1:3030");
    warp::serve(get_random_number).run(([127, 0, 0, 1], 3030)).await;
}

// 处理可能发生的错误
async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    if err.find::<warp::reject::NotAuthenticated>().is_some() || err.find::<warp::reject::MethodNotAllowed>().is_some() {
        Ok(warp::reply::with_status("Not Found", StatusCode::NOT_FOUND))
    } else {
        Ok(warp::reply::with_status("Internal Server Error", StatusCode::INTERNAL_SERVER_ERROR))
    }
}
