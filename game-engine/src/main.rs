fn main() {
    let mut game = Game::new();
    let super_mario = Player::new(1, "Super Mario".to_string());
    let mushroom = Mushroom::new(2, 10);
    game.add_actor(Box::new(super_mario));
    game.add_actor(Box::new(mushroom));
}

struct Player {
    id: u32,
    name: String,
}
impl Player {
    pub fn new(id: u32, name: String) -> Self {
        Player { id, name }
    }
}

impl Actor for Player {
    fn draw(&self) {
        println!("Player draw");
    }
    fn update(&mut self) {
        println!("Update location of player");
    }
}

struct Mushroom {
    id: u32,
    strength: u8,
}

impl Mushroom {
    pub fn new(id: u32, strength: u8) -> Self {
        Mushroom { id, strength }
    }
}

impl Actor for Mushroom {
    fn draw(&self) {
        println!("Mushroom drawing");
    }
    fn update(&mut self) {
        println!("Calculation strength");
    }
}

trait Actor {
    fn draw(&self);
    fn update(&mut self);
}

struct Game {
    actors: Vec<Box<dyn Actor>>,
}

impl Game {
    pub fn new() -> Self {
        Game { actors: Vec::new() }
    }
    pub fn add_actor(&mut self, actor: Box<dyn Actor>) {
        self.actors.push(actor);
    }
}
