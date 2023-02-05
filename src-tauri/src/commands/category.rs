use tauri::{command, State};

use crate::{
    errors::{CommandError, CommandResult},
    schema::{AppState, Category},
};

#[command]
#[allow(clippy::needless_pass_by_value)]
pub fn get_categories(state: State<AppState>) -> CommandResult<Vec<Category>> {
    let db = state.db.conn.lock().unwrap();
    let mut categories = Vec::new();

    let mut stmt = db.prepare("SELECT * FROM categories")?;
    dbg!(&stmt);
    let rows = stmt.query_map([], |row| {
        Ok(Category {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2).unwrap_or(String::new()),
        })
    })?;
    for category in rows {
        categories.push(category?);
    }
    Ok(categories)
}

#[command]
#[allow(clippy::needless_pass_by_value)]
pub fn create_category(
    name: String,
    description: String,
    state: State<AppState>,
) -> CommandResult<()> {
    let db = state.db.conn.lock().unwrap();
    let mut stmt = db.prepare("INSERT INTO categories (name, description) VALUES ( ?, ? )")?;

    if let Err(e) = stmt.execute([name, description]) {
        return Err(CommandError::RusqliteError(e));
    };

    Ok(())
}

#[command]
#[allow(clippy::needless_pass_by_value)]
pub fn delete_category(id: usize, state: State<AppState>) -> CommandResult<()> {
    let db = state.db.conn.lock().unwrap();

    // let mut stmt = db.prepare("DELETE FROM items_to_categories WHERE category_id = ( SELECT id FROM categories WHERE id = ? )")?;
    // if let Err(e) = stmt.execute([id]) {
    //     return Err(CommandError::RusqliteError(e));
    // };

    let mut stmt = db.prepare("DELETE FROM categories WHERE id = ?")?;
    if let Err(e) = stmt.execute([id]) {
        return Err(CommandError::RusqliteError(e));
    };

    Ok(())
}
