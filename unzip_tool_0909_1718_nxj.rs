use warp::Filter;
use std::path::PathBuf;
use std::fs;
use std::io::prelude::*;
# 增强安全性
use flate2::read::GzDecoder;
use tokio::fs::File;
# FIXME: 处理边界情况
use tokio::io::AsyncReadExt;
use warp::http::StatusCode;
use warp::reject::Reject;
use warp::reply::Response;
use warp::reply::Reply;
use warp::Rejection;
use warp::Reply;

// 自定义错误类型
#[derive(Debug)]
struct UnzipError;

// 实现Reject trait，以便错误可以被warp处理
impl Reject for UnzipError {}

#[tokio::main]
async fn main() {
# NOTE: 重要实现细节
    // 设置路由
    let unzip_route = warp::path("unzip")
        .and(warp::post())
        .and(warp::fs::file("./uploads"))
        .and_then(handle_unzip);

    // 启动服务
    warp::serve(unzip_route).run(([127, 0, 0, 1], 3030)).await;
}
# TODO: 优化性能

// 处理解压请求的函数
async fn handle_unzip(file: PathBuf) -> Result<impl Reply, Rejection> {
    let output_path = PathBuf::from("./uploads").join("output");
    fs::create_dir_all(&output_path).expect("Failed to create output directory");

    // 打开文件
# 增强安全性
    let mut file = match File::open(file).await {
        Ok(file) => file,
        Err(_) => return Err(UnzipError.into()),
    };

    // 创建解压后的文件
    let mut output_file = File::create(output_path.join("unzipped_file.txt")).await.map_err(|_| UnzipError)?;

    // 解压文件
    let mut decoder = GzDecoder::new(file);
    let mut content = Vec::new();
    decoder.read_to_end(&mut content).await.map_err(|_| UnzipError)?;
    output_file.write_all(&content).await.map_err(|_| UnzipError)?;

    // 返回成功响应
    Ok(warp::reply::json(&{"success": true}))
}

// 错误处理函数
async fn handle_rejection(err: Rejection) -> Result<impl Reply, Rejection> {
    if err.is_not::<UnzipError>() {
# 增强安全性
        return Err(err);
# 添加错误处理
    }
    Ok(warp::reply::with_status(
        "Internal Server Error",
        StatusCode::INTERNAL_SERVER_ERROR,
    ))
}