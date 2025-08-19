// xss_protection.rs
// 该RUST程序使用WARP框架，提供了XSS攻击防护功能。

use warp::Filter;
use regex::Regex;
use html_escape::encode_html;
use std::str::FromStr;
use warp::reject::Reject;
use warp::Rejection;
use warp::reply::Reply;
use warp::http::Response;
use warp::http::StatusCode;

// 自定义一个错误类型来处理XSS攻击防护失败
#[derive(Debug, Clone)]
struct XssProtectionError;

// 实现Reject trait，以便自定义错误可以被Warp处理
impl Reject for XssProtectionError {}

// 定义一个函数来检查和清理XSS攻击
fn xss_protection(input: String) -> Result<String, XssProtectionError> {
    let mut cleaned_input = input;
    // 使用正则表达式匹配和替换掉潜在的XSS攻击代码
    let script_regex = Regex::from_str("<script[^>]*?>.*?</script>").unwrap();
    cleaned_input = script_regex.replace_all(&cleaned_input, "").to_string();
    // 清理其他潜在的XSS攻击代码...
    Ok(cleaned_input)
}

// 创建一个Warp过滤器，用于捕获和处理输入，以防护XSS攻击
fn with_xss_protection() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::any()
        .map(move || warp::hyper::body::to_bytes)
        .and_then(|bytes| async move {
            // 将请求体转换为字符串
            let input = String::from_utf8(bytes.to_vec()).map_err(|e| warp::reject::custom(XssProtectionError))?;
            // 清理输入以防护XSS攻击
            let cleaned_input = xss_protection(input).map_err(|e| warp::reject::custom(XssProtectionError))?;
            // 返回清理后的内容
            Ok(warp::reply::json(&cleaned_input))
        }).recover(handle_rejection)
}

// 错误处理函数，用于处理XSS攻击防护失败的情况
async fn handle_rejection(err: Rejection) -> Result<impl Reply, Rejection> {
    if err.is_not_found() {
        // 处理404错误...
        Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(warp::reply::with_status("Not Found", StatusCode::NOT_FOUND)).unwrap())
    } else if let Some(_) = err.find::<XssProtectionError>() {
        // 处理XSS攻击防护失败的情况
        Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(warp::reply::with_status("XSS Protection Failed", StatusCode::BAD_REQUEST)).unwrap())
    } else {
        // 处理其他错误...
        Ok(warp::reply::with_status("An error occurred", StatusCode::INTERNAL_SERVER_ERROR))
    }
}

// 主函数，用于启动WARP服务器
#[tokio::main]
async fn main() {
    let xss_protection_route = warp::path("xss")
        .and(warp::post())
        .and(with_xss_protection());
    // 启动服务器
    warp::serve(xss_protection_route).run(([127, 0, 0, 1], 3030)).await;
}
