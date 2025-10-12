use warp::Filter;

// 定义一个结构体来存储URL验证结果
struct UrlValidationResult {
    is_valid: bool,
    message: String,
}

// 定义一个函数来检查URL是否有效
// 这个函数接收一个字符串切片作为URL，并返回一个UrlValidationResult
fn validate_url(url: &str) -> UrlValidationResult {
    // 使用url库来检查URL是否有效
    // 这里假设url库提供了一个名为is_valid的函数来检查URL的有效性
    // 请根据实际情况替换为具体的库和函数
    if url::Url::parse(url).is_ok() {
        UrlValidationResult {
            is_valid: true,
            message: "URL is valid.".to_string(),
        }
    } else {
        UrlValidationResult {
            is_valid: false,
            message: "URL is invalid.".to_string(),
        }
    }
}

// 定义一个路由来处理URL验证请求
// 这个路由接收一个查询参数url，并返回URL验证结果
fn url_validator_route() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("validate_url")
        .and(warp::query::<String>())
        .map(|url| async move {
            let result = validate_url(&url);
            match result {
                Ok(valid_result) => {
                    warp::reply::json(&valid_result)
                },
                Err(e) => {
                    warp::reply::json(&UrlValidationResult {
                        is_valid: false,
                        message: e.to_string(),
                    })
                },
            }
        }).untuple1()
}

#[tokio::main]
async fn main() {
    // 启动WARP服务器，监听所有地址的3030端口
    let validator_route = url_validator_route();
    warp::serve(validator_route).run(([0, 0, 0, 0], 3030)).await;
}

impl warp::Reply for UrlValidationResult {
    fn into_response(self) -> warp::reply::Response {
        warp::reply::json(&self)
    }
}

// 为UrlValidationResult实现Debug和Serialize trait，以便可以打印和序列化为JSON
#[derive(Debug, serde::Serialize)]
struct UrlValidationResult {
    is_valid: bool,
    message: String,
}