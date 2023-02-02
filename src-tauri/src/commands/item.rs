use tauri::State;

use crate::{errors::CommandResult, schema::Item, AppState};

#[tauri::command]
#[allow(clippy::needless_pass_by_value)]
pub fn get_items(state: State<AppState>) -> CommandResult<Vec<Item>> {
    unimplemented!()
}
