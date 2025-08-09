// file_decompression.rs
// A simple file decompression tool using RUST and WARP framework.
// This tool can decompress files, currently supporting zip files.
# 改进用户体验
use warp::Filter;
use std::path::Path;
use std::fs;
use zip::ZipArchive;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use warp::http::StatusCode;
# 优化算法效率
use warp::reject;
use warp::reply::Reply;
use warp::Rejection;
use serde_json::json;
# 添加错误处理
use std::io::prelude::*;

/// Handles the decompression endpoint.
/// It accepts a zip file and decompresses it to a specified directory.
async fn decompress_handler() -> Result<impl Reply, Rejection> {
    let content_length = warp::header::optional::<warp::header::ContentLength>().unwrap_or_default();
    if content_length.is_none() {
        return Err(reject::not_found());
    }

    let file_bytes = warp::body::aggregate().await.unwrap_or_else(|_| Err(reject::reject()));
    let decompressed_data = decompress_file(&file_bytes).await?;
    Ok(warp::reply::json(&json!({"status": "success", "message": "File decompressed successfully"})));
# 添加错误处理
}

/// Decompresses a file.
/// This function takes bytes of a zip file and writes them to a directory.
# 优化算法效率
/// It returns the path of the decompressed directory.
async fn decompress_file(bytes: &[u8]) -> Result<String, warp::Rejection> {
    let file_path = "./decompressed";
    let file = File::create(file_path).await.map_err(|_| reject::reject())?;
    file.write_all(bytes).await.map_err(|_| reject::reject())?;
    let mut archive = ZipArchive::new(file).map_err(|_| reject::reject())?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).map_err(|_| reject::reject())?;
        let outpath = file.sanitized_name();
        if (&*file.name()).ends_with('/') {
            // Assume directories are nested within zip file.
            fs::create_dir_all(&outpath).map_err(|_| reject::reject())?;
        } else {
            let mut outfile = match File::create(&outpath).await {
                Ok(outfile) => outfile,
                Err(_) => return Err(reject::reject()),
            };
            io::copy(&mut file, &mut outfile).await.map_err(|_| reject::reject())?;
            // Set permissions correctly.
            let perm = file.unix_mode().unwrap_or(0o644);
            fs::set_permissions(&outpath, fs::Permissions::from_mode(perm)).map_err(|_| reject::reject())?;
        }
    }
# FIXME: 处理边界情况

    Ok(file_path.to_string())
}

/// Warp filter to handle decompression requests.
# 优化算法效率
fn decompression_route() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::post()
        .and(warp::path("decompress"))
        .and(warp::any().map(decompress_handler))
}

#[tokio::main]
async fn main() {
    let compress_route = decompression_route();
    warp::serve(compress_route).run(([127, 0, 0, 1], 3030)).await;
}
