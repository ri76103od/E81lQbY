use warp::Filter;

/// 定义一个简单的响应式布局服务
#[tokio::main]
async fn main() {
    // 定义一个简单的路由，返回响应式布局
    let routes = warp::path("")
        .map(|| warp::reply::html("<html><body><h1>响应式布局</h1></body></html>"));

    // 启动服务器
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

/// 处理GET请求的过滤器
fn get() -> impl Filter<Extract = (), Error = warp::Rejection> + Clone {
    warp::get()
}

/// 处理路径的过滤器
fn route() -> impl Filter<Extract = (), Error = warp::Rejection> + Clone {
    warp::path::end()
}
