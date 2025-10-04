use warp::http::StatusCode;
use warp::Filter;

// 定义一个简单的用户数据结构
#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct User {
    pub name: String,
    pub age: u8,
}

// 函数来处理GET请求
async fn get_user(name: String) -> Result<impl warp::Reply, warp::Rejection> {
    if name == "Alice" {
        Ok(warp::reply::json(&User {
            name: name.clone(),
            age: 30,
        }))
    } else {
        Err(warp::reject::not_found())
    }
}

// 函数来处理POST请求
async fn create_user(user: User) -> Result<impl warp::Reply, warp::Rejection> {
    // 这里可以添加一些验证逻辑
    Ok(warp::reply::json(&user))
}

// 定义路由
fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let user = warp::path!("api" / "user" / String);

    // 定义GET路由
    let get_user_route = user.clone()
        .and(warp::get())
        .and_then(get_user);

    // 定义POST路由
    let post_user_route = user.clone()
        .and(warp::post())
        .and(warp::body::json())
        .and_then(create_user);

    // 将路由组合在一起
    get_user_route.or(post_user_route)
}

// main函数，程序入口点
#[tokio::main]
async fn main() {
    // 使用warp::Filter配置和启动服务
    warp::serve(routes()).run(([127, 0, 0, 1], 3030)).await;
}
