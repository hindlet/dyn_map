use eframe::egui::Color32;


#[derive(Debug, PartialEq, Clone)]
pub struct Player {
    pub id: i64,
    pub name: String,
    pub faction: String,
    pub colour: Color32
}


impl Player {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_faction(&self) -> &str {
        &self.faction
    }

    pub fn colour_to_db(&self) -> i64 {
        (self.colour.r() as i64) | ((self.colour.g() as i64) << 8) | ((self.colour.b() as i64) << 16)
    }

    pub fn colour_from_db(colour: i64) -> Color32 {
        Color32::from_rgb((colour & 0b11111111i64) as u8, ((colour >> 8) & 0b11111111i64) as u8, ((colour >> 16) & 0b11111111i64) as u8)
    }
}