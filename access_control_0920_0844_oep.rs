use warp::reject:: Reject;
use warp::Filter;
use warp::http::StatusCode;
use warp::reply::Reply;
use serde::{Deserialize, Serialize};
use serde_json::json;

// Define the User struct for deserialization
#[derive(Deserialize, Serialize, Clone, Debug)]
struct User {
    username: String,
    password: String,
}

// Define an error type for authentication failure
#[derive(Debug)]
struct AuthError;

// Implement Reject for AuthError to convert it to a warp::Rejection
impl Reject for AuthError {
    fn reject(&self) -> warp::Rejection {
        warp::reject::custom(
            json!({
                "error": "Authentication failed"
            }).into_response()
        )
    }
}

// Function to check user credentials
async fn check_user_credentials(user: User) -> Result<&'static str, AuthError> {
    // Replace with actual user validation logic
    if user.username == "admin" && user.password == "password123" {
        Ok("User is authenticated")
    } else {
        Err(AuthError)
    }
}

// Define a route to handle user authentication
fn auth_route() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post()
        .and(warp::path("auth"))
        .and(warp::body::json())
        .and_then(|user: User| async move {
            match check_user_credentials(user).await {
                Ok(_) => Ok(warp::reply::json(&json!({
                    "message": "Authentication successful"
                }))),
                Err(_) => Err(warp::reject::custom(AuthError)),
            }
        })
}

// Define a route that requires authentication
fn protected_route() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path("protected"))
        .and(warp::filters::header::optional::<String>("Authorization"))
        .and_then(|auth_header: Option<String>| async move {
            match auth_header {
                Some(header) if header == "Bearer admin" => Ok(warp::reply::json(&json!({
                    "message": "Access granted"
                }))),
                _ => Err(warp::reject::custom(AuthError)),
            }
        })
}

// Define the main function
#[tokio::main]
async fn main() {
    let routes = warp::service(auth_route())
        .or(warp::service(protected_route()));

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
