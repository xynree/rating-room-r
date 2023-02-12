use rusqlite::params;
use tauri::{command, State};

use crate::{
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
    let db = state.db.conn.lock().unwrap();
    let rating = rating.unwrap_or_default();

    dbg!(&item);
    // insert item
    if let Err(e) = db.execute(
        "INSERT INTO items (name, description, comments) VALUES ( ?, ?, ? )",
        [item.name, item.description, item.comments],
    ) {
        return Err(CommandError::RusqliteError(e));
    };

    let item_id = db.last_insert_rowid();

    // insert rating
    if let Err(e) = db.execute(
        "INSERT INTO ratings (rating) VALUES ( ? )",
        params![rating.rating],
    ) {
        return Err(CommandError::RusqliteError(e));
    };

    let rating_id = db.last_insert_rowid();

    // link item with rating
    if let Err(e) = db.execute(
        "INSERT INTO items_to_ratings (item_id, rating_id) VALUES ( ?, ? )",
        params![item_id, rating_id],
    ) {
        return Err(CommandError::RusqliteError(e));
    };

    if let Some(category) = category {
        // insert category
        if let Err(e) = db.execute(
            "INSERT INTO categories (name, description) VALUES ( ? ? )",
            params![category.name, category.description],
        ) {
            return Err(CommandError::RusqliteError(e));
        };

        let category_id = db.last_insert_rowid();

        // link item with category
        if let Err(e) = db.execute(
            "INSERT INTO items_to_categories (item_id, category_id) VALUES ( ? ? )",
            params![item_id, category_id],
        ) {
            return Err(CommandError::RusqliteError(e));
        };
    }

    Ok(item_id.try_into().unwrap())
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
    let db = state.db.conn.lock().unwrap();
    let item = db.query_row(
        "SELECT * FROM items WHERE item_id = ? ORDER BY item_id ",
        params![id],
        |row| {
            Ok(Item {
                item_id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2).unwrap_or(String::new()),
                comments: row.get(3).unwrap_or(String::new()),
            })
        },
    )?;

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
