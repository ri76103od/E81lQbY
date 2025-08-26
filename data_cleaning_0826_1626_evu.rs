use warp::Filter;
use serde::Deserialize;
use regex::Regex;
use std::collections::HashMap;

// 定义输入数据结构
#[derive(Deserialize, Debug)]
struct InputData {
    data: String,
}

// 定义预处理后的数据结构
#[derive(Debug)]
struct CleanData {
    cleaned_data: String,
}

// 正则表达式用于清洗数据
lazy_static::lazy_static! {
    static ref RE: Regex = Regex::new(r"[^a-zA-Z0-9 ]").unwrap();
}

// 数据清洗函数
fn clean_data(input: &str) -> String {
    RE.replace_all(input, "").to_string()
}

// 构造数据清洗和预处理的端点
fn data_cleaning_route() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("clean")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(|input: InputData| async move {
            let cleaned = clean_data(&input.data);
            Ok(warp::reply::json(&CleanData { cleaned_data: cleaned }))
        })
}

#[tokio::main]
async fn main() {
    let data_cleaning_route = data_cleaning_route();
    warp::serve(data_cleaning_route)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
