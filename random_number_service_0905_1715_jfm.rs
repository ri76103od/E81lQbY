use warp::Filter;
use rand::Rng;
use std::sync::Mutex;
use rand::rngs::ThreadRng;

// 定义全局随机数生成器，使用Mutex以保证线程安全。
lazy_static::lazy_static! {
    static ref RNG: Mutex<ThreadRng> = Mutex::new(rand::thread_rng());
}

#[tokio::main]
async fn main() {
    // 定义随机数生成器的端点。
    let random_number = warp::path("random")
        .and(warp::get())
        .and(with_rng())
        .and_then(|| warp::any().map(move || generate_random_number()))
        .map(warp::reply::json);

    // 启动服务。
    warp::serve(random_number)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

// 定义一个过滤器，用于获取全局随机数生成器。
fn with_rng() -> impl Filter<Extract = (Mutex<ThreadRng>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || RNG.clone())
}

// 生成随机数的函数。
fn generate_random_number() -> Result<u32, warp::Rejection> {
    let mut rng = RNG.lock().map_err(|_| warp::reject::custom(generate_random_number::Error))?;
    let random_number = rng.gen_range(1..100);
    Ok(random_number)
}

// 自定义错误类型。
#[derive(Debug)]
pub enum generate_random_number::Error {
    LockError,
}

impl warp::reject::Reject for generate_random_number::Error {}

impl std::fmt::Display for generate_random_number::Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            generate_random_number::Error::LockError => write!(f, "Failed to lock the random number generator."),
        },
    }
}

impl std::error::Error for generate_random_number::Error {}
