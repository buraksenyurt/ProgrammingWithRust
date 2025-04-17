use crate::model::Drone;

pub struct DroneRepository;

#[allow(dead_code)]
impl DroneRepository {
    pub fn save(drone: Drone) -> Result<bool, SaveValidationError> {
        if drone.id == 0 {
            return Err(SaveValidationError::InvalidDroneId);
        }
        if drone.model.len() < 5 {
            return Err(SaveValidationError::WrongModelName);
        }
        Ok(true)
    }
}

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum SaveValidationError {
    InvalidDroneId,
    WrongModelName,
}
