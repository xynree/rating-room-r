use tauri::State;

use crate::{
    errors::CommandResult,
    filters::{get_filtered_items, Filter},
    schema::Item,
    AppState,
};
/// Tauri command exposed to our front-end javascript.
///
/// Returns a Result either containing all `Item`'s found in database
/// or a `CommandError`.
///
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
#[tauri::command]
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
