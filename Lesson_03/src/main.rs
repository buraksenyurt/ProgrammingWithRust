/*
    crates.io' dan harici paketler eklemek için cargo paket yönetim aracı kullanılabilir

        cargo add chrono,
        cargo add rand

    hatta şöyle kullanılabilir.

        cargo add chrono rand actix-web

    -F ile feature set'te istenen ek özellikler de yüklenebilir. Örneğin,

        cargo add serde -F derive

    Yüklü bir paketi kaldırmak içinse

        cargo remove syslog

    kullanılabilir.
*/
use std::thread::sleep;
use std::time::Duration;
// Kodun devam edam kısımlarında kullanılan enstrümanların yer aldığı modüller use ile bildirilmelidir.
use chrono::{DateTime, Utc};

/*
    Lesson_02' deki senaryoyı değiştirip devam ediyoruz.
    Bu sefer sysinfo küfesinden (crate) yararlanarak uygulamanın çalıştığı sistemden
    bir takım loglar üreteceğiz.

    Ödev senaryosu : Uygulamaya terminalden ne kadar süre
    boyunca hangi aralıklarda dinleme yapacağını söyleyelim.
*/
fn main() {
    let mut system = sysinfo::System::new_all();

    // Sürekli metrik çekeceğimiz için bir sonsuz döngü işimizi görebilir
    loop {
        system.refresh_all(); // Tüm metrikleri güncelle

        println!(
            "Free memory size {} Gb",
            system.free_memory() / (1024 * 1024)
        );
        for (idx, cpu) in system.cpus().iter().enumerate() {
            print!(" {} {:2.2}%", idx, cpu.cpu_usage());
        }
        println!();

        sleep(Duration::from_secs(2)); // Şu anki main thread 1 saniye duraksatılır
    }
}

#[derive(Debug)]
pub struct SystemLog {
    pub id: u32,
    // Senaryomuza göre time_stamp bilgisinin olup olmadığı belli değil
    // bu nedenle Option şeklinde ele alınabilir (Yani Some(value) veya None olarak kullanılır)
    // None memory de nasıl tutulur?
    pub time_stamp: Option<DateTime<Utc>>,
    pub level: String,
    pub description: String,
}

impl SystemLog {
    pub fn new(id: u32, time_stamp: Option<DateTime<Utc>>, level: String, desc: String) -> Self {
        SystemLog {
            id,
            level,
            time_stamp,
            // time_stamp: Utc::now(),
            description: desc,
        }
    }
}
