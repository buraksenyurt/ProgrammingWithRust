use crate::model::{Drone, Location};

pub struct SimulationController<'a> {
    drones: Vec<Drone<'a>>,
}

impl<'a> SimulationController<'a> {
    pub fn new() -> Self {
        SimulationController { drones: Vec::new() }
    }
    pub fn load(&mut self, drone_count: i32) -> bool {
        for i in 0..drone_count {
            self.drones.push(Drone {
                id: 1,
                energy_level: 100.0,
                model: "MODEL X",
                is_alive: true,
                location: Location {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
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
