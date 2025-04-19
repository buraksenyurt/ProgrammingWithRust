use std::fmt::Display;

#[derive(Debug)]
pub struct Game {
    pub title: String,
    pub year: u16,
    pub popularity: f32,
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}|{}|{}", self.title, self.year, self.popularity)
    }
}
