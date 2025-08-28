use eframe::egui::Ui;

use crate::{app::{helper::{draw_tile, draw_tile_creation_button}, DynamicMapApp}, data_structs::{Tile, TileType}, db_helper};




pub fn render_map(app: &mut DynamicMapApp, ui: &mut Ui) {
    // let mut tile_creation_pos = vec![TilePos::START];
    // show tiles
    for tile in db_helper::tile_funcs::get_tiles_from_db(app.database.as_ref().unwrap().clone()).unwrap() {
        draw_tile(ui, tile, ui.ctx().screen_rect().center().to_vec2());
    }


    // show tile creation buttons
    if !app.edit_map_mode {
        return;
    }
    for pos in db_helper::tile_funcs::get_tile_creation_spaces_from_db(app.database.as_ref().unwrap().clone()).unwrap() { 
        if draw_tile_creation_button(ui, pos, ui.ctx().screen_rect().center().to_vec2()).clicked() {
            let _ = db_helper::tile_funcs::set_tile_creation_space_used(app.database.as_ref().unwrap().clone(), pos);
            let _ = db_helper::tile_funcs::insert_tile_to_db(app.database.as_ref().unwrap().clone(), Tile {
                id: db_helper::tile_funcs::get_next_tile_id(app.database.as_ref().unwrap().clone()).unwrap(),
                tile_type: TileType::Blank,
                pos
            });
            for neighbour in pos.get_neighbours() {
                let _ = db_helper::tile_funcs::add_creation_space_to_db(app.database.as_ref().unwrap().clone(), neighbour);
            }
        };
    }

}