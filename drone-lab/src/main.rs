use crate::controller::SimulationController;

mod controller;
mod data;
mod model;
mod tests;

fn main() {
    // todo@buraksenyurt Derste yapılacaklar
    // Aynı davranışları uygulayan farklı veri yapılarını kullanabilen generic bir Simulation modeli oluşturmayı deneyelim

    let mut simulator = SimulationController::new();

    let load_result = simulator.load(10);
    if load_result {
        for _ in 0..3 {
            let drone = simulator.get_random();
            println!("{}", drone);
        }
    }

    match simulator.save("Drones.dat") {
        Ok(length) => {
            println!(
                "Veriler dosyaya aktarıldı. Toplam içerik boyutu {} karakter",
                length
            );
        }
        Err(e) => eprintln!("{}", e),
    }

    // Terminal loglarını fiziki dosyaya kaydedelim
    // Programın bir sonraki açılışında son konumları dosyadan okuyalım

    // for _ in 0..10 {
    //     let max_value = DRONE_MODELS.len();
    //     println!("{}", DRONE_MODELS[get_random_number(max_value)]);
    // }
    //
    // for _ in 0..10 {
    //     let number = get_random_between(10, 100);
    //     println!("{number}");
    // }
}
