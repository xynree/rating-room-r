use std::sync::MutexGuard;

use chrono::{NaiveDateTime, Utc};
use rusqlite::{params, Connection};

use crate::{
    errors::{CommandError, CommandResult},
    schema::Rating,
};

pub fn get_ratings(conn: &MutexGuard<Connection>, item_id: usize) -> CommandResult<Vec<Rating>> {
    let mut stmt = conn.prepare("SELECT * FROM ratings JOIN items_to_ratings ON ratings.rating_id = items_to_ratings.rating_id WHERE items_to_ratings.item_id = ?")?;
    let rows: Vec<_> = stmt
        .query_map([item_id], |row| {
            Ok(Rating {
                rating_id: row.get(1)?,
                rating: row.get(2)?,
                date: chrono::NaiveDateTime::parse_from_str(
                    row.get::<usize, String>(3)?.as_str(),
                    "",
                )
                .map_err(|e| {
                    rusqlite::Error::FromSqlConversionFailure(
                        0,
                        rusqlite::types::Type::Text,
                        Box::new(e),
                    )
                })?,
            })
        })?
        .collect();

    let mut ratings = Vec::new();

    for row in rows {
        ratings.push(row?);
    }

    Ok(ratings)
}

pub fn delete_rating(conn: &MutexGuard<Connection>, rating_id: usize) -> CommandResult<()> {
    if let Err(e) = conn.execute("DELETE FROM ratings WHERE rating_id = ?", [rating_id]) {
        return Err(CommandError::RusqliteError(e));
    };

    Ok(())
}

pub fn update_rating(
    conn: &MutexGuard<Connection>,
    rating: usize,
    rating_id: usize,
) -> CommandResult<usize> {
    let mut stmt =
        conn.prepare("UPDATE ratings SET rating = ?, SET date = ? WHERE rating_id = ?")?;
    let id = stmt.execute(params![
        rating,
        Utc::now().naive_utc().to_string(),
        rating_id.to_string(),
    ])?;

    Ok(id)
}

pub fn create_rating(conn: &MutexGuard<Connection>, rating: usize) -> CommandResult<i64> {
    if let Err(e) = conn.execute("INSERT INTO ratings  (rating) VALUES ( ? )", [rating]) {
        return Err(CommandError::RusqliteError(e));
    };

    Ok(conn.last_insert_rowid())
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use chrono::{NaiveDateTime, Utc};
    use rusqlite::types::FromSql;

    use crate::schema::{create_tables, Category, Item, Rating};

    use super::*;

    fn dummy_connection() -> Mutex<Connection> {
        let conn = Connection::open_in_memory().unwrap();
        create_tables(&conn).unwrap();
        Mutex::new(conn)
    }

    #[test]
    fn creates_rating() {
        let rating = 4;

        let conn = dummy_connection();
        let conn = conn.lock().unwrap();

        let rating_id = create_rating(&conn, rating).unwrap();

        let result = conn
            .query_row(
                "SELECT rating FROM ratings WHERE rating_id = ?",
                [rating_id.to_string()],
                |row| {
                    let rating: usize = row.get(0)?;
                    Ok(rating)
                },
            )
            .unwrap();

        assert_eq!(rating, result);
    }
}
