use crate::data::*;
use crate::model::*;
use rand::Rng;
use std::fs::File;
use std::io::Write;

/// # SimulationController Yapısı
///
///  SimulationController, drone'ları yöneten ve simülasyon işlemlerini gerçekleştiren bir yapıdır.
///
/// ## Fields
///
/// * `drones: Vec<Drone>` - Simülasyonda bulunan drone'ların listesini tutar.
///
/// ## Methods
///
/// * `new()` - Yeni bir SimulationController nesnesi oluşturur.
/// * `load(drone_count: i32)` - Sahaya belirtilen sayıda drone yükler.
/// * `get_count()` - Sahada bulunan drone sayısını döndürür.
/// * `get_random()` - Sahada bulunan drone'lar arasından rastgele bir drone döndürür.
/// * `save(path: &str)` - Sahadaki drone'ları belirtilen dosyaya kaydeder.
/// * `load_from(path: &str)` - Belirtilen dosyadan sahaya drone'lar yükler.
///
/// ## Examples
/// ```rust
/// use crate::controller::SimulationController;
/// let mut simulator = SimulationController::new();
/// let load_result = simulator.load(10);
/// let drone_count = simulator.get_count();
/// assert_eq!(drone_count, 10);
/// let random_drone = simulator.get_random();
/// assert!(random_drone.id < 10);
/// let save_result = simulator.save("drones.txt");
/// assert!(save_result.is_ok());
/// let load_result = simulator.load_from("drones.txt");
/// assert!(load_result.is_ok());
/// ```
#[allow(dead_code)]
pub struct SimulationController<'a> {
    drones: Vec<Drone<'a>>,
}

#[allow(dead_code)]
impl SimulationController<'_> {
    /// # Constructor Metot
    ///
    /// Yeni bir SimulationController nesnesi oluşturur.
    ///
    /// ## Returns
    /// * `SimulationController:` Yeni oluşturulan SimulationController nesnesini döndürür.
    ///
    /// ## Examples
    /// ```rust
    /// use crate::controller::SimulationController;
    ///
    /// let simulator = SimulationController::new();
    /// ```
    pub fn new() -> Self {
        SimulationController { drones: Vec::new() }
    }

    /// # Drone Yükleme Fonksiyonu
    ///
    /// Sahaya parametre olarak verilen sayıda drone ekler.
    ///
    /// ## Arguments
    /// * `drone_count: i32` türünden drone sayısını ifade eder
    ///
    /// ## Returns
    /// * `bool:` İşlemin başarılı olup olmadığı bilgisidir
    ///
    /// ## Examples
    /// ```rust
    /// use crate::controller::SimulationController;
    ///
    /// let mut simulator = SimulationController::new();
    /// let load_result = simulator.load(10);
    /// ```
    pub fn load(&mut self, drone_count: i32) -> bool {
        let mut rng = rand::rng();

        for i in 0..drone_count {
            let model = DRONE_MODELS[rng.random_range(0..DRONE_MODELS.len())];
            self.drones.push(Drone {
                id: i as u32,
                energy_level: 100.0,
                model,
                is_alive: true,
                location: Location {
                    x: rng.random_range(0..100) as f32,
                    y: rng.random_range(0..100) as f32,
                    z: rng.random_range(0..100) as f32,
                    caption: LOCATION_CAPTIONS[rng.random_range(0..LOCATION_CAPTIONS.len())],
                },
            })
        }
        true
    }

    /// # Drone Sayısını Alma Fonksiyonu
    ///
    /// Sahada bulunan drone sayısını döndürür.
    ///
    /// ## Returns
    /// * `usize:` Sahada bulunan drone sayısını döndürür.
    ///
    /// ## Examples
    /// ```rust
    /// use crate::controller::SimulationController;
    ///
    /// let mut simulator = SimulationController::new();
    /// let load_result = simulator.load(10);
    /// let drone_count = simulator.get_count();
    /// assert_eq!(drone_count, 10);
    /// ```
    pub fn get_count(&self) -> usize {
        self.drones.len()
    }

    /// # Rastgele Drone Alma Fonksiyonu
    ///
    /// Sahada bulunan drone'lar arasından rastgele bir drone döndürür.
    ///
    /// ## Returns
    /// * `Drone:` Rastgele seçilen drone nesnesini döndürür.
    ///
    /// ## Examples
    /// ```rust
    /// use crate::controller::SimulationController;
    ///
    /// let mut simulator = SimulationController::new();
    /// let load_result = simulator.load(10);
    /// let random_drone = simulator.get_random();
    /// assert!(random_drone.id < 10);
    /// ```
    pub fn get_random(&self) -> Drone {
        let mut rng = rand::rng();
        self.drones[rng.random_range(0..self.drones.len())]
    }

    /// # Kaydetme Fonksiyonu
    ///
    /// Sahadaki drone'ları belirtilen dosyaya kaydeder.
    ///
    /// ## Arguments
    /// * `path: &str` türünden dosya yolunu ifade eder.
    ///
    /// ## Returns
    /// * `std::io::Result<u32>` İşlemin başarılı olup olmadığı bilgisidir.
    ///
    /// ## Examples
    /// ```rust
    /// use crate::controller::SimulationController;
    /// use std::fs::File;
    ///
    /// let mut simulator = SimulationController::new();
    /// let load_result = simulator.load(10);
    /// let save_result = simulator.save("drones.txt");
    /// assert!(save_result.is_ok());
    /// ```
    /// ## Panics
    ///
    /// * Dosya oluşturulamadığında panik yapar.
    /// * Dosya yazma işlemi başarısız olduğunda panik yapar.
    pub fn save(&self, path: &str) -> std::io::Result<u32> {
        let mut content = String::new();
        for drone in &self.drones {
            content.push_str(format!("{}\n", &drone.to_string()).as_str());
        }

        let mut f = File::create(path)?;
        // let mut f = OpenOptions::new().append(true).write(true).open(path)?;
        f.write_all(content.as_bytes())?;
        Ok(content.len() as u32)
    }

    /// # Yükleme Fonksiyonu
    ///
    /// Sahadaki drone'ları belirtilen dosyadan yükler.
    ///
    /// ## Arguments
    /// * `path: &str` türünden dosya yolunu ifade eder.
    ///
    /// ## Returns
    /// * `std::io::Result<()>` İşlemin başarılı olup olmadığı bilgisidir.
    ///
    /// ## Examples
    /// ```rust
    /// use crate::controller::SimulationController;
    /// use std::fs::File;
    ///
    /// let mut simulator = SimulationController::new();
    /// let load_result = simulator.load(10);
    /// let save_result = simulator.save("drones.txt");
    /// let load_result = simulator.load_from("drones.txt");
    /// assert!(load_result.is_ok());
    /// ```
    ///
    /// ## Panics
    /// * Dosya bulunamadığında panik yapar.
    /// * Dosya okuma işlemi başarısız olduğunda panik yapar.
    pub fn load_from(_path: &str) -> std::io::Result<()> {
        //todo@buraksenyurt Aşağıdaki maddeler tamamlanmalı

        // Dosya gerçekten var mı kontrolü?
        // Dosya içeriği dolu mu?
        // Dosya satırları gerçekten Drone verisi mi içeriyor mu? (Satır bazında yapılabilir)
        // Dosya satırlarını Drone türünden nesneler çevirip self üstünde vektöre eklemek lazım
        Ok(())
    }
}

impl Default for SimulationController<'_> {
    fn default() -> Self {
        Self::new()
    }
}
