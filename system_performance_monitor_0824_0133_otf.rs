use warp::Filter;
use std::collections::HashMap;
use sysinfo::{System, SystemExt, ProcessorExt, DiskExt, CpuExt};

// Initialize the System struct to get system information.
let sys = System::new_all();
sys.refresh_all();

// Define a simple struct to hold system performance data.
#[derive(serde::Serialize)]
struct SystemPerformance {
    cpu_usage: f32,
    memory_usage: f32,
    disk_usage: HashMap<String, f32>,
}

// Create a filter to handle GET requests and return system performance data.
let get_system_performance = warp::path("system")
    .and(warp::get())
    .map(move || {
        // Retrieve CPU usage.
        let cpu_usage = sys.global_processor_info().cpu_usage();
        
        // Retrieve memory usage.
        let memory_usage = sys.used_memory() as f32 / sys.total_memory() as f32 * 100.0;
        
        // Retrieve disk usage for each disk.
        let mut disk_usage = HashMap::new();
        for disk in sys.disks() {
            disk_usage.insert(disk.name().clone(), disk.used_space() as f32 / disk.total_space() as f32 * 100.0);
        }
        
        // Create and return the SystemPerformance struct with the current system data.
        SystemPerformance {
            cpu_usage,
            memory_usage,
            disk_usage,
        }
    }).with(warp::reply::json);

// Start the Warp server on localhost port 3030.
let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3030));
warp::serve(get_system_performance).run(addr).await;
