// folder_structure_organizer.rs
//
// 程序功能：整理文件夹结构，按照指定规则将文件和文件夹重新组织。
//
// 作者：Your Name
// 日期：YYYY-MM-DD

// 引入RUST标准库中的模块
use std::fs::{self, DirEntry};
use std::path::{Path, PathBuf};
use warp::Filter;

// 定义一个结构体，用于处理文件夹整理的逻辑
struct FolderOrganizer;

impl FolderOrganizer {
    // 创建一个新的FolderOrganizer实例
    pub fn new() -> Self {
        FolderOrganizer
    }

    // 整理给定的文件夹路径
    pub fn organize(&self, path: PathBuf) -> Result<(), std::io::Error> {
        // 读取给定路径下的所有文件和文件夹
        let entries = fs::read_dir(path)?;
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                // 如果是文件夹，则递归调用organize方法
                self.organize(path)?;
            } else {
                // 如果是文件，根据需要进行处理（这里留空，具体逻辑根据需求实现）
            }
        }
        Ok(())
    }
}

// 设置WARP过滤器和路由
fn main() {
    let organizer = FolderOrganizer::new();
    let route = warp::path("organize")
        .and(warp::post())
        .and(warp::path::param())
        .and_then(move |path: String| async move {
            let path = PathBuf::from(path);
            match organizer.organize(path) {
                Ok(_) => Ok(warp::reply::json(&{"success": true, "message": "Folder organized successfully."})),
                Err(e) => Ok(warp::reply::json(&{"success": false, "message": format!("Failed to organize folder: {}", e)})),
            }
        });

    // 启动WARP服务器
    println!("Server running on http://127.0.0.1:3030/");
    warp::serve(route).run(([127, 0, 0, 1], 3030)).await;
}
