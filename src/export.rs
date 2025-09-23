use std::{collections::BTreeMap, fs::write};

use crate::{app::DynamicMapApp, data_structs::{Tile, TileType}, db_helper};
use chrono::Datelike;
use anyhow::{Error, Ok};



pub fn export_report(app: &DynamicMapApp) -> Result<(), Error>{
    let now = chrono::Local::now();

    if let Some(path) = rfd::FileDialog::new().add_filter("text", &["txt"]).set_file_name(format!("{}_report-{}-{}-{}.txt", app.maps[app.selected_map.unwrap()].0.name, now.year(), now.month(), now.day())).save_file() {
        let mut file_string = "".to_string();

        let controlled = db_helper::control_funcs::get_controlled_tiles(app.database.as_ref().unwrap().clone()).unwrap();
        println!("{:?}", controlled);
        let players = db_helper::player_funcs::get_players_from_db(app.database.as_ref().unwrap().clone()).unwrap();
        let tiles = db_helper::tile_funcs::get_tiles_from_db(app.database.as_ref().unwrap().clone()).unwrap();
        let mut tile_map: BTreeMap<i64, Tile> = BTreeMap::new();
        
        for tile in tiles {
            tile_map.insert(tile.id, tile);
        }

        for player in players {
            let mut p_controlled = (0, 0, 0, 0); // blank, mineral, artifact, mystery
            for (p_id, t_id) in controlled.iter() { // fuck it we iterate I don't care anymore
                if *p_id == player.id {
                    match tile_map.get(t_id).unwrap().tile_type {
                        TileType::Blank => {p_controlled.0 += 1;},
                        TileType::Mineral => {p_controlled.1 += 1;},
                        TileType::Artifact => {p_controlled.2 += 1;},
                        TileType::Mystery => {p_controlled.3 += 1;},
                    }
                }
            }

            file_string += &format!("{}:\n    Blank: {}\n    Mineral: {}\n    Artifact: {}\n    Mystery: {}\n", player.name, p_controlled.0, p_controlled.1, p_controlled.2, p_controlled.3);
        }

        write(path, file_string)?;
    }

    Ok(())
}