#[cfg(test)]
mod tests {

    use crate::controller::*;
    use crate::model::*;

    #[test]
    fn should_return_offline_test() {
        let discovery_drone = Drone {
            id: 0,
            model: "DIS Drone TX-258",
            energy_level: 9.0,
            location: Location {
                x: 450.0,
                y: 500.0,
                z: 801.0,
                caption: "Sahra II Bölgesi",
            },
            is_alive: false,
        };
        let actual = FlightController::check_status(&discovery_drone);
        assert_eq!(actual, DroneStatus::Offline);
    }

    #[test]
    fn should_return_low_battery_test() {
        let energy_level = 25.0;
        let discovery_drone = Drone {
            id: 0,
            model: "DIS Drone TX-258",
            energy_level,
            location: Location {
                x: 450.0,
                y: 500.0,
                z: 801.0,
                caption: "Sahra II Bölgesi",
            },
            is_alive: true,
        };
        let actual = FlightController::check_status(&discovery_drone);
        assert_eq!(actual, DroneStatus::LowBattery(energy_level));
    }

    #[test]
    fn should_return_out_off_range_test() {
        let location = Location {
            x: 450.0,
            y: 500.0,
            z: 801.0,
            caption: "Sahra II Bölgesi",
        };
        let discovery_drone = Drone {
            id: 0,
            model: "DIS Drone TX-258",
            energy_level: 55.0,
            location,
            is_alive: true,
        };
        let actual = FlightController::check_status(&discovery_drone);
        assert_eq!(actual, DroneStatus::OutOffRange(location));
    }

    #[test]
    fn should_return_fine_test() {
        let location = Location {
            x: 450.0,
            y: 500.0,
            z: 750.0,
            caption: "Sahra II Bölgesi",
        };
        let discovery_drone = Drone {
            id: 19,
            model: "DIS Drone TX-258",
            energy_level: 80.0,
            location,
            is_alive: true,
        };
        let actual = FlightController::check_status(&discovery_drone);
        assert_eq!(actual, DroneStatus::Fine);
    }
}
