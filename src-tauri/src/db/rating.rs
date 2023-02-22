use std::sync::MutexGuard;

use chrono::Utc;
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
                rating_id: row.get(0)?,
                rating: row.get(1)?,
                date: chrono::NaiveDateTime::parse_from_str(
                    row.get::<usize, String>(2)?.as_str(),
                    "%Y-%m-%d %H:%M:%S",
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
        conn.prepare("UPDATE ratings SET rating = ? , creation_timestamp = ? WHERE rating_id = ?")?;
    let id = stmt.execute(params![
        rating.to_string(),
        Utc::now().naive_utc().to_string(),
        rating_id.to_string(),
    ])?;

    Ok(id)
}

pub fn create_rating(conn: &MutexGuard<Connection>, rating: usize) -> CommandResult<i64> {
    if let Err(e) = conn.execute("INSERT INTO ratings (rating) VALUES ( ? )", [rating]) {
        return Err(CommandError::RusqliteError(e));
    };

    Ok(conn.last_insert_rowid())
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use rusqlite::Error;

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

    #[test]
    fn updates_rating() {
        let conn = dummy_connection();
        let conn = conn.lock().unwrap();

        let rating = 4;

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

        let new_rating = 2;

        let new_rating_id = update_rating(&conn, new_rating, rating_id as usize).unwrap();

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

        assert_eq!(new_rating, result);
    }

    #[test]
    fn deletes_rating() {
        let conn = dummy_connection();
        let conn = conn.lock().unwrap();

        let rating = 4;

        let rating_id = create_rating(&conn, rating).unwrap();

        delete_rating(&conn, rating_id as usize).unwrap();

        let result = conn
            .query_row(
                "SELECT rating FROM ratings WHERE rating_id = ?",
                [rating_id.to_string()],
                |row| {
                    let rating: usize = row.get(0)?;
                    Ok(rating)
                },
            )
            .unwrap_err();

        let expected = Error::QueryReturnedNoRows;

        assert_eq!(expected, result);
    }

    #[test]
    fn gets_ratings() {
        let conn = dummy_connection();
        let conn = conn.lock().unwrap();
        let ratings = vec![4, 2, 3, 1, 5];
        let item = Item {
            item_id: 1,
            name: String::from("Chips"),
            description: String::from("Crunchy"),
            comments: String::from("None"),
        };

        create_item(&conn, item.clone()).unwrap();

        for rating in ratings.clone() {
            conn.execute("INSERT INTO ratings (rating) VALUES ( ? )", [rating])
                .unwrap();
            let rating_id = conn.last_insert_rowid();
            add_rating_to_item(&conn, rating_id as usize, item.item_id).unwrap();
        }

        let result: Vec<_> = get_ratings(&conn, item.item_id)
            .unwrap()
            .iter()
            .map(|r| r.rating)
            .collect();

        assert_eq!(ratings, result);
    }
}
