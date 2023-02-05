use tauri::{command, State};

use crate::{
    errors::CommandResult,
    schema::{AppState, Category, CategoryDetails},
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

pub fn create_category(details: CategoryDetails, state: State<AppState>) -> CommandResult<()> {
    let CategoryDetails { name, description } = details;
    let db = state.db.conn.lock().unwrap();
    let mut stmt = db.prepare("INSERT INTO Categories (name, description) VALUE ( ?, ? )")?;

    match stmt.execute([name, description]) {
        Ok(_) => Ok(()),
        Err(e) => Err(crate::errors::CommandError::Other(anyhow::anyhow!(
            "Error creating cateogry: {e}"
        ))),
    }
}

pub fn delete_category(details: CategoryDetails, state: State<AppState>) -> CommandResult<()> {
    let CategoryDetails { name, description } = details;
    let db = state.db.conn.lock().unwrap();
    let mut stmt = db.prepare("DELETE Categories (name, description) VALUE ( ?, ? )")?;

    match stmt.execute([name, description]) {
        Ok(_) => Ok(()),
        Err(e) => Err(crate::errors::CommandError::Other(anyhow::anyhow!(
            "Error creating cateogry: {e}"
        ))),
    }
}
