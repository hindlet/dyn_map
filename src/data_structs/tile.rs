use eframe::egui::{pos2, Color32, Pos2, Vec2};


#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TileType {
    Blank,
    Mineral,
    Artifact,
    Mystery
}

impl TileType {
    pub fn to_string(&self) -> &str {
        match self {
            TileType::Blank => "Basic",
            TileType::Mineral => "Mineral",
            TileType::Artifact => "Artifact",
            TileType::Mystery => "Mystery",
        }
    }

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
    pub fn to_world_pos(&self, window_centre: Vec2, zoom: f32) -> Pos2 {
        let pos = if !self.top_row {
            pos2(self.x as f32 * TILE_X_STEP + TILE_X_HALF_STEP, self.y as f32 * TILE_Y_STEP + TILE_Y_HALF_STEP)
        } else {pos2(self.x as f32 * TILE_X_STEP, self.y as f32 * TILE_Y_STEP)};

        pos * zoom + window_centre
    }
}


#[derive(Clone, Copy)]
pub enum TileTag {
    Settlement,
    TombWorld,
    Virtuous,
    Corrupted
}

impl TileTag {
    pub const TAG_LIST: [TileTag; 4] = [Self::Settlement, Self::TombWorld, Self::Virtuous, Self::Corrupted];

    pub fn get_tag_value(&self) -> i64 {
        match self {
            Self::Settlement => 1,
            Self::TombWorld => 2,
            Self::Virtuous => 4,
            Self::Corrupted => 8
        }
    }

    pub fn get_tag_power(&self) -> i64 {
        match self {
            Self::Settlement => 0,
            Self::TombWorld => 1,
            Self::Virtuous => 2,
            Self::Corrupted => 3
        }
    }

    pub fn get_tag_name(&self) -> &str {
        match self {
            Self::Settlement => "Settlement",
            Self::TombWorld => "Tomb World",
            Self::Virtuous => "Virtuous",
            Self::Corrupted => "Corrupted"
        }
    }

    pub fn get_tag_mask(&self) -> i64 {
        match self {
            Self::Settlement => 1 | 4 | 8,
            Self::TombWorld => 2,
            Self::Virtuous => 1 | 4,
            Self::Corrupted => 1 | 8
        }
    }

    pub fn get_icon_points(&self) -> Vec<Pos2> {
        match self {
            Self::Settlement => vec![pos2(-3.5, 6.0), pos2(-3.5, -1.0), pos2(5.0, -1.0), pos2(0.0, -6.0), pos2(-5.0, -1.0), pos2(3.5, -1.0), pos2(3.5, 6.0), pos2(-3.5, 6.0)],
            Self::TombWorld => vec![pos2(-2.0, 6.0), pos2(-4.0, -2.0), pos2(-2.0, -6.0), pos2(2.0, -6.0), pos2(4.0, -2.0), pos2(2.0, 6.0), pos2(-2.0, 6.0)],
            Self::Virtuous => vec![pos2(-1.5, 6.0), pos2(-1.5, 0.0), pos2(-5.0, 0.0), pos2(-5.0, -3.0), pos2(-1.5, -3.0), pos2(-1.5, -6.0), pos2(1.5, -6.0), pos2(1.5, -3.0), pos2(5.0, -3.0), pos2(5.0, 0.0), pos2(1.5, 0.0), pos2(1.5, 6.0), pos2(-1.5, 6.0)],
            Self::Corrupted => vec![pos2(-1.5, -6.0), pos2(-1.5, 0.0), pos2(-5.0, 0.0), pos2(-5.0, 3.0), pos2(-1.5, 3.0), pos2(-1.5, 6.0), pos2(1.5, 6.0), pos2(1.5, 3.0), pos2(5.0, 3.0), pos2(5.0, 0.0), pos2(1.5, 0.0), pos2(1.5, -6.0), pos2(-1.5, -6.0)]
        }
    }

    pub fn get_icon_offset(&self, tile_type: TileType) -> Vec2 {
        match (self, tile_type) {
            (Self::Settlement, TileType::Blank) => Vec2::new(-10.0, 0.0),
            (Self::TombWorld, TileType::Blank) => Vec2::new(0.0, 0.0),
            (Self::Virtuous, TileType::Blank) => Vec2::new(10.0, 0.0),
            (Self::Corrupted, TileType::Blank) => Vec2::new(10.0, 0.0),
            (Self::Settlement, TileType::Mineral) => Vec2::new(-12.0, 8.0),
            (Self::TombWorld, TileType::Mineral) => Vec2::new(0.0, -16.0),
            (Self::Virtuous, TileType::Mineral) => Vec2::new(12.0, 8.0),
            (Self::Corrupted, TileType::Mineral) => Vec2::new(12.0, 8.0),
            (Self::Settlement, TileType::Artifact) => Vec2::new(-8.0, -8.0),
            (Self::TombWorld, TileType::Artifact) => Vec2::new(0.0, -16.0),
            (Self::Virtuous, TileType::Artifact) => Vec2::new(8.0, 8.0),
            (Self::Corrupted, TileType::Artifact) => Vec2::new(8.0, 8.0),
            (Self::Settlement, TileType::Mystery) => Vec2::new(-12.0, 0.0),
            (Self::TombWorld, TileType::Mystery) => Vec2::new(-12.0, 0.0),
            (Self::Virtuous, TileType::Mystery) => Vec2::new(12.0, 0.0),
            (Self::Corrupted, TileType::Mystery) => Vec2::new(12.0, 0.0),
            _ => Vec2::ZERO
        }
    }

    pub fn get_icon_colour(&self) -> Color32 {
        match self {
            Self::Settlement => Color32::BLUE,
            Self::TombWorld => Color32::GREEN,
            Self::Virtuous => Color32::YELLOW,
            Self::Corrupted => Color32::PURPLE
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TileTags(pub i64);



impl TileTags {
    pub const NONE: TileTags = TileTags(0);

    pub fn has_tag(&self, tag: TileTag) -> bool {
        (self.0 >> tag.get_tag_power()) % 2 == 1
    }

    pub fn apply_tag_mask(&self, tag: TileTag) -> TileTags {
        TileTags((self.0 | tag.get_tag_value()) & tag.get_tag_mask())
    }
}


#[derive(Debug, PartialEq, Clone)]
pub struct Tile {
    pub id: i64,
    pub tile_type: TileType,
    pub pos: TilePos,
    pub tags: TileTags
}
