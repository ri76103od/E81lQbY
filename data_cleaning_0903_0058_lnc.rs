// data_cleaning.rs
//
// 这个Rust程序实现了一个基于WARP框架的数据清洗和预处理工具。

use warp::Filter;
use serde::Deserialize;
use serde_json::json;
use std::collections::HashMap;

// 定义请求体结构
#[derive(Deserialize, Clone, Debug)]
struct DataPreprocessingRequest {
    records: Vec<HashMap<String, String>>,
}

#[tokio::main]
async fn main() {
    // 定义处理数据清洗和预处理的逻辑
    let clean_and_preprocess = warp::post()
        .and(warp::path("clean"))
        .and(warp::body::json())  // JSON请求体
        .map(|data: DataPreprocessingRequest| async move {
            let cleaned_data = clean_data(&data.records).await?;
            Ok::<_, warp::Rejection>(json!({
                "cleaned_data": cleaned_data,
            }))
        });

    // 启动WARP服务器
    warp::serve(clean_and_preprocess)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

// 数据清洗函数
async fn clean_data(records: &[HashMap<String, String>]) -> Result<Vec<HashMap<String, String>>, warp::Rejection> {
    let mut cleaned_records = Vec::new();
    for record in records {
        let mut cleaned_record = HashMap::new();
        for (key, value) in record {
            // 假设我们需要去除空白字符
            let cleaned_value = value.trim().to_string();
            cleaned_record.insert(key.clone(), cleaned_value);
        }
        cleaned_records.push(cleaned_record);
    }
    Ok(cleaned_records)
}
