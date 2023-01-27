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
    id: usize,
    name: String,
    description: String,
    rating: usize,
    categories: Vec<Category>,
    comments: String,
    image: Vec<u8>,
    date: String,
}

#[derive(Deserialize, Serialize)]
struct Category {
    id: usize,
    name: String,
    description: String,
}

struct RatingState {
    ratings: Tree<Rating, JsonEncoding>,
}
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(state: tauri::State<RatingState>) -> String {
    if let Some(rating) = state.ratings.get("Takis").unwrap() {
        format!("{}'s are {}", rating.name, rating.description)
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
            id: 1,
            name: "Takis".to_owned(),
            description: "A Delicious Snack".to_owned(),
            rating: 4,
            categories: Vec::from([Category {
                id: 1,
                name: "snack".to_owned(),
                description: "yummy stuff".to_owned(),
            }]),
            comments: "yummy yummy".to_owned(),
            date: "today".to_owned(),
            image: vec![2],
        },
    )?;

    tauri::Builder::default()
        .manage(RatingState { ratings })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}
