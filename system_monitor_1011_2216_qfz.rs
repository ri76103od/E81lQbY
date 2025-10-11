use warp::Filter;
use sysinfo::{System, SystemExt};
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::http::StatusCode;

// Define the structure for system resources
#[derive(Debug, Clone, serde::Serialize)]
struct SystemResources {
    total_memory: u64,
    free_memory: u64,
    used_memory: u64,
    cpu_usage: f32,
}

// Initialize the system information
fn init_system_info() -> Arc<Mutex<System>> {
    let mut system = System::new_all();
    system.refresh_all();
    Arc::new(Mutex::new(system))
}

// Create a route to get system resources
fn resources_route() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("resources")
        .and(warp::get())
        .and(with_system_info())
        .and_then(|system| async move {
            let system = system.lock().await;
            let resources = SystemResources {
                total_memory: system.get_total_memory(),
                free_memory: system.get_free_memory(),
                used_memory: system.get_used_memory(),
                cpu_usage: system.get_global_cpu_usage(),
            };
            Ok(warp::reply::json(&resources))
        })
}

// Middleware to handle the system info
fn with_system_info() -> impl Filter<Extract = Arc<Mutex<System>>, Error = std::convert::Infallible> + Clone {
    warp::any().map(move || init_system_info())
}

#[tokio::main]
async fn main() {
    let system_info = init_system_info();
    let resources_route = resources_route();

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3030));
    println!("System Monitor running on http://{}/resources", addr);

    warp::serve(resources_route)
        .run(addr)
        .await;
}
