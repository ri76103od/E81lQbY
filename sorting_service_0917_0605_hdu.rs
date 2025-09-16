 * This service provides an endpoint to sort arrays
 * of integers using different sorting algorithms.
 */

use warp::Filter;

// Define a struct to hold the sorting request
pub struct SortingRequest {
    pub numbers: Vec<i32>,
    pub algorithm: String,
}

// Define the response struct
pub struct SortingResponse {
    pub sorted_numbers: Vec<i32>,
    pub algorithm_used: String,
}

// Implement a function to sort the numbers using bubble sort
fn bubble_sort(numbers: Vec<i32>) -> Vec<i32> {
    let mut numbers = numbers;
    for i in 0..numbers.len() {
        for j in 0..numbers.len() - i - 1 {
            if numbers[j] > numbers[j + 1] {
                numbers.swap(j, j + 1);
            }
        }
    }
    numbers
}

// Implement a function to sort the numbers using insertion sort
fn insertion_sort(numbers: Vec<i32>) -> Vec<i32> {
    let mut numbers = numbers;
    for i in 1..numbers.len() {
        let mut j = i;
        while j > 0 && numbers[j - 1] > numbers[j] {
            numbers.swap(j - 1, j);
            j -= 1;
        }
    }
    numbers
}

// Sorting handler function
fn sort_handler(request: SortingRequest) -> Result<SortingResponse, warp::Rejection> {
    let algorithm = request.algorithm.as_str();
    let mut numbers = request.numbers;

    let sorted_numbers = match algorithm {
        "bubble" => bubble_sort(numbers),
        "insertion" => insertion_sort(numbers),
        _ => return Err(warp::reject::not_found()),
    };

    Ok(SortingResponse {
        sorted_numbers,
        algorithm_used: algorithm.to_string(),
    })
}

// Warp filter to parse the request body and extract the sorting parameters
fn with_sorting_request() -> impl Filter<Extract = (SortingRequest,), Error = warp::Rejection> + Clone {
    warp::path("sort")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(|numbers: Vec<i32>, algorithm: String| {
            Ok(SortingRequest { numbers, algorithm })
        })
}

// Main function to start the Warp server
#[tokio::main]
async fn main() {
    let sort_route = warp::post()
        .and(with_sorting_request())
        .and_then(sort_handler)
        .and_then(|response: SortingResponse| async move {
            warp::reply::json(&response)
        });

    warp::serve(sort_route)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
