pub struct BotContainer {
    is_active: bool,
    bot: Box<dyn Bot>,
}
impl BotContainer {
    pub fn new(is_active: bool, bot: Box<dyn Bot>) -> Self {
        BotContainer { is_active, bot }
    }
}

pub trait Bot {
    fn apply(&self) {
        println!("Default AI movements");
    }
}

#[allow(dead_code)]
pub trait Actor {
    fn draw(&self);
    fn update(&mut self);
}

#[derive(Default)]
#[allow(dead_code)]
pub struct Game {
    // width: f32,
    // height: f32,
    size: Size,
    actors: Vec<Box<dyn Actor>>,
    bots_container: Vec<BotContainer>,
}

impl Game {
    // pub fn new() -> Self {
    //     Game {
    //         actors: Vec::new(),
    //         size:Size::default()
    //     }
    // }
    pub fn add_actor(&mut self, actor: Box<dyn Actor>) {
        self.actors.push(actor);
    }
    pub fn draw(&self) {
        for actor in &self.actors {
            actor.draw();
        }
    }
    pub fn add_bot(&mut self, bot: Box<dyn Bot>) {
        self.bots_container.push(BotContainer::new(true, bot));
    }
    pub fn update(&mut self) {
        for actor in &mut self.actors {
            actor.update();
        }
    }
    pub fn apply(&self) {
        for container in &self.bots_container {
            if container.is_active {
                container.bot.apply();
            }
        }
    }
}

// #[derive(Default)]
#[allow(dead_code)]
pub struct Size {
    width: f32,
    height: f32,
}

impl Default for Size {
    fn default() -> Self {
        Size {
            width: 1280_f32,
            height: 960_f32,
        }
    }
}
