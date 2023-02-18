use rusqlite::params;
use tauri::{command, State};

use crate::{
    db,
    errors::{CommandError, CommandResult},
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
    let item_id = db::create_item(&conn, item, rating, category)?;
    Ok(item_id)
}

#[command]
#[allow(clippy::needless_pass_by_value)]
pub fn get_items(state: State<AppState>) -> CommandResult<Vec<Item>> {
    let db = state.db.conn.lock().unwrap();
    let mut items = Vec::new();
    let mut stmt = db.prepare("SELECT * FROM items")?;
    let rows = stmt.query_map([], |row| {
        Ok(Item {
            item_id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2).unwrap_or(String::new()),
            comments: row.get(3).unwrap_or(String::new()),
        })
    })?;
    for item in rows {
        items.push(item?);
    }
    Ok(items)
}

#[command]
#[allow(clippy::needless_pass_by_value)]
pub fn get_item(state: State<AppState>, id: usize) -> CommandResult<Item> {
    let conn = state.db.conn.lock().unwrap();
    let item = db::get_item(&conn, id)?;
    Ok(item)
}

#[command]
#[allow(clippy::needless_pass_by_value)]
pub fn update_item(state: State<AppState>, item: Item) -> CommandResult<usize> {
    let db = state.db.conn.lock().unwrap();
    let mut stmt =
        db.prepare("UPDATE items SET name = ?, description = ?, comments = ? WHERE item_id = ?")?;
    let id = stmt.execute([
        item.name,
        item.description,
        item.comments,
        item.item_id.to_string(),
    ])?;

    Ok(id)
}
