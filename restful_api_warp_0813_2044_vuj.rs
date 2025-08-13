use warp::Filter;

// 定义一个简单的数据结构用于POST请求
#[derive(Debug, serde::Deserialize)]
struct MyData {
    name: String,
    value: i32,
}

#[tokio::main]
async fn main() {
    // 定义GET请求路由
    let get_route = warp::path!("api" / "get")
        .map(|| warp::reply::json(&{"message": "This is a GET request"}));

    // 定义POST请求路由
    let post_route = warp::path!("api" / "post")
        .and(warp::post())
        .and(warp::body::json())
        .map(|data: MyData| {
            // 简单的错误处理：如果name为空，返回错误信息
            if data.name.is_empty() {
                return warp::reject::custom(MyError("name cannot be empty"));
            }

            // 简单的响应处理
            warp::reply::json(&{"result": format!("Received {} with value {}", data.name, data.value)});
        });

    // 定义错误处理
    let error_route = warp::any().map(move |err| {
        let code = match err.find::<warp::reject::Not Found>() {
            Some(_) => 404,
            None => 500,
        };
        warp::reply::with_status(warp::reply::json(&{"error": err.to_string() }), warp::http::StatusCode::from(code))
    });

    // 启动服务
    let routes = get_route.or(post_route).recover(error_route);
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

// 自定义错误类型
#[derive(Debug)]
struct MyError(&'static str);

impl warp::reject::Reject for MyError {}
