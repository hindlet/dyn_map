use std::f32;

use eframe::{egui::{pos2, Color32, Pos2, Sense, Shape, Stroke, Ui, Vec2}, emath};



pub fn colour_display_box(ui: &mut Ui, colour: Color32) {
    let size = ui.spacing().interact_size;
    let (rect, _response) = ui.allocate_exact_size(size, Sense::all());

    ui.painter().rect_filled(rect, 0.0, colour);
}

// 0.0 50.0], [-43.3 25.0], [-43.3 -25.0], [0.0 -50.0], [43.3 -25.0], [43.3 25.0]
const HEX_POINTS: [Pos2; 6] = [pos2(0.0, 50.0), pos2(-43.3, 25.0), pos2(-43.3, -25.0), pos2(0.0, -50.0), pos2(43.3, -25.0), pos2(43.3, 25.0)];

pub fn paint_tile_outline(ui: &mut Ui, center: Vec2) {

    let mut points = vec![];
    for i in 0..6 {
        points.push(HEX_POINTS[i] + center);
    }

    ui.painter().add(Shape::convex_polygon(points, ui.style().visuals.panel_fill, Stroke::new(2.0, Color32::LIGHT_GRAY)));
}