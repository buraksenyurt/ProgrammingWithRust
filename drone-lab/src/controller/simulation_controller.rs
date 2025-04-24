//! # Simulation Module
//! Dron simulasyon nesnelerini oluşturan, yöneten modül enstrümanlarını içerir
//!
//! ## Structs
//! * `SimulationController`

use crate::data::*;
use crate::model::*;
use rand::Rng;
use std::fs::File;
use std::io::Write;

#[allow(dead_code)]
pub struct SimulationController<'a> {
    drones: Vec<Drone<'a>>,
}

#[allow(dead_code)]
impl SimulationController<'_> {
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
    pub fn get_count(&self) -> usize {
        self.drones.len()
    }
    pub fn get_random(&self) -> Drone {
        let mut rng = rand::rng();
        self.drones[rng.random_range(0..self.drones.len())]
    }

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

    pub fn load_from(path: &str) -> std::io::Result<()> {
        //todo@buraksenyurt Aşağıdaki maddeler tamamlanmalı

        // Dosya gerçekten var mı kontrolü?
        // Dosya içeriği dolu mu?
        // Dosya satırları gerçekten Drone verisi mi içeriyor mu? (Satır bazında yapılabilir)
        // Dosya satırlarını Drone türünden nesneler çevirip self üstünde vektöre eklemek lazım
        Ok(())
    }
}
