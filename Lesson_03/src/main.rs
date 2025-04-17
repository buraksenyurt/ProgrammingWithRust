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
    let mut cpu_usages = Vec::new();

    // Sürekli metrik çekeceğimiz için bir sonsuz döngü işimizi görebilir
    /*
        cargo run -- 100 2 cpu json
            (2 saniyede bir CPU kullanımlarından toplam 100 tane log çek ve json formatında yaz)
        cargo run -- 100 2 mem
            (2 saniyede bir Memory kullanımlarından toplam 100 tane log çek
            ve standart formatta csv olarak yaz)

        cargo run -- -c 100 -p 2 -s mem -f json

        teknik ? Tüm vector içeriğin program kapanırken topluca dosya yazmak bir seçenek
        veya hiç vector kullanmadan stdout üzerinde log ürettikçe dosyaya append et
    */
    loop {
        system.refresh_all(); // Tüm metrikleri güncelle

        println!(
            "Free memory size {} Gb",
            system.free_memory() / (1024 * 1024)
        );
        /*
           Plan :

           CPU kullanım değeri %0 ile %50 arasında olanlar için log seviyesi NORMAL
           CPU kullanım değeri %50 ile %75 arasında olanlar için log seviye WARM
           CPU kullanım değeri %70 ile %90 arasında olanlar için log seviye BURNING
           CPU kullanım değeri %90 üstü olanlar için log seviye ALARM
           diğer CPU kullanım değerleri için log seviye UNKNOWN

           Bu planı uygulamak için pattern matchin'i aşağıdaki gibi kullandık

        */

        for (idx, cpu) in system.cpus().iter().enumerate() {
            // print!(" {} {:2.2}%", idx, cpu.cpu_usage());
            let description = format!("{} ({:2.2}%)", idx, cpu.cpu_usage());
            let level = match cpu.cpu_usage() {
                0.0..50.0 => Level::NORMAL,
                50.0..75.0 => Level::WARM,
                75.0..90.0 => Level::BURNING,
                90.0..100.0 => Level::ALARM,
                _ => Level::UNKNOWN,
            };

            /*
                Sınavda bu konudan soru gelir...

                Neden clone kullandık?
                clone kullanmadığımızda nasıl bir hata mesajı oluştu?
                Value used after being moved [E0382] Hatasının sebebi nedir?
            */
            let cpu_log = SystemLog::new(idx as u32, Some(Utc::now()), level, description.clone());
            print!("{}", description);

            cpu_usages.push(cpu_log);
        }
        println!();
        sleep(Duration::from_secs(2)); // Şu anki main thread 1 saniye duraksatılır
    }
    //TODO@buraksenyurt Sonsuz döngüden çıkardıktan sonra sonuçları bir dosyaya yazdıralım (CSV veya JSON)
}

#[derive(Debug)]
pub enum Level {
    NORMAL,
    WARM,
    BURNING,
    ALARM,
    UNKNOWN,
}
#[derive(Debug)]
#[allow(dead_code)]
struct SystemLog {
    id: u32,
    // Senaryomuza göre time_stamp bilgisinin olup olmadığı belli değil
    // bu nedenle Option şeklinde ele alınabilir (Yani Some(value) veya None olarak kullanılır)
    // None memory de nasıl tutulur?
    time_stamp: Option<DateTime<Utc>>,
    level: Level,
    description: String,
}

impl SystemLog {
    pub fn new(id: u32, time_stamp: Option<DateTime<Utc>>, level: Level, desc: String) -> Self {
        SystemLog {
            id,
            level,
            time_stamp,
            description: desc,
        }
    }
}
