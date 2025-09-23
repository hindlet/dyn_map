use std::{fs::{self, File}, path::PathBuf};
use anyhow::{Error, Ok};
use app_dirs::{app_dir};
use crate::{db_helper, APP_INFO};
use ron::{
    de::from_reader, ser::{to_string_pretty, PrettyConfig},
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct GameMap {
    pub name: String,
    pub password: String,
}


impl GameMap {
    pub fn new(name: String, password: String) -> (Self, PathBuf) {
        let folder_path = app_dir(app_dirs::AppDataType::UserData, &APP_INFO, &format!("data/maps/{}", name.clone().to_lowercase().replace(" ", "_"))).unwrap();

        let new = GameMap {
            name,
            password
        };

        let config = PrettyConfig::new()
            .depth_limit(2)
            .separate_tuple_members(true)
            .enumerate_arrays(true);

        let s = to_string_pretty(&new, config).expect("Failed to Serialize");

        let map_path: PathBuf = {
            let mut p = folder_path.clone().into_os_string();
            p.push("/map.ron");
            p.into()
        };
        let database_path: PathBuf = {
            let mut p = folder_path.clone().into_os_string();
            p.push("/map.db");
            p.into()
        };
        
        let _ = File::create(database_path.clone());

        let db = db_helper::open_database(folder_path.clone());
        db_helper::init_database(db);
        
        let _ = fs::write(map_path, s);

        (new, folder_path)
    }


    pub fn load_map_paths() -> Result<Vec<(GameMap, PathBuf)>, Error>{
        let path = app_dir(app_dirs::AppDataType::UserData, &APP_INFO, "data/maps")?;

        let mut maps = Vec::new();
        for map_dir in fs::read_dir(path).unwrap() {
            let map_dir = map_dir?;

            if !map_dir.file_type()?.is_dir() {continue;}

            let map_data_path: PathBuf = {
                let mut p = map_dir.path().into_os_string();
                p.push("/map.ron");
                p.into()
            };

            let map_data_file = File::open(map_data_path)?;
            let map_data: GameMap = from_reader(map_data_file)?;
            maps.push((map_data, map_dir.path()));
        }

        Ok(maps)
    }
}