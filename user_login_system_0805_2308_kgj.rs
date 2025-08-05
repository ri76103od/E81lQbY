use warp::http::StatusCode;
use warp::Filter;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::RwLock;
use std::collections::HashMap;

// Define a simple user struct to store user credentials
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct User {
    username: String,
    password: String,
}

// Define a struct to store logged in users
struct LoggedInUsers {
    users: RwLock<HashMap<String, User>>,
}

impl LoggedInUsers {
    // Initialize the logged in users map
    fn new() -> Self {
        LoggedInUsers {
            users: RwLock::new(HashMap::new()),
        }
    }

    // Function to add a user to the logged in users map
    fn add_user(&self, user: User) {
        let mut users = self.users.write().unwrap();
        users.insert(user.username.clone(), user);
    }

    // Function to check if a user is logged in
    fn is_logged_in(&self, username: &str) -> bool {
        let users = self.users.read().unwrap();
        users.contains_key(username)
    }
}

#[tokio::main]
async fn main() {
    // Create a new instance of LoggedInUsers
    let logged_in_users = LoggedInUsers::new();

    // Define a user to be logged in for demonstration purposes
    let demo_user = User {
        username: "user123".to_string(),
        password: "password123".to_string(),
    };
    logged_in_users.add_user(demo_user.clone());

    // Define the login endpoint
    let login = warp::post()
        .and(warp::path("login"))
        .and(warp::body::json())
        .and(with_logged_in_users(&logged_in_users))
        .map(|user: User, logged_in_users: LoggedInUsers| async move {
            if logged_in_users.is_logged_in(&user.username) {
                Ok::<_, warp::Rejection>(json({"message": "User is already logged in"}.to_string()))
            } else if user == demo_user {
                logged_in_users.add_user(user.clone());
                Ok::<_, warp::Rejection>(json("User logged in successfully"))
            } else {
                Err(warp::reject::custom(LoginError::InvalidCredentials))
            }
        });

    // Define the logout endpoint
    let logout = warp::post()
        .and(warp::path("logout"))
        .and(warp::body::json())
        .and(with_logged_in_users(&logged_in_users))
        .map(|user: User, logged_in_users: LoggedInUsers| async move {
            if logged_in_users.is_logged_in(&user.username) {
                let mut users = logged_in_users.users.write().unwrap();
                users.remove(&user.username);
                Ok::<_, warp::Rejection>(json("User logged out successfully"))
            } else {
                Err(warp::reject::custom(LoginError::UserNotLoggedIn))
            }
        });

    // Start the Warp server
    warp::serve(login.or(logout)).run(([127, 0, 0, 1], 3030)).await;
}

// Define a filter to extract the logged in users reference from the state
fn with_logged_in_users(logged_in_users: &LoggedInUsers) -> impl Filter<Extract = (LoggedInUsers), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || logged_in_users.clone())
}

// Define custom error types for login errors
#[derive(Debug)]
enum LoginError {
    InvalidCredentials,
    UserNotLoggedIn,
}

impl warp::reject::Reject for LoginError {}
