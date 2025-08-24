use std::f32;

use eframe::{egui::{pos2, vec2, Color32, Pos2, Rect, Sense, Shape, Stroke, Ui, Vec2}, emath};

use crate::{app::tile_widget::TileWidget, data_structs::Tile};



pub fn colour_display_box(ui: &mut Ui, colour: Color32) {
    let size = ui.spacing().interact_size;
    let (rect, _response) = ui.allocate_exact_size(size, Sense::all());

    ui.painter().rect_filled(rect, 0.0, colour);
}


pub fn draw_tile(ui: &mut Ui, tile: Tile, window_centre: Vec2) {

    if ui.put(Rect::from_center_size(tile.pos.to_world_pos(window_centre), vec2(88.6, 102.0)), TileWidget(tile)).hovered() {
        println!("Hovered");
    }
}