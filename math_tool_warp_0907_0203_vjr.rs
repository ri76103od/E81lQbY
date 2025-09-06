use warp::Filter;

#[tokio::main]
async fn main() {
    // 定义HTTP路由
    let routes = warp::path("math")
        .and(warp::path::param())
        .map(|operation: String| match operation.as_str() {
            "add" => add(1, 2),
            "subtract" => subtract(10, 5),
            "multiply" => multiply(3, 4),
            "divide" => divide(20, 4),
            _ => "Invalid operation",
        }).recover(|e: warp::Rejection| async move {
            // 错误恢复，返回404错误
            warp::reply::with_status(e, warp::http::StatusCode::NOT_FOUND)
        }).with(warp::reply::json);

    // 启动服务器
    println!("Server running on http://127.0.0.1:3030/");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

// 加法函数
fn add(a: i32, b: i32) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::json(&a + b))
}

// 减法函数
fn subtract(a: i32, b: i32) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::json(&a - b))
}

// 乘法函数
fn multiply(a: i32, b: i32) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::json(&a * b))
}

// 除法函数
fn divide(a: i32, b: i32) -> Result<impl warp::Reply, warp::Rejection> {
    if b == 0 {
        return Err(warp::reject::custom("math_error: Division by zero"));
    }
    Ok(warp::reply::json(&a / b))
}