use std::sync::Mutex;

use rusqlite::Connection;
use serde::{Deserialize, Serialize};

pub struct Database {
    pub conn: Mutex<Connection>,
}

pub struct AppState {
    pub db: Database,
}

/// Representation of `items` table in our schema.
///
/// # Sqlite Representation
/// ```sql
///    id                   INTEGER NOT NULL  PRIMARY KEY
///    name                 VARCHAR(100)
///    description          VARCHAR(255)
///    comments             VARCHAR(255)
///    date                 VARCHAR(200)  DEFAULT CURRENT_DATE   
/// ````
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Item {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub comments: String,
    pub date: String,
}

/// Representation of `category` table in our schema.
///
/// # Sqlite Representation
/// ```sql
///    id                   INTEGER NOT NULL  PRIMARY KEY
///    name                 VARCHAR(100)
///    description          VARCHAR(255)
/// ````
#[derive(Deserialize, Serialize)]
pub struct Category {
    pub id: usize,
    pub name: String,
    pub description: String,
}

pub fn create_tables(conn: &Connection) -> anyhow::Result<()> {
    Ok(conn.execute_batch(
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
	date                 TEXT(200) DEFAULT CURRENT_DATE   
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
	date                 TEXT(200) DEFAULT CURRENT_DATE   
 );

CREATE TABLE items_to_ratings ( 
	id                   INTEGER NOT NULL  PRIMARY KEY  ,
	item_id              INTEGER     ,
	rating_id            INTEGER     ,
	FOREIGN KEY ( item_id ) REFERENCES items( id )  ,
	FOREIGN KEY ( rating_id ) REFERENCES ratings( id )  
 );",
    )?)
}
