use axum::{http::StatusCode, response::{Html, IntoResponse}, routing::{get, post}, Router};
use diesel::prelude::*;
use dotenv::*;
use diesel::r2d2::*;
use routes::notes::notes::{create_note, get_notes};

mod routes;

#[derive(Clone)]
struct AppState {
    pool: Pool<ConnectionManager<PgConnection>>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let databse_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // connect to the database
    let manager = ConnectionManager::<PgConnection>::new(databse_url);

    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    let app = Router::new()
      .route("/health_check", get(health_check))
      .route("/notes", get(get_notes).post(create_note))
      .fallback(handle_404)
      .with_state(AppState { pool });

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, Html("I'm alive uwu".to_string()))
}

async fn handle_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, Html("Page not found".to_string()))
}