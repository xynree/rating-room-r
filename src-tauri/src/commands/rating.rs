use tauri::{command, State};

use crate::{db, errors::CommandResult, schema::Rating, AppState};

#[command]
#[allow(clippy::needless_pass_by_value)]
pub fn delete_rating(state: State<AppState>, rating_id: usize) -> CommandResult<()> {
    let conn = state.db.conn.lock().unwrap();
    db::delete_rating(&conn, rating_id)
}

#[command]
#[allow(clippy::needless_pass_by_value)]
pub fn get_ratings(state: State<AppState>, item_id: usize) -> CommandResult<Vec<Rating>> {
    let conn = state.db.conn.lock().unwrap();
    db::get_ratings(&conn, item_id)
}

#[command]
#[allow(
    clippy::needless_pass_by_value,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
pub fn create_rating(state: State<AppState>, rating: usize, item_id: usize) -> CommandResult<()> {
    let conn = state.db.conn.lock().unwrap();
    let rating_id = db::create_rating(&conn, rating)?;
    db::add_rating_to_item(&conn, rating_id as usize, item_id)
}

#[command]
#[allow(clippy::needless_pass_by_value)]
pub fn update_rating(
    state: State<AppState>,
    rating: usize,
    rating_id: usize,
) -> CommandResult<usize> {
    let conn = state.db.conn.lock().unwrap();
    db::update_rating(&conn, rating, rating_id)
}
