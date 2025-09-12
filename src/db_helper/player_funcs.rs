use std::sync::{Arc, Mutex};
use anyhow::{anyhow, Error};
use sqlite::Connection;

use crate::data_structs::Player;


const GET_PLAYER_BY_ID: &str = "SELECT id, player_name, faction, colour, claim_points FROM Players where id = ?";
const DELETE_PLAYER_BY_ID: &str = "DELETE FROM Players where id = ?";
const INSERT_PLAYER: &str = "INSERT INTO Players (id, player_name, faction, colour, claim_points) VALUES (?, ?, ?, ?, ?)";
const GET_PLAYERS: &str = "SELECT id, player_name, faction, colour, claim_points FROM Players";
const UPDATE_PLAYER: &str = "UPDATE Players SET player_name = ?, faction = ?, colour = ?, claim_points = ? WHERE id = ?";
const MAX_PLAYER_ID: &str = "SELECT Max(id) as max_id FROM Players";

const GET_PLAYER_CLAIM_POINTS: &str = "SELECT claim_points FROM Players WHERE id = ?";
const UPDATE_PLAYER_CLAIM_POINTS: &str = "UPDATE Players SET claim_points = ? WHERE id = ?";


pub fn insert_player_to_db(db_con: Arc<Mutex<Connection>>, player: Player) -> Result<(), Error> {
    let con = db_con
        .lock()
        .map_err(|_| anyhow!("Error while locking db connection"))?;

    let mut stmt = con.prepare(INSERT_PLAYER)?;
    stmt.bind((1, player.id))?;
    stmt.bind((2, player.get_name()))?;
    stmt.bind((3, player.get_faction()))?;
    stmt.bind((4, player.colour_to_db()))?;
    stmt.bind((5, player.claim_points))?;

    if stmt.next()? == sqlite::State::Done {
        Ok(())
    } else {
        Err(anyhow!("error while inserting player"))
    }
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
        let colour_db = stmt.read::<i64, _>(3)?;
        let claim_points = stmt.read::<i64, _>(4)?;

        return Ok(Some(Player {id, name, faction, colour: Player::colour_from_db(colour_db), claim_points}));
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
        let colour_db = row.read::<i64, _>(3);
        let claim_points = row.read::<i64, _>(4);

        players.push(Player {id, name: name.to_string(), faction: faction.to_string(), colour: Player::colour_from_db(colour_db), claim_points});
    }

    Ok(players)
}

pub fn update_player_in_db(db_con: Arc<Mutex<Connection>>, player: Player) -> Result<(), Error> {
    let con = db_con
        .lock()
        .map_err(|_| anyhow!("Error while locking db connection"))?;

    let mut stmt = con.prepare(UPDATE_PLAYER)?;
    stmt.bind((1, player.get_name()))?;
    stmt.bind((2, player.get_faction()))?;
    stmt.bind((3, player.colour_to_db()))?;
    stmt.bind((4, player.id))?;

    if stmt.next()? == sqlite::State::Done {
        Ok(())
    } else {
        Err(anyhow!("error while updating player"))
    }
}

pub fn get_next_player_id(db_con: Arc<Mutex<Connection>>) -> Result<i64, Error> {
    let con = db_con
        .lock()
        .map_err(|_| anyhow!("Error while locking db connection"))?;

    let mut stmt = con.prepare(MAX_PLAYER_ID)?;

    if stmt.next()? == sqlite::State::Row {
        let id = stmt.read::<i64, _>(0)?;

        return Ok(id + 1);
    }

    Ok(0)
}

pub fn get_player_claim_points(db_con: Arc<Mutex<Connection>>, player_id: i64) -> Result<Option<i64>, Error> {
    let con = db_con
        .lock()
        .map_err(|_| anyhow!("Error while locking db connection"))?;

    let mut stmt = con.prepare(GET_PLAYER_CLAIM_POINTS)?;
    stmt.bind((1, player_id))?;

    if stmt.next()? == sqlite::State::Row {
        let claim_points = stmt.read::<i64, _>(0)?;

        return Ok(Some(claim_points));
    }

    Ok(None)
}

pub fn change_player_claim_points(db_con: Arc<Mutex<Connection>>, player_id: i64, change: i64) -> Result<(), Error> {
    let con = db_con
        .lock()
        .map_err(|_| anyhow!("Error while locking db connection"))?;

    let mut stmt = con.prepare(GET_PLAYER_CLAIM_POINTS)?;
    stmt.bind((1, player_id))?;


    let claim_points: i64;
    if stmt.next()? == sqlite::State::Row {
        claim_points = stmt.read::<i64, _>(0)?;
    } else {return Err(anyhow!("Could not get player claim points"));}

    let mut stmt = con.prepare(UPDATE_PLAYER_CLAIM_POINTS)?;
    stmt.bind((1, (claim_points + change).max(0)))?;
    stmt.bind((2, player_id))?;

    if stmt.next()? == sqlite::State::Done {
        Ok(())
    } else {
        Err(anyhow!("error while changing claim points"))
    }
}