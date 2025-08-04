 * Features:
 * - Token-based access control
 * - Error handling
 * - Documentation and comments for maintainability
 */

use warp::Filter;

// Define the error type for our application.
#[derive(Debug, Clone)]
struct AuthError;

// Define the error response structure for unauthorized access.
#[derive(serde::Serialize)]
struct UnauthorizedResponse {"error": String}

// Implement the `warp::reject::Reject` trait for our error type.
impl warp::reject::Reject for AuthError {}

// Define a filter to check for the presence of a token in the request header.
fn with_token() -> impl Filter<Extract = (String,), Error = warp::Rejection> + Clone {
    warp::header::<&str>::<String>("Authorization")
        .and_then(|token: String| async move {
            if token.starts_with("Bearer ") {
                Ok(token[7..].to_string())
            } else {
                Err(warp::reject::custom(AuthError))
            }
        })
}

// Define the route that requires authentication.
#[tokio::main]
async fn main() {
    // Define the route that requires a token in the request header to access.
    let auth_route = warp::path("secure")
        .and(with_token()) // Apply the token filter to this route.
        .map(|token: String| {
            // If the token is valid, return a success message.
            warp::reply::json(&{"message": format!("Access granted with token: {}", token), "token": token})
        }).recover(handle_rejection); // Handle any rejections from this route.

    // Start the Warp server on localhost port 3030.
    warp::serve(auth_route)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

// Function to handle rejections, turning them into a JSON response.
async fn handle_rejection(err: warp::Rejection) -> Result<impl warp::Reply, warp::Rejection> {
    if err.find::<std::convert::Infallible>().is_some() || err.find::<AuthError>().is_some() {
        // Handle the case where authorization fails.
        Ok(warp::reply::with_status(
            warp::reply::json(&UnauthorizedResponse {"error": "Unauthorized".to_string()}),
            warp::http::StatusCode::UNAUTHORIZED,
        ))
    } else {
        // For any other kind of error, forward it to the next error handler.
        Err(err)
    }
}