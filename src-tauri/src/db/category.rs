use std::sync::MutexGuard;

use rusqlite::Connection;

use crate::{
    errors::{CommandError, CommandResult},
    schema::Category,
};

pub fn get_categories(conn: &MutexGuard<Connection>) -> CommandResult<Vec<Category>> {
    let mut categories = Vec::new();
    let mut stmt = conn.prepare("SELECT * FROM categories")?;
    let rows = stmt.query_map([], |row| {
        Ok(Category {
            category_id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2).unwrap_or(String::new()),
        })
    })?;
    for category in rows {
        categories.push(category?);
    }
    Ok(categories)
}

pub fn create_category(
    conn: &MutexGuard<Connection>,
    name: String,
    description: String,
) -> CommandResult<()> {
    let mut stmt = conn.prepare("INSERT INTO categories (name, description) VALUES ( ?, ? )")?;

    if let Err(e) = stmt.execute([name, description]) {
        return Err(CommandError::RusqliteError(e));
    };
    Ok(())
}

pub fn delete_category(conn: &MutexGuard<Connection>, id: usize) -> CommandResult<()> {
    let mut stmt = conn.prepare("DELETE FROM categories WHERE category_id = ?")?;
    if let Err(e) = stmt.execute([id]) {
        return Err(CommandError::RusqliteError(e));
    };

    Ok(())
}
