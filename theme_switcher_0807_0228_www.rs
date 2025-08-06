documentation, and is structured for maintainability and extensibility.
*/

use warp::Filter;

#[tokio::main]
async fn main() {
    // Define the theme endpoint
    let theme = warp::path("theme")
        .and(warp::post()) // Only allow POST requests for theme switching
        .and(with_theme()) // A filter to extract the theme from the request body
        .and_then(switch_theme);

    // Start the server on port 3030
    warp::serve(theme)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

// A filter to extract the theme from the request body
fn with_theme() -> impl Filter<Extract = (String,), Error = warp::Rejection> + Clone {
    warp::body::json().map(|theme: String| theme)
}

// The handler function to switch themes
async fn switch_theme(theme: String) -> Result<impl warp::Reply, warp::Rejection> {
    // Here you would have logic to actually switch the theme, e.g., by updating a database or a configuration file
    // For this example, we'll just simulate a successful theme switch
    println!("Switching theme to: {}", theme);

    if theme.is_empty() {
        // If the theme is empty, return a bad request error
        return Err(warp::reject::custom(ThemeError::EmptyTheme));
    }

    // Return a successful response
    Ok(warp::reply::json(&ThemeResponse {
        message: format!("Theme switched to {}", theme),
    }))
}

// Define a custom error type for theme-related errors
#[derive(Debug)]
enum ThemeError {
    EmptyTheme,
}

impl warp::reject::Reject for ThemeError {}

// Define a response struct to return the result of the theme switch
#[derive(serde::Serialize)]
struct ThemeResponse {
    message: String,
}
