CREATE TABLE IF NOT EXISTS TileType (
    type_id TEXT PRIMARY KEY NOT NULL,
    seq INTEGER
);
INSERT INTO TileType(type_id, seq) VALUES ("B", 1);
INSERT INTO TileType(type_id, seq) VALUES ("M", 2);
INSERT INTO TileType(type_id, seq) VALUES ("A", 3);
INSERT INTO TileType(type_id, seq) VALUES ("?", 4);

CREATE TABLE IF NOT EXISTS Tiles (
    id INTEGER PRIMARY KEY,
    tile_type TEXT NOT NULL,
    pos_x INTEGER,
    pos_y INTEGER,
    top_row INTEGER,
    FOREIGN KEY(tile_type) REFERENCES TileType(type_id)
);

CREATE TABLE IF NOT EXISTS Players (
    id INTEGER PRIMARY KEY,
    player_name TEXT NOT NULL,
    faction TEXT NOT NULL,
    colour INTEGER
);

CREATE TABLE IF NOT EXISTS ControlLevels (
    id INTEGER PRIMARY KEY,
    tile_id INTEGER,
    player_id INTEGER,
    control_level INTEGER
);


-- INSERT INTO Tiles (tile_type, pos_x, pos_y, top_row) VALUES ("B", 0, 0, 1);
-- INSERT INTO Tiles (tile_type, pos_x, pos_y, top_row) VALUES ("M", 0, 0, 0);

-- INSERT INTO Players (player_name, faction) VALUES ("Thomas", "Tau");
-- INSERT INTO Players (player_name, faction) VALUES ("Fred", "AdMech");

-- INSERT INTO ControlLevels (tile_id, player_id, control_level) VALUES (1, 1, 3);
-- INSERT INTO ControlLevels (tile_id, player_id, control_level) VALUES (1, 2, 2);

-- INSERT INTO ControlLevels (tile_id, player_id, control_level) VALUES (2, 1, 3);
-- INSERT INTO ControlLevels (tile_id, player_id, control_level) VALUES (2, 2, 5);