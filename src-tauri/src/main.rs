#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
mod errors;
mod filters;
mod schema;

use commands::get_items;
use rusqlite::Connection;
use std::sync::Mutex;
use tauri::Manager;

use crate::schema::{create_tables, AppState, Database};

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let data_dir = app
                .path_resolver()
                .app_data_dir()
                .expect("failed to find Data Dir");

            let mut db_doesnt_exist = !data_dir.join("ratings.db").exists();

            let conn = Connection::open(data_dir.join("ratings.db")).unwrap();

            if db_doesnt_exist {
                create_tables(&conn)?;
            }

            app.manage(AppState {
                db: Database {
                    conn: Mutex::new(conn),
                },
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}
