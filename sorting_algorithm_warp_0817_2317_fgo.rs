use warp::Filter;

// 导入Rust标准库中的向量和错误处理模块
use std::vec::Vec;
use std::error::Error;

// 排序算法的实现
fn sort_algorithm(numbers: Vec<i32>) -> Result<Vec<i32>, Box<dyn Error>> {
    // 尝试对数字进行排序，如果发生错误则返回错误
    numbers.sort();
    Ok(numbers)
}

// 处理排序请求的函数
async fn sort_numbers(numbers: Vec<i32>) -> Result<Vec<i32>, warp::Rejection> {
    match sort_algorithm(numbers) {
        Ok(sorted) => Ok(sorted),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

// 设置WARP路由和服务器
#[tokio::main]
async fn main() {
    // 创建一个路由，它接受JSON数组并返回排序后的数组
    let sort_route = warp::path!("sort")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(sort_numbers);

    // 启动服务器
    warp::serve(sort_route)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

// 错误类型定义，用于处理排序算法中可能出现的错误
#[derive(Debug)]
struct SortError;

// 实现Error Trait
impl Error for SortError {}

// 实现Display Trait，用于错误信息的展示
impl std::fmt::Display for SortError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "An error occurred during sorting")
    }
}

// 实现From Trait，将我们的自定义错误类型转换为Box<dyn Error>
impl From<SortError> for Box<dyn Error> {
    fn from(_: SortError) -> Box<dyn Error> {
        Box::new(SortError)
    }
}