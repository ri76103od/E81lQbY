// xss_protection.rs
// 这个RUST程序使用WARP框架创建一个简单的HTTP服务器来展示如何防护XSS攻击。
// 它通过清理输入来防止恶意脚本的执行。

use warp::http::Response;
use warp::Filter;
use warp::reject;

#[tokio::main]
async fn main() {
    // 创建一个简单的路由来接收GET请求，并返回防护XSS的HTML页面。
    let xss_protected_html = warp::path("xss")
        .and_then(|| async {
            // 这里模拟防护XSS攻击的过程，实际应用中应使用更复杂的HTML清理库。
            let safe_html = "<p>Hello, <script>alert('XSS');</script></p>"
                .replace("<script>