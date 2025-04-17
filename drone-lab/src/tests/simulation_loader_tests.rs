#[cfg(test)]
mod tests {
    use crate::model::{Drone, Location};

    struct Simulation<'a> {
        drones: Vec<Drone<'a>>,
    }
    impl<'a> Simulation<'a> {
        fn new() -> Self {
            Simulation { drones: Vec::new() }
        }
        fn load(&mut self, drone_count: i32) -> bool {
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
        fn get_count(&self) -> usize {
            self.drones.len()
        }
        fn get_random(&self) -> Drone {
            self.drones[0]
        }
    }

    #[test]
    fn should_allocate_one_drone_test() {
        let mut simulation = Simulation::new();
        let actual = simulation.load(1);
        let expected = true;
        assert_eq!(actual, expected);
    }

    #[test]
    fn should_allocate_ten_drone_returns_ten_test() {
        let mut simulation = Simulation::new();
        let _ = simulation.load(10);
        let actual = simulation.get_count();
        assert_eq!(actual, 10);
    }

    // #[test]
    // fn should_any_drone_in_any_location_test() {
    //     let mut simulation = Simulation::new();
    //     let _ = simulation.load(10);
    //     let any_drone: Drone = simulation.get_random();
    //     assert!(any_drone.location.x > 0.0);
    //     // assert!(any_drone.location.y > 0.0);
    //     // assert!(any_drone.location.z > 0.0);
    // }
}
