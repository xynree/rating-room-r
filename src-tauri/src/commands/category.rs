use tauri::State;

use crate::{
    errors::CommandResult,
    schema::{AppState, Item},
};

#[tauri::command]
pub fn get_categories(state: State<AppState>) -> CommandResult<Vec<Item>> {
    unimplemented!()
}
