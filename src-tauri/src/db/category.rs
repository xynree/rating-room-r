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

pub fn update_category(conn: &MutexGuard<Connection>, category: Category) -> CommandResult<usize> {
    let mut stmt =
        conn.prepare("UPDATE categories SET name = ?, description = ? WHERE category_id = ?")?;
    let id = stmt.execute([
        category.name,
        category.description,
        category.category_id.to_string(),
    ])?;
    Ok(id)
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

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use rusqlite::{params, Error};

    use crate::{
        db::{add_rating_to_item, create_item},
        schema::{create_tables, Item},
    };

    use super::*;

    fn dummy_connection() -> Mutex<Connection> {
        let conn = Connection::open_in_memory().unwrap();
        create_tables(&conn).unwrap();
        Mutex::new(conn)
    }

    #[test]
    fn creates_category() {
        let category = Category {
            category_id: 1,
            name: String::from("Snacks"),
            description: String::from("Tasty"),
        };

        let conn = dummy_connection();
        let conn = conn.lock().unwrap();

        create_category(
            &conn,
            category.name.to_owned(),
            category.description.to_owned(),
        )
        .unwrap();

        let result = conn
            .query_row(
                "SELECT * FROM categories WHERE name = ? ",
                [category.name.to_owned()],
                |row| {
                    Ok(Category {
                        category_id: row.get(0)?,
                        name: row.get(1)?,
                        description: row.get(2)?,
                    })
                },
            )
            .unwrap();
        assert_eq!(category, result);
    }

    #[test]
    fn gets_categories() {
        let categories = vec![
            Category {
                category_id: 1,
                name: String::from("Snacks"),
                description: String::from("Tasty"),
            },
            Category {
                category_id: 2,
                name: String::from("Movies"),
                description: String::from("Pretty"),
            },
            Category {
                category_id: 3,
                name: String::from("Art"),
                description: String::from("Colorful"),
            },
        ];

        let conn = dummy_connection();
        let conn = conn.lock().unwrap();

        for category in categories.clone() {
            create_category(
                &conn,
                category.name.to_owned(),
                category.description.to_owned(),
            )
            .unwrap();
        }

        let result = get_categories(&conn).unwrap();
        assert_eq!(categories, result);
    }

    #[test]
    fn updates_categories() {
        let category = Category {
            category_id: 1,
            name: String::from("Snacks"),
            description: String::from("Tasty"),
        };

        let conn = dummy_connection();
        let conn = conn.lock().unwrap();

        create_category(
            &conn,
            category.name.to_owned(),
            category.description.to_owned(),
        )
        .unwrap();

        let new_category = Category {
            name: String::from("Meals"),
            ..category
        };

        let new_category_id = update_category(&conn, new_category.clone()).unwrap();

        let result = conn
            .query_row(
                "SELECT * FROM categories WHERE category_id = ?",
                [new_category_id],
                |row| {
                    Ok(Category {
                        category_id: row.get(0)?,
                        name: row.get(1)?,
                        description: row.get(2)?,
                    })
                },
            )
            .unwrap();

        assert_eq!(new_category, result);
    }

    #[test]
    fn deletes_categories() {
        let category = Category {
            category_id: 1,
            name: String::from("Snacks"),
            description: String::from("Tasty"),
        };

        let conn = dummy_connection();
        let conn = conn.lock().unwrap();

        conn.execute(
            "INSERT INTO categories (category_id, name, description ) VALUES (?, ?, ? )",
            params![category.category_id, category.name, category.description],
        )
        .unwrap();

        let category_id = conn.last_insert_rowid() as usize;

        delete_category(&conn, category_id).unwrap();

        let result = conn
            .query_row(
                "SELECT * FROM categories WHERE category_id = ?",
                [category_id],
                |row| Ok(()),
            )
            .unwrap_err();

        assert_eq!(result, rusqlite::Error::QueryReturnedNoRows);
    }
}
