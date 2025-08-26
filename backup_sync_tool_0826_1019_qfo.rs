use std::fs::{self, File, Metadata};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use warp::Filter;

// 定义一个结构体来封装备份和同步配置
struct BackupSyncConfig {
    src: PathBuf,
    dst: PathBuf,
}

// 实现BackupSyncConfig
impl BackupSyncConfig {
    // 创建一个新的BackupSyncConfig实例
    pub fn new(src: PathBuf, dst: PathBuf) -> Self {
        BackupSyncConfig { src, dst }
    }

    // 执行文件备份和同步操作
    pub fn perform_backup_sync(&self) -> io::Result<()> {
        // 检查源路径是否存在
        let metadata = fs::metadata(&self.src)?;

        // 检查路径是否为文件
        if !metadata.is_file() {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Source must be a file"));
        }

        // 创建目标目录
        fs::create_dir_all(&self.dst)?;

        // 构建目标文件路径
        let dest_path = self.dst.join(&self.src.file_name().ok_or_else(||
            io::Error::new(io::ErrorKind::InvalidInput, "Source path has no file name")
        )?);

        // 执行文件复制操作
        let mut source_file = File::open(&self.src)?;
        let mut dest_file = File::create(&dest_path)?;
        io::copy(&mut source_file, &mut dest_file)?;

        Ok(())
    }
}

// 设置WARP路由
#[tokio::main]
async fn main() {
    let backup_sync = warp::any()
        .map(move || {
            let config = BackupSyncConfig::new(
                PathBuf::from("./src"),
                PathBuf::from("./dst"),
            );
            config.perform_backup_sync()
        })
        .and_then(|res| async move {
            match res {
                Ok(_) => warp::reply::with_status("Backup and sync successful", warp::http::StatusCode::OK),
                Err(e) => warp::reply::with_status(format!("Backup and sync failed: {}", e), warp::http::StatusCode::INTERNAL_SERVER_ERROR),
            }
        });

    warp::serve(backup_sync).run(([127, 0, 0, 1], 3030)).await;
}
