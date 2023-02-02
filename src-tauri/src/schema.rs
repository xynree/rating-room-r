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

pub fn seed_db(conn: &Connection) -> anyhow::Result<()> {
    Ok(conn.execute_batch(
        r#"
INSERT INTO items ( name )
VALUES
("Moisturizing Cream"),
("Zero Sugar Ginger Beer"),
("V60 Buono Electric Drip Kettle"),
("Slim Open Can 50L"),
("Reflections Double Sided LED Vanity Mirror"),
("16oz Narrow Mouth Water Bottle"),
("Dracaena Marginata"),
("Parla Scratching Post"),
("Airpods Pro"),
("M1 Macbook Pro"),
("Air Purifier"),
("Qahwa Brew");

INSERT INTO categories ( name )
VALUES
("Media"),
("Kitchen"),
("Snack"),
("Pet"),
("Art"),
("Plant"),
("Audio"),
("Tool"),
("Food"),
("Exercise"),
("Beauty"),
("Entertainment"),
("Accessory"),
("Electronics"),
("Alcohol"),
("Appliance"),
("Drink");

INSERT INTO items_to_categories ( item_id, category_id )
VALUES
( 1, 11 ),
( 2, 17 ),
( 2, 9 ),
( 3, 16 ),
( 4, 2 ),
( 5, 11 ),
( 5, 13 ),
( 6, 13 ),
( 6, 10 ),
( 7, 4 ),
( 7, 6 ),
( 8, 4 ),
( 9, 14 ),
( 9, 13 ),
( 10, 13 ),
( 11, 16 ),
( 12, 2 );

INSERT INTO ratings ( rating )
VALUES
(5),
(4),
(4),
(5),
(5),
(4),
(4),
(4),
(4),
(5),
(3),
(5);

INSERT INTO items_to_ratings ( item_id, rating_id )
VALUES
(1, 1),
(2, 2),
(3, 3),
(4, 4),
(5, 5),
(6, 6),
(7, 7),
(8, 8),
(9, 9),
(10, 10),
(11, 11),
(12, 12);
"#,
    )?)
}
