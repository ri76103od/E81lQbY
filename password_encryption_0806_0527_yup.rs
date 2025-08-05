use warp::http::StatusCode;
use warp::Filter;
use bcrypt::hash;
# NOTE: 重要实现细节
use bcrypt::verify;
# 添加错误处理
use warp::http::Response;
use serde::Deserialize;
use serde_json;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::sync::RwLock;
# NOTE: 重要实现细节

// Define a struct to hold the bcrypt cost factor
# 扩展功能模块
struct BcryptCost {
# 扩展功能模块
    cost: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Create a cost factor for bcrypt
    let bcrypt_cost = Arc::new(RwLock::new(BcryptCost { cost: 12 }));
    
    // Define the routes
    let routes = warp::path!("encrypt")
        .and(warp::post())
        .and(with_bcrypt_cost(bcrypt_cost.clone()))
        .and(warp::body::json())
        .and_then(handle_encrypt)
        .or(warp::path!("decrypt")
        .and(warp::post())
# FIXME: 处理边界情况
        .and(with_bcrypt_cost(bcrypt_cost.clone()))
        .and(warp::body::json())
        .and_then(handle_decrypt));
    
    // Start the server
# 扩展功能模块
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

// Handler for encrypting passwords
async fn handle_encrypt(password: String, bcrypt_cost: Arc<RwLock<BcryptCost>>) -> Result<impl warp::Reply, warp::Rejection> {
    let cost = bcrypt_cost.read().await.cost;
    match hash(password.as_str(), cost) {
        Ok(hashed_password) => Ok(Response::builder()
            .status(StatusCode::OK)
            .body(serde_json::json!({
                "original": password,
                "hashed": hashed_password
            }).to_string())
        ),
        Err(e) => Ok(Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(format!("Encryption error: {}", e))
# 优化算法效率
        ),
    }
}

// Handler for decrypting passwords (verification)
async fn handle_decrypt(password: String, bcrypt_cost: Arc<RwLock<BcryptCost>>) -> Result<impl warp::Reply, warp::Rejection> {
    let hashed_password = include_str!("./hashed_password.txt"); // Placeholder, replace with actual hashed password retrieval
    let cost = bcrypt_cost.read().await.cost;
# 优化算法效率
    match verify(password.as_str(), hashed_password) {
# 添加错误处理
        Ok(true) => Ok(Response::builder()
            .status(StatusCode::OK)
            .body(serde_json::json!({
                "message": "Password is correct"
            }).to_string())
        ),
        Ok(false) => Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(serde_json::json!({
                "message": "Password is incorrect"
            }).to_string())
        ),
        Err(e) => Ok(Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(format!("Verification error: {}", e))
        ),
# 改进用户体验
    }
}

// Clone the bcrypt cost factor to share between filters
fn with_bcrypt_cost(bcrypt_cost: Arc<RwLock<BcryptCost>>) -> impl Filter<Extract = (Arc<RwLock<BcryptCost>>,), Error = std::convert::Infallible> + Clone {
# 扩展功能模块
    warp::any().map(move || bcrypt_cost.clone())
}

// Define a struct to hold the password for serialization
#[derive(Deserialize, Debug)]
struct Password {
    password: String,
}
