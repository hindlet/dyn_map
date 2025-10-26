use std::path::PathBuf;
use sqlite::Connection;
pub mod tile_funcs;
pub mod player_funcs;
pub mod control_funcs;


// opens the "map.db" file at a given path
pub fn open_database(path: PathBuf) -> Connection {
    let db_path = {
        let mut p = path.into_os_string();
        p.push("/map.db");
        p
    };
    sqlite::open(db_path).expect("Failed to create sqlite database")
}

pub fn init_database(db_con: Connection) {
    let init_query = "
    CREATE TABLE IF NOT EXISTS TileType (
        type_id TEXT PRIMARY KEY NOT NULL,
        seq INTEGER
    );
    INSERT INTO TileType(type_id, seq) VALUES (\"B\", 1);
    INSERT INTO TileType(type_id, seq) VALUES (\"M\", 2);
    INSERT INTO TileType(type_id, seq) VALUES (\"A\", 3);
    INSERT INTO TileType(type_id, seq) VALUES (\"?\", 4);
    INSERT INTO TileType(type_id, seq) VALUES (\"V\", 5);

    CREATE TABLE IF NOT EXISTS Tiles (
        id INTEGER PRIMARY KEY,
        tile_type TEXT NOT NULL,
        pos_x INTEGER,
        pos_y INTEGER,
        top_row INTEGER,
        tags INTEGER,
        FOREIGN KEY(tile_type) REFERENCES TileType(type_id)
    );

    CREATE TABLE IF NOT EXISTS Players (
        id INTEGER PRIMARY KEY,
        player_name TEXT NOT NULL,
        faction TEXT NOT NULL,
        colour INTEGER,
        claim_points INTEGER
    );

    CREATE TABLE IF NOT EXISTS ControlLevels (
        id INTEGER PRIMARY KEY,
        tile_id INTEGER,
        player_id INTEGER,
        control_level INTEGER
    );

    CREATE TABLE IF NOT EXISTS NextTileSpaces (
        id INTEGER PRIMARY KEY,
        pos_x INTEGER NOT NULL,
        pos_y INTEGER NOT NULL,
        top_row INTEGER NOT NULL,
        used INTEGER NOT NULL,
        UNIQUE(pos_x, pos_y, top_row) ON CONFLICT ABORT
    );

    INSERT INTO NextTileSpaces (pos_x, pos_y, top_row, used) VALUES (0, 0, 1, 0);
    ";
    db_con
        .execute(init_query)
        .expect("can initialise sqlite db");
}