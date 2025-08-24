use std::path::PathBuf;

use anyhow::Ok;
use eframe::App;
use sqlite::Connection;

use crate::{app::layout::draw_app, data_structs::{self, GameMap}};

mod tile;
mod map_render;
mod layout;
mod pop_up_menus;



pub struct DynamicMapApp {
    database: Option<Connection>,
    maps: Vec<(GameMap, PathBuf)>,
    selected_map: (bool, usize),
    new_map: (bool, String), // temp data
    delete_map: (bool, usize, String), // temp data
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
            database: None,
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