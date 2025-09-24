use eframe::egui::{Color32, Painter, Shape, Stroke, Vec2};
use crate::data_structs::{TileTag, TileTags, TileType};




pub fn draw_icons(tags: TileTags, tile_type: TileType, scale: f32, centre: Vec2, painter: &Painter) {
    for tag in TileTag::TAG_LIST {
        if !tags.has_tag(tag) {continue;}
        let mut points = Vec::new();
        for pos in tag.get_icon_points() {
            points.push(pos * 2.0 * scale + centre + tag.get_icon_offset(tile_type) * 2.0);
        }
        painter.add(Shape::line(points, Stroke::new(2.0, tag.get_icon_colour())));
    }
}