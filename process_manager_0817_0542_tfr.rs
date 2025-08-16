use warp::Filter;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};

// Define a Process struct to hold process details
struct Process {
    id: u32,
    status: ProcessStatus,
}

// Define the status of a process
enum ProcessStatus {
    Running,
    Stopped,
}

// A struct to hold the process manager state
struct ProcessManager {
    processes: Arc<Mutex<HashMap<u32, Process>>>,
    next_id: AtomicUsize,
}

impl ProcessManager {
    // Create a new ProcessManager
    fn new() -> Self {
        ProcessManager {
            processes: Arc::new(Mutex::new(HashMap::new())),
            next_id: AtomicUsize::new(0),
        }
    }

    // Start a new process
    fn start_process(&self) -> u32 {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst) as u32;
        let process = Process {
            id,
            status: ProcessStatus::Running,
        };
        let mut processes = self.processes.lock().unwrap();
        processes.insert(id, process);
        id
    }

    // Stop a process by its ID
    fn stop_process(&self, id: u32) -> Result<(), String> {
        let mut processes = self.processes.lock().unwrap();
        if let Some(process) = processes.get_mut(&id) {
            process.status = ProcessStatus::Stopped;
            Ok(())
        } else {
            Err("Process not found".to_string())
        }
    }

    // List all processes
    fn list_processes(&self) -> Vec<Process> {
        let processes = self.processes.lock().unwrap();
        processes.values().cloned().collect()
    }
}

#[tokio::main]
async fn main() {
    // Create a new process manager
    let process_manager = ProcessManager::new();

    // Define the routes
    let start_process = warp::path!("start")
        .and_then(|| async move {
            let id = process_manager.start_process();
            Ok::<_, warp::Rejection>(warp::reply::json(&id))
        });

    let stop_process = warp::path!("stop" / u32)
        .and_then(move |id| async move {
            match process_manager.stop_process(id) {
                Ok(_) => Ok(warp::reply::json(&true)),
                Err(e) => Ok(warp::reply::json(&false).with_status(warp::http::StatusCode::NOT_FOUND)),
            }
        });

    let list_processes = warp::path!("list")
        .and_then(|| async move {
            let processes = process_manager.list_processes();
            Ok::<_, warp::Rejection>(warp::reply::json(&processes))
        });

    // Combine the routes
    let routes = start_process.or(stop_process).or(list_processes);

    // Start the server
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
