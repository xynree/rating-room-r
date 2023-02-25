use tauri::{command, State};

use crate::{
    db,
    errors::CommandResult,
    schema::{AppState, Category},
};

#[command]
#[allow(clippy::needless_pass_by_value)]
pub fn add_categories_to_item(
    state: State<AppState>,
    item_id: usize,
    categories: Vec<Category>,
) -> CommandResult<Vec<Category>> {
    let conn = state.db.conn.lock().unwrap();
    for category in categories {
        db::add_category_to_item(&conn, category.category_id, item_id)?;
    }
    db::get_categories(&conn)
}

#[command]
#[allow(clippy::needless_pass_by_value)]
pub fn get_categories(state: State<AppState>) -> CommandResult<Vec<Category>> {
    let conn = state.db.conn.lock().unwrap();
    db::get_categories(&conn)
}

#[command]
#[allow(clippy::needless_pass_by_value)]
pub fn get_categories_for_item(state: State<AppState>, id: usize) -> CommandResult<Vec<Category>> {
    let conn = state.db.conn.lock().unwrap();
    db::get_categories_for_item(&conn, id)
}

#[command]
#[allow(clippy::needless_pass_by_value)]
pub fn create_category(
    name: String,
    description: String,
    state: State<AppState>,
) -> CommandResult<usize> {
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
#[allow(clippy::needless_pass_by_value)]
pub fn delete_category(id: usize, state: State<AppState>) -> CommandResult<()> {
    let conn = state.db.conn.lock().unwrap();
    db::delete_category(&conn, id)
}
