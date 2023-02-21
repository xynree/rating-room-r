use tauri::{command, State};

use crate::{
    db,
    errors::CommandResult,
    schema::{Category, Item, Rating},
    AppState,
};

#[command]
pub fn create_item(
    state: State<AppState>,
    item: Item,
    rating: Option<Rating>,
    category: Option<Category>,
) -> CommandResult<usize> {
    let conn = state.db.conn.lock().unwrap();
    db::create_item(&conn, item, rating, category)
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
pub fn update_item(state: State<AppState>, item: Item) -> CommandResult<usize> {
    let conn = state.db.conn.lock().unwrap();
    db::update_item(&conn, item)
}
