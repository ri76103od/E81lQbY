use warp::Filter;

// 定义一个简单的HTTP请求处理器
async fn handle_request() -> Result<impl warp::Reply, warp::Rejection> {
    // 处理请求并返回响应
    Ok(warp::reply::json(&"Hello, World!"))
}

// 设置路由和启动服务器的函数
fn main() {
    // 使用`warp::path!("...")`来设置路径
    let route = warp::path("hello")
        // 使用`warp::get()`来设置HTTP方法
        .and(warp::get())
        // 使用`handle_request`函数来处理请求
        .and_then(handle_request);

    // 启动WARP服务器，并监听localhost的3030端口
    println!("Server running on http://localhost:3030/");
    warp::serve(route).run(([127, 0, 0, 1], 3030)).await;
}

// 错误处理
// 如果`handle_request`函数中出现错误，这里可以定义错误处理逻辑
// 例如，可以自定义一个错误类型并实现`warp::reject::Reject` trait来处理特定的错误。