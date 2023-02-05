use rusqlite::params;
use tauri::{command, State};

use crate::{
    errors::CommandResult,
    filters::{get_filtered_items, Filter},
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
    let db = state.db.conn.lock().unwrap();
    let rating = rating.unwrap_or_default();

    dbg!(&item);
    // insert item
    match db.execute(
        "INSERT INTO items (name, description, comments) VALUES ( ?, ?, ? )",
        [item.name, item.description, item.comments],
    ) {
        Err(e) => {
            return Err(crate::errors::CommandError::Other(anyhow::anyhow!(
                "Error inserting row into items: {e}"
            )))
        }
        _ => {}
    };

    let item_id = db.last_insert_rowid();

    // insert rating
    match db.execute(
        "INSERT INTO ratings (rating) VALUES ( ? )",
        params![rating.rating],
    ) {
        Err(e) => {
            return Err(crate::errors::CommandError::Other(anyhow::anyhow!(
                "Error inserting row into ratings: {e}"
            )))
        }
        _ => {}
    };

    let rating_id = db.last_insert_rowid();

    // link item with rating
    match db.execute(
        "INSERT INTO items_to_ratings (item_id, rating_id) VALUES ( ?, ? )",
        params![item_id, rating_id],
    ) {
        Err(e) => {
            return Err(crate::errors::CommandError::Other(anyhow::anyhow!(
                "Error inserting row into items_to_ratings: {e}"
            )))
        }
        _ => {}
    };

    if let Some(category) = category {
        // insert category
        match db.execute(
            "INSERT INTO categories (name, description) VALUES ( ? ? )",
            params![category.name, category.description],
        ) {
            Err(e) => {
                return Err(crate::errors::CommandError::Other(anyhow::anyhow!(
                    "Error inserting row into categories: {e}"
                )))
            }
            _ => {}
        };

        let category_id = db.last_insert_rowid();

        // link item with category
        match db.execute(
            "INSERT INTO items_to_categories (item_id, category_id) VALUES ( ? ? )",
            params![item_id, category_id],
        ) {
            Err(e) => {
                return Err(crate::errors::CommandError::Other(anyhow::anyhow!(
                    "Error inserting row into items_to_categories: {e}"
                )))
            }
            _ => {}
        };
    }

    Ok(item_id.try_into().unwrap())
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
    let item = db.query_row("SELECT * FROM items WHERE id = ?", params![id], |row| {
        Ok(Item {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2).unwrap_or(String::new()),
            comments: row.get(3).unwrap_or(String::new()),
        })
    })?;

    Ok(item)
}
