use std::path::PathBuf;

use anyhow::Ok;
use eframe::App;
use serde::{Deserialize, Serialize};

use crate::{app::layout::draw_app, data_structs::{self, GameMap}};

mod tile;
mod map_render;
mod layout;
mod pop_up_menus;


#[derive(Serialize, Deserialize)]
pub struct DynamicMapApp {
    open_map: Option<GameMap>,
    #[serde(default)]
    maps: Vec<(GameMap, PathBuf)>,
    #[serde(default)]
    selected_map: (bool, usize),
    #[serde(default)]
    new_map: (bool, String),
    #[serde(default)]
    delete_map: (bool, usize, String),
    

}

impl DynamicMapApp {
    /// gets the list of existing files
    pub fn init(&mut self) -> Result<(), anyhow::Error> {

        self.maps = data_structs::GameMap::load_map_paths()?;


        Ok(())
    }
}

impl Default for DynamicMapApp {
    fn default() -> Self {
        DynamicMapApp {
            open_map: None,
            maps: Vec::new(),
            selected_map: (false, 0),
            new_map: (false, "".to_string()),
            delete_map: (false, 0, "".to_string()),
        }
    }
}

impl App for DynamicMapApp {

    


    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        draw_app(ctx, self);
    }
}