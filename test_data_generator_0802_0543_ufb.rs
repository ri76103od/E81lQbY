use warp::Filter;

// 定义一个简单的测试数据生成器
fn main() {
    // 定义一个生成随机字符串的过滤器
    let generate_random_string = warp::path("random")
        .and(warp::get())
        .map(|| generate_random_string_data());

    // 启动WARP服务器
    warp::serve(generate_random_string)
        .run(([127, 0, 0, 1], 3030));
}

// 生成随机字符串数据的函数
fn generate_random_string_data() -> impl warp::Reply {
    // 这里使用了Rust的内置库来生成随机字符串
    let random_string = rand::random::<u32>().to_string();

    // 返回生成的随机字符串
    warp::reply::json(&random_string)
}

// 引入必要的库
#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
fn main() {
    // 定义WARP过滤器
    let generate_random_string = warp::path("random")
        .and(warp::get())
        .map(|| generate_random_string_data());

    // 启动WARP服务器
    warp::serve(generate_random_string)
        .run(([127, 0, 0, 1], 3030));
}

// 引入必要的库
use warp::Filter;
use serde::Serialize;
use serde_json::json;
use rand::Rng;

// 定义一个简单的测试数据生成器
fn main() {
    // 定义一个生成随机字符串的过滤器
    let generate_random_string = warp::path("random")
        .and(warp::get())
        .map(|| generate_random_string_data());

    // 启动WARP服务器
    warp::serve(generate_random_string)
        .run(([127, 0, 0, 1], 3030));
}

// 生成随机字符串数据的函数
fn generate_random_string_data() -> impl warp::Reply {
    // 这里使用了Rust的内置库来生成随机字符串
    let random_string: String = (0..10)
        .map(|_| rand::random::<char>() )
        .collect();

    // 返回生成的随机字符串
    warp::reply::json(&json!({ "random_string": random_string }))
}

// 引入必要的库
use warp::Filter;
use serde::Serialize;
use serde_json::json;
use rand::Rng;
use warp::reply::Json;
use warp::Rejection;
use warp::http::StatusCode;

// 定义一个简单的测试数据生成器
fn main() {
    // 定义一个生成随机字符串的过滤器
    let generate_random_string = warp::path("random")
        .and(warp::get())
        .and_then(handle_generate_random_string)
        .recover(handle_rejection);

    // 启动WARP服务器
    warp::serve(generate_random_string)
        .run(([127, 0, 0, 1], 3030));
}

// 处理生成随机字符串的请求
async fn handle_generate_random_string() -> Result<impl warp::Reply, warp::Rejection> {
    // 这里使用了Rust的内置库来生成随机字符串
    let random_string: String = (0..10)
        .map(|_| rand::random::<char>() )
        .collect();

    // 返回生成的随机字符串
    Ok(warp::reply::json(&json!({ "random_string": random_string })))
}

// 错误处理函数
fn handle_rejection(err: warp::Rejection) -> Result<impl warp::Reply, warp::Rejection> {
    // 将错误转换为合适的响应
    let code = match err.find::<warp::reject::NotFound>() {
        Some(_) => warp::http::StatusCode::NOT_FOUND,
        None => warp::http::StatusCode::INTERNAL_SERVER_ERROR,
    };
    
    // 返回错误响应
    Err(warp::reject::custom(code))
}
