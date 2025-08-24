use eframe::egui::Ui;

use crate::{app::{helper::{draw_tile, draw_tile_creation_button}, DynamicMapApp}, db_helper};




pub fn render_map(app: &mut DynamicMapApp, ui: &mut Ui) {
    // let mut tile_creation_pos = vec![TilePos::START];
    // show tiles
    for tile in db_helper::tile_funcs::get_tiles_from_db(app.database.as_ref().unwrap().clone()).unwrap() {
        draw_tile(ui, tile, ui.ctx().screen_rect().center().to_vec2());
    }


    // show tile creation buttons
    for pos in db_helper::tile_funcs::get_tile_creation_spaces_from_db(app.database.as_ref().unwrap().clone()).unwrap() { 
        draw_tile_creation_button(ui, pos, ui.ctx().screen_rect().center().to_vec2());
    }

}