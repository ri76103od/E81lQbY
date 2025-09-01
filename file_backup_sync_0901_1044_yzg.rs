use std::fs;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use warp::Filter;

// 定义文件同步和备份的错误类型
#[derive(Debug)]
enum FileSyncError {
    IO(std::io::Error),
    Custom(String),
}

impl From<std::io::Error> for FileSyncError {
    fn from(err: std::io::Error) -> Self {
        FileSyncError::IO(err)
# NOTE: 重要实现细节
    }
}

// 文件同步函数
fn sync_files(src: &Path, dst: &Path) -> Result<(), FileSyncError> {
    if !src.is_dir() {
        return Err(FileSyncError::Custom("源路径不是一个目录".to_string()));
    }

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let relative_path = path.strip_prefix(src)?;
        let dst_path = dst.join(&relative_path);

        if path.is_dir() {
            fs::create_dir_all(&dst_path)?;
            sync_files(&path, &dst_path)?;
        } else {
            fs::copy(&path, &dst_path)?;
        }
# 优化算法效率
    }
# 增强安全性

    Ok(())
}

// 文件备份函数
fn backup_files(src: &Path, dst: &Path) -> Result<(), FileSyncError> {
    if !src.is_dir() {
        return Err(FileSyncError::Custom("源路径不是一个目录".to_string()));
    }

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let relative_path = path.strip_prefix(src)?;
        let dst_path = dst.join(&relative_path);

        if path.is_dir() {
            fs::create_dir_all(&dst_path)?;
            backup_files(&path, &dst_path)?;
        } else {
            fs::copy(&path, &dst_path)?;
            fs::remove_file(&path)?;
        }
    }

    Ok(())
}

// 创建WARP路由
fn create_routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let sync_route = warp::path("sync")
        .and(warp::post())
        .and(warp::path::tail())
        .map(move |tail: String| {
# FIXME: 处理边界情况
            let src = Path::new(&tail);
            let dst = src.join("backup");
            match sync_files(&src, &dst) {
                Ok(_) => warp::reply::json({"status": "success", "message": "同步成功"}),
                Err(e) => warp::reply::json({"status": "error", "message": format!("同步失败: {:?}