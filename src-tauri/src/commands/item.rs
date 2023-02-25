use tauri::{command, State};

use crate::{
    db,
    errors::CommandResult,
    schema::{Category, Item, Rating},
    AppState,
};

#[command]
#[allow(clippy::needless_pass_by_value)]
pub fn create_item(
    state: State<AppState>,
    name: String,
    description: String,
    comments: String,
    img_path: String,
) -> CommandResult<usize> {
    let conn = state.db.conn.lock().unwrap();
    db::create_item(&conn, name, description, comments, img_path)
}

#[command]
#[allow(clippy::needless_pass_by_value)]
pub fn delete_item(state: State<AppState>, id: usize) -> CommandResult<()> {
    let conn = state.db.conn.lock().unwrap();
    db::delete_item(&conn, id)
}

#[command]
#[allow(clippy::needless_pass_by_value)]
pub fn get_items(state: State<AppState>) -> CommandResult<Vec<Item>> {
    let conn = state.db.conn.lock().unwrap();
    db::get_items(&conn)
}

#[command]
#[allow(clippy::needless_pass_by_value)]
pub fn get_item(state: State<AppState>, id: usize) -> CommandResult<Item> {
    let conn = state.db.conn.lock().unwrap();
    db::get_item(&conn, id)
}

#[command]
#[allow(clippy::needless_pass_by_value)]
pub fn update_item(
    state: State<AppState>,
    item: Item,
    categories: Vec<Category>,
) -> CommandResult<()> {
    let conn = state.db.conn.lock().unwrap();
    db::update_item(&conn, item.clone())?;
    db::update_items_categories(&conn, item.item_id, &categories)
}
