 * It provides a simple HTTP endpoint that accepts a list of integers and returns
 * the sorted list.
 *
 * The program showcases error handling, documentation, and best practices in Rust.
# TODO: 优化性能
 */

use warp::Filter;
use serde::Deserialize;
use serde_json::json;
use std::cmp::Ordering;
use std::error::Error;

// Define a struct to deserialize the incoming JSON data.
# 改进用户体验
#[derive(Deserialize)]
struct IntegerList {
    numbers: Vec<i32>,
}

// Define a function to sort the list of integers.
fn sort_list(numbers: Vec<i32>) -> Result<Vec<i32>, Box<dyn Error>> {
    // Use Rust's built-in sort method with a custom comparator.
# TODO: 优化性能
    numbers.sort_by(|a, b| a.cmp(b));
# 优化算法效率
    Ok(numbers)
}

// Define a function to handle the incoming HTTP request.
async fn sort_numbers(numbers: IntegerList) -> Result<impl warp::Reply, warp::Rejection> {
    // Attempt to sort the numbers and handle any potential errors.
    let sorted_numbers = match sort_list(numbers.numbers) {
        Ok(sorted) => sorted,
# 添加错误处理
        Err(e) => return Err(warp::reject::custom(e)),
    };

    // Return the sorted numbers as JSON.
    Ok(warp::reply::json(&json!({
        "status": "success",
        "sorted_numbers": sorted_numbers,
    })));
}

// Define the main function that sets up and starts the Warp server.
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Define the route for the sorting endpoint.
    let sort_route = warp::path("sort")
        .and(warp::post())
# TODO: 优化性能
        .and(warp::body::json())
        .and_then(sort_numbers);

    // Start the server and print out the address.
    warp::serve(sort_route)
# 扩展功能模块
        .run(([127, 0, 0, 1], 3030))
# 增强安全性
        .await;

    Ok(())
}
