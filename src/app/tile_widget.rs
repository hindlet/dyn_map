use eframe::egui::{pos2, Color32, Pos2, Sense, Shape, Stroke, Vec2, Widget};

use crate::data_structs::{Tile, TilePos};

pub const HEX_POINTS: [Pos2; 6] = [pos2(0.0, 50.0), pos2(-43.3, 25.0), pos2(-43.3, -25.0), pos2(0.0, -50.0), pos2(43.3, -25.0), pos2(43.3, 25.0)];
const THREE_TO_THE_HALF: f32 = 1.732050808;

#[derive(Clone)]
pub struct TileWidget(pub Tile, pub Color32);

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

        let stroke_col = if response.hovered() && self.pointer_within(ui.ctx().pointer_latest_pos().unwrap().to_vec2() - centre) {
            Color32::WHITE
        } else {Color32::DARK_GRAY};
        painter.add(Shape::convex_polygon(points, self.1, Stroke::new(2.0, stroke_col)));

        response
    }
}



pub const PLUS_WIDTH: f32 = 20.0;
const PLUS_HALF_WIDTH: f32 = PLUS_WIDTH / 2.0;
const PLUS_EIGHTH_WIDTH: f32 = PLUS_HALF_WIDTH / 4.0;

const SQUARE_POINTS: [Pos2; 4] = [
    pos2(-PLUS_EIGHTH_WIDTH, PLUS_EIGHTH_WIDTH),
    pos2(PLUS_EIGHTH_WIDTH, PLUS_EIGHTH_WIDTH),
    pos2(PLUS_EIGHTH_WIDTH, -PLUS_EIGHTH_WIDTH),
    pos2(-PLUS_EIGHTH_WIDTH, -PLUS_EIGHTH_WIDTH),
];

const PLUS_POINTS: [Pos2; 12] = [
    pos2(-PLUS_EIGHTH_WIDTH, PLUS_HALF_WIDTH),
    pos2(PLUS_EIGHTH_WIDTH, PLUS_HALF_WIDTH),
    pos2(PLUS_EIGHTH_WIDTH, PLUS_EIGHTH_WIDTH),
    pos2(PLUS_HALF_WIDTH, PLUS_EIGHTH_WIDTH),
    pos2(PLUS_HALF_WIDTH, -PLUS_EIGHTH_WIDTH),
    pos2(PLUS_EIGHTH_WIDTH, -PLUS_EIGHTH_WIDTH),
    pos2(PLUS_EIGHTH_WIDTH, -PLUS_HALF_WIDTH),
    pos2(-PLUS_EIGHTH_WIDTH, -PLUS_HALF_WIDTH),
    pos2(-PLUS_EIGHTH_WIDTH, -PLUS_EIGHTH_WIDTH),
    pos2(-PLUS_HALF_WIDTH, -PLUS_EIGHTH_WIDTH),
    pos2(-PLUS_HALF_WIDTH, PLUS_EIGHTH_WIDTH),
    pos2(-PLUS_EIGHTH_WIDTH, PLUS_EIGHTH_WIDTH),
];


pub struct TileCreationWidget(pub TilePos);

impl Widget for TileCreationWidget {
    fn ui(self, ui: &mut eframe::egui::Ui) -> eframe::egui::Response {
        let centre = self.0.to_world_pos(ui.ctx().screen_rect().center().to_vec2()).to_vec2();

        
        let (response, painter) = ui.allocate_painter(Vec2::splat(PLUS_WIDTH), Sense::click());

        if response.hovered() || response.is_pointer_button_down_on() {
            let mut points = vec![];
            for i in 0..12 {
                points.push(PLUS_POINTS[i] + centre);
            }
            painter.add(Shape::convex_polygon(points, Color32::LIGHT_GREEN, Stroke::NONE));
        } else {
            let mut points = vec![];
            for i in 0..4 {
                points.push(SQUARE_POINTS[i] + centre);
            }
            painter.add(Shape::convex_polygon(points, Color32::DARK_GREEN, Stroke::NONE));
        };
        

        response
    }
}



#[derive(Clone)]
pub struct TileHighlightWidget(pub TilePos);

impl Widget for TileHighlightWidget {
    fn ui(self, ui: &mut eframe::egui::Ui) -> eframe::egui::Response {
        let centre = self.0.to_world_pos(ui.ctx().screen_rect().center().to_vec2()).to_vec2();

        let mut points = vec![];
        for i in 0..6 {
            points.push(HEX_POINTS[i] + centre);
        }
        let (response, painter) = ui.allocate_painter(Vec2::new(86.6, 100.0), Sense::empty());

        painter.add(Shape::closed_line(points, Stroke::new(2.0, Color32::WHITE)));

        response
    }
}