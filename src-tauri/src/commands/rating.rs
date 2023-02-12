use tauri::State;

use crate::{
    errors::{CommandError, CommandResult},
    schema::Rating,
    AppState,
};

pub fn delete_rating(state: State<AppState>, rating_id: usize) -> CommandResult<()> {
    let db = state.db.conn.lock().unwrap();

    if let Err(e) = db.execute("DELETE FROM ratings WHERE rating_id = ?", [rating_id]) {
        return Err(CommandError::RusqliteError(e));
    };

    Ok(())
}

pub fn get_ratings(state: State<AppState>, item_id: usize) -> CommandResult<Vec<Rating>> {
    let db = state.db.conn.lock().unwrap();

    let mut stmt = db.prepare("SELECT * FROM ratings JOIN items_to_ratings ON ratings.rating_id = items_to_ratings.rating_id WHERE items_to_ratings.item_id = ?")?;

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
