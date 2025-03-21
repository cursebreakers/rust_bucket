// main.rs

mod make_index;

use axum::{routing::{get, get_service}, Router};
use std::{net::SocketAddr, fs};
use std::env;
use tower_http::services::ServeDir;
use tokio::net::TcpListener;
use tracing::{info, error};
use tracing_subscriber;
use axum::response::{Html, IntoResponse};

// Serves files from the bucket folder
#[tokio::main]
async fn main() {
    info!("Initializing server console...");
    // Initialize console logging
    tracing_subscriber::fmt::init();

    info!("Setting address & port...");
    // Set the listening address and port
    let addr = SocketAddr::from(([0, 0, 0, 0], 1111));

    // Create the Axum router to serve static files from the "bucket" folder

    let app = Router::new()
    .route("/index", get(serve_index))
	.nest_service("/", get_service(ServeDir::new("bucket")));

    // Create a TCP listener that binds to the given address (addr)
    let listener = TcpListener::bind(addr).await.unwrap(); 

	// Log the listening address and port (addr)
    info!("Bucket created.");
    info!("Server listening @ http://{}", addr);
    info!("Bucket Index ready @ http://{}/index", addr);
    // Serve the application using Axum's `serve` function
    if let Err(e) = axum::serve(listener, app.into_make_service()).await {
        // If the server encounters an error, log the error message.
        error!("Server error: {}", e);
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


