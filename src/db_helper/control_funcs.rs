use std::sync::{Arc, Mutex};
use anyhow::{anyhow, Error};
use sqlite::Connection;



// const GET_CONTROL_LEVELS_BY_TILE_ID: &str = "SELECT player_id, control_level FROM ControlLevels WHERE tile_id = ?";
// const GET_CONTROL_LEVELS_BY_PLAYER_ID: &str = "SELECT tile_id, control_level FROM ControlLevels WHERE player_id = ?";
// const GET_CONTROL_LEVEL_BY_TILE_ID_AND_PLAYER_ID: &str = "SELECT control_level FROM ControlLevels WHERE tile_id = ? AND player_id = ?";
const GET_HIGHEST_CONTROL_LEVEL_FOR_TILE_ID: &str = "SELECT player_id, MAX(control_level) as max_control FROM ControlLevels WHERE tile_id = ?";
const GET_HIGHEST_CONTROL_LEVELS: &str = "SELECT tile_id, player_id, Max(control_level) as max_control FROM ControlLevels GROUP BY tile_id";
const GET_PLAYER_CONTROLLED_TILES: &str = "SELECT tile_id, max_control FROM (SELECT tile_id, player_id, Max(control_level) as max_control FROM ControlLevels GROUP BY tile_id) WHERE player_id = ?";


/// Returns (player_id, control_level) for the player with the highest control_level of the given tile
pub fn get_highest_tile_control(db_con: Arc<Mutex<Connection>>, tile_id: i64) -> Result<Option<(i64, i64)>, Error> {
    let con = db_con
        .lock()
        .map_err(|_| anyhow!("Error while locking db connection"))?;

    let mut stmt = con.prepare(GET_HIGHEST_CONTROL_LEVEL_FOR_TILE_ID)?;
    stmt.bind((1, tile_id))?;

    if stmt.next()? == sqlite::State::Row {
        let player_id = stmt.read::<i64, _>(0)?;
        let max_control = stmt.read::<i64, _>(1)?;

        return Ok(Some((player_id, max_control)));
    }

    Ok(None)
}

/// Returns a Vec<(tile_id, player_id, control_level)> where each player and control level is the highest of the tile
pub fn get_max_control_levels(db_con: Arc<Mutex<Connection>>) -> Result<Vec<(i64, i64, i64)>, Error> {
    let con = db_con
        .lock()
        .map_err(|_| anyhow!("Error while locking db connection"))?;

    let mut stmt = con.prepare(GET_HIGHEST_CONTROL_LEVELS)?;

    let mut levels = Vec::new();
    for row in stmt.iter() {
        let row = row?;
        let tile_id = row.read::<i64, _>(0);
        let player_id = row.read::<i64, _>(1);
        let control_level = row.read::<i64, _>(2);
        

        levels.push((tile_id, player_id, control_level));
    }


    Ok(levels)
}

/// Returns a Vec<(tile_id, control_level)> where each tile_id is controlled by the given player
pub fn get_player_controlled_tiles(db_con: Arc<Mutex<Connection>>, player_id: i64) -> Result<Vec<(i64, i64)>, Error> {
    let con = db_con
        .lock()
        .map_err(|_| anyhow!("Error while locking db connection"))?;

    let mut stmt = con.prepare(GET_PLAYER_CONTROLLED_TILES)?;
    stmt.bind((1, player_id))?;

    let mut tiles = Vec::new();
    for row in stmt.iter() {
        let row = row?;
        let tile_id = row.read::<i64, _>(0);
        let control_level = row.read::<i64, _>(1);

        tiles.push((tile_id, control_level));
    }


    Ok(tiles)
}

