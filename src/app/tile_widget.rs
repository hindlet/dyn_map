use eframe::egui::{pos2, Color32, Painter, Pos2, Sense, Shape, Stroke, Vec2, Widget};

use crate::data_structs::{Tile, TilePos, TileType};

pub const HEX_POINTS: [Pos2; 6] = [pos2(0.0, 50.0), pos2(-43.3, 25.0), pos2(-43.3, -25.0), pos2(0.0, -50.0), pos2(43.3, -25.0), pos2(43.3, 25.0)];
const THREE_TO_THE_HALF: f32 = 1.732050808;

const ARTIFACT_ONE: [Pos2; 10] = [
    pos2(-20.0, 20.0), pos2(-20.0, 10.0), pos2(-15.0, 5.0), pos2(-12.0, 8.0), pos2(8.0, -12.0),
    pos2(12.0, -8.0), pos2(-8.0, 12.0), pos2(-5.0, 15.0), pos2(-10.0, 20.0), pos2(-20.0, 20.0),
];
const ARTIFACT_TWO: [Pos2; 6] = [
    pos2(8.0, -12.0), pos2(10.0, -20.0), pos2(15.0, -25.0), pos2(25.0, -15.0), pos2(20.0, -10.0), pos2(12.0, -8.0),
];
const MINERAL: [Pos2; 18] = [
    pos2(-17.5, -8.0), pos2(0.0, 20.0), pos2(-8.75, -8.0), // left base
    pos2(-17.5, -8.0), pos2(-12.5, -13.0), pos2(-8.75, -8.0), // left mid
    pos2(0.0, -13.0), pos2(-12.5, -13.0), pos2(-8.75, -8.0), // left top
    pos2(8.75, -8.0), pos2(0.0, -13.0), pos2(12.5, -13.0), // true mid
    pos2(8.75, -8.0), pos2(17.5, -8.0), pos2(12.5, -13.0), // right mid,
    pos2(8.75, -8.0), pos2(0.0, 20.0), pos2(17.5, -8.0) // right base
];
const MYSTERY_ONE: [Pos2; 11] = [
    pos2(-5.0, -5.0), pos2(-5.0,15.0), pos2(5.0, 15.0), pos2(5.0, -2.5), 
    pos2(15.0, -15.0), pos2(0.0, -30.0), pos2(-15.0, -15.0),
    pos2(-5.0, -15.0), pos2(0.0, -20.0), pos2(5.0, -15.0), pos2(-5.0, -5.0)
];
const MYSTERY_TWO: [Pos2; 5] = [
    pos2(0.0, 20.0), pos2(5.0, 25.0), pos2(0.0, 30.0), pos2(-5.0, 25.0), pos2(0.0, 20.0)
];

#[derive(Clone)]
pub struct TileWidget(pub Tile, pub Color32, pub f32, pub Vec2);

impl TileWidget {
    pub fn pointer_within(&self, local_pointer_pos: Vec2, zoom: f32) -> bool {
        let abs_pos = local_pointer_pos.abs();
        abs_pos.y < THREE_TO_THE_HALF * (25.0 * zoom).min((50.0 * zoom) - abs_pos.x)
    }
}

impl Widget for TileWidget {
    fn ui(self, ui: &mut eframe::egui::Ui) -> eframe::egui::Response {
        let centre = self.0.pos.to_world_pos(ui.ctx().screen_rect().center().to_vec2(), self.2).to_vec2() + self.3;

        let mut points = vec![];
        for i in 0..6 {
            points.push(HEX_POINTS[i] * self.2 + centre);
        }
        let (response, painter) = ui.allocate_painter(Vec2::new(86.6 * self.2, 100.0 * self.2), Sense::hover());

        let stroke_col = if response.hovered() && self.pointer_within(ui.ctx().pointer_latest_pos().unwrap().to_vec2() - centre, self.2) {
            Color32::WHITE
        } else {Color32::DARK_GRAY};
        painter.add(Shape::convex_polygon(points, self.1, Stroke::new(2.0, stroke_col)));

        match self.0.tile_type {
            TileType::Artifact => {
                let mut points = Vec::new();
                for id in 0..10 {
                    points.push(ARTIFACT_ONE[id] * self.2 + centre);
                }
                painter.add(Shape::line(points, Stroke::new(2.0, Color32::LIGHT_GRAY)));
                let mut points = Vec::new();
                for id in 0..6 {
                    points.push(ARTIFACT_TWO[id] * self.2 + centre);
                }
                painter.add(Shape::line(points, Stroke::new(2.0, Color32::LIGHT_GRAY)));
            },
            TileType::Mineral => {
                let mut points = Vec::new();
                for id in 0..18 {
                    points.push(MINERAL[id] * 1.25 * self.2 + centre);
                }
                painter.add(Shape::line(points, Stroke::new(2.0, Color32::LIGHT_GRAY)));
            },
            TileType::Mystery => {
                let mut points = Vec::new();
                for id in 0..11 {
                    points.push(MYSTERY_ONE[id] * self.2 + centre);
                }
                painter.add(Shape::line(points, Stroke::new(2.0, Color32::LIGHT_GRAY)));
                let mut points = Vec::new();
                for id in 0..5 {
                    points.push(MYSTERY_TWO[id] * self.2 + centre);
                }
                painter.add(Shape::line(points, Stroke::new(2.0, Color32::LIGHT_GRAY)));
            },
            _ => {}
        }

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


pub struct TileCreationWidget(pub TilePos, pub f32, pub Vec2);

impl Widget for TileCreationWidget {
    fn ui(self, ui: &mut eframe::egui::Ui) -> eframe::egui::Response {
        let centre = self.0.to_world_pos(ui.ctx().screen_rect().center().to_vec2(), self.1).to_vec2() + self.2;

        
        let (response, painter) = ui.allocate_painter(Vec2::splat(PLUS_WIDTH), Sense::click());

        if response.hovered() || response.is_pointer_button_down_on() {
            let mut points = vec![];
            for i in 0..12 {
                points.push(PLUS_POINTS[i] * self.1 + centre);
            }
            painter.add(Shape::convex_polygon(points, Color32::LIGHT_GREEN, Stroke::NONE));
        } else {
            let mut points = vec![];
            for i in 0..4 {
                points.push(SQUARE_POINTS[i] * self.1 + centre);
            }
            painter.add(Shape::convex_polygon(points, Color32::DARK_GREEN, Stroke::NONE));
        };
        

        response
    }
}



#[derive(Clone)]
pub struct TileHighlightWidget(pub TilePos, pub f32, pub Vec2);

impl Widget for TileHighlightWidget {
    fn ui(self, ui: &mut eframe::egui::Ui) -> eframe::egui::Response {
        let centre = self.0.to_world_pos(ui.ctx().screen_rect().center().to_vec2(), self.1).to_vec2() + self.2;

        let mut points = vec![];
        for i in 0..6 {
            points.push(HEX_POINTS[i] * self.1 + centre);
        }
        let (response, painter) = ui.allocate_painter(Vec2::new(86.6, 100.0), Sense::empty());

        painter.add(Shape::closed_line(points, Stroke::new(2.0, Color32::WHITE)));

        response
    }
}