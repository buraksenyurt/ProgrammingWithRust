use crate::data::*;
use crate::model::*;

#[allow(dead_code)]
pub struct SimulationController<'a> {
    drones: Vec<Drone<'a>>,
}

#[allow(dead_code)]
impl SimulationController<'_> {
    pub fn new() -> Self {
        SimulationController { drones: Vec::new() }
    }
    pub fn load(&mut self, drone_count: i32) -> bool {
        for i in 0..drone_count {
            let model = DRONE_MODELS[get_random_number(DRONE_MODELS.len())];
            self.drones.push(Drone {
                id: i as u32,
                energy_level: 100.0,
                model,
                is_alive: true,
                location: Location {
                    x: get_random_between(1, 100) as f32,
                    y: get_random_between(1, 100) as f32,
                    z: get_random_between(1, 100) as f32,
                    caption: "",
                },
            })
        }
        true
    }
    pub fn get_count(&self) -> usize {
        self.drones.len()
    }
    pub fn get_random(&self) -> Drone {
        self.drones[0]
    }
}
