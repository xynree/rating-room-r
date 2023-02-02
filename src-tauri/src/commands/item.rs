use rusqlite::params;
use tauri::{command, State};

use crate::{
    errors::CommandResult,
    filters::{get_filtered_items, Filter},
    schema::Item,
    AppState,
};

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
pub fn get_items(state: State<AppState>, filter: Option<Filter>) -> CommandResult<Vec<Item>> {
    let db = state.db.conn.lock().unwrap();
    let mut items = Vec::new();
    if let Some(filter) = filter {
        items = get_filtered_items(&db, filter).unwrap();
    } else {
        println!("No Filter!");
        let mut stmt = db.prepare("SELECT * FROM items")?;
        let rows = stmt.query_map([], |row| {
            Ok(Item {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2).unwrap_or(String::new()),
                comments: row.get(3).unwrap_or(String::new()),
                date: row.get(4)?,
            })
        })?;
        for item in rows {
            items.push(item?);
        }
    }

    Ok(items)
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
