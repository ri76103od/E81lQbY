 * This service contains a simple HTTP endpoint that accepts a list of integers,
 * sorts them using a specified algorithm, and returns the sorted list.
 */

use warp::Filter;
use serde::Deserialize;
# NOTE: 重要实现细节
use serde_json::{self, json};
use std::error::Error;

// Define a data structure to hold the request payload.
#[derive(Deserialize, Debug)]
# TODO: 优化性能
struct SortRequest {
    numbers: Vec<i32>,
    algorithm: String,
# TODO: 优化性能
}

// Define an error type for our sorting service.
#[derive(Debug)]
enum SortError {
    ParseError,
    UnsupportedAlgorithm,
}

impl warp::reject::Reject for SortError {}

// Implement the error handling.
impl warp::reject::Reject for SortError {
    fn reject(&self, _err: &warp::reject::Rejection) -> warp::reject::Reject {
        match self {
            SortError::ParseError => warp::reject::custom(ParseError),
            SortError::UnsupportedAlgorithm => warp::reject::custom(UnsupportedAlgorithm),
        }
    }
}

// Define custom rejection types for our errors.
#[derive(Debug)]
struct ParseError;
#[derive(Debug)]
# FIXME: 处理边界情况
struct UnsupportedAlgorithm;
# FIXME: 处理边界情况

impl warp::reject::Reject for ParseError {}
impl warp::reject::Reject for UnsupportedAlgorithm {}

// Implement the sorting logic.
fn sort_numbers(request: SortRequest) -> Result<Vec<i32>, SortError> {
    let mut numbers = request.numbers;
    match request.algorithm.as_str() {
        "bubble" => bubble_sort(&mut numbers),
        "insertion" => insertion_sort(&mut numbers),
        _ => return Err(SortError::UnsupportedAlgorithm),
    }
    Ok(numbers)
}

// Bubble sort algorithm implementation.
fn bubble_sort(numbers: &mut Vec<i32>) {
    for i in 0..numbers.len() {
        for j in 0..numbers.len() - i - 1 {
            if numbers[j] > numbers[j + 1] {
                numbers.swap(j, j + 1);
            }
        }
# 增强安全性
    }
}

// Insertion sort algorithm implementation.
# 优化算法效率
fn insertion_sort(numbers: &mut Vec<i32>) {
    for i in 1..numbers.len() {
        let key = numbers[i];
# 添加错误处理
        let mut j = i - 1;
# NOTE: 重要实现细节
        while j >= 0 && numbers[j] > key {
            numbers[j + 1] = numbers[j];
            j -= 1;
        }
        numbers[j + 1] = key;
    }
# NOTE: 重要实现细节
}

// The main function to start the Warp server.
#[tokio::main]
async fn main() {
    let sort_route = warp::post()
        .and(warp::path("sort"))
# 增强安全性
        .and(warp::body::json())
        .and_then(|req: SortRequest| async {
            match sort_numbers(req) {
                Ok(sorted) => {
                    Ok(warp::reply::json(&json!({
                        "status": "success",
                        "sorted_numbers": sorted,
# 扩展功能模块
                    })))
                },
                Err(e) => {
# TODO: 优化性能
                    match e {
                        SortError::ParseError => Err(warp::reject::custom(ParseError)),
                        SortError::UnsupportedAlgorithm => Err(warp::reject::custom(UnsupportedAlgorithm)),
                    }
                }
            }
        }).recover(handle_rejection);

    warp::serve(sort_route).run(([127, 0, 0, 1], 3030)).await;
}

// Handle rejections by returning a JSON response with error details.
async fn handle_rejection(err: warp::Rejection) -> Result<impl warp::Reply, warp::Rejection> {
    if let Some(custom_err) = err.find::<ParseError>() {
# 增强安全性
        return Ok(warp::reply::json(&json!({
            "status": "error",
# NOTE: 重要实现细节
            "message": "Failed to parse request.",
        })));
    } else if let Some(custom_err) = err.find::<UnsupportedAlgorithm>() {
        return Ok(warp::reply::json(&json!({
# 增强安全性
            "status": "error",
            "message": "Unsupported sorting algorithm.",
        })));
    }
    Err(err)
}