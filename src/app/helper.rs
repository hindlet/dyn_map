use std::f32;

use eframe::{egui::{Color32, Pos2, Sense, Shape, Stroke, Ui, Vec2}, emath};



pub fn colour_display_box(ui: &mut Ui, colour: Color32) {
    let size = ui.spacing().interact_size;
    let (rect, _response) = ui.allocate_exact_size(size, Sense::all());

    ui.painter().rect_filled(rect, 0.0, colour);
}


pub fn paint_tile_outline(ui: &mut Ui, center: Pos2) {

    let mut points = vec![center + Vec2::new(0.0, 50.0)];
    let rot = emath::Rot2::from_angle(f32::consts::FRAC_PI_3);
    for _ in 0..5 {
        points.push(center + rot * (points[points.len()-1] - center));
    }


    ui.painter().add(Shape::convex_polygon(points, ui.style().visuals.panel_fill, Stroke::new(2.0, Color32::LIGHT_GRAY)));
}