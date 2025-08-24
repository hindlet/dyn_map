use std::sync::{Arc, Mutex};
use anyhow::{anyhow, Error};
use sqlite::Connection;

use crate::data_structs::{Tile, TilePos, TileType};



const GET_TILE_BY_ID: &str = "SELECT id, tile_type, pos_x, pos_y, top_row FROM Tiles where id = ?";
const DELETE_TILE_BY_ID: &str = "DELETE FROM Tiles where id = ?";
const INSERT_TILE: &str = "INSERT INTO Tiles (id, tile_type, pos_x, pos_y, top_row) VALUES (?, ?, ?, ?, ?) RETURNING id, tile_type, pos_x, pos_y, top_row";
const GET_TILES: &str = "SELECT id, tile_type, pos_x, pos_y, top_row FROM Tiles";


pub fn insert_tile_to_db(db_con: Arc<Mutex<Connection>>, tile: Tile) -> Result<Tile, Error> {
    let con = db_con
        .lock()
        .map_err(|_| anyhow!("Error while locking db connection"))?;

    let mut stmt = con.prepare(INSERT_TILE)?;
    stmt.bind((1, tile.id))?;
    stmt.bind((2, tile.tile_type.to_db()))?;
    stmt.bind((3, tile.pos.x))?;
    stmt.bind((4, tile.pos.y))?;
    stmt.bind((5, tile.pos.top_row as i64))?;

    if stmt.next()? == sqlite::State::Row {
        let id = stmt.read::<i64, _>(0)?;
        let tile_type = stmt.read::<String, _>(1)?;
        let pos_x = stmt.read::<i64, _>(2)?;
        let pos_y = stmt.read::<i64, _>(3)?;
        let top_row = stmt.read::<i64, _>(4)?;

        return Ok(Tile {
            id,
            tile_type: TileType::from_db(&tile_type),
            pos: TilePos {
                x: pos_x,
                y: pos_y,
                top_row: top_row != 0
            }
        });
    }

    Err(anyhow!("error while inserting tile"))
}


pub fn delete_tile_from_db(db_con: Arc<Mutex<Connection>>, tile_id: i64) -> Result<(), Error> {
    let con = db_con
        .lock()
        .map_err(|_| anyhow!("error while locking db connection"))?;
    let mut stmt = con.prepare(DELETE_TILE_BY_ID)?;
    stmt.bind((1, tile_id))?;

    if stmt.next()? == sqlite::State::Done {
        Ok(())
    } else {
        Err(anyhow!("error while deleting tile with id {}", tile_id))
    }
}


pub fn get_tile_from_db(db_con: Arc<Mutex<Connection>>, tile_id: i64) -> Result<Option<Tile>, Error> {
    let con = db_con
        .lock()
        .map_err(|_| anyhow!("Error while locking db connection"))?;

    let mut stmt = con.prepare(GET_TILE_BY_ID)?;
    stmt.bind((1, tile_id))?;

    if stmt.next()? == sqlite::State::Row {
        let id = stmt.read::<i64, _>(0)?;
        let tile_type = stmt.read::<String, _>(1)?;
        let pos_x = stmt.read::<i64, _>(2)?;
        let pos_y = stmt.read::<i64, _>(3)?;
        let top_row = stmt.read::<i64, _>(4)?;

        return Ok(Some(Tile {
            id,
            tile_type: TileType::from_db(&tile_type),
            pos: TilePos {
                x: pos_x,
                y: pos_y,
                top_row: top_row != 0
            }
        }));
    }

    Ok(None)
}


pub fn get_tiles_from_db(db_con: Arc<Mutex<Connection>>) -> Result<Vec<Tile>, Error> {
    let con = db_con
        .lock()
        .map_err(|_| anyhow!("Error while locking db connection"))?;

    let mut stmt = con.prepare(GET_TILES)?;

    let mut tiles = Vec::new();
    for row in stmt.iter() {
        let row = row?;
        let id = row.read::<i64, _>(0);
        let tile_type = row.read::<&str, _>(1);
        let pos_x = row.read::<i64, _>(2);
        let pos_y = row.read::<i64, _>(3);
        let top_row = row.read::<i64, _>(4);

        tiles.push(Tile {
            id,
            tile_type: TileType::from_db(&tile_type),
            pos: TilePos {
                x: pos_x,
                y: pos_y,
                top_row: top_row == 1
            }
        });
    }

    Ok(tiles)
}


const DELETE_TILE_CREATION_SPACE: &str = "DELETE FROM NextTileSpaces where pos_x = ? AND pos_y = ? AND top_row = ?";
const INSERT_TILE_CREATION_SPACE: &str = "INSERT INTO NextTileSpaces (pos_x, pos_y, top_row, used) VALUES (?, ?, ?, 0)";
const GET_TILE_CREATION_SPACES: &str = "SELECT pos_x, pos_y, top_row FROM NextTileSpaces WHERE used = 0";

pub fn add_creation_space_to_db(db_con: Arc<Mutex<Connection>>, pos: TilePos) -> Result<(), Error> {
    let con = db_con
        .lock()
        .map_err(|_| anyhow!("Error while locking db connection"))?;

    let mut stmt = con.prepare(INSERT_TILE_CREATION_SPACE)?;
    stmt.bind((1, pos.x))?;
    stmt.bind((2, pos.y))?;
    stmt.bind((3, pos.top_row as i64))?;

    if stmt.next()? == sqlite::State::Done {
        Ok(())
    } else {
        Err(anyhow!("error while inserting tile creation space"))
    }
}

pub fn delete_tile_creation_space_from_db(db_con: Arc<Mutex<Connection>>, pos: TilePos) -> Result<(), Error> {
    let con = db_con
        .lock()
        .map_err(|_| anyhow!("error while locking db connection"))?;
    let mut stmt = con.prepare(DELETE_TILE_CREATION_SPACE)?;
    stmt.bind((1, pos.x))?;
    stmt.bind((2, pos.y))?;
    stmt.bind((3, pos.top_row as i64))?;

    if stmt.next()? == sqlite::State::Done {
        Ok(())
    } else {
        Err(anyhow!("error while deleting tile creation space"))
    }
}

pub fn get_tile_creation_spaces_from_db(db_con: Arc<Mutex<Connection>>) -> Result<Vec<TilePos>, Error> {
    let con = db_con
        .lock()
        .map_err(|_| anyhow!("Error while locking db connection"))?;

    let mut stmt = con.prepare(GET_TILE_CREATION_SPACES)?;

    let mut positions = Vec::new();
    for row in stmt.iter() {
        let row = row?;
        let pos_x = row.read::<i64, _>(0);
        let pos_y = row.read::<i64, _>(1);
        let top_row = row.read::<i64, _>(2);

        positions.push(
            TilePos {
                x: pos_x,
                y: pos_y,
                top_row: top_row == 1
            }
        );
    }

    Ok(positions)
}