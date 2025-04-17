use data::{DRONE_MODELS, get_random_between, get_random_number};

mod controller;
mod data;
mod model;
mod tests;

fn main() {
    //todo@buraksenyurt Derste yapılacaklar

    // Başlangıçta sahaya belli sayıda rastgele ama makul koordinatlara çıkacak drone'lar yerleştirelim
    // Belirli periyotlarda sahadaki drone konumlarını terminale loglayalım
    // Terminal loglarını fiziki dosyaya kaydedelim
    // Programın bir sonraki açılışında son konumları dosyadan okuyalım

    for _ in 0..10 {
        let max_value = DRONE_MODELS.len();
        println!("{}", DRONE_MODELS[get_random_number(max_value)]);
    }

    for _ in 0..10 {
        let number = get_random_between(10, 100);
        println!("{number}");
    }
}
