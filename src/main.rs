use std::sync::{Arc, Mutex};

use anyhow::{anyhow, Error};

use crate::tile::{Tile, TilePos};

mod tile;
mod db_helper;
mod player;


fn main() -> Result<(), Error> {
    env_logger::init();

    let db = Arc::new(Mutex::new(db_helper::init_database()));


    println!("{:?}", db_helper::tile_funcs::get_tile_from_db(db.clone(), 1));
    println!("{:?}", db_helper::tile_funcs::get_tile_from_db(db.clone(), 2));

    Ok(())
}
