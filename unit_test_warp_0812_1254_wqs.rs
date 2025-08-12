use warp::Filter;
use warp::http::StatusCode;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

// A struct to hold test results
struct TestResult {
    tests_run: AtomicUsize,
    tests_failed: AtomicUsize,
}

// Function to initialize test results
impl TestResult {
    fn new() -> TestResult {
        TestResult {
            tests_run: AtomicUsize::new(0),
            tests_failed: AtomicUsize::new(0),
        }
    }

    // Function to increment the number of tests run
    fn increment_tests_run(&self) {
        self.tests_run.fetch_add(1, Ordering::SeqCst);
    }

    // Function to increment the number of tests failed
    fn increment_tests_failed(&self) {
        self.tests_failed.fetch_add(1, Ordering::SeqCst);
    }
}

// Function to run a test case
fn run_test_case(test_case: fn() -> Result<(), &'static str>, test_name: &str, test_result: Arc<TestResult>) {
    let test_result_clone = Arc::clone(&test_result);
    let test_name_clone = test_name.to_string();

    // Run the test case and handle any errors
    let test_result = test_case();
    match test_result {
        Ok(_) => {
            println!("Test passed: {}", test_name_clone);
            test_result_clone.increment_tests_run();
        },
        Err(e) => {
            println!("Test failed: {}
Error: {}", test_name_clone, e);
            test_result_clone.increment_tests_failed();
        },
    }
}

// Example test case
fn example_test_case() -> Result<(), &'static str> {
    // This is a simple test case that checks if 2 + 2 equals 4
    if 2 + 2 != 4 {
        Err("2 + 2 does not equal 4")
    } else {
        Ok(())
    }
}

// Warp filter to handle test results
fn test_results_filter(test_result: Arc<TestResult>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("test_results")
        .and(warp::get())
        .map(move || {
            let tests_run = test_result.tests_run.load(Ordering::SeqCst);
            let tests_failed = test_result.tests_failed.load(Ordering::SeqCst);
            warp::reply::json(&json!({
                "tests_run": tests_run,
                "tests_failed": tests_failed,
            }))
        })
}

#[tokio::main]
async fn main() {
    let test_result = Arc::new(TestResult::new());

    // Run test cases
    run_test_case(example_test_case, "Example Test Case", Arc::clone(&test_result));

    // Create Warp server
    let routes = warp::service(test_results_filter(test_result))
        .with(warp::log("unit_test_warp"));
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
