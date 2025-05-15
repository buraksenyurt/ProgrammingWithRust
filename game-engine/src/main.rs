use std::{thread::sleep, time::Duration};

use framework::*;

mod framework;

fn main() {
    // let mut game = Game::new();
    let mut game = Game::default();
    let super_mario = Player::new(1, "Super Mario".to_string());
    let mushroom = Mushroom::new(2, 10);
    game.add_actor(Box::new(super_mario));
    game.add_actor(Box::new(mushroom));

    loop {
        sleep(Duration::from_secs(2));
        game.draw();
        game.update();
        // state check
    }
}

#[allow(dead_code)]
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

#[allow(dead_code)]
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
