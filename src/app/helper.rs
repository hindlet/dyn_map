use eframe::egui::{vec2, Color32, Rect, Response, Sense, Ui, Vec2};

use crate::{app::tile_widget::{TileCreationWidget, TileHighlightWidget, TileWidget, PLUS_WIDTH}, data_structs::{Tile, TilePos}};



pub fn colour_display_box(ui: &mut Ui, colour: Color32) {
    let size = ui.spacing().interact_size;
    let (rect, _response) = ui.allocate_exact_size(size, Sense::all());

    ui.painter().rect_filled(rect, 0.0, colour);
}


pub fn draw_tile(ui: &mut Ui, tile: Tile, window_centre: Vec2, fill_col: Color32) -> Option<Response> {
    let centre = tile.pos.to_world_pos(window_centre);

    let widget = TileWidget(tile, fill_col);

    // let pointer_within = widget.pointer_within(ui.ctx().pointer_latest_pos().unwrap().to_vec2() - centre.to_vec2());
    let response = ui.put(Rect::from_center_size(centre, vec2(88.6, 102.0)), widget.clone());
    if let Some(pos) = ui.ctx().pointer_latest_pos() {
        if widget.pointer_within(pos.to_vec2() - centre.to_vec2()) {
            return Some(response);
        } else {
            return None;
        }
    }
    None
}


pub fn draw_tile_hightlight(ui: &mut Ui, pos: TilePos, window_centre: Vec2) {
    let centre = pos.to_world_pos(window_centre);

    let widget = TileHighlightWidget(pos);
    ui.put(Rect::from_center_size(centre, vec2(88.6, 102.0)), widget);
}

pub fn draw_tile_creation_button(ui: &mut Ui, pos: TilePos, window_centre: Vec2) -> Response {
    let centre = pos.to_world_pos(window_centre);

    let widget = TileCreationWidget(pos);

    ui.put(Rect::from_center_size(centre, Vec2::splat(PLUS_WIDTH)), widget)
}