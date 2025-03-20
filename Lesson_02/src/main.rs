/*
    Senaryomuz : Elimizde farklı seviyelerden LOG kayıtları olduğunu düşünelim.
    Bunlar içinden en son gerçekleşen 3 hata logunu almak istiyoruz(veya buna benzer sorgular)

    Log seviyeleri : INFO, ERROR, WARN, DEBUG
    Örnek log kaynağımızın bir dosyadan okunmuş aşağıdaki içerik olduğunu düşünelim.

    10001|DEBUG|Application started
    10002|WARN|Too many request
    10003|ERROR|Bad request HTTP 400
    10004|INFO|User request accepted
    10005|ERROR|Authorization error
    10006|ERROR|Malicious request
    10007|INFO|Payment process has been completed
*/
use crate::system_log::SystemLog;

mod system_log;

fn main() {
    let log_lines = vec![
        "10001|DEBUG|Application started",
        "10002|WARN|Too many request",
        "10003|ERROR|Bad request HTTP 400",
        "10004|INFO|User request accepted",
        "10005|ERROR|Authorization error",
        "10006|ERROR|Malicious request",
        "10007|INFO|Payment process has been completed",
    ];
    // Her değişken başlangıç itibariyle immutable'dır
    // Değiştirilebilir olması için mut keyword ile mutable hale getirilir.
    let mut system_logs = Vec::new();

    for line in log_lines {
        let columns: Vec<&str> = line.split('|').collect();
        let sys_log = SystemLog::new(
            columns[0].parse::<u32>().unwrap(),
            // Geliştirme modunda olduğumuz için unwrap ile geçtik.
            // Gerçek hayat senaryolarında hata yönetimi eklemek gerekir
            columns[1].to_string(),
            columns[2].to_string(),
        );
        system_logs.push(sys_log);
        //println!("{:?}", sys_log); // # işareti daha okunabilir bir çıktı verir
        // println!("{} {} {}", columns[0], columns[1], columns[2]);
    }

    // println!("{:#?}", system_logs);
    let error_logs = filter_error_logs(&system_logs, 3, Order::Ascending);
    println!("Error logs\n {:#?}", error_logs);
}
enum Order {
    Ascending,
    Descending,
}

/*
    Bu fonksiyon ilk parametrede Vector' ün referansını almaktadır. Tüm vektörü kopyalamak yerine,
    referansı ile bu metoda ödünç vermek performans açısından daha idealdir.

    count ile kaç adet hata logu almak istediğimizi, order ile de hangi sıralamaya göre almak
    istediğimizi belirtiyoruz.

    Fonksiyonumuz geriye yine bir Vector dönüyor ancak dikkat edileceği üzere,
    kaynak olarak gelen vektördeki SystemLog değişkenlerinin referansları ile dönülüyor.

    İyileştirme noktaları;

    - Sadece Error loglar için değil herhangi bir seviyedeki log için genişletilebilir
    - Hatta log seviyesi de bir Enum türü ile ifade edilebilir
    - Order kabiliyeti genişletilebilir. Varsayılan olarak Id alanına göre sıralama yapar
      ama başka bir alana göre sıralama seçeneği de sunulabilir
    - Aynı işlemler rust'ın iterasyon fonksiyonları ile daha kolay yapılabilir. (Zero-Cost Abstraction)
*/

//TODO@buraksenyurt İyileştirme noktalarını tamamlayalım
fn filter_error_logs(system_logs: &Vec<SystemLog>, count: u8, order: Order) -> Vec<&SystemLog> {
    let mut error_logs = Vec::new();
    let mut counter = 0;

    for log in system_logs {
        if counter == count {
            break;
        }
        if log.level.to_uppercase() == "ERROR" {
            counter += 1;
            error_logs.push(log);
        }
    }

    error_logs
}
