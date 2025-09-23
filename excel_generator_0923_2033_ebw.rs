use warp::Filter;
use serde::Serialize;
use open_xlsx::XlsxFile;
use std::io::Write;
use std::error::Error;
use std::fs::File;

// 定义一个结构体来存储生成的Excel数据
#[derive(Debug, Serialize)]
struct ExcelData {
    rows: Vec<Vec<String>>,
}

// 定义一个生成Excel文件的函数
async fn generate_excel(data: ExcelData) -> Result<(), Box<dyn Error>> {
    let mut file = XlsxFile::new();
    let mut sheet = file.add_sheet("Sheet1");
    for row in data.rows {
        let mut excel_row = sheet.add_row();
        for cell in row {
            excel_row.add_cell().string(&cell);
        }
    }
    let mut out_file = File::create("generated.xlsx")?;
    file.write(&mut out_file)?;
    Ok(())
}

// 创建一个简单的GET路由，用于生成Excel文件
fn excel_route() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("generate")
        .and(warp::post())
        .and(warp::any().map(move || ExcelData {
            rows: vec![vec!["Header1".to_string(), "Header2".to_string()]],
        }))
        .and_then(|data: ExcelData| async move {
            match generate_excel(data).await {
                Ok(_) => Ok(warp::reply::json(&{"message": "Excel file generated successfully"})),
                Err(e) => Ok(warp::reply::json(&{"error": format!("Failed to generate Excel file: {}", e)})),
            }
        })
}

// Warp 启动函数
#[tokio::main]
async fn main() {
    let routes = warp::service(excel_route());
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

/*
注释：
- 使用`open_xlsx`库来创建和写入Excel文件。
- `generate_excel`函数接受一个`ExcelData`结构体，并将其转换为Excel文件。
- `excel_route`设置一个POST路由，用于处理Excel生成请求。
- `main`函数启动Warp服务器，监听指定端口。
*/