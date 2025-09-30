// greedy_algorithm.rs
// 该程序使用RUST和WARP框架实现了一个贪心算法框架。

use warp::Filter;
use std::error::Error;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

// 定义一个贪心算法的配置结构，其中包含贪心算法需要的参数。
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GreedyConfig {
    pub items: Vec<i32>,
    pub capacity: i32,
}

// 定义贪心算法的结果结构。
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GreedyResult {
    pub items: Vec<i32>,
    pub total_value: i32,
}

// 实现贪心算法的函数。
pub fn greedy_algorithm(config: GreedyConfig) -> Result<GreedyResult, Box<dyn Error>> {
    // 按照物品的重量从大到小排序
    let mut sorted_items = config.items.clone();
    sorted_items.sort_by(|a, b| b.cmp(a));

    let mut total_value = 0;
    let mut selected_items = Vec::new();

    // 遍历物品并选择能装入背包的物品
    for item in sorted_items {
        if (total_value + item as i32) <= config.capacity {
            total_value += item as i32;
            selected_items.push(item);
        }
    }

    Ok(GreedyResult {
        items: selected_items,
        total_value,
    })
}

// 创建一个Warp过滤器来处理HTTP请求。
#[tokio::main]
async fn main() {
    let config_route = warp::path("greedy")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(greedy_config_handler);

    warp::serve(config_route)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

// 处理POST请求并执行贪心算法。
async fn greedy_config_handler(item: GreedyConfig) -> Result<impl warp::Reply, warp::Rejection> {
    match greedy_algorithm(item) {
        Ok(result) => Ok(warp::reply::json(&result)),
        Err(e) => Ok(warp::reply::json(&json!({
            "error": e.to_string(),
        }))),
    }
}
