use warp::Filter;
use serde::Deserialize;
use serde_json::json;
use std::fs::File;
use std::io::Write;
# FIXME: 处理边界情况
use std::path::Path;
use xlsxwriter::XlsxWriter;

// 定义请求体结构体，用于反序列化JSON请求
# TODO: 优化性能
#[derive(Deserialize)]
struct RequestBody {
    columns: Vec<String>,
    rows: Vec<Vec<String>>,
}

// 创建一个函数来生成Excel文件
async fn generate_excel(req_body: RequestBody) -> Result<impl warp::Reply, warp::Rejection> {
    let mut file = File::create("output.xlsx").map_err(|e| warp::reject::custom(e))?;
    let mut workbook = XlsxWriter::new(file);

    let worksheet = workbook.add_worksheet(None).map_err(|e| warp::reject::custom(e))?;

    for (i, column) in req_body.columns.iter().enumerate() {
        worksheet.write(0, i as u32, column).map_err(|e| warp::reject::custom(e))?;
    }

    for (row_index, row) in req_body.rows.iter().enumerate().skip(1) {
        for (col_index, value) in row.iter().enumerate() {
            worksheet.write((row_index + 1) as u32, col_index as u32, value).map_err(|e| warp::reject::custom(e))?;
        }
    }
# 增强安全性

    workbook.close().map_err(|e| warp::reject::custom(e))?;
    Ok(warp::reply::json(&json!({ "message": "Excel file generated successfully" })))
}

// 定义路由和过滤器
fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
# 改进用户体验
    warp::post()
        .and(warp::path("generate"))
        .and(warp::body::json())
# FIXME: 处理边界情况
        .and(with_request_body())
        .and_then(generate_excel)
}

// helper function to extract request body
fn with_request_body() -> impl Filter<Extract = (RequestBody,), Error = warp::Rejection> + Clone {
# FIXME: 处理边界情况
    warp::filters::body::content_length_limit(1024 * 16).and_then(|_length, _body: warp::filters::body::Body| async {
        Ok(RequestBody {
            columns: vec![],
            rows: vec![],
        })
    })
}

#[tokio::main]
async fn main() {
    println!("Server running on http://localhost:3030/");
    warp::serve(routes()).run(([127, 0, 0, 1], 3030)).await;
}

/*
# 优化算法效率
Documentation for the generate_excel function
# 改进用户体验
 * This function takes a RequestBody object containing the columns and rows for the Excel file,
 * generates an Excel file, and returns a JSON response indicating success.
# NOTE: 重要实现细节
 * Errors are handled by returning a custom warp rejection.
 */
