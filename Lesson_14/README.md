# Ders 14: Asenkron Programlama

Rust eşzamanlı _(concurrent)_ programlama haricinde asenkron programlamayı da destekler. Asenkron programlama özellikle
dosya I/O işlemleri, network operasyonları, zaman bazlı görevler _(scheduled tasks)_ ve servis iletişimi gibi beklemeye
neden olan süreçlerde CPU'nun idle kalmak yerine bahsedilen operasyonları icra etmesi için kullanılan bir yaklaşımdır.
Rust bu ihtiyaca async ve await anahtar kelimeleri ile cevap verir. Rust tarafında asenkron programlama süreçleri
genellikle defacto hale gelmiş çeşitli küfeler _(crates)_ ile sağlanır. Tokio küfesi bunlar arasında en popüler
olanlarındandır. Bunun sebebi async fn çağrıları sonucu dönen Future nesnelerini yönetecek hazır bir ortamın built-in
olarak gelmemesidir.

## Thread vs Async/Await

Thread ve async/await kullanımları sıklıkla birbirlerine karıştırılır. Her iki enstrüman arasındaki farklılıklar
aşağıdaki tabloda özetlenmiştir.

| **Kriter**            | **Thread**                                                                                                                                   | **Async/Await**                                                                                                                                             |
|-----------------------|----------------------------------------------------------------------------------------------------------------------------------------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------|
| **Paralellik**        | Gerçek paralell çalışma ortamı söz konusudur _(çok çekirdek desteği)_                                                                        | Genellikle tek thread üzerinde iş birliği ile çalışır _(Bu bir çalışma zamanı da gerektirir, async-std, tokio gibi)_                                        |
| **Kaynak Kullanımı**  | Ağır _(her thread kendi stack alanını taşır ve bu varsayılan olarak 2Mbtır)_ Thread'ler Idle durumdayken bile enerji sarf ettirir.           | Hafif _(runtime tarafından yönetilen task' lar söz konusudur)_                                                                                              |
| **Bloklama**          | I/O seviyesinde bloklamalar varsa tüm thread'ler etkilenir.                                                                                  | I/O bloklama diğer görevleri etkilemez.                                                                                                                     |
| **Ölçeklenebilirlik** | Thread sayısı fiziksel sınırlamalara bağlıdır _(Çekirdek sayısı gibi)_ Çok fazla thread açılması sistemde aşırı yüklenmelere neden olabilir. | Binlerce asenkron görev oluşturulabilir.                                                                                                                    |
| **Kod Karmaşıklığı**  | Göreceli bir durumdur, nispeten basittir.                                                                                                    | Hata ve bağımlılıkların yönetimi karmaşık olabilir.                                                                                                         |
| **Senaryolar**        | İşlemci yoğun/öncelikli işler için uygundur.                                                                                                 | Daha çok I/O yoğun işler için uygundur.                                                                                                                     |
| **Ne zaman?**         | CPU yoğun işlerde _(Ağır matematiksel hesaplamalar)_, her görevin tam bağımsız ve paralel çalışması gerektiği durumlar                       | Web istekleri, dosya erişimleri gibi I/O yoğun işler, yüksek ölçeklenebilirlik gerektiren hafif işler, enerji ve kaynak tasarrufunun önemli olduğu durumlar |

## Örnekler

Asenkron programlama konseptini anlamanın en iyi yolu gerçek hayat örnekleri üzerinden ilerlemektir. Bir sunucudaki
işlemci, bellek ve disk kullanım durumlarını anlık olarak takip eden bir sistem programı geliştirmek istediğimizi
düşünelim. Burada donanım bazında çalışan fonksiyonellikler olduğunu ifade edebiliriz. Senkron bir okuma yerine asenkron
olarak bu değerlerin okunması sağlanabilir. Aşağıdaki örnek kod parçasında bu durumu tokio küfesi kullanılarak simüle
edilmektedir.

```rust
use rand::Rng;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::task;
use tokio::time::sleep;
#[tokio::main]
async fn main() {
    let (log_transmitter, mut log_receiver) = mpsc::channel(100);

    let cpu_task = task::spawn(fetch_metrics("CPU Service", log_transmitter.clone()));
    let memory_task = task::spawn(fetch_metrics("Memory Service", log_transmitter.clone()));
    let disk_task = task::spawn(fetch_metrics("Disk Service", log_transmitter));

    let logger_task = task::spawn(async move {
        while let Some(metric) = log_receiver.recv().await {
            println!("LOG: {}", metric);
        }
    });

    let _ = tokio::join!(cpu_task, memory_task, disk_task, logger_task);
}

async fn fetch_metrics(service_name: &str, tx: mpsc::Sender<String>) {
    let interval = Duration::from_secs(5);
    for i in 1..=10 {
        let metric = format!("{} - Metric {}: {}", service_name, i, get_metric());
        if tx.send(metric).await.is_err() {
            println!("{}: Channel isn't active!", service_name);
            break;
        }
        sleep(interval).await;
    }
}

fn get_metric() -> f64 {
    let mut rng = rand::rng();
    rng.random_range(50.0..100.0)
}
```

Örnek kod sembolik olarak işlemci, bellek ve disk kullanım oranlarını takip eden fonksiyonellikleri ele alır. Bu tip
işlevler senkron çalışmak yerine eş zamanlı olarak işletilebilirler. task::spawn çağrısı bu görevleri başlatmak için
kullanılır. fetch_metrics metodu **async** keyword'ü ile imzalandığından task::spawn tarafından kullanılabilir. tokio
küfesinden gelen spawn metodunun tanımı aşağıdaki gibidir.

```rust
pub fn spawn<F>(future: F) -> JoinHandle<F::Output>
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{}
```

Dikkat edileceği üzere JoinHandle nesnesi Future ve Send trait'lerini implemente eden, statik yaşam ömrüne sahip bir
enstrümandır. Future trait esasında poll tekniğine göre asenkron olarak başlatılan operasyon tamamlandığında devreye
girileceğini ifade eder. Burada thread'ler arası haberleşme de söz konusudur ve unsafe olan Send trait bunu garanti
eder. Kısacası elimizde asenkron olarak başlatılan operasyonlar ve bu operasyonlar tamamlandığında devreye giren, diğer
thread'leri kesintiye uğratmayan Handler türleri vardır. Tüm JoinHandle nesnelerinin işlerinin tamamlanmasını beklemek
için yine Join mekanizması kullanılır.

Örnekte kullanılan fetch_metrics fonksiyonu metrikler üretildikçe bu değerleri transmitter araclığı ile bir kanala
bırakır. Kanala bırakılan bu bilgiler başka bir task içerisinden receiver nesnesi ile yakalanır ve terminale basılır.

Eş zamanlı görevlerin sık kullanıldığı bir başka senaryo ise, web servislerine gönderilen taleplerle ilgilidir.
Aşağıdaki örnek kod parçasında bir web api hizmetine örnek talepler gönderilmekte ve bu talepler asenkron çalışan
görevler içerisinde ele alınmaktadır.

```rust
#[tokio::main]
pub async fn main() {
    let task_a = task::spawn(fetch_data_async(
        "https://jsonplaceholder.typicode.com/posts/1",
    ));
    let task_b = task::spawn(fetch_data_async(
        "https://jsonplaceholder.typicode.com/posts/2",
    ));
    let task_c = task::spawn(fetch_data_async(
        "https://jsonplaceholder.typicode.com/posts/3",
    ));

    let (res_a, res_b, res_c) = tokio::join!(task_a, task_b, task_c);

    match (res_a, res_b, res_c) {
        (Ok(a), Ok(b), Ok(c)) => {
            println!("{:?}", a);
            println!("{:?}", b);
            println!("{:?}", c);
        }
        _ => println!("Failed to fetch data"),
    }
}

async fn fetch_data_async(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::get(url).await?;
    response.text().await
}
```

Web Api türünden servisler HTTP protokolünün Get, Post, Put, Delete, Patch gibi metotlarını kullanan Restful mimariye
göre tasarlanmış hizmetlerdir. Genellikle JSON türünden veriler kullanırlar. Örnekte kullanılan dummy servis ile olan
iletişimi kolaylaştırmak için reqwest isimli bir küfe kullanılmıştır. task_a, task_b ve task_c nesneleri ile söz konusu
servise üç ayrı talep yapılır. Tüm bu talepler fetch_data_async isimli asenkron fonksiyon tarafından eş zamanlı olarak
ele alınır. Api servisinden HTTP Get metodu ile veri çekme işi reqwest'in get metodu ile gerçekleştirilir ki bu metot da
asenkron olarak çağrılabilir. await çağrısı söz konusu fonksiyona ait Future'un bir sonuç elde edene kadar beklenmesini
söyler ancak bu diğer thread'leri engelleyen bir durum değildir.

Servis haberleşmeleri ağ ortamlarında gerçekleşen süreçler olduğundan ana akışı bekletmeye neden olurlar. Cevap süreleri
çok yüksek olsa dahi eş zamanlı olarak sayısız talebin ele alındığı durumlarda servis görevlerini asenkron başlatmak
tercih edilen bir çözümdür. Bu mantık bir web sunucusu yazarken de geçerlidir.
