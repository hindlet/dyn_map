use eframe::egui::{pos2, Pos2, Vec2};


#[derive(Debug, PartialEq, Clone)]
pub enum TileType {
    Blank,
    Mineral,
    Artifact,
    Mystery
}

impl TileType {
    pub fn to_db(&self) -> &str {
        match self {
            TileType::Blank => "B",
            TileType::Mineral => "M",
            TileType::Artifact => "A",
            TileType::Mystery => "?",
        }
    }

    pub fn from_db(from: &str) -> Self {
        match from {
            "M" => TileType::Mineral,
            "A" => TileType::Artifact,
            "?" => TileType::Mystery,
            _ => TileType::Blank
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct TilePos {
    pub x: i64,
    pub y: i64,
    pub top_row: bool,
}

impl TilePos {
    pub fn get_neighbours(&self) -> [TilePos; 6] {
        [
            TilePos{x: self.x + 1, y: self.y, top_row: self.top_row},
            TilePos{x: self.x - 1, y: self.y, top_row: self.top_row},

            TilePos{x: self.x - (self.top_row as i64), y: self.y - (self.top_row as i64), top_row: !self.top_row},
            TilePos{x: self.x + (!self.top_row as i64), y: self.y - (self.top_row as i64), top_row: !self.top_row},

            TilePos{x: self.x - (self.top_row as i64), y: self.y + (!self.top_row as i64), top_row: !self.top_row},
            TilePos{x: self.x + (!self.top_row as i64), y: self.y + (!self.top_row as i64), top_row: !self.top_row},
        ]
    }
}

const TILE_X_STEP: f32 = 86.60254038;
const TILE_X_HALF_STEP: f32 = TILE_X_STEP / 2.0;
const TILE_Y_STEP: f32 = 150.0;
const TILE_Y_HALF_STEP: f32 = TILE_Y_STEP / 2.0;

impl TilePos {
    pub fn to_world_pos(&self, window_centre: Vec2) -> Pos2 {
        let pos = if !self.top_row {
            pos2(self.x as f32 * TILE_X_STEP + TILE_X_HALF_STEP, self.y as f32 * TILE_Y_STEP + TILE_Y_HALF_STEP)
        } else {pos2(self.x as f32 * TILE_X_STEP, self.y as f32 * TILE_Y_STEP)};

        pos + window_centre
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Tile {
    pub id: i64,
    pub tile_type: TileType,
    pub pos: TilePos,
}
