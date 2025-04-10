use crate::model::Drone;

pub struct DroneRepository;

impl DroneRepository {
    pub fn save(drone: Drone) -> Result<bool, DroneSaveError> {
        if drone.energy_level <= 10.0 {
            return Err(DroneSaveError::OutOfRange);
        }
        Ok(true)
    }
}

#[derive(Debug, PartialEq)]
pub enum DroneSaveError {
    InvalidDroneId,
    OutOfRange,
}
