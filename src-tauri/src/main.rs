#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::error::Error;

use serde::{Deserialize, Serialize};
use sled_extensions::json::JsonEncoding;
use sled_extensions::structured::Tree;
use sled_extensions::DbExt;
use tauri::Manager;

#[derive(Default, Deserialize, Serialize)]
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

            app.manage(RatingState { ratings });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}
