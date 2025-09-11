// SortingService.rs
// This module provides sorting algorithm implementations using Rust and Warp framework.

use warp::Filter;

// Define a struct to represent a sorting service.
struct SortingService;

impl SortingService {
    // Constructor for SortingService
    pub fn new() -> Self {
        SortingService
    }

    // Method to sort an array of integers using bubble sort algorithm.
    pub fn bubble_sort(&self, arr: Vec<i32>) -> Result<Vec<i32>, String> {
        if arr.is_empty() {
            return Err("Array is empty".to_string());
        }
        let mut arr = arr;
        for i in 0..arr.len() {
            for j in 0..arr.len() - i - 1 {
                if arr[j] > arr[j + 1] {
                    arr.swap(j, j + 1);
                }
            }
        }
        Ok(arr)
    }

    // Method to sort an array of integers using selection sort algorithm.
    pub fn selection_sort(&self, arr: Vec<i32>) -> Result<Vec<i32>, String> {
        if arr.is_empty() {
            return Err("Array is empty".to_string());
        }
        let mut arr = arr;
        for i in 0..arr.len() {
            let mut min_idx = i;
            for j in i + 1..arr.len() {
                if arr[j] < arr[min_idx] {
                    min_idx = j;
                }
            }
            arr.swap(min_idx, i);
        }
        Ok(arr)
    }
}

// Define a route to handle sorting requests.
fn sorting_route() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("sort" / "bubble" / "int")
        .and(warp::post())
        .and(warp::body::json::<Vec<i32>>())
        .map(|arr: Vec<i32>| {
            let sorting_service = SortingService::new();
            match sorting_service.bubble_sort(arr) {
                Ok(sorted_arr) => warp::reply::json(&sorted_arr),
                Err(e) => warp::reply::json(&e),
            }
        })
}

// Main function to start the Warp server.
#[tokio::main]
async fn main() {
    let routes = sorting_route();
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
