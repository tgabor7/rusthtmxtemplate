use axum::{extract::State, response::Html, routing::get, Json, Router};
use diesel::prelude::*;
use dotenv::*;
use notes::models::*;
use notes::note::*;
use diesel::r2d2::*;

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
      .route("/query_from_db", get(query_from_db))
      .route("/get_notes", get(get_notes))
      .route("/", get(handler))
      .with_state(AppState { pool });

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> &'static str {
    "I'm alive!"
}

async fn handler() -> Html<String> {
    let file_path = "./src/public/index.html";

    let html = std::fs::read_to_string(file_path);

    match html {
        Ok(html) => {
          Html(html)
        },
        Err(_) => Html(String::from("Error reading file")),
    }
}

async fn query_from_db(State(state): State<AppState>) -> String {

    use notes::schema::posts;

    let pool = &state.pool;

    let mut connection = pool.get().expect("Couldn't get connection from pool");

    let results = posts::table
        .limit(5)
        .load::<Post>(&mut connection)
        .expect("Error loading posts");

    for post in results {
        println!("{:?}", post.title);
    }

    String::from("Query from db")
}

async fn get_notes(State(state): State<AppState>) -> Json<Vec<Note>> {
    use notes::schema::notes;

    let pool = &state.pool;

    let mut connection = pool.get().expect("Couldn't get connection from pool");

    let results = notes::table
        .limit(5)
        .load::<Note>(&mut connection);

    match results {
        Ok(results) => Json(results),
        Err(_) => Json(Vec::new()),
    }
}