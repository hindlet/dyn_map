
#[derive(Debug, PartialEq, Clone)]
pub struct Player {
    pub id: i64,
    pub name: String,
    pub faction: String,
}


impl Player {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_faction(&self) -> &str {
        &self.faction
    }

    pub fn get_colour_db(&self) -> i64 {
        7
    }
}