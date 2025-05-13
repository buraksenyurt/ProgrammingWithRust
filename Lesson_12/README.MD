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

Bu örnekte spawn metodu ile açılan thread içerisinde message değişkeninin sahip olduğu veri ana thread'de receiver ile yakalanır ve ekrana basılır. channel çağrısı generic Sender ve Receiver veri yapılarının generic nesne örneklerini gönderir. Buna göre transmitter _(yani Sender)_ nesnesini kullanarak bir thread içerisinde kanala mesaj gönderimi sağlanabilir. Bu örnekte String türünden bir içerik gönderilmektedir. Receiver nesne örneğinin recv fonksiyonu ile de kanala bırakılan mesaj yakalanmaktadır.
