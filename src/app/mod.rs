use std::{path::PathBuf, sync::{Arc, Mutex}};

use anyhow::Ok;
use eframe::App;
use sqlite::Connection;

use crate::{app::{layout::draw_app, map_render::MapCamera}, data_structs::{self, GameMap, Player, TileTags, TileType}};

mod map_render;
mod layout;
mod pop_up_menus;
mod helper;
pub mod tile_widget;
mod tile_type_icons;
mod tile_tags_icons;



pub struct DynamicMapApp {
    admin_pass: String,
    admin_mode: bool,
    current_player: Option<(i64, String)>,
    
    pub database: Option<Arc<Mutex<Connection>>>,
    pub maps: Vec<(GameMap, PathBuf)>,
    pub selected_map: Option<usize>,
    new_map: Option<(String, String)>, // temp data: name, password
    delete_map: Option<(String, usize)>, // temp data,
    edit_map_mode: bool,
    selected_tile: Option<(i64, TileType, TileTags)>,
    camera: MapCamera,

    add_player: Option<Player>,
    edit_player: Option<Player>,
    delete_player: Option<(String, i64)>
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
            admin_pass: "".to_string(),
            admin_mode: false,
            current_player: None,

            database: None,
            maps: Vec::new(),
            selected_map: None,
            new_map: None,
            delete_map: None,
            edit_map_mode: false,
            selected_tile: None,
            camera: MapCamera::default(),

            add_player: None,
            edit_player: None,
            delete_player: None
        }
    }
}

impl App for DynamicMapApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        draw_app(ctx, self);
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        for (map, path) in self.maps.iter() {
            map.save(path.clone());
        }
    }
}