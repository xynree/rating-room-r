use tauri::{command, State};

use crate::{
    errors::CommandResult,
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

    match stmt.execute([name, description]) {
        Ok(_) => Ok(()),
        Err(e) => Err(crate::errors::CommandError::Other(anyhow::anyhow!(
            "Error creating cateogry: {e}"
        ))),
    }
}

#[command]
#[allow(clippy::needless_pass_by_value)]
pub fn delete_category(id: usize, state: State<AppState>) -> CommandResult<()> {
    let db = state.db.conn.lock().unwrap();
    let mut stmt = db.prepare("DELETE FROM categories WHERE id = ?")?;

    match stmt.execute([id]) {
        Ok(_) => Ok(()),
        Err(e) => Err(crate::errors::CommandError::Other(anyhow::anyhow!(
            "Error creating cateogry: {e}"
        ))),
    }
}
