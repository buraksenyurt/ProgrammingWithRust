use crate::model::Drone;

/// # Drone Repository
///
/// Drone nesneleri için CRUD işlemlerini gerçekleştiren yapı.
pub struct DroneRepository;

#[allow(dead_code)]
impl DroneRepository {
    /// # Drone Kaydetme Metodu
    ///
    /// Drone nesnesini kaydeder.
    ///
    /// ## Arguments
    /// * `drone: Drone` - Kaydedilecek drone nesnesi
    ///
    /// ## Returns
    /// * `Result<bool, SaveValidationError>` - İşlem sonucu
    ///
    /// ## Errors
    /// * `SaveValidationError::InvalidDroneId` - Geçersiz drone ID hatası
    /// * `SaveValidationError::WrongModelName` - Yanlış model adı hatası
    ///
    /// ## Examples
    /// ```rust
    /// use crate::controller::DroneRepository;
    /// use crate::model::Drone;
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
    /// let result = DroneRepository::save(drone);
    /// assert!(result.is_ok());
    /// ```
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

/// # Kaydetme Hata Enum'u
/// 
/// Drone kaydetme işlemi sırasında oluşabilecek hataları temsil eder.
/// 
/// ## Enum Variants
/// 
/// * `InvalidDroneId` - Geçersiz drone ID hatası
/// * `WrongModelName` - Yanlış model adı hatası
#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum SaveValidationError {
    InvalidDroneId,
    WrongModelName,
}
