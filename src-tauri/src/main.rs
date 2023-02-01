#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{
    error::Error,
    sync::{Arc, Mutex},
};

use rusqlite::Connection;
use serde::{Deserialize, Serialize};
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
    conn: Arc<Mutex<Connection>>,
}

fn main() -> Result<(), Box<dyn Error>> {
    tauri::Builder::default()
        .setup(|app| {
            let data_dir = app
                .path_resolver()
                .app_data_dir()
                .expect("failed to find Data Dir");

            let conn = Connection::open(data_dir).unwrap();

            // ratings.insert(
            //     "Takis",
            //     Rating {
            //         id: 1,
            //         name: "Takis".to_owned(),
            //         description: "A Delicious Snack".to_owned(),
            //         rating: 4,
            //         categories: Vec::from([Category {
            //             id: 1,
            //             name: "snack".to_owned(),
            //             description: "yummy stuff".to_owned(),
            //         }]),
            //         comments: "yummy yummy".to_owned(),
            //         date: "today".to_owned(),
            //         image: vec![2],
            //     },
            // )?;

            app.manage(RatingState {
                conn: Arc::new(Mutex::new(conn)),
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}
