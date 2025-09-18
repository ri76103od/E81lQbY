use warp::Filter;
use serde::Serialize;
use serde_json::json;
use std::collections::HashMap;

// Define the User struct to hold user data
#[derive(Serialize)]
struct User {
    id: u32,
    username: String,
    permissions: Vec<String>,
}

// Define the Permission struct to hold permission data
#[derive(Serialize)]
struct Permission {
    id: u32,
    name: String,
}

// Define the state to hold the in-memory database of users and permissions
struct AppState {
    users: HashMap<u32, User>,
    permissions: HashMap<u32, Permission>,
}

// Implement the AppState struct
impl AppState {
    // Function to create a new AppState
    fn new() -> Self {
        AppState {
            users: HashMap::new(),
            permissions: HashMap::new(),
        }
    }

    // Function to add a new user
    fn add_user(&mut self, user: User) {
        self.users.insert(user.id, user);
    }

    // Function to add a new permission
    fn add_permission(&mut self, permission: Permission) {
        self.permissions.insert(permission.id, permission);
    }

    // Function to get a user by ID
    fn get_user(&self, id: u32) -> Option<&User> {
        self.users.get(&id)
    }

    // Function to get a permission by ID
    fn get_permission(&self, id: u32) -> Option<&Permission> {
        self.permissions.get(&id)
    }
}

// Define the routes for the user permission management system
fn routes(state: AppState) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let user_route = warp::path!("user" / u32)
        .map(move |id| warp::any().map(move || state.get_user(id).cloned()));

    let permission_route = warp::path!("permission" / u32)
        .map(move |id| warp::any().map(move || state.get_permission(id).cloned()));

    user_route.or(permission_route)
        .and_then(|user| async move {
            match user {
                Some(user) => Ok(warp::reply::json(&user)),
                None => Ok(warp::reply::json(&json!({"error": "User not found"}))),
            }
        })
}

// Main function to start the server
#[tokio::main]
async fn main() {
    let state = AppState::new();

    // Add sample users and permissions to the state
    state.add_user(User { id: 1, username: "Alice".to_string(), permissions: vec!["read".to_string(), "write".to_string()] });
    state.add_permission(Permission { id: 1, name: "read".to_string() });
    state.add_permission(Permission { id: 2, name: "write".to_string() });

    let routes = routes(state);
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
