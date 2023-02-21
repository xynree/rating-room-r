use std::sync::MutexGuard;

use rusqlite::{params, Connection};

use crate::{
    db::create_rating,
    errors::{CommandError, CommandResult},
    schema::{Category, Item, Rating},
};

pub fn get_item(conn: &MutexGuard<Connection>, id: usize) -> CommandResult<Item> {
    let item = conn.query_row(
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

pub fn get_items(conn: &MutexGuard<Connection>) -> CommandResult<Vec<Item>> {
    let mut items = Vec::new();
    let mut stmt = conn.prepare("SELECT * FROM items")?;
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

pub fn update_item(conn: &MutexGuard<Connection>, item: Item) -> CommandResult<usize> {
    let mut stmt =
        conn.prepare("UPDATE items SET name = ?, description = ?, comments = ? WHERE item_id = ?")?;
    let id = stmt.execute([
        item.name,
        item.description,
        item.comments,
        item.item_id.to_string(),
    ])?;
    Ok(id)
}

pub fn add_rating_to_item(
    conn: &MutexGuard<Connection>,
    rating_id: usize,
    item_id: usize,
) -> CommandResult<()> {
    if let Err(e) = conn.execute(
        "INSERT INTO items_to_ratings (item_id, rating_id) VALUES ( ?, ? )",
        params![item_id, rating_id],
    ) {
        return Err(CommandError::RusqliteError(e));
    };
    Ok(())
}

pub fn add_category_to_item(
    conn: &MutexGuard<Connection>,
    category_id: usize,
    item_id: usize,
) -> CommandResult<()> {
    if let Err(e) = conn.execute(
        "INSERT INTO items_to_categories (item_id, category_id) VALUES ( ? ? )",
        params![item_id, category_id],
    ) {
        return Err(CommandError::RusqliteError(e));
    };
    Ok(())
}

pub fn create_item(conn: &MutexGuard<Connection>, item: Item) -> CommandResult<usize> {
    if let Err(e) = conn.execute(
        "INSERT INTO items (name, description, comments) VALUES ( ?, ?, ? )",
        [item.name, item.description, item.comments],
    ) {
        return Err(CommandError::RusqliteError(e));
    };

    Ok(conn.last_insert_rowid() as usize)
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use crate::schema::create_tables;

    use super::*;

    fn dummy_connection() -> Mutex<Connection> {
        let conn = Connection::open_in_memory().unwrap();
        create_tables(&conn).unwrap();
        Mutex::new(conn)
    }

    #[test]
    fn gets_item() {
        let item = Item {
            item_id: 1,
            name: String::from("Chips"),
            description: String::from("Crunchy"),
            comments: String::from("Love Them"),
        };

        let conn = dummy_connection();
        let conn = conn.lock().unwrap();

        conn.execute(
            "INSERT INTO items (item_id, name, description, comments) VALUES (?, ?, ?, ?)",
            params![item.item_id, item.name, item.description, item.comments],
        )
        .unwrap();

        assert_eq!(item, get_item(&conn, 1).unwrap());
    }

    #[test]
    fn creates_item() {
        let item = Item {
            item_id: 1,
            name: String::from("Chips"),
            description: String::from("Crunchy"),
            comments: String::from("Love Them"),
        };

        let conn = dummy_connection();
        let conn = conn.lock().unwrap();

        let item_id = create_item(&conn, item.clone()).unwrap();
        assert_eq!(item, get_item(&conn, item_id).unwrap());
    }
}
