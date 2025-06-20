# Rust ile Sistem Programlama - Final Sınavı Soruları

**Süre :** 75 Dakika

**Soru Sayısı :** 16 _(Sadece istenilen 10 soruyu cevaplayınız)_

## Soru 1

Rust programlama dili **generic** tür kullanımlarını da destekler. Özellikle aynı işlevsellik veya davranışların farklı türler için kod tekrarı yapmaya gerek kalmadan yazılmasında kullanılır. Tür güvenliği _(Type Safety)_ için kısıtlamalar _(Constraints)_ kullanılabilir ve türün belli **Trait** davranışlarına sahip olması şart koşulabilir. Aşağıdaki **Point** isimli veri yapısı _(struct)_ generic tip kullanmaktadır.

```rust
use std::fmt::Debug;
use std::ops::Add;

struct Point<T: Copy + Debug + Add<Output=T>> {
    x: T,
    y: T,
    z: T,
}
```

Buna göre aşağıdakiler hangisi veya hangileri doğrudur?

- **I.** T: Debug: T türü Debug trait’ini uygulamış olmalıdır.
- **II.** T: Copy: T türü kopyalanabilir (Copy trait’ini implemente etmiş) olmalıdır.
- **III.** T: Add<Output = T>: T türü toplama (+) işlemini desteklemelidir.

**a)** Sadece I

**b)** I ve II

**c)** II ve III

**d)** Hepsi doğrudur

## Soru 2

**Enum** veri yapılarında da **generic** türler kullanılabilir. Aşağıdaki örnek kod parçasında **Success** değeri generic **T** türü ile tanımlanmıştır.

```rust
#[derive(Debug)]
enum ApiResponse<T> {
    Success(T),
    Error(String),
}
```

Buna göre aşağıdaki ifadelerden hangisi söz konusu enum veri yapısının doğru kullanımlarından **değildir**?

**a)** let res: ApiResponse<i32> = ApiResponse::Success(200);

**b)** let res: ApiResponse<&str> = ApiResponse::Error("404 Bad Request".to_string());

**c)** let res: ApiResponse = ApiResponse::Success(201);

**d)** let res: ApiResponse<i16> = ApiResponse::Success(201);

## Soru 3

**Trait** türü belirli bir davranış kümesini tanımlayan ve bunu bir veya birden fazla veri yapısına uygulayabilen bir enstrümandır. Bazı durumlarda **trait**'i implemente eden nesnelerin dinamik olarak saklanması gerekir. Bu gibi durumlarda **Trait Object** kullanılır. Aşağıdaki kod parçasında **Draw** isimli bir trait ve örnek uygulamalarına yer verilmiştir.

```rust
trait Draw {
    fn draw(&self);
}

struct Circle;
struct Square;
struct Player;

impl Draw for Circle {
    fn draw(&self) {
        println!("Drawing a circle");
    }
}

impl Draw for Square {
    fn draw(&self) {
        println!("Drawing a square");
    }
}

impl Draw for Player {
    fn draw(&self) {
        println!("Drawing a player");
    }
}
```

Bu tasarıma göre **draw** davranışını uygulamış belli sayıda nesne için tek bir **for** döngüsü üzerinden işlem yapan ve gelen nesnelerin **draw** fonksiyonlarını işleten bir metot yazılması istenmektedir. Aşağıdaki metotlardan hangisi doğrudur?

**a)**

```rust
fn draw_shapes(shapes: &Vec<&dyn Draw>) {
    for shape in shapes.iter() {
        shape.draw();
    }
}
```

**b)**

```rust
fn draw_shapes(shapes: &Vec<Draw>) {
    for shape in shapes.iter() {
        shape.draw();
    }
}
```

**c)**

```rust
fn draw_shapes(shapes: [&dyn Draw]) {
    for shape in shapes.iter() {
        shape.draw();
    }
}
```

**d)**

```rust
fn draw_shapes(shapes: &Vec<&Draw>) {
    for shape in shapes.iter() {
        shape.draw();
    }
}
```

## Soru 4

Rust standart kütüphanesi ile birlikte gelen önceden tanımlı birçok **trait** vardır. Örneğin **println** makrosunda **:?** formatının kullanılabilmesi için **Debug** trait’inin uyarlanmış olması beklenir. Sık kullanılan tarit’ lerden ikisi de veri sahipliği _(Ownership)_ ile ilgili olan **Clone** ve **Copy trait**’leridir. Bu bilgi ışığında aşağıda verilen kod parçasını dikkatlice inceleyiniz.

```rust
#[derive(Debug, PartialEq)]
struct Velocity {
    x: f32,
    y: f32,
}

fn accelerate(mut speed: Velocity) {
    speed.x += 1.0;
    speed.y += 1.0;
    println!("Accelerating: {:#?}", speed);
}

fn main() {
    let my_speed = Velocity { x: 10.0, y: 0.0 };
    let your_speed = Velocity { x: 10.0, y: 0.0 };

    accelerate(your_speed);

    if my_speed == your_speed {
        println!("{my_speed:?} is equal to {your_speed:?}")
    } else {
        println!("{my_speed:?} is not equal to {your_speed:?}")
    }
}
```

Yukarıdaki kod parçası derleme zamanında **Value used after being moved [E0382]** hatası vermektedir. Şıklardan hangisi hatanın sebebi ve çözüm yolunu belirtir.

**a)** Hatanın sebebi PartialEq trait'inin eksik olmasıdır. Velocity yapısına Eq trait'i de eklenirse == karşılaştırması çalışır ve hata giderilir.

**b)** Rust dilinde struct türleri varsayılan olarak zaten Copy trait'ini uygular. Bu nedenle kod hatasız derlenir.

**c)** Hata, accelerate fonksiyonunun parametresinin mutable olarak tanımlanmış olmasından kaynaklanır. mut kaldırılırsa ownership problemi ortadan kalkar.

**d)** your_speed değişkeni accelerate fonksiyonuna geçirilirken kopyalanmadan taşınır ve accelerate işleyişini tamamlandığında sahipliği alınan your_speed bellekten düşer. Bu, devam eden satırda olmayan bir referans erişimine sebebiyet verir ve value moved hatası oluşur. Hatanın çözümü için Velocity yapısına Clone trait'ini uygulamak gerekir ve accelerate(your_speed.clone()) şeklinde çağrılmalıdır. 

## Soru 5

