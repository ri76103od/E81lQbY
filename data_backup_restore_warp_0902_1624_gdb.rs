// data_backup_restore_warp.rs
// 使用RUST和WARP框架实现数据备份恢复程序

use warp::Filter;
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use anyhow::Result;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

// 定义一个结构体来处理备份和恢复操作
struct DataBackupRestore;

impl DataBackupRestore {
    // 创建备份文件
    async fn create_backup(file_path: String, backup_path: String) -> Result<()> {
        let mut reader = File::open(file_path).await?;
        let mut writer = File::create(backup_path).await?;
        io::copy(&mut reader, &mut writer).await?;
        Ok(())
    }

    // 恢复备份文件
    async fn restore_backup(file_path: String, backup_path: String) -> Result<()> {
        let mut reader = File::open(backup_path).await?;
        let mut writer = File::create(file_path).await?;
        io::copy(&mut reader, &mut writer).await?;
        Ok(())
    }
}

// 设置WARP路由以处理备份和恢复请求
fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let backup_route = warp::path("backup")
        .and(warp::post())
        .and(warp::path::param::<String>().map(move |file_path| file_path))
        .and(warp::path::param::<String>().map(move |backup_path| backup_path))
        .and_then(|file_path, backup_path| {
            async move {
                DataBackupRestore::create_backup(file_path, backup_path).await.map(|_| "Backup created successfully")
            }
        });

    let restore_route = warp::path("restore")
        .and(warp::post())
        .and(warp::path::param::<String>().map(move |file_path| file_path))
        .and(warp::path::param::<String>().map(move |backup_path| backup_path))
        .and_then(|file_path, backup_path| {
            async move {
                DataBackupRestore::restore_backup(file_path, backup_path).await.map(|_| "Backup restored successfully")
            }
        });

    backup_route.or(restore_route)
}

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:3030";
    println!("Server running on http://{} ", addr);
    warp::serve(routes()).run(([addr, 3030]).await);
}
