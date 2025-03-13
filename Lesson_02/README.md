# Ders 02: Akış Kontrol İfadeleri ve Döngüler

Kodun akışını yönlendirmek, tekrarlı bazlı işlemleri ele almak için diğer programlama dillerinde de olduğu gibi Rust
tarafındada koşullu ifade ve döngü enstrümanlarından yararlanılır. Aşağıdaki dokümanda bu kullanımlara ait örnekler yer
almaktadır.

## if, else Kullanımı

if yapıları, koşullu ifadeleri yönetmek için kullanılır. Aşağıdaki örnekte number değerinin 2 veya 3 ile bölünüp
bölünemediği kontrol edilmekte.

```rust
fn main() {
    let number = 42;
    if number % 2 == 0 {
        println!("{} is even.", number);
    } else if number % 3 == 0 {
        println!("{} is odd", number);
    } else {
        println!("{} is neither even nor divisible by 3", number);
    }
}
```

Sıradaki örnekte ise bir sınav puanına göre geçme/kalma durumu değerlendirilmektedir.

```rust
fn check_exam_score(score: i32) {
    if score == 0 {
        println!("Blank paper! Fails");
    } else if score > 70 {
        println!("{} is enough for pass.", score);
    } else if score < 50 {
        println!("{} is not enough for pass.", score);
    } else {
        println!("{} is greater than 50 but less than 70. Come in September!", score);
    }
}

fn main() {
    check_exam_score(65);
}
```

## loop Döngüleri

loop, eğer açıkça bir sayaç değişkeni kullanılmazsa sonsuz döngü oluşturmak için de kullanılır. **break** komutu döngüyü
sonlandırırken, **continue** mevcut iterasyonun atlanıp bir sonrakine geçilmesini sağlar. Bazı senaryolar sonsuz
döngülerin çalıştırılmasını gerektirir. Örneğin bir oyun motorunda tüm state'ler oyun döngüsü içerisinde ele alınır.
Aşağıdaki basit kod parçasında bir sayaç kullanılarak ileri yönlü basit bir iterasyon işlenmiştir.

```rust
fn main() {
    let mut counter = 0;
    loop {
        counter += 1;
        if counter % 5 == 0 {
            println!("Reached {}", counter);
            break;
        } else {
            continue;
        }
    }
}
```

## while Döngüsü

Belirli bir koşul sağlandığı sürece çalışmaya devam eden döngülerdir. Bu açıdan bakıldığında sonsuz döngüdeki manuel
sayaç mekanizmasına göre belli vakalar için idealdir. Örneğin bir web sunucusu motorunda sunucu açık olduğu sürece
stream'e gelen içeriklerin ele alınması gerektiği durumlar örnek olarak gösterilebilir. Pek tabii bu ve buna bağlı
birçok
detay modern modüllerde soyutlanır _(abstract edilir)_ ve aslında bir döngü ile karşılaşmadan TCP isteklerini dinleriz.

Aşağıdaki basit döngü 0'dan başlayarak 9 dahil sayan bir iterasyonu örnekler. Koşul, count değerinin 10'dan küçük
olmasıdır.

```rust
fn main() {
    let mut count = 0;
    while count < 10 {
        println!("Count: {}", count);
        count += 1;
    }
}
```

## for Döngüsü ve enumerate() Kullanımı

for döngüsü, koleksiyonlar _(vektörler, diziler vb.)_ üzerinde ileri yönlü iterasyonlar tesis etmek için oldukça
kullanışlıdır. Aşağıdaki örnekte numbers isimli vektörün elemanlarında ileri yönlü bir iterasyon işlemi icra
edilmektedir. enumerate fonksiyonu sadece indeks sayacına ulaşmak için kullanılmıştır.

_**Not:** Rust dili aynı zamanda **Zero Cost Abstraction** kabiliyeti ile de öne çıkar. iter gibi fonksiyonlar döngüsel
mekanizmaların karmaşıklığını soyutlarken çalışma zamanı maliyetlerini de sıfıra indirir. Ayrıca fonksiyonel dillerde
olduğu gibi Rust tarafında da **Higher Order Function** kullanımları yaygındır. Iterasyonlar sırasında nesne
elemanlarında değişiklik yapılmak istenirse **mutable** iterasyonlar da _(iter_mut)_ başlatılabilir._

```rust
fn main() {
    let numbers = vec![10, 20, 30, 40];
    for (index, value) in numbers.iter().enumerate() {
        println!("Index {}: Value {}", index, value);
    }
}
```

## Rastgele Sayılarla Koşullu Kontroller

Pek çok senaryoda _(simülasyonlarda, oyun programlamada vs)_ rastgele sayılara sıklıkla ihtiyaç duyulur. Bu noktada
Rust'ın popüler crate'lerinden birisi olaran **rand** kullanılabilir. Aşağıdaki örnek kod parçasında 100 adet rastgele
sayı üretilmekte ve içlerinde çift veya tek olanlar ekrana yazdırılmaktadır.

```rust
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(1..101);
    println!("Generated number: {}", random_number);

    if random_number % 2 == 0 {
        println!("{} is even", random_number);
    } else {
        println!("{} is odd", random_number);
    }
}
```

rand crate'ini uygulamada kullanabilmek için Cargo.toml dosyasına eklenmesi gerekir. Bu elle yapılabileceği gibi komut
satırından cargo aracı ile aşağıdaki gibi de gerçekleştirilebilir.

```bash
cargo add rand
```

## Sonuç

Bu dokümanda akış kontrol ve döngü mekanizmaları incelenmiş ve şu konular ele alınmıştır;

- **Koşullu ifadeler (if, else if, else)**
- **Sonsuz döngüler ve loop kullanımı**
- **while döngüsü ve koşullu yineleme**
- **for döngüsü ile koleksiyonlar üzerinde yineleme**
- **Rastgele sayı üretimi ve koşullu ifadelerle kullanımı**
