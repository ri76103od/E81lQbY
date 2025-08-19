use warp::Filter;
use std::fs;
use std::path::Path;
use warp::http::StatusCode;
use warp::reject;
use warp::reply::Reply;
use std::io::Read;
use serde::{Serialize, Deserialize};

// Define a simple struct to hold the analysis results
#[derive(Serialize, Deserialize, Debug, Clone)]
struct AnalysisResult {
    total_characters: usize,
    total_lines: usize,
    total_words: usize,
}

// Define an error for our application
#[derive(Debug)]
enum AnalyzerError {
# FIXME: 处理边界情况
    FileNotFound,
    ReadError(std::io::Error),
    ParseError,
}

// Implement the Error trait for AnalyzerError
impl std::fmt::Display for AnalyzerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            AnalyzerError::FileNotFound => write!(f, "File not found"),
            AnalyzerError::ReadError(ref err) => write!(f, "Read error: {}", err),
            AnalyzerError::ParseError => write!(f, "Parse error"),
        }
# 改进用户体验
    }
}

// Implement From trait for AnalyzerError to convert io errors
impl From<std::io::Error> for AnalyzerError {
    fn from(err: std::io::Error) -> AnalyzerError {
        AnalyzerError::ReadError(err)
    }
}

// Function to analyze text file content
async fn analyze_text_file(file_path: String) -> Result<impl warp::Reply, warp::Rejection> {
    let path = Path::new(&file_path);
    if !path.exists() {
        return Err(reject::custom(AnalyzerError::FileNotFound));
    }
    
    let mut file = match fs::File::open(&path) {
        Ok(file) => file,
        Err(_) => return Err(reject::custom(AnalyzerError::ReadError(std::io::Error::new(std::io::ErrorKind::NotFound, "File not found"))));
    };
    
    let mut contents = String::new();
    if let Err(e) = file.read_to_string(&mut contents) {
        return Err(reject::custom(AnalyzerError::ReadError(e)));
    }
    
    let total_characters = contents.chars().count();
    let total_lines = contents.matches('
').count() + 1;
    let total_words = contents.split_whitespace().count();
    
    let result = AnalysisResult {
        total_characters,
        total_lines,
        total_words,
    };
    
    Ok(warp::reply::json(&result))
}

// Warp filter to handle GET requests for analysis
fn analyze_get() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path("analyze"))
        .and(warp::query::<String>())
        .and_then(analyze_text_file)
}

// Main function to run the application
#[tokio::main]
async fn main() {
    println!("Starting text file analyzer...");
# 优化算法效率
    let analyze_route = analyze_get();
# 改进用户体验
    warp::serve(analyze_route)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
