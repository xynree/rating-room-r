#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
#![warn(clippy::style, clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

mod commands;
mod db;
mod errors;
mod filters;
mod schema;

use rusqlite::Connection;
use std::sync::Mutex;
use tauri::Manager;

use schema::{create_tables, seed_db, AppState, Database};

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let data_dir = app
                .path_resolver()
                .app_data_dir()
                .expect("failed to find Data Dir");

            let db_doesnt_exist = !data_dir.join("ratings.db").exists();

            let conn = Connection::open(data_dir.join("ratings.db")).unwrap();

            if db_doesnt_exist {
                create_tables(&conn)?;
                // if dev
                seed_db(&conn)?;
            }

            app.manage(AppState {
                db: Database {
                    conn: Mutex::new(conn),
                },
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_categories,
            commands::get_items,
            commands::get_item,
            commands::update_item,
            commands::create_item,
            commands::create_category,
            commands::delete_category
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
