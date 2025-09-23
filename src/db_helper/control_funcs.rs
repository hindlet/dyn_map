use std::sync::{Arc, Mutex};
use anyhow::{anyhow, Error, Ok};
use image::codecs::qoi;
use sqlite::Connection;



// const GET_CONTROL_LEVELS_BY_TILE_ID: &str = "SELECT player_id, control_level FROM ControlLevels WHERE tile_id = ?";
// const GET_CONTROL_LEVELS_BY_PLAYER_ID: &str = "SELECT tile_id, control_level FROM ControlLevels WHERE player_id = ?";
// const GET_CONTROL_LEVEL_BY_TILE_ID_AND_PLAYER_ID: &str = "SELECT control_level FROM ControlLevels WHERE tile_id = ? AND player_id = ?";
const GET_HIGHEST_CONTROL_LEVEL_FOR_TILE_ID: &str = "SELECT player_id, control_level FROM ControlLevels WHERE tile_id = ? AND control_level = (SELECT MAX(control_level) as max_control FROM ControlLevels WHERE tile_id = ?) AND control_level >= 2";
const CREATE_TILE_CONTROL: &str = "INSERT INTO ControlLevels (tile_id, player_id, control_level) SELECT ?, id, 0 FROM Players";
const CREATE_PLAYER_CONTROL: &str = "INSERT INTO ControlLevels (tile_id, player_id, control_level) SELECT id, ?, 0 FROM Tiles";
const GET_TILE_CONTROL_LEVELS: &str = "SELECT player_id, control_level FROM ControlLevels WHERE tile_id = ? ORDER BY control_level DESC";
const GET_PLAYER_CONTROL: &str = "SELECT control_level FROM ControlLevels WHERE player_id = ? AND tile_id = ?";
const UPDATE_PLAYER_CONTROL: &str = "UPDATE ControlLevels SET control_level = ? WHERE player_id = ? AND tile_id = ?";
const GET_CONTROLLED_TILES: &str = "SELECT player_id, tile_id FROM ControlLevels, (SELECT MAX(control_level) as max, tile_id as max_id FROM ControlLevels WHERE control_level >= 2 GROUP BY tile_id) WHERE tile_id = max_id AND control_level = max GROUP BY tile_id HAVING COUNT(player_id) = 1";
// const TEST: &str = "SELECT tile_id, player_id FROM ControlLevels, (SELECT MAX(control_level) as max, tile_id as max_id FROM ControlLevels WHERE control_level >= 2 GROUP BY tile_id) WHERE tile_id = max_id AND control_level = max GROUP BY tile_id HAVING COUNT(player_id) = 1";
const RESET_CONTROL: &str = "UPDATE ControlLevels SET control_level = 0";

// [(3, 2), (6, 2), (7, 1), (12, 2), (13, 1)]
// [(3, 2), (7, 1), (12, 2)]
// [(7, 1), (13, 1), (3, 2), (6, 2), (12, 2), (13, 2)]
// [(3, 2), (6, 2), (7, 1), (12, 2)]
// pub fn test(db_con: Arc<Mutex<Connection>>) -> Result<Vec<(i64, i64)>, Error> {
//     let con = db_con
//         .lock()
//         .map_err(|_| anyhow!("Error while locking db connection"))?;

//     let mut stmt = con.prepare(TEST)?;
    
//     let mut test = Vec::new();
//     for row in stmt.iter() {
//         let row = row?;
//         let piss = row.read::<i64, _>(0);
//         let cum = row.read::<i64, _>(1);

//         test.push((piss, cum));
//     }

//     Ok(test)
// }


/// Returns (player_id, control_level) for the player with the highest control_level of the given tile
pub fn get_highest_tile_control(db_con: Arc<Mutex<Connection>>, tile_id: i64) -> Result<Option<(i64, i64)>, Error> {
    let con = db_con
        .lock()
        .map_err(|_| anyhow!("Error while locking db connection"))?;

    let mut stmt = con.prepare(GET_HIGHEST_CONTROL_LEVEL_FOR_TILE_ID)?;
    stmt.bind((1, tile_id))?;
    stmt.bind((2, tile_id))?;
    

    if stmt.next()? == sqlite::State::Row {
        let player_id = stmt.read::<i64, _>(0)?;
        let max_control = stmt.read::<i64, _>(1)?;
        if player_id == 0 || max_control < 2 {return Ok(None);}
        if stmt.next()? == sqlite::State::Row {
            return Ok(None);
        }

        return Ok(Some((player_id, max_control)));
    }

    Ok(None)
}

// /// Returns a Vec<(tile_id, player_id, control_level)> where each player and control level is the highest of the tile
// pub fn get_max_control_levels(db_con: Arc<Mutex<Connection>>) -> Result<Vec<(i64, i64, i64)>, Error> {
//     let con = db_con
//         .lock()
//         .map_err(|_| anyhow!("Error while locking db connection"))?;

//     let mut stmt = con.prepare(GET_HIGHEST_CONTROL_LEVELS)?;

//     let mut levels = Vec::new();
//     for row in stmt.iter() {
//         let row = row?;
//         let tile_id = row.read::<i64, _>(0);
//         let player_id = row.read::<i64, _>(1);
//         let control_level = row.read::<i64, _>(2);
        

//         levels.push((tile_id, player_id, control_level));
//     }


//     Ok(levels)
// }

// /// Returns a Vec<(tile_id, control_level)> where each tile_id is controlled by the given player
// pub fn get_player_controlled_tiles(db_con: Arc<Mutex<Connection>>, player_id: i64) -> Result<Vec<(i64, i64)>, Error> {
//     let con = db_con
//         .lock()
//         .map_err(|_| anyhow!("Error while locking db connection"))?;

//     let mut stmt = con.prepare(GET_PLAYER_CONTROLLED_TILES)?;
//     stmt.bind((1, player_id))?;

//     let mut tiles = Vec::new();
//     for row in stmt.iter() {
//         let row = row?;
//         let tile_id = row.read::<i64, _>(0);
//         let control_level = row.read::<i64, _>(1);

//         tiles.push((tile_id, control_level));
//     }


//     Ok(tiles)
// }

pub fn get_controlled_tiles(db_con: Arc<Mutex<Connection>>) -> Result<Vec<(i64, i64)>, Error> {
    let con = db_con
        .lock()
        .map_err(|_| anyhow!("Error while locking db connection"))?;

    let mut stmt = con.prepare(GET_CONTROLLED_TILES)?;


    let mut tiles = Vec::new();
    for row in stmt.iter() {
        let row = row?;
        let player_id = row.read::<i64, _>(0);
        let tile_id = row.read::<i64, _>(1);

        tiles.push((player_id, tile_id));
    }


    Ok(tiles)
}

pub fn create_tile_control(db_con: Arc<Mutex<Connection>>, tile_id: i64) -> Result<(), Error>{
    let con = db_con
        .lock()
        .map_err(|_| anyhow!("Error while locking db connection"))?;

    let mut stmt = con.prepare(CREATE_TILE_CONTROL)?;
    stmt.bind((1, tile_id))?;

    if stmt.next()? == sqlite::State::Done {
        Ok(())
    } else {
        Err(anyhow!("error while creating tile control levels"))
    }
}

pub fn create_player_control(db_con: Arc<Mutex<Connection>>, player_id: i64) -> Result<(), Error>{
    let con = db_con
        .lock()
        .map_err(|_| anyhow!("Error while locking db connection"))?;

    let mut stmt = con.prepare(CREATE_PLAYER_CONTROL)?;
    stmt.bind((1, player_id))?;

    if stmt.next()? == sqlite::State::Done {
        Ok(())
    } else {
        Err(anyhow!("error while creating player control levels"))
    }
}


pub fn get_tile_control_levels(db_con: Arc<Mutex<Connection>>, tile_id: i64) -> Result<Vec<(i64, i64)>, Error> {
    let con = db_con
        .lock()
        .map_err(|_| anyhow!("Error while locking db connection"))?;

    let mut stmt = con.prepare(GET_TILE_CONTROL_LEVELS)?;
    stmt.bind((1, tile_id))?;

    let mut players = Vec::new();
    for row in stmt.iter() {
        let row = row?;
        let player_id = row.read::<i64, _>(0);
        let control_level = row.read::<i64, _>(1);

        players.push((player_id, control_level));
    }

    Ok(players)
}

pub fn get_player_control_level(db_con: Arc<Mutex<Connection>>, player_id: i64, tile_id: i64) -> Result<Option<i64>, Error> {
    let con = db_con
        .lock()
        .map_err(|_| anyhow!("Error while locking db connection"))?;

    let mut stmt = con.prepare(GET_PLAYER_CONTROL)?;
    stmt.bind((1, player_id))?;
    stmt.bind((2, tile_id))?;

    if stmt.next()? == sqlite::State::Row {
        let control_level = stmt.read::<i64, _>(0)?;

        return Ok(Some(control_level));
    }

    Ok(None)
}

pub fn change_player_control_level(db_con: Arc<Mutex<Connection>>, player_id: i64, tile_id: i64, change: i64) -> Result<(), Error> {
    let con = db_con
        .lock()
        .map_err(|_| anyhow!("Error while locking db connection"))?;

    let mut stmt = con.prepare(GET_PLAYER_CONTROL)?;
    stmt.bind((1, player_id))?;
    stmt.bind((2, tile_id))?;

    let control_level: i64;
    if stmt.next()? == sqlite::State::Row {
        control_level = stmt.read::<i64, _>(0)?;
    } else {return Err(anyhow!("Could not get player control level"));}

    let mut stmt = con.prepare(UPDATE_PLAYER_CONTROL)?;
    stmt.bind((1, (control_level + change).max(0)))?;
    stmt.bind((2, player_id))?;
    stmt.bind((3, tile_id))?;

    if stmt.next()? == sqlite::State::Done {
        Ok(())
    } else {
        Err(anyhow!("error while changing control level"))
    }
}

pub fn reset_control_levels(db_con: Arc<Mutex<Connection>>) -> Result<(), Error> {
    let con = db_con
        .lock()
        .map_err(|_| anyhow!("Error while locking db connection"))?;

    let mut stmt = con.prepare(RESET_CONTROL)?;

    if stmt.next()? == sqlite::State::Done {
        Ok(())
    } else {
        Err(anyhow!("error while resetting control"))
    }
}