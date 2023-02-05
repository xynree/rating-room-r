use rusqlite::params;
use tauri::{command, State};

use crate::{
    errors::{CommandError, CommandResult},
    schema::{Category, Item, Rating},
    AppState,
};

pub fn delete_rating(state: State<AppState>, rating_id: usize) -> CommandResult<()> {
    let db = state.db.conn.lock().unwrap();

    if let Err(e) = db.execute("DELETE FROM ratings WHERE id = ?", [rating_id]) {
        return Err(CommandError::RusqliteError(e));
    };

    Ok(())
}

pub fn get_ratings(state: State<AppState>, item_id: usize) -> CommandResult<Vec<Rating>> {
    let db = state.db.conn.lock().unwrap();

    let mut stmt = db.prepare("SELECT * FROM ratings JOIN items_to_ratings ON ratings.id = items_to_ratings.rating_id WHERE items_to_ratings.item_id = ?")?;

    let rows: Vec<_> = stmt
        .query_map([item_id], |row| {
            Ok(Rating {
                id: row.get(1)?,
                rating: row.get(2)?,
                date: row.get(3)?,
            })
        })?
        .collect();

    let mut ratings = Vec::new();

    for row in rows {
        ratings.push(row?)
    }

    Ok(ratings)
}
