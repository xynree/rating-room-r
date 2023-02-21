use tauri::{command, State};

use crate::{
    db,
    errors::CommandResult,
    schema::{AppState, Category},
};

#[command]
pub fn get_categories(state: State<AppState>) -> CommandResult<Vec<Category>> {
    let conn = state.db.conn.lock().unwrap();
    db::get_categories(&conn)
}

#[command]
pub fn create_category(
    name: String,
    description: String,
    state: State<AppState>,
) -> CommandResult<()> {
    let conn = state.db.conn.lock().unwrap();
    db::create_category(&conn, name, description)
}

#[command]
pub fn delete_category(id: usize, state: State<AppState>) -> CommandResult<()> {
    let conn = state.db.conn.lock().unwrap();
    db::delete_category(&conn, id)
}
