#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::error::Error;

use serde::{Deserialize, Serialize};
use sled_extensions::json::JsonEncoding;
use sled_extensions::structured::Tree;
use sled_extensions::DbExt;
use tauri::{
    api::path::{app_data_dir, BaseDirectory},
    Manager, PathResolver,
};

#[derive(Deserialize, Serialize)]
struct Rating {
    name: String,
    cost: String,
}

struct RatingState {
    ratings: Tree<Rating, JsonEncoding>,
}
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn get(state: tauri::State<RatingState>, key: String) -> String {
    if let Some(rating) = state.ratings.get(key).unwrap() {
        format!("{}'s cost {}", rating.name, rating.cost)
    } else {
        String::new()
    }
}

#[tauri::command]
fn insert(state: tauri::State<RatingState>, rating: Rating) {
    state
        .ratings
        .insert(rating.name.clone().as_str(), rating)
        .unwrap();
}

fn main() -> Result<(), Box<dyn Error>> {
    tauri::Builder::default()
        .setup(|app| {
            let data_dir = app
                .path_resolver()
                .app_data_dir()
                .expect("failed to find Data Dir");

            let db = sled_extensions::Config::default()
                .path(data_dir.join("db"))
                .open()?;

            let ratings = db.open_json_tree("ratings")?;

            app.manage(RatingState { ratings });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get, insert])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}
