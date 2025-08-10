use warp::Filter;
use sysinfo::{System, SystemExt};

// Define a structure to hold the performance data.
#[derive(serde::Serialize, serde::Deserialize)]
struct PerformanceMetrics {
    pub cpu_usage: f32,
    pub memory_usage: f32,
    pub disk_usage: f32,
}

// Function to retrieve system performance metrics.
async fn get_performance_metrics() -> Result<PerformanceMetrics, warp::Rejection> {
    let mut sys = System::new_all();
    sys.refresh_all();
    let cpu_usage = sys.global_processor_info().cpu_usage();
    let memory_usage = sys.used_memory() as f32 / sys.total_memory() as f32 * 100.0;
    let disk_usage = sys.disks().iter().map(|disk| {
        disk.used_space() as f32 / disk.total_space() as f32 * 100.0
    }).max().unwrap_or(0.0);

    Ok(PerformanceMetrics {
        cpu_usage,
        memory_usage,
        disk_usage,
    })
}

// Warp filter to handle GET requests to the /metrics endpoint.
fn metrics_route() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("metrics")
        .and(warp::get())
        .and(with_system().map(get_performance_metrics))
        .and_then(handle_metrics)
}

// Function to handle the metrics response.
async fn handle_metrics(metrics: Result<PerformanceMetrics, warp::Rejection>) -> Result<impl warp::Reply, warp::Rejection> {
    match metrics {
        Ok(metrics) => Ok(warp::reply::json(&metrics)),
        Err(_) => Err(warp::reject::not_found()),
    }
}

// Main function to start the Warp server.
#[tokio::main]
async fn main() {
    let metrics_route = metrics_route();
    warp::serve(metrics_route)
        .run(([0, 0, 0, 0], 3030))
        .await;
}
