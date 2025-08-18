 * interactive_chart_generator.rs
 *
 * An interactive chart generator using Rust and Warp.
 *
 * This service allows users to generate interactive charts by submitting data.
 * The charts are generated asynchronously and can be accessed through a URL.
 *
 * Requirements:
 * - Warp: A web server framework for Rust.
 * - Plotters: A plotting library for Rust.
 * - Tokio: An event-driven, non-blocking I/O platform for writing asynchronous I/O backed applications.
 *
 * Usage:
 * - Start the server with `cargo run`.
 * - Access the `/generate` endpoint to submit chart data and receive a URL for the generated chart.
 */

use warp::http::StatusCode;
use warp::{Filter, Rejection, Reply, ReplyWith, ReplyWithFuture};
use std::collections::HashMap;
use plotters::prelude::*;
use tokio::sync::Mutex;
use std::sync::Arc;
use once_cell::sync::Lazy;

// Global state to store generated charts
static CHARTS: Lazy<Arc<Mutex<HashMap<String, String>>>> = Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));
def generate_chart(data: &str) -> Result<String, warp::Rejection> {
    // Parse the input data into a vector of tuples (x, y)
    let points: Vec<(f64, f64)> = data
        .split(',')
        .filter_map(|point| {
            point.trim().split_once(':').map(|(x, y)| (x.parse().ok(), y.parse().ok()))
                .and_then(|(x, y)| x.zip(y))
        })
        .collect::<Result<_, _>>()?;

    // Create a new chart with the points
    let drawing_area = BitMapBackend::new("chart.png", (640, 480)).into_drawing_area();
    drawing_area.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&drawing_area)
        .caption("Interactive Chart", ("sans-serif", 50))
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(f64::MIN..=f64::MAX, f64::MIN..=f64::MAX)
        .unwrap();

    chart.configure_mesh().draw().unwrap();
    chart.draw_series(PointSeries::of_element(points, &|coord, size| {
        (*coord, Circle::new((0, 0), 3, &RED))
    }).into_iter().map(|e| e.into_styled(Color::from(RED), size::Stroke::new(2, BLACK))).collect()).unwrap();

    // Return the URL to access the generated chart
    let chart_id = chrono::Utc::now().to_rfc3339();
    let _ = CHARTS.lock().unwrap().insert(chart_id.clone(), "chart.png".to_string());
    Ok(chart_id)
}

// Define the route to handle chart generation requests
fn generate_route() -> impl Filter<Extract = impl Reply + Send + 'static, Error = Rejection> + Clone {
    warp::post()
        .and(warp::path("generate"))
        .and(warp::body::content_length_limit(1024 * 1024)) // Limit the request body to 1MB
        .and(warp::body::json())
        .and_then(|data: HashMap<String, String>| {
            let chart_data = data.get("data").unwrap_or(&"0:0".to_string());
            async move {
                generate_chart(chart_data).await.map(|chart_id| {
                    warp::reply::json(&chart_id)
                }).map_err(|_| warp::reject::not_found())
            }
        })
}

#[tokio::main]
async fn main() {
    let generate = generate_route();
    warp::serve(generate).run(([127, 0, 0, 1], 3030)).await;
}
