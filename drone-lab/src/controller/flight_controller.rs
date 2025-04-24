use crate::model::{Drone, Location};
pub struct FlightController;

#[allow(dead_code)]
impl FlightController {
    pub fn check_status<'a>(drone: &'a Drone<'a>) -> DroneStatus<'a> {
        // Harici bir REST Api'den bilgileri çektiğimizi düşünelim.
        // Örneğin http://localhost:4980/drone/api/states/{id} HTTP Get
        if !drone.is_alive {
            return DroneStatus::Offline;
        }
        if drone.energy_level < 30.0 {
            return DroneStatus::LowBattery(BatteryRate(drone.energy_level));
        }
        if drone.location.x > 800.0 || drone.location.y > 800.0 || drone.location.z > 800.0 {
            return DroneStatus::OutOffRange(drone.location);
        }

        DroneStatus::Fine
    }
}
#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum DroneStatus<'a> {
    OutOffRange(Location<'a>),
    Offline,
    LowBattery(BatteryRate),
    Fine,
}

#[derive(Debug, PartialEq)]
pub struct BatteryRate(pub f32);
