use tauri::State;

use crate::{db, errors::CommandResult, schema::Rating, AppState};

pub fn delete_rating(state: State<AppState>, rating_id: usize) -> CommandResult<()> {
    let conn = state.db.conn.lock().unwrap();
    db::delete_rating(&conn, rating_id)
}

pub fn get_ratings(state: State<AppState>, item_id: usize) -> CommandResult<Vec<Rating>> {
    let conn = state.db.conn.lock().unwrap();
    db::get_ratings(&conn, item_id)
}
