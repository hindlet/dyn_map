use eframe::egui::{ComboBox, Sense, Ui, Vec2};

use crate::{app::{helper::{self, draw_tile, draw_tile_creation_button, draw_tile_hightlight}, DynamicMapApp}, data_structs::{Tile, TileType}, db_helper};


pub struct MapCamera {
    pub pos: Vec2,
    pub zoom: f32,
}


pub fn render_map(app: &mut DynamicMapApp, ui: &mut Ui) {
    // let mut tile_creation_pos = vec![TilePos::START];
    // show tiles
    let mut hovered = None;
    let mut selected = None;
    for tile in db_helper::tile_funcs::get_tiles_from_db(app.database.as_ref().unwrap().clone()).unwrap() {
        let fill_col = {
            let player = db_helper::control_funcs::get_highest_tile_control(app.database.as_ref().unwrap().clone(), tile.id).unwrap();
            if player.is_some() {
                db_helper::player_funcs::get_player_from_db(app.database.as_ref().unwrap().clone(), player.unwrap().0).unwrap().unwrap().colour
            } else {
                ui.style().visuals.panel_fill
            }
        };

        let centre = tile.pos;
        let id = tile.id;
        let tile_type = tile.tile_type;
        if let Some(resp) = draw_tile(ui, tile, ui.ctx().screen_rect().center().to_vec2(), fill_col) {
            if resp.interact(Sense::click()).clicked() {
                if app.selected_tile.is_none() || app.selected_tile.as_ref().unwrap().0 != id {
                    app.selected_tile = Some((id, tile_type));
                }
            }
            if resp.hovered() || resp.context_menu_opened() || resp.clicked() || resp.secondary_clicked() {
                hovered = Some(centre);
            }
        }
        if let Some(selected_id) = app.selected_tile.as_ref() {
            if selected_id.0 == id {
                selected = Some(centre)
            }
        }
    }
    if let Some(pos) = hovered {
        draw_tile_hightlight(ui, pos, ui.ctx().screen_rect().center().to_vec2());
    }
    if let Some(pos) = selected {
        draw_tile_hightlight(ui, pos, ui.ctx().screen_rect().center().to_vec2());
    }


    // show tile creation buttons
    if !(app.edit_map_mode && app.admin_mode){
        return;
    }
    for pos in db_helper::tile_funcs::get_tile_creation_spaces_from_db(app.database.as_ref().unwrap().clone()).unwrap() { 
        if draw_tile_creation_button(ui, pos, ui.ctx().screen_rect().center().to_vec2()).clicked() {
            let _ = db_helper::tile_funcs::set_tile_creation_space_used(app.database.as_ref().unwrap().clone(), pos);
            let id = db_helper::tile_funcs::get_next_tile_id(app.database.as_ref().unwrap().clone()).unwrap();
            let _ = db_helper::tile_funcs::insert_tile_to_db(app.database.as_ref().unwrap().clone(), Tile {
                id: id,
                tile_type: TileType::Blank,
                pos
            });
            let res = db_helper::control_funcs::create_tile_control(app.database.as_ref().unwrap().clone(), id);
            println!("{:?}", res);
            for neighbour in pos.get_neighbours() {
                let _ = db_helper::tile_funcs::add_creation_space_to_db(app.database.as_ref().unwrap().clone(), neighbour);
            }
        };
    }

}