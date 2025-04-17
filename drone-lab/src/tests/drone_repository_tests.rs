#[cfg(test)]
mod tests {

    use crate::controller::*;
    use crate::model::*;

    #[test]
    fn should_drone_save_returns_ok_test() {
        let temperature_drone = Drone {
            id: 1145,
            model: "UHD Drone TX-19",
            energy_level: 100.0,
            location: Location {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                caption: "Kuzey Blogu",
            },
            is_alive: true,
        };
        let actual = DroneRepository::save(temperature_drone);
        assert!(actual.is_ok());
    }

    #[test]
    fn should_wrong_drone_id_returns_validation_error() {
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
            is_alive: true,
        };
        let actual = DroneRepository::save(discovery_drone);
        assert!(actual.is_err());
        assert_eq!(actual.err(), Some(SaveValidationError::InvalidDroneId));
    }
}
