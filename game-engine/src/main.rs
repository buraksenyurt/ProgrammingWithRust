use std::{
    sync::mpsc::channel,
    thread::{self, sleep},
    time::Duration,
};

use framework::*;

mod framework;

fn main() {
    let (transmitter, receiver) = channel::<String>();

    let mut game = Game::new(transmitter.clone());
    // let mut game = Game::default();
    let super_mario = Player::new(1, "Super Mario".to_string());
    let mushroom = Mushroom::new(2, 10);
    game.add_actor(Box::new(super_mario));
    game.add_actor(Box::new(mushroom));

    let mega_mind = MindController::default();
    game.add_bot(Box::new(mega_mind));
    game.add_bot(Box::new(Confuser::default()));

    // GameEngine::run(&game);

    game.apply();
    game.draw();
    game.update();

    // drop(transmitter);
    // Not: Sürekli dinlemede kalınacağı için loop döngüsü hiçbir zaman başlamaz.
    for r in receiver {
        println!("{}", r);
    }

    loop {
        sleep(Duration::from_secs(2));
        game.draw();
        game.update();
        // state check
    }
}

#[derive(Default)]
struct MindController {}

impl Bot for MindController {
    fn apply(&self) {
        thread::spawn(move || {
            loop {
                println!("\tApplying simulation...{:?}", thread::current().id());
                sleep(Duration::from_secs(5));
            }
        });
    }
}

#[derive(Default)]
struct Confuser {}

impl Bot for Confuser {
    fn apply(&self) {
        thread::spawn(|| {
            loop {
                println!(
                    "\t\tBrain confusing in progress...{:?}",
                    thread::current().id()
                );
                sleep(Duration::from_secs(1));
            }
        });
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
