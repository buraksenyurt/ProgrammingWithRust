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
