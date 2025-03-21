// main.rs

mod make_index;

use axum::{routing::{get, get_service}, Router};
use std::{net::SocketAddr, fs};
use std::env;
use std::path::{Path};
use tower_http::services::ServeDir;
use axum::response::{Html, IntoResponse};
use tokio::net::TcpListener;
use tracing::{info, error};
use tracing_subscriber;


const DEFAULT_PORT: u16 = 1111;

// Serves files from the bucket folder
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    info!("Initializing server console...");

    let port = env::var("PORT").unwrap_or_else(|_| DEFAULT_PORT.to_string()).parse::<u16>().unwrap_or(DEFAULT_PORT);
	println!();
	println!("Setting address & port...");
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

	println!("Configuring routes...");
    let app = Router::new()
	    .route("/index", get(serve_index))
		.nest_service("/", get_service(ServeDir::new("bucket")));

    println!("Creating bucket...");
	println!();
    let bucket_path = env::current_dir().unwrap().join("bucket");
    print_tree(&bucket_path, "", &bucket_path); 
	println!();
    println!("Bucket created.");
	println!();
    // Create a TCP listener that binds to the given address (addr)
    let listener = TcpListener::bind(addr).await.unwrap(); 
    // Serve the application using Axum's `serve` function, logging any errors
    info!("Server (Axum) listening @ http://{}", addr);
    info!("Index ready @ http://{}/index", addr);
    if let Err(e) = axum::serve(listener, app.into_make_service()).await {
        error!("Server error: {}", e);
    }
}

// Prints a directory tree of the bucket to console
fn print_tree(path: &Path, prefix: &str, base: &Path) {
    if let Ok(entries) = fs::read_dir(path) {
        let mut entries: Vec<_> = entries.filter_map(Result::ok).collect();
        entries.sort_by_key(|e| e.file_name()); // Sort alphabetically

        let count = entries.len();
        for (i, entry) in entries.iter().enumerate() {
            let entry_path = entry.path();
            let relative_path = entry_path.strip_prefix(base).unwrap(); // Get path relative to "bucket"
            let is_last = i == count - 1;

            let connector = if is_last { "└── " } else { "├── " };
            println!("{}{}{}", prefix, connector, relative_path.display());

            if entry_path.is_dir() {
                let new_prefix = if is_last { format!("{}    ", prefix) } else { format!("{}│   ", prefix) };
                print_tree(&entry_path, &new_prefix, base);
            }
        }
    }
}

// Creates, updates ad serves an index.html by reading the files in bucket
async fn serve_index() -> impl IntoResponse {
    let bucket_path = env::current_dir().unwrap().join("bucket");

    // Regenerate the index.html if necessary
    if let Err(e) = make_index::generate_index_html(bucket_path.to_str().unwrap()) {
        error!("Error generating index.html: {}", e);
        return Html("<h1>Error generating file list</h1>").into_response();
    }

    // Serve the generated index.html
    let index_html_path = bucket_path.join("index.html");
    if let Ok(contents) = fs::read_to_string(index_html_path) {
        Html(contents).into_response()
    } else {
        Html("<h1>Error: index.html not found</h1>").into_response()
    }
}


