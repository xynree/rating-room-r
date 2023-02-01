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

struct Database {
    conn: Mutex<Connection>,
}

struct AppState {
    db: Database,
}

fn main() -> Result<(), Box<dyn Error>> {
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

fn create_tables(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute_batch(
        "
CREATE TABLE categories ( 
	id                   INTEGER NOT NULL  PRIMARY KEY  ,
	name                 VARCHAR(100)     ,
	description          VARCHAR(255)     
 );

CREATE TABLE items ( 
	id                   INTEGER NOT NULL  PRIMARY KEY  ,
	name                 VARCHAR(100)     ,
	description          VARCHAR(255)     ,
	comments             VARCHAR(255)     ,
	date                 DATE  DEFAULT CURRENT_DATE   
 );

CREATE TABLE items_to_categories ( 
	id                   INTEGER NOT NULL  PRIMARY KEY  ,
	item_id              INTEGER     ,
	category_id          INTEGER     ,
	FOREIGN KEY ( category_id ) REFERENCES categories( id )  ,
	FOREIGN KEY ( item_id ) REFERENCES items( id )  
 );

CREATE TABLE ratings ( 
	id                   INTEGER NOT NULL  PRIMARY KEY  ,
	rating               INTEGER     ,
	date                 DATE  DEFAULT CURRENT_DATE   
 );

CREATE TABLE items_to_ratings ( 
	id                   INTEGER NOT NULL  PRIMARY KEY  ,
	item_id              INTEGER     ,
	rating_id            INTEGER     ,
	FOREIGN KEY ( item_id ) REFERENCES items( id )  ,
	FOREIGN KEY ( rating_id ) REFERENCES ratings( id )  
 );",
    )
}
