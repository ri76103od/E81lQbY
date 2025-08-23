// text_file_analyzer.rs
// 这是一个使用RUST和WARP框架实现的文本文件内容分析器。

use std::fs;
use warp::Filter;
use warp::http::StatusCode;
use warp::reject::Reject;
use warp::reply::{Json, Reply};
use serde::Serialize;
use serde_json::json;
use std::io::Error;

// 定义错误类型
#[derive(Debug, Serialize)]
struct AnalysisError {
    message: String,
}

impl Reject for AnalysisError {}

// 定义分析结果结构
#[derive(Serialize)]
struct AnalysisResult {
    text_length: usize,
    word_count: usize,
    unique_words: usize,
}

// 分析文本文件内容的函数
async fn analyze_text_file(file_path: String) -> Result<impl Reply, AnalysisError> {
    let contents = fs::read_to_string(file_path).map_err(|e| AnalysisError {
        message: e.to_string(),
    })?.0;
    let text_length = contents.chars().count();
    let words: Vec<&str> = contents.split_whitespace().collect();
    let word_count = words.len();
    let unique_words: Vec<&str> = words.into_iter().fold(std::collections::HashSet::new(), |mut acc, x| {
        acc.insert(x); acc
    }).into_iter().collect();
    let result = AnalysisResult {
        text_length,
        word_count,
        unique_words: unique_words.len(),
    };
    Ok(Json::<AnalysisResult>::json(&result))
}

// 创建WARP路由
fn create_routes() -> impl Filter<Extract = impl Reply, Error = warp::Rejection> + Clone {
    warp::post()
        .and(warp::path("analyze"))
        .and(warp::path::param())
        .and_then(|file_path: String| warp::any().then(||
            analyze_text_file(file_path)
        ))
}

fn main() {
    let routes = create_routes();
    println!("Server running on http://127.0.0.1:3030");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
