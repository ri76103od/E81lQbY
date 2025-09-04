use std::io::Write;
use warp::Filter;
use sha2::{Sha256, Digest};
use warp::http::Response;
use warp::reply::json;

// 定义一个结构体来封装我们的哈希计算工具
struct HashCalculator;

impl HashCalculator {
    // 提供一个方法来计算SHA-256哈希值
    pub fn calculate_sha256(input: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(input);
        // 将哈希值转换为十六进制字符串
        format!("%x", hasher.finalize())
    }
}

// 定义一个路由来处理哈希计算请求
fn hash_route() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post()
        .and(warp::path("hash"))
        .and(warp::path::end())
        .and(warp::body::content_length_limit(1024 * 1024 * 5)) // 限制请求体大小为5MB
        .and(warp::body::bytes())
        .and_then(handle_hash_request)
}

// 处理哈希计算请求的函数
async fn handle_hash_request(body: warp::bytes::Bytes) -> Result<impl warp::Reply, warp::Rejection> {
    let input = match String::from_utf8(body.to_vec()) {
        Ok(s) => s,
        Err(_) => return Ok(warp::reply::with_status("Bad Request", warp::http::StatusCode::BAD_REQUEST)),
    };

    let hash = HashCalculator::calculate_sha256(&input);
    let reply = warp::reply::json(&json!({"hash": hash}));
    Ok(reply)
}

// 启动服务的主函数
#[tokio::main]
async fn main() {
    let hash_route = hash_route();
    warp::serve(hash_route)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
