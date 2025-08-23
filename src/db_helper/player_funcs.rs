use std::sync::{Arc, Mutex};
use anyhow::{anyhow, Error};
use sqlite::Connection;

use crate::data_structs::Player;


const GET_PLAYER_BY_ID: &str = "SELECT id, player_name, faction FROM Players where id = ?";
const DELETE_PLAYER_BY_ID: &str = "DELETE FORM Players where id = ?";
const INSERT_PLAYER: &str = "INSERT INTO Players (id, player_name, faction) VALUES (?, ?, ?) RETURNING id, player_name, faction";
const GET_PLAYERS: &str = "SELECT id, player_name, faction FROM Players";


pub fn insert_player_to_db(db_con: Arc<Mutex<Connection>>, player: Player) -> Result<Player, Error> {
    let con = db_con
        .lock()
        .map_err(|_| anyhow!("Error while locking db connection"))?;

    let mut stmt = con.prepare(INSERT_PLAYER)?;
    stmt.bind((1, player.id))?;
    stmt.bind((2, player.get_name()))?;
    stmt.bind((3, player.get_faction()))?;

    if stmt.next()? == sqlite::State::Row {
        let id = stmt.read::<i64, _>(0)?;
        let name = stmt.read::<String, _>(1)?;
        let faction = stmt.read::<String, _>(2)?;

        return Ok(Player {id, name, faction});
    }

    Err(anyhow!("error while inserting player"))
}

pub fn delete_player_from_db(db_con: Arc<Mutex<Connection>>, player_id: i64) -> Result<(), Error> {
    let con = db_con
        .lock()
        .map_err(|_| anyhow!("error while locking db connection"))?;
    let mut stmt = con.prepare(DELETE_PLAYER_BY_ID)?;
    stmt.bind((1, player_id))?;

    if stmt.next()? == sqlite::State::Done {
        Ok(())
    } else {
        Err(anyhow!("error while deleting player with id {}", player_id))
    }
}


pub fn get_player_from_db(db_con: Arc<Mutex<Connection>>, player_id: i64) -> Result<Option<Player>, Error> {
    let con = db_con
        .lock()
        .map_err(|_| anyhow!("Error while locking db connection"))?;

    let mut stmt = con.prepare(GET_PLAYER_BY_ID)?;
    stmt.bind((1, player_id))?;

    if stmt.next()? == sqlite::State::Row {
        let id = stmt.read::<i64, _>(0)?;
        let name = stmt.read::<String, _>(1)?;
        let faction = stmt.read::<String, _>(2)?;

        return Ok(Some(Player {id, name, faction}));
    }

    Ok(None)
}


pub fn get_players_from_db(db_con: Arc<Mutex<Connection>>) -> Result<Vec<Player>, Error> {
    let con = db_con
        .lock()
        .map_err(|_| anyhow!("Error while locking db connection"))?;

    let mut stmt = con.prepare(GET_PLAYERS)?;

    let mut players = Vec::new();
    for row in stmt.iter() {
        let row = row?;
        let id = row.read::<i64, _>(0);
        let name = row.read::<&str, _>(1);
        let faction = row.read::<&str, _>(2);

        players.push(Player {id, name: name.to_string(), faction: faction.to_string()});
    }

    Ok(players)
}