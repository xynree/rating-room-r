use std::sync::MutexGuard;

use rusqlite::{params, params_from_iter, Connection};
use tauri::utils::config::parse::folder_has_configuration_file;

use crate::{
    errors::{CommandError, CommandResult},
    schema::{Category, FullItem, Item, Rating},
};

pub fn get_item(conn: &MutexGuard<Connection>, id: usize) -> CommandResult<Item> {
    let item = conn.query_row(
        "SELECT * FROM items WHERE item_id = ? ORDER BY item_id ",
        params![id],
        |row| {
            Ok(Item {
                item_id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2).unwrap_or_default(),
                comments: row.get(3).unwrap_or_default(),
                img_path: row.get(4).unwrap_or_default(),
            })
        },
    )?;

    Ok(item)
}

fn repeat_vars(count: usize) -> String {
    assert_ne!(count, 0);
    let mut s = "?,".repeat(count);
    // Remove trailing comma
    s.pop();
    s
}

pub fn filter_by_category(
    conn: &MutexGuard<Connection>,
    cats: Vec<String>,
) -> CommandResult<Vec<Item>> {
    let vars = repeat_vars(cats.len());
    let sql = format!(
        r#"select i.*, c.*
from items i
join items_to_categories itc on (i.item_id = itc.item_id)
join categories c
on itc.category_id = c.category_id
and c.name in ({})
group by i.item_id;
"#,
        vars
    );

    let mut items = Vec::new();
    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt
        .query_map(params_from_iter(cats), |row| {
            Ok(Item {
                item_id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                comments: row.get(3)?,
                img_path: row.get(4)?,
            })
        })
        .unwrap();

    for row in rows {
        items.push(row?);
    }

    Ok(items)
}

pub fn filter_by_rating(
    conn: &MutexGuard<Connection>,
    range: Vec<usize>,
) -> CommandResult<Vec<Item>> {
    let vars = repeat_vars(range.len());
    let sql = format!(
        r#"
        select i.*, r.*, max(r.date)
        from items i
        join items_to_ratings itr on (i.item_id = itr.item_id)
        left outer join ratings r 
        on itr.rating_id = r.rating_id
        where r.rating in ({})
        group by i.item_id;"#,
        vars
    );

    let mut items = Vec::new();
    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt
        .query_map(params_from_iter(range), |row| {
            Ok(Item {
                item_id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                comments: row.get(3)?,
                img_path: row.get(4)?,
            })
        })
        .unwrap();

    for row in rows {
        items.push(row?);
    }

    Ok(items)
}

pub fn get_items(conn: &MutexGuard<Connection>) -> CommandResult<Vec<FullItem>> {
    let mut items = Vec::new();
    let mut stmt = conn.prepare("SELECT i.*, c.*, r.* FROM items i JOIN items_to_ratings itr ON itr.item_id = i.item_id JOIN ratings r ON itr.rating_id = r.rating_id JOIN items_to_categories itc ON itc.item_id = i.item_id JOIN categories c on itc.category_id = c.category_id ")?;
    let rows = stmt.query_map([], |row| {
        let item = FullItem {
            item_id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2).unwrap_or_default(),
            comments: row.get(3).unwrap_or_default(),
            img_path: row.get(4).unwrap_or_default(),
            date: chrono::NaiveDateTime::parse_from_str(
                row.get::<usize, String>(5)?.as_str(),
                "%Y-%m-%d %H:%M:%S",
            )
            .map_err(|e| {
                rusqlite::Error::FromSqlConversionFailure(
                    0,
                    rusqlite::types::Type::Text,
                    Box::new(e),
                )
            })?,
            categories: vec![Category {
                category_id: row.get(6)?,
                name: row.get(7)?,
                description: row.get(8).unwrap_or_default(),
            }],
            rating: Rating {
                rating_id: row.get(9)?,
                rating: row.get(10)?,
                date: chrono::NaiveDateTime::parse_from_str(
                    row.get::<usize, String>(11)?.as_str(),
                    "%Y-%m-%d %H:%M:%S",
                )
                .map_err(|e| {
                    rusqlite::Error::FromSqlConversionFailure(
                        0,
                        rusqlite::types::Type::Text,
                        Box::new(e),
                    )
                })?,
            },
        };

        Ok(item)
    })?;

    for item in rows {
        let item = item?;
        match items
            .iter()
            .position(|i: &FullItem| i.item_id == item.item_id)
        {
            Some(i) => {
                if !items[i].categories.contains(&item.categories[0]) {
                    items[i].categories.push(item.clone().categories[0].clone());
                }
            }
            None => items.push(item.clone()),
        }
    }

    Ok(items)
}

pub fn update_item(conn: &MutexGuard<Connection>, item: Item) -> CommandResult<usize> {
    let mut stmt = conn.prepare(
        "UPDATE items SET name = ?, description = ?, comments = ?, img_path = ? WHERE item_id = ?",
    )?;
    let id = stmt.execute([
        item.name,
        item.description,
        item.comments,
        item.img_path,
        item.item_id.to_string(),
    ])?;
    Ok(id)
}

pub fn update_items_categories(
    conn: &mut MutexGuard<Connection>,
    item_id: usize,
    categories: &Vec<Category>,
) -> CommandResult<()> {
    dbg!(categories, item_id);
    let mut tx = conn.transaction()?;
    tx.execute(
        "DELETE from items_to_categories where item_id = ?",
        [item_id],
    )?;
    let mut vars = format!("({}, ?),", item_id).repeat(categories.len());
    vars.pop();

    let sql = format!(
        "INSERT into items_to_categories (item_id, category_id) values {} ",
        vars
    );

    {
        let sp = tx.savepoint()?;
        let cat_ids: Vec<_> = categories.iter().map(|cat| cat.category_id).collect();
        {
            let mut stmt = sp.prepare_cached(sql.as_str())?;
            stmt.execute(params_from_iter(cat_ids))?;
        }
        sp.commit()?;
    }

    tx.execute("DELETE FROM categories WHERE category_id NOT IN (select category_id from items_to_categories)",[])?;
    tx.commit()?;

    Ok(())
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

pub fn delete_item(mut conn: MutexGuard<Connection>, id: usize) -> CommandResult<()> {
    let tx = conn.transaction()?;
    if let Err(e) = tx.execute("DELETE FROM items WHERE item_id = ?", [id]) {
        return Err(CommandError::RusqliteError(e));
    };
    // Delete any 'hanging' categories
    tx.execute("DELETE FROM categories WHERE category_id NOT IN (select category_id from items_to_categories)",[])?;
    tx.commit()?;
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

#[allow(
    clippy::needless_pass_by_value,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
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

    use chrono::Utc;

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

    #[allow(
        clippy::needless_pass_by_value,
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss
    )]
    #[test]
    fn filters_by_category() {
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

        let cats: Vec<String> = vec![
            String::from("Food"),
            String::from("Exercise"),
            String::from("Appliance"),
            String::from("Random"),
        ];

        let conn = dummy_connection();
        let conn = conn.lock().unwrap();

        let items_cat: Vec<(Item, String)> = items
            .clone()
            .into_iter()
            .zip(cats.iter().cloned())
            .collect();

        for (item, cat) in items_cat {
            conn.execute(
            "INSERT INTO items (item_id, name, description, comments, img_path) VALUES (?, ?, ?, ?, ?)",
            params![item.item_id, item.name, item.description, item.comments, item.img_path],
        )
        .unwrap();
            conn.execute("INSERT INTO categories (name) VALUES (?)", params![cat])
                .unwrap();

            let category_id = conn.last_insert_rowid();

            conn.execute(
                "INSERT INTO items_to_categories (item_id, category_id) VALUES (?, ?)",
                params![item.item_id, category_id as usize],
            )
            .unwrap();
        }

        let result = filter_by_category(
            &conn,
            vec![String::from("Exercise"), String::from("Appliance")],
        )
        .unwrap();

        assert_eq!(items[1..=2], result);
    }

    #[allow(
        clippy::needless_pass_by_value,
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss
    )]
    #[test]
    fn filters_by_rating() {
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

        let ratings: Vec<usize> = vec![1, 2, 3, 4];

        let conn = dummy_connection();
        let conn = conn.lock().unwrap();

        let items_ratings: Vec<(Item, usize)> = items
            .clone()
            .into_iter()
            .zip(ratings.iter().copied())
            .collect();

        for (item, rating) in items_ratings {
            conn.execute(
            "INSERT INTO items (item_id, name, description, comments, img_path) VALUES (?, ?, ?, ?, ?)",
            params![item.item_id, item.name, item.description, item.comments, item.img_path],
        )
        .unwrap();
            conn.execute("INSERT INTO ratings (rating) VALUES (?)", params![rating])
                .unwrap();

            let rating_id = conn.last_insert_rowid();

            conn.execute(
                "INSERT INTO items_to_ratings (item_id, rating_id) VALUES (?, ?)",
                params![item.item_id, rating_id as usize],
            )
            .unwrap();
        }

        let result = filter_by_rating(&conn, vec![3, 4]).unwrap();

        assert_eq!(items[2..=3], result);
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
        assert_eq!(item, get_item(&conn, item_id).unwrap());
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
    #[allow(
        clippy::needless_pass_by_value,
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss
    )]
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
            comments: String::from("New Comment"),
            img_path: String::from("random"),
            ..item
        };

        update_item(&conn, new_item.clone()).unwrap();

        assert_eq!(new_item, get_item(&conn, item.item_id).unwrap());
    }

    #[test]
    #[allow(
        clippy::needless_pass_by_value,
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss
    )]
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
                    description: row.get(2).unwrap_or_default(),
                    comments: row.get(3).unwrap_or_default(),
                    img_path: row.get(4).unwrap_or_default(),
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
