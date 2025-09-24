// batch_file_renamer.rs
//
// 一个使用RUST和WARP框架的批量文件重命名工具。

use std::fs;
use std::path::Path;
use warp::Filter;

// 定义一个结构体来处理文件重命名的请求。
struct RenameRequest {
    directory: String,
    pattern: String,
    
    // 可以添加更多的字段来支持更多的重命名规则
}

// 定义一个响应结构体，用于返回结果。
struct RenameResponse {
# 增强安全性
    success: bool,
    message: String,
}

// 重命名文件的函数。
fn rename_files(directory: &str, pattern: &str) -> Result<(), String> {
    let path = Path::new(directory);
    if !path.exists() {
        return Err("Directory does not exist".to_string());
    }
    
    let mut counter = 1;
    for entry in fs::read_dir(path).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.is_file() {
            let new_name = format!("{}{}", pattern, counter);
            fs::rename(&path, path.with_file_name(new_name))
                .map_err(|e| e.to_string())?;
            counter += 1;
        }
# TODO: 优化性能
    }
    
    Ok(())
}
# TODO: 优化性能

// 创建一个WARP过滤器来处理HTTP请求。
fn main() {
    let rename_files_route = warp::post()
        .and(warp::path("rename"))
        .and(warp::path::end())
        .and(warp::json()).map(|request: RenameRequest| {
            let result = rename_files(&request.directory, &request.pattern);
            match result {
                Ok(_) => warp::reply::json(&RenameResponse {
                    success: true,
                    message: "Files renamed successfully".to_string(),
                }),
# TODO: 优化性能
                Err(e) => warp::reply::json(&RenameResponse {
                    success: false,
# TODO: 优化性能
                    message: e,
                }),
            }
        });

    // 启动WARP服务器。
    warp::serve(rename_files_route).run(([127, 0, 0, 1], 3030)).await;
}
