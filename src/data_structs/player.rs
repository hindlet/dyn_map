use eframe::egui::Color32;


#[derive(Debug, PartialEq, Clone)]
pub struct Player {
    pub id: i64,
    pub name: String,
    pub faction: String,
    pub colour: [u8; 3]
}


impl Player {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_faction(&self) -> &str {
        &self.faction
    }

    pub fn colour_to_db(&self) -> i64 {
        self.colour[0] as i64 & ((self.colour[1] as i64) << 8)& ((self.colour[2] as i64) << 16)
    }

    pub fn colour_from_db(colour: i64) -> [u8; 3] {
        [(colour & 0b11111111i64) as u8, ((colour >> 8) & 0b11111111i64) as u8, ((colour >> 16) & 0b11111111i64) as u8]
    }
}