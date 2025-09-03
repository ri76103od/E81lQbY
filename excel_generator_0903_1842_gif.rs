// excel_generator.rs
// This program uses the WARP framework to create an HTTP server that generates Excel files.

use warp::Filter;
use serde::Deserialize;
use serde_json::json;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use xlsxwriter::XlsxWriter;

// The structure for the incoming request's body.
#[derive(Deserialize, Debug)]
struct GenerateRequest {
    sheet_name: String, // The name of the sheet in the Excel file.
    data: Vec<Vec<String>>, // The data to be written in the Excel file.
}

// The main function that sets up the server.
#[tokio::main]
async fn main() {
    // The route to generate an Excel file.
    let generate_excel = warp::path!("generate")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(generate_excel_handler);

    // Start the server.
    warp::serve(generate_excel).run(([127, 0, 0, 1], 3030)).await;
}

// The handler function for the /generate route.
async fn generate_excel_handler(body: GenerateRequest) -> Result<impl warp::Reply, warp::Rejection> {
    // Create a new Excel file.
    let mut file = match File::create("generated_excel.xlsx") {
        Ok(file) => file,
        Err(e) => return Err(warp::reject::custom(e)),
    };

    // Create a new XlsxWriter object.
    let mut workbook = match XlsxWriter::new(file) {
        Ok(writer) => writer,
        Err(e) => return Err(warp::reject::custom(e)),
    };

    // Add a new worksheet.
    let worksheet = match workbook.add_worksheet(&body.sheet_name) {
        Ok(worksheet) => worksheet,
        Err(e) => return Err(warp::reject::custom(e)),
    };

    // Write data to the worksheet.
    for (row, row_data) in body.data.iter().enumerate() {
        for (col, value) in row_data.iter().enumerate() {
            match worksheet.write_string(row, col, value) {
                Ok(_) => (),
                Err(e) => return Err(warp::reject::custom(e)),
            }
        }
    }

    // Close the worksheet and the workbook to finalize the file.
    drop(worksheet);
    drop(workbook);

    // Return a JSON response indicating success.
    Ok(warp::reply::json(&json!({
        "status": "success",
        "message": "Excel file generated successfully.",
    })))
}
