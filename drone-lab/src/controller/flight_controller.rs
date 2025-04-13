use crate::model::Drone;

pub struct FlightController;

impl FlightController {
    pub fn check_status(drone: &Drone) -> DroneStatus {
        if !drone.is_alive {
            return DroneStatus::Offline;
        }
        if drone.energy_level < 30.0 {
            return DroneStatus::LowBattery(drone.energy_level);
        }
        if drone.location.x > 800.0 || drone.location.y < 800.0 || drone.location.z < 800.0 {
            return DroneStatus::LowBattery(drone.location.x);
        }

        DroneStatus::Fine
    }
}

pub enum DroneStatus {
    OutOffRange,
    Offline,
    LowBattery(f32),
    Fine,
}
