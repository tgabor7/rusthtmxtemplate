use axum::extract::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use notes::note::NewNote;
use ::notes::note::Note;
use crate::AppState;
use diesel::prelude::*;

pub async fn get_notes(State(state): State<AppState>) -> impl IntoResponse {
    use notes::schema::notes::table;

    let pool = &state.pool;

    let mut connection = pool.get()
      .expect("Couldn't get connection from pool");

    let results = table
        .limit(5)
        .load::<Note>(&mut connection);

    match results {
        Ok(results) => (StatusCode::OK, Json(results)),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(Vec::new())),
    }
}

pub async fn create_note(State(state): State<AppState>, Json(note): Json<NewNote>) -> impl IntoResponse {
    use notes::schema::notes::table;

    let pool = &state.pool;

    let mut connection = pool.get()
      .expect("Couldn't get connection from pool");

    let result = diesel::insert_into(table)
        .values(&note)
        .get_result::<Note>(&mut connection);

    match result {
        Ok(result) => (StatusCode::CREATED, Json(result)),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(Note {
            id: 0,
            title: "".to_string(),
            body: "".to_string(),
            created_at: chrono::NaiveDateTime::from_timestamp(0, 0),
        })),
    }
}