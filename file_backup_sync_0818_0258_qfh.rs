use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use warp::Filter;

// 定义备份同步工具的结构
struct BackupSyncTool {
    src: PathBuf, // 源目录
    dst: PathBuf, // 目标目录
}

impl BackupSyncTool {
    /// 创建一个新的备份同步工具实例
    pub fn new(src: PathBuf, dst: PathBuf) -> Self {
        BackupSyncTool { src, dst }
    }

    /// 同步文件至目标目录
    pub fn sync_files(&self) -> io::Result<()> {
        for entry in fs::read_dir(&self.src)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                // 递归同步子目录
                let dst_dir = self.dst.join(path.file_name().unwrap());
                fs::create_dir_all(&dst_dir)?;
                let tool = BackupSyncTool::new(path, dst_dir);
                tool.sync_files()?;
            } else {
                // 同步文件
                let dst_file = self.dst.join(path.file_name().unwrap());
                let mut src_file = File::open(&path)?;
                let mut dst_file = File::create(&dst_file)?;
                io::copy(&mut src_file, &mut dst_file)?;
            }
        }
        Ok(())
    }
}

// 定义WARP路由
fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post()
        .and(warp::path("backup_sync"))
        .and(warp::path::end())
        .and(warp::body::json::<BackupSyncTool>())
        .map(|tool: BackupSyncTool| {
            let result = tool.sync_files();
            match result {
                Ok(_) => warp::reply::json(&"Sync completed successfully"),
                Err(e) => warp::reply::json(&format!("Error: {}", e)),
            }
        })
}

fn main() {
    let addr = ([127, 0, 0, 1], 3030).into();
    println!("Server running on http://{}.", addr);
    warp::serve(routes()).run(addr);
}
