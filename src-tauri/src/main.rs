#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::error::Error;

use serde::{Deserialize, Serialize};
use sled_extensions::json::JsonEncoding;
use sled_extensions::structured::Tree;
use sled_extensions::DbExt;

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
fn greet(state: tauri::State<RatingState>) -> String {
    if let Some(rating) = state.ratings.get("Takis").unwrap() {
        format!("{}'s cost {}", rating.name, rating.cost)
    } else {
        String::new()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let db = sled_extensions::Config::default()
        .path("db")
        .open()
        .expect("error while opening sled database");

    let ratings = db
        .open_json_tree("ratings")
        .expect("error while opening ratings tree");

    ratings.insert(
        "Takis",
        Rating {
            name: "Takis".to_owned(),
            cost: "2.0".to_owned(),
        },
    )?;

    tauri::Builder::default()
        .manage(RatingState { ratings })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}
