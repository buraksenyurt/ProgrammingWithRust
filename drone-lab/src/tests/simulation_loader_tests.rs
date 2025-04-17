#[cfg(test)]
mod tests {
    use crate::controller::SimulationController;

    #[test]
    fn should_allocate_one_drone_test() {
        let mut simulation = SimulationController::new();
        let actual = simulation.load(1);
        let expected = true;
        assert_eq!(actual, expected);
    }

    #[test]
    fn should_allocate_ten_drone_returns_ten_test() {
        let mut simulation = SimulationController::new();
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
