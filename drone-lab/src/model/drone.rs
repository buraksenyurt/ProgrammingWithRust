use crate::model::location::Location;
use std::fmt::Display;

#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
pub struct Drone<'a> {
    pub id: u32,
    pub model: &'a str,
    pub energy_level: f32,
    pub location: Location<'a>,
    pub is_alive: bool,
}

impl Display for Drone<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{} ({}%),{},{}",
            self.model,
            self.energy_level,
            self.location,
            // self.location.x,
            // self.location.y,
            // self.location.z,
            match self.is_alive {
                true => "alive",
                false => "dead",
            }
        )
    }
}

// #[allow(dead_code)]
// pub trait Update {
//     fn update(&mut self) {
//         println!("Default behavior")
//     }
// }
//
// pub struct Player {}
// impl Update for Player {
//     //fn update(&mut self) {}
// }
// pub struct Orc {}
// impl Update for Orc {
//     fn update(&mut self) {}
// }
// fn update_all(objects: Vec<&dyn Update>) {}
