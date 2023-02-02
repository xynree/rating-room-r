use rusqlite::params;
use tauri::{command, State};

use crate::{errors::CommandResult, schema::Item, AppState};

#[tauri::command]
#[allow(clippy::needless_pass_by_value)]
pub fn get_items(state: State<AppState>) -> CommandResult<Vec<Item>> {
    unimplemented!()
}

/// Wraps a basic `SELECT * FROM items` query.
///
/// # Examples
/// ```javascript
///     let items: Item[] = [];
///     async function get_items() {
///         items = await invoke("get_items");
///     };
/// ```
///
#[command]
#[allow(clippy::needless_pass_by_value)]
pub fn get_item(state: State<AppState>, id: usize) -> CommandResult<Item> {
    let db = state.db.conn.lock().unwrap();
    let item = db.query_row(
        "SELECT * FROM items WHERE items.id = ?",
        params![id],
        |row| {
            Ok(Item {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2).unwrap_or(String::new()),
                comments: row.get(3).unwrap_or(String::new()),
                date: row.get(4)?,
            })
        },
    )?;

    Ok(item)
}
