 *        It fetches the images, resizes them to the specified dimensions, and returns a JSON response.
 *
 * Note: This application assumes that the input URLs are valid and the images are accessible.
 *
 * Dependencies: warp, image, serde, serde_json, reqwest
 */

use warp::http::Response;
use warp::{Filter, Rejection, Reply};
use serde::{Deserialize};
use image::imageops::resize;
use image::{DynamicImage, GenericImageView};
use reqwest::get;
use std::error::Error;
use serde_json::{json, Value};
use std::io::Cursor;

// Define a struct to deserialize the incoming JSON body.
#[derive(Deserialize)]
struct ResizeRequest {
    urls: Vec<String>,
    width: u32,
    height: u32,
}

// Define the resize filter.
fn resize_filter() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::post()
        .and(warp::path("resize"))
        .and(warp::body::json())
        .and_then(handle_resize_request)
}

// Handle the resizing request.
async fn handle_resize_request(req: ResizeRequest) -> Result<impl Reply, Rejection> {
    let resized_images = futures::future::join_all(req.urls.into_iter().map(|url| {
        async move {
            let img_response = get(url).await;
            if let Ok(resp) = img_response {
                let img_bytes = resp.bytes().await;

                if let Ok(bytes) = img_bytes {
                    let img = image::load(Cursor::new(bytes), image::ImageFormat::Png)
                        .expect("Failed to load image");
                    let resized_img = resize(&img, req.width, req.height, image::imageops::FilterType::Nearest);

                    Ok::<_, Box<dyn Error>>(json!({
                        "url": url,
                        "width": resized_img.width(),
                        "height": resized_img.height(),
                        "data": format!("data:image/png;base64,{}", base64::encode_config(&resized_img.to_bytes(), base64::Config::new(base64::CharacterSet::Standard)))
                    }))
                } else {
                    Err("Failed to fetch image bytes")?
                }
            } else {
                Err("Failed to fetch image response")?
            }
        }
    })).await;

    Ok(warp::reply::json(&resized_images))
}

// Define the main function.
#[tokio::main]
async fn main() {
    // Start the server.
    let routes = resize_filter();
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
