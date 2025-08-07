use warp::Filter;

// 定义一个简单的消息通知系统，用于展示如何使用WARP框架进行HTTP服务的开发。

// 定义消息结构体，用于存储消息内容和接收者。
struct Notification {
    message: String,
    recipient: String,
}

// 实现消息结构体的方法，用于生成HTTP响应。
impl Notification {
    fn respond(&self) -> String {
        format!("Notification to {}: {}", self.recipient, self.message)
    }
}

// 创建一个路由，用于处理POST请求，接收消息通知。
fn notification_route() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("notify")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(|notification: Notification| async move {
            // 处理接收到的消息通知，生成HTTP响应。
            Ok(warp::reply::json(&notification))
        })
}

// 定义HTTP服务入口点。
#[tokio::main]
async fn main() {
    // 配置并启动HTTP服务。
    let routes = notification_route();
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
    println!("Message notification system started on http://127.0.0.1:3030/notify");
}

// 以下是使用WARP框架创建HTTP服务的详细说明：

// 1. 使用`warp::Filter`定义路由和处理逻辑。
// 2. 定义`Notification`结构体，用于存储消息内容和接收者。
// 3. 实现`Notification`的`respond`方法，生成HTTP响应。
// 4. 创建`notification_route`函数，定义处理POST请求的路由。
// 5. 在`main`函数中配置并启动HTTP服务。
// 6. 使用`warp::serve`和`warp::run`启动HTTP服务。
