 * Features:
 * - Batch processing of image files.
 * - Error handling for file I/O operations.
 * - Configurable image resizing dimensions.
 */

use warp::Filter;
use std::path::Path;
use image::{DynamicImage, GenericImageView, ImageResult};
use std::io::BufReader;
use tokio::fs::File;
use std::io::BufRead;
use warp::http::StatusCode;
use warp::reject::Rejection;
use std::error::Error;
use image::imageops::resize;
use tokio::fs::metadata;
use warp::reply::Json;
use serde::Serialize;
use serde_json::json;
use tokio::fs::read;
use tokio::fs::File as TokioFile;

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
}

#[derive(Debug, Serialize)]
struct ImageSize {
    width: u32,
    height: u32,
}

#[derive(Debug, Serialize)]
struct ResizeResponse {
    images: Vec<ImageSize>,
}

async fn resize_images(input_files: Vec<String>, new_width: u32, new_height: u32) -> Result<ResizeResponse, Rejection> {
    let mut resized_images = Vec::new();
    for file_path in input_files {
        let path = Path::new(&file_path);
        if !path.exists() {
            return Err(warp::reject::not_found().map_err(|_| warp::reject::custom(ErrorResponse {
                error: format!("File not found: {}", file_path),
            })));
        }
        match resize_image(&file_path, new_width, new_height).await {
            Ok(size) => resized_images.push(size),
            Err(e) => return Err(warp::reject::custom(ErrorResponse {
                error: e.to_string(),
            })),
        }
    }
    Ok(ResizeResponse {
        images: resized_images,
    })
}

async fn resize_image(file_path: &str, new_width: u32, new_height: u32) -> Result<ImageSize, Box<dyn Error>> {
    let file = TokioFile::open(file_path).await?;
    let image = image::open(file).await?;
    let resized_image = resize(&image, new_width, new_height, image::imageops::FilterType::Nearest);
    let size = ImageSize {
        width: resized_image.width(),
        height: resized_image.height(),
    };
    Ok(size)
}

fn with_error_handling() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::any().map(move || {
        warp::reject::custom(ErrorResponse {
            error: "An error occurred.".to_string(),
        })
    })
}

#[tokio::main]
async fn main() {
    let image_files = warp::path::param::<String>().and_then(|input_file: String| async move {
        Ok(vec![input_file])
    });
    let dimensions = warp::path::param::<u32>().and(warp::path::param::<u32>());
    let resize_images = image_files
        .and(dimensions)
        .and_then(|input_files: Vec<String>, (new_width, new_height): (u32, u32)| {
            resize_images(input_files, new_width, new_height)
        })
        .recover(with_error_handling);

    let routes = resize_images.with(warp::reply::json::json);
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}