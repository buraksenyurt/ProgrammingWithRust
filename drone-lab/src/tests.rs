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
        };
        let actual = DroneRepository::save(temperature_drone);
        assert!(actual.is_ok());
    }

    #[test]
    fn should_drone_energy_level_is_low_returns_out_of_range() {
        let temperature_drone = Drone {
            id: 1145,
            model: "UHD Drone TX-19",
            energy_level: 9.0,
            location: Location {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                caption: "Kuzey Blogu",
            },
        };
        let actual = DroneRepository::save(temperature_drone);
        assert!(actual.is_err());
        assert_eq!(actual.err(), Some(DroneSaveError::OutOfRange));
    }
}
