use crate::model::{Drone, Location};
pub struct FlightController;

impl FlightController {
    pub fn check_status<'a>(drone: &'a Drone<'a>) -> DroneStatus<'a> {
        if !drone.is_alive {
            return DroneStatus::Offline;
        }
        if drone.energy_level < 30.0 {
            return DroneStatus::LowBattery(drone.energy_level);
        }
        if drone.location.x > 800.0 || drone.location.y > 800.0 || drone.location.z > 800.0 {
            return DroneStatus::OutOffRange(drone.location);
        }

        DroneStatus::Fine
    }
}
#[derive(Debug, PartialEq)]
pub enum DroneStatus<'a> {
    OutOffRange(Location<'a>),
    Offline,
    LowBattery(f32),
    Fine,
}
