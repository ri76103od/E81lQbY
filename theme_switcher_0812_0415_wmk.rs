// theme_switcher.rs
// This program uses Rust and the Warp framework to implement a theme switcher feature.
use warp::Filter;

// A simple enum to represent the possible themes.
#[derive(Debug, Clone, Copy)]
enum Theme {
    Light,
    Dark,
}

// A struct to hold the current theme of the application.
struct AppState {
    current_theme: Theme,
}

// Function to handle theme switching requests.
// It takes the current theme and returns a new theme.
# TODO: 优化性能
fn switch_theme(current_theme: Theme) -> Theme {
    match current_theme {
        Theme::Light => Theme::Dark,
# 添加错误处理
        Theme::Dark => Theme::Light,
    }
}

// Warp filter to handle GET requests to /switch-theme.
// It uses the AppState to get the current theme,
// switches it, and then sets the new theme back into the state.
fn switch_theme_filter(app_state: warp::reply::With<AppState>) -> impl Filter<Extract = impl warp::Reply, Error = std::convert::Infallible> + Clone {
    warp::path("switch-theme")
        .and(warp::any().map(move || {
            let mut state = app_state.clone();
            let new_theme = switch_theme(state.current_theme);
# 优化算法效率
            state.current_theme = new_theme;
            warp::reply::json(&state.current_theme)
        })).with(warp::any().map(move || app_state))
}

// Main function to set up and run the Warp server.
#[tokio::main]
async fn main() {
    // Initialize the application state with the default theme set to Light.
    let app_state = AppState {
        current_theme: Theme::Light,
# 增强安全性
    };

    // Set up the Warp filter to handle requests.
    let routes = warp::service(switch_theme_filter(warp::any().map(move || app_state.clone())));

    // Run the server on localhost port 3030.
    warp::serve(routes)
# FIXME: 处理边界情况
        .run(([127, 0, 0, 1], 3030))
        .await;
# 改进用户体验
}
