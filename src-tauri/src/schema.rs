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
/// ````
#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct Item {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub comments: String,
}

/// Representation of `category` table in our schema.
///
/// # Sqlite Representation
/// ```sql
///    id                   INTEGER NOT NULL  PRIMARY KEY
///    name                 VARCHAR(100)
///    description          VARCHAR(255)
/// ````
#[derive(Default, Debug, Deserialize, Serialize)]
pub struct Category {
    pub id: usize,
    pub name: String,
    pub description: String,
}

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct Rating {
    pub id: usize,
    pub rating: usize,
    pub date: String,
}

pub fn create_tables(conn: &Connection) -> anyhow::Result<()> {
    Ok(conn.execute_batch(
        "
CREATE TABLE categories ( 
    id                   INTEGER PRIMARY KEY,
	name                 VARCHAR(255) NOT NULL UNIQUE,
	description          TEXT     NOT NULL DEFAULT ''
 );

CREATE TABLE items ( 
    id                   INTEGER PRIMARY KEY, 
	name                 VARCHAR(255) NOT NULL UNIQUE,
	description          TEXT     NOT NULL DEFAULT '',
	comments             TEXT     NOT NULL DEFAULT '' 
 );

CREATE TABLE ratings ( 
	id                   INTEGER PRIMARY KEY,
	rating               INTEGER NOT NULL CHECK ( rating <= 5 AND rating >= 0 ),
	date                 VARCHAR(10) NOT NULL DEFAULT CURRENT_DATE 
); 

CREATE TABLE items_to_categories ( 
	item_id            INTEGER NOT NULL,
	category_id        INTEGER NOT NULL,
	FOREIGN KEY ( item_id ) REFERENCES items( id )  ,
	FOREIGN KEY ( category_id ) REFERENCES categories( id )  ,
    PRIMARY KEY ( item_id, category_id )
 );

CREATE TABLE items_to_ratings (
    item_id INTEGER NOT NULL,
    rating_id INTEGER  NOT NULL    ,
	FOREIGN KEY ( item_id ) REFERENCES items( id )  ,
	FOREIGN KEY ( rating_id ) REFERENCES ratings( id )  ,
    PRIMARY KEY ( item_id, rating_id )
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
("Air Purifier");


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
( 2, 17),
( 2, 9),
( 3, 16),
( 4, 2),
( 5, 11),
( 5, 13),
( 6, 13),
( 6, 10),
( 7, 4),
( 7, 6),
( 8, 4),
( 9, 14),
( 9, 15),
( 10, 14),
( 11, 16);

INSERT INTO ratings ( rating )
VALUES 
(4),
(3),
(4),
(5),
(4),
(3),
(4),
(4),
(5),
(4),
(3);

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
(11, 11);
"#,
    )?)
}
