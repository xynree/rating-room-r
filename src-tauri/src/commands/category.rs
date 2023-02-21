use tauri::{command, State};

use crate::{
    db,
    errors::CommandResult,
    schema::{AppState, Category},
};

#[command]
#[allow(clippy::needless_pass_by_value)]
pub fn get_categories(state: State<AppState>) -> CommandResult<Vec<Category>> {
    let conn = state.db.conn.lock().unwrap();
    db::get_categories(&conn)
}

#[command]
#[allow(clippy::needless_pass_by_value)]
pub fn create_category(
    name: String,
    description: String,
    state: State<AppState>,
) -> CommandResult<()> {
    let conn = state.db.conn.lock().unwrap();
    db::create_category(&conn, name, description)
}

#[command]
#[allow(clippy::needless_pass_by_value)]
pub fn update_category(category: Category, state: State<AppState>) -> CommandResult<usize> {
    let conn = state.db.conn.lock().unwrap();
    db::update_category(&conn, category)
}

#[command]
pub fn delete_category(id: usize, state: State<AppState>) -> CommandResult<()> {
    let conn = state.db.conn.lock().unwrap();
    db::delete_category(&conn, id)
}
