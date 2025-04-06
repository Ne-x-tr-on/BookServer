// Update the top imports
use axum::{
    routing::get,
    Router, Json, extract::State,
    response::{Html, IntoResponse},  // Fixed import
};
use serde::{Serialize, Deserialize};
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::Mutex;
use tower_http::{services::ServeDir, cors::CorsLayer};

// Rest of the code remains the same...

#[derive(Serialize, Deserialize, Clone)]
struct Book {
    id: u32,
    name: String,
    author: String,
    price: f64,
    pages: u32,
}

#[derive(Clone)]
struct AppState {
    books: Arc<Mutex<Vec<Book>>>,
}

#[tokio::main]
async fn main() {
    let initial_books = vec![
        Book {
            id: 1,
            name: "The Psychology of Money".into(),
            author: "Morgan Housel".into(),
            price: 15.99,
            pages: 256,
        },
        Book {
            id: 2,
            name: "Sapiens: A Brief History of Humankind".into(),
            author: "Yuval Noah Harari".into(),
            price: 22.50,
            pages: 512,
        },
    ];

    let state = AppState {
        books: Arc::new(Mutex::new(initial_books)),
    };

    let app = Router::new()
    .route("/books", get(get_books))
    .nest_service(
        "/", 
        ServeDir::new("static")
            .append_index_html_on_directories(true)
    )
    .fallback(get(serve_index))  // Keep this last
    .with_state(state)
    .layer(
        CorsLayer::new()
            .allow_origin(tower_http::cors::Any)
            .allow_methods([axum::http::Method::GET])
    );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_books(State(state): State<AppState>) -> Json<Vec<Book>> {
    let books = state.books.lock().await;
    Json(books.clone())
}

async fn serve_index() -> impl IntoResponse {
    match tokio::fs::read_to_string("static/index.html").await {
        Ok(content) => Html(content),
        Err(_) => Html("<h1>404 - Page Not Found</h1>".to_string()),
    }
}