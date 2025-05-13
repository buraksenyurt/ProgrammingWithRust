# Ders 12: Channels

Bazı durumlarda thread'ler arasında veri aktarımı gerekebilir. Channel enstrümanı farklı thread'ler arasında veri taşınması için kullanılmaktadır. Rust standart kütüphanesi varsayılan olarak **Multi Producer - Single Consumer** türünü destekler. Farklı senaryolar için tokio, crossbeam gibi küfelerden de yararlanılabilir. **MPSC** senaryosunda producer yani veri yayımını gerçekleştiren n sayıda iş parçacığı varken tek bir tüketici ya da dinleyici söz konusudur. Kanallar aşağıdaki gibi senaryolarda sıklıkla tercih edilir;

- Görev _(task)_ sonuçlarının ana bir thread içerisine toplanması
- HTTP isteklerinin paralel olarak işlenmesi
- Sistemde gerçekleşen olayların ana bir döngüye yönlendirilmesi
- GUI _(Graphic User Interface)_ olaylarının merkezi bir yürütücüye aktarılması
- Okuma → işleme → yazma akışının parçalanarak thread’lere dağıtılması
- Merkezi log toplayıcı yapılar

Kanallarda sahiplik _(Ownership)_ kurallarına uygunluk vardır. Veri gönderildiğinde, alıcı taraf bunu recv() veya iter() fonksiyonları ile alır. Sahiplik göndericiden devralınır. Asenkron ve senkron uyarlamaları yazmak mümkündür. **Tokio** veya **async-std** gibi asenkron çalışma ortamları için defacto haline gelmiş olan **tokio::sync::mpsc** küfesi kullanılır. Performans gereksinimi olan durumlarda ise **crossbeam-channel** tercih edilebilir.

## Temel Kullanım

Kanal kullanımını en basit haliyle aşağıdaki gibi ele alabiliriz.

```rust
use std::sync::mpsc::channel;
use std::thread;

fn main() {
    hello_channels();
}

pub fn hello_channels() {
    let (transmitter, reciever) = channel();
    let message = String::from("Sample content");

    thread::spawn(move || {
        transmitter.send(message).unwrap();
    });

    let data = reciever.recv().unwrap();
    println!("{}", data);
}
```

Bu örnekte spawn metodu ile açılan thread içerisinde message değişkeninin sahip olduğu veri ana thread'de receiver ile yakalanır ve ekrana basılır. channel çağrısı generic Sender ve Receiver veri yapılarının generic nesne örneklerini gönderir. Buna göre transmitter _(yani Sender)_ nesnesini kullanarak bir thread içerisinde kanala mesaj gönderimi sağlanabilir. Bu örnekte String türünden bir içerik gönderilmektedir. Receiver nesne örneğinin recv fonksiyonu ile de kanala bırakılan mesaj yakalanmaktadır. recv fonksiyonu kanaldaki mesaj gelene kadar çalıştığı thread'i bekletir. Örnekte dikkat edilmesi gereken noktalardan birisi de message değişkenini kanala gönderdikten sonra yeniden kullanmaya çalışmaktır. Bu aşağıdaki hatanın oluşmasına sebebp olur.

```text
error[E0382]: borrow of moved value: `message`
  --> Lesson_12/src/main.rs:17:28
   |
   |     let message = String::from("Sample content");
   |         ------- move occurs because `message` has type `String`, which does not implement the `Copy` trait
   |
   |     thread::spawn(move || {
   |                   ------- value moved into closure here
   |         transmitter.send(message).unwrap();
   |                          ------- variable moved due to use in closure
...
   |     println!("{},{}", data,message);
   |                            ^^^^^^^ value borrowed here after move
   |
```

Tabii bu durumda copy trait'i ile beslenen türler için söz konusu olmaz zira ilgili veriler kanala kopyalanarak taşınır.

## Multi-Producer Kullanımı

Aşağıdaki örnek kod parçasında ise birden fazla gönderici ele alınır.

```rust
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

fn main() {
    multi_producer();
}

pub fn multi_producer() {
    let (transmitter, receiver) = channel();

    for i in 0..10 {
        let transmitter_clone = transmitter.clone();
        thread::spawn(move || {
            transmitter_clone
                .send(format!("Sending message is {}", i))
                .unwrap();
            thread::sleep(Duration::from_secs(2));
        });
    }

    drop(transmitter);

    for received in receiver {
        println!("Incoming message is '{}'", received);
    }

    println!("End of program");
}
```

Bu örnekte 10 farklı **thread** kanala mesaj bırakır. Thread'ler **spawn** çağırısı ile ayağa kaldırılmadan önce **transmitter** nesnesinin bir klonunun oluşturulduğunda dikkat edilmelidir. Her bir **thread** kendi **transmitter** klonunu kullanarak kanala mesaj bırakır. Mesajlar kanala senkron sırada bırakılır. İlerleyen satırlarda bir **for** döngüsü ile kanala gelen mesajların **Receiver** nesnesi üzerinden yakalanması işlemi gerçekleştirilir. Dikkat edilmesi gereken noktalardan birisi de **drop** çağrısıdır. Açık bir şekilde **transmitter** nesnesi açıkça **drop** edilmiştir. Bu yapılmadığı durumda receiver dan mesajlar dinlenmeye devam eder ve program sonlanmaz. Bu bazı durumlarda zaten istenen bir durumdur. Sürekli dinlemede kalması gereken bir receiver gerektiren senaryolar buna örnek verilebilir.
