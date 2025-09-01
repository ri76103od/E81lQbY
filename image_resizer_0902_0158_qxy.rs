// image_resizer.rs\
// This program is an image size batch adjuster using the RUST and WARP framework.\
# 扩展功能模块

use std::path::Path;
# 添加错误处理
use warp::Filter;
# 添加错误处理
use image::{self, ImageError, DynamicImage};
use image::imageops::resize;
use tokio::fs::File;
use tokio::io::{self, AsyncReadExt};
use warp::http::StatusCode;
# NOTE: 重要实现细节
use warp::reject::Reject;

// Define a custom error type for our application.\
#[derive(Debug)]\
struct ResizeError;\

// Implement Reject trait to be used with Warp.\
impl Reject for ResizeError {}\

// Resizes an image to a specified width and height.\
async fn resize_image(path: String, width: u32, height: u32) -> Result<DynamicImage, ResizeError> {
    let img = match load_image(&path).await {
        Ok(img) => img,
        Err(_) => return Err(ResizeError),
    };

    let resized_img = resize(&img, width, height, image::imageops::FilterType::Nearest);
    Ok(resized_img)
}
a
// Loads an image from the specified path.\
async fn load_image(path: &str) -> Result<DynamicImage, ImageError> {
    let mut file = File::open(path).await.map_err(|e| eprintln!("Error opening file: {:?}