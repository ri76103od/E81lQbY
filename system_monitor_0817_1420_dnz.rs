// system_monitor.rs
// This Rust program uses the Warp framework to create a system performance monitoring tool.

use warp::Filter;
use serde::Deserialize;
use sysinfo::{System, SystemExt};
use std::sync::Arc;
use tokio::{sync::Mutex, runtime::Runtime};

// Define a struct to represent the system performance data
#[derive(Deserialize, Clone)]
struct PerformanceData {
    system_load: f32,
    system_uptime: u64,
    cpu_usage: f32,
    memory_usage: u64,
    process_count: usize,
}

// Function to get the system performance data
async fn get_system_performance() -> Result<impl warp::Reply, warp::Rejection> {
    let system = System::new_all();
    let load1 = system.get_load_average_1();
    let uptime = system.get_uptime();
    let cpu_usage = system.get_global_cpu_usage();
    let memory_usage = system.get_used_memory();
    let process_count = system.get_processes().len();

    let performance_data = PerformanceData {
        system_load: load1,
        system_uptime: uptime.as_secs(),
        cpu_usage,
        memory_usage,
        process_count,
    };

    Ok(warp::reply::json(&performance_data))
}

// The main function to set up the Warp server
#[tokio::main]
async fn main() {
    // Create a shared system instance
    let system = Arc::new(Mutex::new(System::new_all()));
    let runtime = Runtime::new().unwrap();
    runtime.handle().spawn(async move {
        loop {
            // Periodically update the system info
            system.lock().await.refresh_all();
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    });

    // Define the route for getting system performance data
    let get_performance = warp::path("performance")
        .and_then(get_system_performance);

    // Start the Warp server
    warp::serve(get_performance)
        .run(([0, 0, 0, 0], 3030))
        .await;
}
