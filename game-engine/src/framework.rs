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
    bots: Vec<Box<dyn Bot>>,
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
        self.bots.push(bot);
    }
    pub fn update(&mut self) {
        for actor in &mut self.actors {
            actor.update();
        }
        for bot in &self.bots {
            bot.apply();
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
