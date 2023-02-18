use std::sync::MutexGuard;

use rusqlite::{params, Connection};

use crate::{
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

pub fn create_item(
    conn: &MutexGuard<Connection>,
    item: Item,
    rating: Option<Rating>,
    category: Option<Category>,
) -> CommandResult<usize> {
    let rating = rating.unwrap_or_default();

    // insert item
    if let Err(e) = conn.execute(
        "INSERT INTO items (name, description, comments) VALUES ( ?, ?, ? )",
        [item.name, item.description, item.comments],
    ) {
        return Err(CommandError::RusqliteError(e));
    };

    let item_id = conn.last_insert_rowid();

    // insert rating
    if let Err(e) = conn.execute(
        "INSERT INTO ratings (rating) VALUES ( ? )",
        params![rating.rating],
    ) {
        return Err(CommandError::RusqliteError(e));
    };

    let rating_id = conn.last_insert_rowid();

    // link item with rating
    if let Err(e) = conn.execute(
        "INSERT INTO items_to_ratings (item_id, rating_id) VALUES ( ?, ? )",
        params![item_id, rating_id],
    ) {
        return Err(CommandError::RusqliteError(e));
    };

    if let Some(category) = category {
        // insert category
        if let Err(e) = conn.execute(
            "INSERT INTO categories (name, description) VALUES ( ? ? )",
            params![category.name, category.description],
        ) {
            return Err(CommandError::RusqliteError(e));
        };

        let category_id = conn.last_insert_rowid();

        // link item with category
        if let Err(e) = conn.execute(
            "INSERT INTO items_to_categories (item_id, category_id) VALUES ( ? ? )",
            params![item_id, category_id],
        ) {
            return Err(CommandError::RusqliteError(e));
        };
    }

    Ok(item_id.try_into().unwrap())
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

        conn.execute(
            "INSERT INTO items (item_id, name, description, comments) VALUES (?, ?, ?, ?)",
            params![item.item_id, item.name, item.description, item.comments],
        )
        .unwrap();

        assert_eq!(item, get_item(&conn, 1).unwrap());
    }
}
