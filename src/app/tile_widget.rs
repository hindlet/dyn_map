use eframe::egui::{pos2, vec2, AtomLayoutResponse, Button, Color32, Frame, Pos2, Rect, Sense, Shape, Stroke, Ui, Vec2, Widget};

use crate::data_structs::Tile;

const HEX_POINTS: [Pos2; 6] = [pos2(0.0, 50.0), pos2(-43.3, 25.0), pos2(-43.3, -25.0), pos2(0.0, -50.0), pos2(43.3, -25.0), pos2(43.3, 25.0)];
const THREE_TO_THE_HALF: f32 = 1.732050808;

pub struct TileWidget(pub Tile);

impl TileWidget {
    pub fn pointer_within(&self, local_pointer_pos: Vec2) -> bool {
        let abs_pos = local_pointer_pos.abs();
        abs_pos.y < THREE_TO_THE_HALF * 25.0_f32.min(50.0 - abs_pos.x)
    }
}

impl Widget for TileWidget {
    fn ui(self, ui: &mut eframe::egui::Ui) -> eframe::egui::Response {
        let centre = self.0.pos.to_world_pos(ui.ctx().screen_rect().center().to_vec2()).to_vec2();

        let mut points = vec![];
        for i in 0..6 {
            points.push(HEX_POINTS[i] + centre);
        }
        let (response, painter) = ui.allocate_painter(Vec2::new(86.6, 100.0), Sense::hover());

        let colour = if response.hovered() && self.pointer_within(ui.ctx().pointer_latest_pos().unwrap().to_vec2() - centre) {
            ui.style().visuals.extreme_bg_color
        } else {ui.style().visuals.panel_fill};
        painter.add(Shape::convex_polygon(points, colour, Stroke::new(2.0, Color32::LIGHT_GRAY)));

        response
    }
}
