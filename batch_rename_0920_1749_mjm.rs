use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use warp::Filter;

// 定义一个结构体，用于存储批量重命名的配置
struct RenameConfig {
# 增强安全性
    src_pattern: String, // 源文件名模式
    dst_pattern: String, // 目标文件名模式
    base_path: PathBuf,  // 基础路径
}

// 实现RenameConfig的方法
impl RenameConfig {
    // 创建一个新的RenameConfig实例
# TODO: 优化性能
    fn new(src_pattern: &str, dst_pattern: &str, base_path: &str) -> Self {
        RenameConfig {
            src_pattern: src_pattern.to_string(),
            dst_pattern: dst_pattern.to_string(),
            base_path: PathBuf::from(base_path),
        }
    }

    // 执行批量重命名操作
    fn rename_files(&self) -> io::Result<()> {
        for entry in fs::read_dir(&self.base_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                let file_name = path.file_name().unwrap().to_str().unwrap();
                if file_name.starts_with(&self.src_pattern) {
                    let new_name = self.dst_pattern.replace("{{file}}", file_name);
                    let new_path = self.base_path.join(&new_name);
# FIXME: 处理边界情况
                    fs::rename(&path, &new_path)?;
                }
            }
        }
        Ok(())
    }
}
# TODO: 优化性能

// 设置Warp路由并启动服务
# NOTE: 重要实现细节
fn main() {
    // 配置重命名规则
    let config = RenameConfig::new("old_", "new_{{file}}", "./");

    // 重命名文件
    if let Err(e) = config.rename_files() {
        eprintln!("Error renaming files: {}", e);
    }
}
