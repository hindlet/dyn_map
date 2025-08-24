use eframe::egui::{pos2, vec2, AtomLayoutResponse, Button, Color32, Frame, Pos2, Rect, Sense, Shape, Stroke, Ui, Vec2, Widget};

use crate::data_structs::Tile;

const HEX_POINTS: [Pos2; 6] = [pos2(0.0, 50.0), pos2(-43.3, 25.0), pos2(-43.3, -25.0), pos2(0.0, -50.0), pos2(43.3, -25.0), pos2(43.3, 25.0)];

pub struct TileWidget(pub Tile);

impl TileWidget {
}

impl Widget for TileWidget {
    fn ui(self, ui: &mut eframe::egui::Ui) -> eframe::egui::Response {
        let centre = self.0.pos.to_world_pos(ui.ctx().screen_rect().center().to_vec2()).to_vec2();

        let mut points = vec![];
        for i in 0..6 {
            points.push(HEX_POINTS[i] + centre);
        }
        let (response, painter) = ui.allocate_painter(Vec2::new(86.6, 100.0), Sense::hover());
        painter.add(Shape::convex_polygon(points, ui.style().visuals.panel_fill, Stroke::new(2.0, Color32::LIGHT_GRAY)));

        response
    }
}
