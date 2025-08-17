
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

#[derive(Debug, PartialEq, Clone)]
pub struct TilePos {
    pub x: i64,
    pub y: i64,
    pub top_row: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Tile {
    pub id: i64,
    pub tile_type: TileType,
    pub pos: TilePos,
}