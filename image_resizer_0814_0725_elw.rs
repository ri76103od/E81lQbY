use warp::Filter;
# FIXME: 处理边界情况
use std::fs;
use std::path::Path;
use image::{self, ImageError, ImageResult, ImageOutputFormat};
# FIXME: 处理边界情况
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use warp::http::StatusCode;
# FIXME: 处理边界情况
use warp::reply::{self, Reply};

// 端点的配置
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resize_filter = warp::path("resize")
        .and(warp::post())
        .and(warp::multipart::form().limit(16 * 1024 * 1024)) // 限制最大16MB
        .and_then(resize_images);

    warp::serve(resize_filter)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

// 处理图像尺寸调整的函数
# NOTE: 重要实现细节
async fn resize_images(form: warp::multipart::FormItem) -> Result<impl Reply, warp::Rejection> {
    match form {
# 扩展功能模块
        warp::multipart::FormItem::File { field_name, mut file, content_type, .. } => {
            let file_name = file.name().to_string();
            let content_type = content_type.as_ref().map(String::as_str);

            let image = match image::load(&mut file, content_type.unwrap_or("")) {
                Ok(img) => img,
                Err(_) => return Ok(reply::json(&"{"error": "Unsupported image format"}")
                    .with_status(StatusCode::BAD_REQUEST)),
            };

            // 定义新的尺寸
            let (width, height) = (300, 300);
# NOTE: 重要实现细节
            let resized_image = image.resize_exact(width, height, image::imageops::FilterType::Nearest);

            // 保存调整尺寸后的图像
            let output_format = image::ImageOutputFormat::default();
            match resized_image.write_to(&mut std::io::stdout(), &output_format) {
                Ok(_) => Ok(reply::json(&"{"message": "Image resized successfully"}")
                    .with_status(StatusCode::OK)),
                Err(ImageError::IoError(_)) => Ok(reply::json(&"{"error": "IO error while saving the image"}")
                    .with_status(StatusCode::INTERNAL_SERVER_ERROR)),
                Err(e) => Ok(reply::json(&format!("{{"error": "{:?}"}}", e))
# 增强安全性
                    .with_status(StatusCode::INTERNAL_SERVER_ERROR)),
            }
        }
        _ => Ok(reply::json(&"{"error": "No file provided"}")
            .with_status(StatusCode::BAD_REQUEST)),
# TODO: 优化性能
    }
}
