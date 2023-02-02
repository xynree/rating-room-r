use crate::errors::{CommandError, CommandResult};
use crate::schema::Item;
use rusqlite::{params_from_iter, Connection};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Filter {
    category: Option<Vec<String>>,
    rating: Option<Vec<String>>,
}

pub fn get_filtered_items(db: &Connection, filter: Filter) -> CommandResult<Vec<Item>> {
    let mut items = Vec::new();
    match (filter.category, filter.rating) {
        (None, None) => return Err(CommandError::Other(anyhow::anyhow!("Filter is empty"))),
        (None, Some(rating)) => {
            let sql = format!(
"SELECT items.* FROM items INNER JOIN items_to_ratings ON items_to_ratings.item_id = items.id
INNER JOIN ratings ON items_to_ratings.rating_id = ratings.id
WHERE ratings.rating IN ({})",
                    repeat_vars(rating.len())
            );
            let mut stmt = db.prepare(&sql)?;
            let rows = stmt.query_map(params_from_iter(rating), |row| {
                Ok(Item {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    description: row.get(2).unwrap_or(String::new()),
                    comments: row.get(3).unwrap_or(String::new()),
                    date: row.get(4)?,
                })
            })?;
            for item in rows {
                items.push(item?);
            }
        }
        (Some(category), None) => {
            let sql = format!(
"SELECT items.* FROM items INNER JOIN items_to_categories ON items_to_categories.item_id = items.id
INNER JOIN categories ON items_to_categories.category_id = categories.id
WHERE categories.name IN ({})",
                    repeat_vars(category.len())
            );
            let mut stmt = db.prepare(&sql)?;
            let rows = stmt.query_map(params_from_iter(category), |row| {
                Ok(Item {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    description: row.get(2).unwrap_or(String::new()),
                    comments: row.get(3).unwrap_or(String::new()),
                    date: row.get(4)?,
                })
            })?;
            for item in rows {
                items.push(item?);
            }
        }
        (Some(category), Some(rating)) => {
            let sql = format!(
                "SELECT items.* FROM items
  LEFT JOIN items_to_categories ON items_to_categories.item_id = items.id
  LEFT JOIN categories ON items_to_categories.category_id = categories.id
  LEFT JOIN items_to_ratings ON items_to_ratings.item_id = items.id
  LEFT JOIN ratings ON items_to_ratings.rating_id = ratings.id WHERE
  categories.name IN ({})
  AND ratings.rating IN ({})",
                repeat_vars(category.len()),
                repeat_vars(rating.len())
            );
            let mut stmt = db.prepare(&sql)?;
            let rows = stmt.query_map(
                params_from_iter(category.into_iter().chain(rating.into_iter())),
                |row| {
                    Ok(Item {
                        id: row.get(0)?,
                        name: row.get(1)?,
                        description: row.get(2).unwrap_or(String::new()),
                        comments: row.get(3).unwrap_or(String::new()),
                        date: row.get(4)?,
                    })
                },
            )?;
            for item in rows {
                items.push(item?);
            }
        }
    };
    Ok(items)
}

fn repeat_vars(count: usize) -> String {
    assert_ne!(count, 0);
    let mut s = "?,".repeat(count);
    // Remove trailing comma
    s.pop();
    s
}
