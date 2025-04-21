use std::fmt::Display;

#[derive(Debug)]
pub struct Game {
    pub title: String,
    pub year: u16,
    pub popularity: f32,
}

#[derive(Debug)]
pub struct Player {
    pub id: u32,
    pub position: (f32, f32),
    pub velocity: (f32, f32),
    pub score: u32,
}

#[derive(Debug)]
pub struct GameWorld {
    pub players: Vec<Player>,
}

#[derive(Debug)]
pub struct Log {
    pub level: Level,
    pub explanation: String,
}

impl Log {
    pub fn new(level: Level, explanation: String) -> Self {
        Log { level, explanation }
    }
}

impl Display for Log {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.level, self.explanation)
    }
}

#[derive(Debug)]
pub enum Level {
    Info,
    Warn,
    Error,
}

impl Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Level::Info => write!(f, "INFO"),
            Level::Warn => write!(f, "WARN"),
            Level::Error => write!(f, "ERROR"),
        }
    }
}
