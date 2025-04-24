use crate::model::{Drone, Location};
/// # Drone Uçuş Kontrol Nesnesi
///
/// Sahada bulunan drone'lara ait uçuş kontrollerini sağlar.
pub struct FlightController;

#[allow(dead_code)]
impl FlightController {
    /// # Durum Kontrol Metodu
    ///
    /// Drone nesnesinin durumunu kontrol eder.
    ///
    /// ## Arguments
    /// * `drone: &Drone` - Kontrol edilecek drone nesnesi
    ///
    /// ## Returns
    /// * `DroneStatus` - Drone'un durumunu temsil eden enum
    ///
    /// ## Examples
    /// ```rust
    /// use crate::controller::FlightController;
    /// use crate::model::{Drone, Location};
    ///
    /// let drone = Drone {
    ///     id: 1,
    ///     is_alive: true,
    ///     energy_level: 50.0,
    ///     model: "DJI Mavic".to_string(),
    ///     location: Location {
    ///         x: 100.0,
    ///         y: 200.0,
    ///         z: 300.0,
    ///         caption: "Test Location".to_string(),
    ///     },
    /// };
    ///
    /// let status = FlightController::check_status(&drone);
    /// assert_eq!(status, DroneStatus::LowBattery(BatteryRate(50.0)));
    /// ```
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

/// # Drone Durum Enum'u
/// 
/// Drone'un durumunu temsil eden enum'dur.
/// 
/// ## Enum Variants
/// 
/// * `OutOffRange(Location)` - Drone'un uçuş alanının dışına çıktığını belirtir.
/// * `Offline` - Drone'un çevrimdışı olduğunu belirtir.
/// * `LowBattery(BatteryRate)` - Drone'un pil seviyesinin düşük olduğunu belirtir.
/// * `Fine` - Drone'un durumunun iyi olduğunu belirtir.
#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum DroneStatus<'a> {
    OutOffRange(Location<'a>),
    Offline,
    LowBattery(BatteryRate),
    Fine,
}

/// # Pil Seviyesi Yapısı
/// 
/// Pil seviyesini temsil eden yapıdır.
/// 
/// ## Fields
/// 
/// * `0` - Pil seviyesini temsil eden f32 değeridir.
/// 
/// ## Links
/// 
/// Bu konuda [Primitive Obsession](https://refactoring.guru/smells/primitive-obsession) makalesinden
/// daha detaylı bilgi alınabilir.
#[derive(Debug, PartialEq)]
pub struct BatteryRate(pub f32);
