use tauri::{command, State};

use crate::{db, errors::CommandResult, schema::Rating, AppState};

#[command]
pub fn delete_rating(state: State<AppState>, rating_id: usize) -> CommandResult<()> {
    let conn = state.db.conn.lock().unwrap();
    db::delete_rating(&conn, rating_id)
}

#[command]
pub fn get_ratings(state: State<AppState>, item_id: usize) -> CommandResult<Vec<Rating>> {
    let conn = state.db.conn.lock().unwrap();
    db::get_ratings(&conn, item_id)
}

#[command]
pub fn create_rating(state: State<AppState>, rating: usize) -> CommandResult<i64> {
    let conn = state.db.conn.lock().unwrap();
    db::create_rating(&conn, rating)
}

#[command]
pub fn update_rating(
    state: State<AppState>,
    rating: usize,
    rating_id: usize,
) -> CommandResult<usize> {
    let conn = state.db.conn.lock().unwrap();
    db::update_rating(&conn, rating, rating_id)
}
