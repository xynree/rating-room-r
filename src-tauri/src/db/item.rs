use std::sync::MutexGuard;

use rusqlite::{params, Connection};

use crate::{
    errors::{CommandError, CommandResult},
    schema::Item,
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
                img_path: row.get(4).unwrap_or(String::new()),
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
            img_path: row.get(4).unwrap_or(String::new()),
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

pub fn delete_item(conn: &MutexGuard<Connection>, id: usize) -> CommandResult<()> {
    if let Err(e) = conn.execute("DELETE FROM items WHERE item_id = ?", [id]) {
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
        "INSERT INTO items_to_categories (item_id, category_id) VALUES ( ?, ? )",
        params![item_id, category_id],
    ) {
        return Err(CommandError::RusqliteError(e));
    };
    Ok(())
}

pub fn create_item(
    conn: &MutexGuard<Connection>,
    name: String,
    description: String,
    comments: String,
    img_path: String,
) -> CommandResult<usize> {
    if let Err(e) = conn.execute(
        "INSERT INTO items (name, description, comments, img_path) VALUES ( ?, ?, ?, ?)",
        [name, description, comments, img_path],
    ) {
        return Err(CommandError::RusqliteError(e));
    };

    Ok(conn.last_insert_rowid() as usize)
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use chrono::{NaiveDateTime, Utc};
    use rusqlite::types::FromSql;

    use crate::schema::{create_tables, Category, Rating};

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
            img_path: String::from("test"),
        };

        let conn = dummy_connection();
        let conn = conn.lock().unwrap();

        conn.execute(
            "INSERT INTO items (item_id, name, description, comments, img_path) VALUES (?, ?, ?, ?, ?)",
            params![item.item_id, item.name, item.description, item.comments, item.img_path],
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
            img_path: String::from("/test"),
        };

        let conn = dummy_connection();
        let conn = conn.lock().unwrap();

        let item_id = create_item(
            &conn,
            item.name.clone(),
            item.description.clone(),
            item.comments.clone(),
            item.img_path.clone(),
        )
        .unwrap();
        assert_eq!(item.clone(), get_item(&conn, item_id).unwrap());
    }

    #[test]
    fn gets_items() {
        let items = vec![
            Item {
                item_id: 1,
                name: String::from("Chips"),
                description: String::from("Crunchy"),
                comments: String::from("Love Them"),
                img_path: String::from("test"),
            },
            Item {
                item_id: 2,
                name: String::from("Airpods"),
                description: String::from("Bumpin"),
                comments: String::from("Loud in the ears"),
                img_path: String::from("test"),
            },
            Item {
                item_id: 3,
                name: String::from("Random"),
                description: String::from("Random"),
                comments: String::from("Random"),
                img_path: String::from("test"),
            },
            Item {
                item_id: 4,
                name: String::from("Bottle"),
                description: String::from("Water"),
                comments: String::from("Testing"),
                img_path: String::from("test"),
            },
        ];

        let conn = dummy_connection();
        let conn = conn.lock().unwrap();

        for item in items.clone() {
            conn.execute(
                "INSERT INTO items (item_id, name, description, comments, img_path) VALUES (?, ?, ?, ?, ?)",
                params![item.item_id, item.name, item.description, item.comments, item.img_path],
            )
            .unwrap();
        }

        assert_eq!(items, get_items(&conn).unwrap());
    }

    #[test]
    fn updates_item() {
        let item = Item {
            item_id: 1,
            name: String::from("Chips"),
            description: String::from("Crunchy"),
            comments: String::from("Love Them"),
            img_path: String::from("test"),
        };

        let conn = dummy_connection();
        let conn = conn.lock().unwrap();

        conn.execute(
            "INSERT INTO items (item_id, name, description, comments, img_path) VALUES (?, ?, ?, ?, ?)",
            params![item.item_id, item.name, item.description, item.comments, item.img_path],
        )
        .unwrap();

        let new_item = Item {
            name: String::from("Ranch Dressing"),
            description: String::from("Smooth"),
            ..item
        };

        update_item(&conn, new_item.clone()).unwrap();

        assert_eq!(new_item, get_item(&conn, item.item_id).unwrap());
    }

    #[test]
    fn deletes_item() {
        let item = Item {
            item_id: 1,
            name: String::from("Chips"),
            description: String::from("Crunchy"),
            comments: String::from("Love Them"),
            img_path: String::from("test"),
        };

        let conn = dummy_connection();
        let conn = conn.lock().unwrap();

        conn.execute(
            "INSERT INTO items (item_id, name, description, comments, img_path) VALUES (?, ?, ?, ?, ?)",
            params![item.item_id, item.name, item.description, item.comments, item.img_path],
        )
        .unwrap();

        let item_id = conn.last_insert_rowid() as usize;

        delete_item(&conn, item_id).unwrap();

        let result = conn
            .query_row("SELECT * FROM items WHERE item_id = ?", [item_id], |row| {
                Ok(Item {
                    item_id: row.get(0)?,
                    name: row.get(1)?,
                    description: row.get(2).unwrap_or(String::new()),
                    comments: row.get(3).unwrap_or(String::new()),
                    img_path: row.get(4).unwrap_or(String::new()),
                })
            })
            .unwrap_err();

        assert_eq!(result, rusqlite::Error::QueryReturnedNoRows);
    }

    #[test]
    fn adds_category_to_item() {
        let item = Item {
            item_id: 1,
            name: String::from("Chips"),
            description: String::from("Crunchy"),
            comments: String::from("Love Them"),
            img_path: String::from("test"),
        };

        let category = Category {
            category_id: 1,
            name: String::from("Snacks"),
            description: String::from("Crunchy Things"),
        };

        let conn = dummy_connection();
        let conn = conn.lock().unwrap();

        conn.execute(
            "INSERT INTO items (item_id, name, description, comments, img_path) VALUES (?, ?, ?, ?, ?)",
            params![item.item_id, item.name, item.description, item.comments, item.img_path],
        )
        .unwrap();

        conn.execute(
            "INSERT INTO categories (category_id, name, description) VALUES (?, ?, ?)",
            params![category.category_id, category.name, category.description],
        )
        .unwrap();

        add_category_to_item(&conn, item.item_id, category.category_id).unwrap();

        let result = conn
            .query_row(
                "SELECT * FROM items_to_categories WHERE item_id = ? AND category_id = ?",
                [item.item_id, category.category_id],
                |row| {
                    let item_id: usize = row.get(0)?;
                    let category_id: usize = row.get(1)?;
                    Ok((item_id, category_id))
                },
            )
            .unwrap();

        assert_eq!((item.item_id, category.category_id), result);
    }

    #[test]
    fn adds_rating_to_item() {
        let item = Item {
            item_id: 1,
            name: String::from("Chips"),
            description: String::from("Crunchy"),
            comments: String::from("Love Them"),
            img_path: String::from("test"),
        };

        let rating = Rating {
            rating_id: 1,
            rating: 3,
            date: Utc::now().naive_utc(),
        };

        let conn = dummy_connection();
        let conn = conn.lock().unwrap();

        conn.execute(
            "INSERT INTO items (item_id, name, description, comments, img_path) VALUES (?, ?, ?, ?, ?)",
            params![item.item_id, item.name, item.description, item.comments, item.img_path],
        )
        .unwrap();

        conn.execute(
            "INSERT INTO ratings (rating_id, rating, date) VALUES (?, ?, ?)",
            params![
                rating.rating_id,
                rating.rating.to_string(),
                rating.date.to_string()
            ],
        )
        .unwrap();

        add_rating_to_item(&conn, item.item_id, rating.rating_id).unwrap();

        let result = conn
            .query_row(
                "SELECT * FROM items_to_ratings WHERE item_id = ? AND rating_id = ?",
                [item.item_id, rating.rating_id],
                |row| {
                    let item_id: usize = row.get(0)?;
                    let rating_id: usize = row.get(1)?;
                    Ok((item_id, rating_id))
                },
            )
            .unwrap();

        assert_eq!((item.item_id, rating.rating_id), result);
    }
}
