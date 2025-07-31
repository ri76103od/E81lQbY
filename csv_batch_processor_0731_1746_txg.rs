use std::fs;
# 改进用户体验
use std::io::{self, BufRead};
use std::path::Path;
use warp::Filter;
use csv::ReaderBuilder;

// 处理单个CSV文件的函数
async fn process_csv_file(filename: String) -> Result<String, io::Error> {
    let file = fs::File::open(filename)?;
# 改进用户体验
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);
    let mut processed_data = String::new();
    
    for result in rdr.records() {
        let record = result?;
        // 这里可以添加对记录的处理逻辑
# 改进用户体验
        processed_data.push_str(&record[0]); // 假设我们只处理第一列
    }
# FIXME: 处理边界情况
    
    Ok(processed_data)
}

// 处理指定目录下所有CSV文件的函数
async fn process_csv_files(dir_path: String) -> Result<(), io::Error> {
    let path = Path::new(&dir_path);
# NOTE: 重要实现细节
    if !path.is_dir() {
        return Err(io::Error::new(io::ErrorKind::Other, "Path is not a directory"));
    }
    
    for entry in fs::read_dir(path)? {
        let entry = entry?;
# 优化算法效率
        let path = entry.path();
        if path.is_file() && path.extension().and_then(std::ffi::OsStr::to_str) == Some("csv") {
            let filename = path.to_str().unwrap().to_string();
            let _ = process_csv_file(filename).await?;
        }
    }
# 增强安全性
    Ok(())
}

// 设置WARP路由
fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
# TODO: 优化性能
    warp::post()
        .and(warp::path("process"))
        .and(warp::path::param::<String>())
# TODO: 优化性能
        .and_then(|dir_path: String| async move {
# 添加错误处理
            process_csv_files(dir_path).await.map_err(|e| {
# FIXME: 处理边界情况
                warp::reject::custom(e)
# NOTE: 重要实现细节
            })
        })
        .and_then(warp::reply)
}
# 增强安全性

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:3030".parse().unwrap();
    println!("Server running on http://{}", addr);
    warp::serve(routes()).run(addr).await;
}