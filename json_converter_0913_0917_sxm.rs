use warp::http::StatusCode;
use warp::{Filter, Reply, Rejection, reject, Reply as WarpReply};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::str::FromStr;
use warp::http::Response;

// 定义输入和输出的JSON结构体
#[derive(Serialize, Deserialize, Debug)]
struct JsonInput {
    #[serde(rename = "json")]
    json: String,
}

// 定义路由和处理函数
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let json_converter = warp::path("convert")
        .and(warp::post())
        .and(warp::body::content_length_limit(1024 * 1024)) // 限制请求体大小
        .and(warp::body::json())
        .and_then(handle_json_conversion);

    warp::serve(json_converter)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

// 处理JSON转换的函数
async fn handle_json_conversion(json_input: JsonInput) -> Result<impl WarpReply, Rejection> {
    // 解析输入的JSON字符串
    let input_value: Value = match serde_json::from_str(&json_input.json) {
        Ok(value) => value,
        Err(_) => return Err(reject::custom(ConversionError::InvalidJson)),
    };

    // 转换JSON数据（示例：将数字转换为字符串）
    let converted_value = match convert_json(&input_value) {
        Ok(converted) => converted,
        Err(e) => return Err(reject::custom(e)),
    };

    // 返回转换后的JSON响应
    Ok(warp::reply::json(&converted_value))
}

// JSON转换函数（可以根据需要进行扩展）
fn convert_json(value: &Value) -> Result<Value, ConversionError> {
    match value {
        Value::Number(n) => Ok(Value::String(n.to_string())),
        _ => Err(ConversionError::UnsupportedType),
    }
}

// 定义错误类型
#[derive(Debug)]
enum ConversionError {
    InvalidJson,
    UnsupportedType,
}

// 实现自定义错误类型的Reject转换
impl warp::reject::Reject for ConversionError {}

// 实现自定义错误类型的Display和Error
impl std::fmt::Display for ConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            ConversionError::InvalidJson => write!(f, "Invalid JSON input"),
            ConversionError::UnsupportedType => write!(f, "Unsupported JSON type"),
        }
    }
}

impl std::error::Error for ConversionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
