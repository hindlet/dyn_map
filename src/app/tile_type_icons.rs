use eframe::egui::{pos2, Color32, Painter, Pos2, Shape, Stroke, Vec2};

use crate::data_structs::TileType;



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
const VAULT_ONE: [Pos2; 5] = [
    pos2(-10.0, -10.0), pos2(10.0, -10.0), pos2(10.0, 10.0), pos2(-10.0, 10.0), pos2(-10.0, -10.0)
];
const VAULT_TWO: [Pos2; 2] = [
    pos2(0.0, -5.0), pos2(0.0, 5.0)
];
const VAULT_THREE: [Pos2; 8] = [
    pos2(-5.0, -10.0), pos2(-5.0, -15.0), pos2(-2.5, -18.0), pos2(0.0, -19.0), pos2(2.5, -18.0), pos2(5.0, -15.0), pos2(5.0, -10.0), pos2(-5.0, -10.0)
];

pub fn draw_icon(tile_type: TileType, scale: f32, centre: Vec2, painter: &Painter) {
    match tile_type {
        TileType::Artifact => {
            let mut points = Vec::new();
            for id in 0..10 {
                points.push(ARTIFACT_ONE[id] * scale + centre);
            }
            painter.add(Shape::line(points, Stroke::new(2.0, Color32::LIGHT_GRAY)));
            let mut points = Vec::new();
            for id in 0..6 {
                points.push(ARTIFACT_TWO[id] * scale + centre);
            }
            painter.add(Shape::line(points, Stroke::new(2.0, Color32::LIGHT_GRAY)));
        },
        TileType::Mineral => {
            let mut points = Vec::new();
            for id in 0..18 {
                points.push(MINERAL[id] * 1.25 * scale + centre);
            }
            painter.add(Shape::line(points, Stroke::new(2.0, Color32::LIGHT_GRAY)));
        },
        TileType::Mystery => {
            let mut points = Vec::new();
            for id in 0..11 {
                points.push(MYSTERY_ONE[id] * scale + centre);
            }
            painter.add(Shape::line(points, Stroke::new(2.0, Color32::LIGHT_GRAY)));
            let mut points = Vec::new();
            for id in 0..5 {
                points.push(MYSTERY_TWO[id] * scale + centre);
            }
            painter.add(Shape::line(points, Stroke::new(2.0, Color32::LIGHT_GRAY)));
        },
        TileType::Vault => {
            let mut points = Vec::new();
            for id in 0..5 {
                points.push(VAULT_ONE[id] * scale + centre);
            }
            painter.add(Shape::line(points, Stroke::new(2.0, Color32::LIGHT_GRAY)));
            let mut points = Vec::new();
            for id in 0..2 {
                points.push(VAULT_TWO[id] * scale + centre);
            }
            painter.add(Shape::line(points, Stroke::new(2.0, Color32::LIGHT_GRAY)));
            let mut points = Vec::new();
            for id in 0..8 {
                points.push(VAULT_THREE[id] * scale + centre);
            }
            painter.add(Shape::line(points, Stroke::new(2.0, Color32::LIGHT_GRAY)));
        }
        _ => {}
    }
}