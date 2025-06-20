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

**Trait** ve **closure**’lar sıklıkla bir arada ele alınır. Bir **closure** anonim fonksiyon olarak da ifade edilir. Özellikle fonksiyonel dillerde yaygın olarak kullanılan closure'lar, bir değişkene atanabilir ve bu sayede fonksiyonlara parametre olarak kod bloklarının taşınması sağlanabilir. Benzer şekilde fonksiyonlardan dönüş türü olarak da kullanılabilir. Böylece nesne toplulukları üzerinde aynı kod bloğunu işleten fonksiyonları geliştirmek oldukça kolaylaşır. Tüm iteratif fonksiyonlar bu tekniğe göre geliştirilmiştir. Rust, closure kullanımlarında kısıtlar _(Constraints)_ da kullanır ve bunu üç farklı **trait** ile sağlar.

- **Fn:** Closure, dışarıdan yakaladığı değişkenleri salt okunur _(read only)_ şekilde kullanır.
- **FnMut:** Closure, dış değişkenleri değiştirerek _(mutable)_ kullanabilir.
- **FnOnce:** Closure, dış değişkenleri sahiplenir _(move eder)_ ve yalnızca bir kez çağrılabilir.

Geliştirdiğimiz örnekte bir oyun sahasındaki nesneler üzerinde parametre olarak gönderilen kod bloğunu işletecek genel bir fonksiyon yazmak istediğimizi düşünelim.

```rust
pub fn main() {
    let mut world = GameWorld {
        players: vec![
            Player {
                id: 1,
                position: (0.0, 0.0),
                velocity: (2.0, 0.0),
                score: 0,
            },
            Player {
                id: 2,
                position: (100.0, 0.0),
                velocity: (8.0, 0.0),
                score: 0,
            },
        ],
    };

    let apply_gravity = |entity: &mut Player| {
        entity.position.0 += entity.velocity.0 * 0.9;
        entity.position.1 += entity.velocity.1 * 0.9;
    };

    println!("Before Update: {:?}", world.players);
    update_players_system(&mut world, apply_gravity);
}
```

Bu kod parçasını göz önüne alırsak aşağıdakilerden hangisi update_players_system’in doğru bir tanımıdır?

**a)**
```rust
fn update_players_system<F>(world: &mut GameWorld, mut f: F)
where
    F: Fn(&mut Player),
{
    for p in &mut world.players {
        f(p);
    }
}
```

**b)**
```rust
fn update_players_system<F>(mut world: GameWorld, mut f: F)
where
    F: FnMut(&Player),
{
    for p in &world.players {
        f(p);
    }
}
```

**c)**
```rust
fn update_players_system<F>(world: &mut GameWorld, f: F)
where
    F: FnMut(Player),
{
    for mut p in world.players {
        f(p);
    }
}
```

**d)**
```rust
fn update_players_system<F>(world: &mut GameWorld, mut f: F)
where
    F: FnOnce(&mut Player),
{
    for p in &mut world.players {
        f(p);
    }
}
```

## Soru 6

Aşağıda verilen kod parçası bir closure ifadesi döndürüyor.

```rust
use crate::models::{Level, Log};
use std::io::{Write, stdout};

pub fn log() -> impl FnMut(&Log) {
    let mut error_count = 0;
    let mut warn_count = 0;
    let mut info_count = 0;
    move |l| {
        stdout()
            .write(format!("{}\n", l.to_string()).as_bytes())
            .unwrap();
        match l.level {
            Level::Error => error_count += 1,
            Level::Warn => warn_count += 1,
            Level::Info => info_count += 1,
            _ => {}
        }
        stdout()
            .write(
                format!(
                    "Log Tracker: {} errors, {} warnings, {} infos\n",
                    error_count, warn_count, info_count
                )
                    .as_bytes(),
            )
            .unwrap();
    }
}
```

Buna göre aşağıdaki şıklarda belirtilen ifadelerden hangisi doğrudur?

**a)** Closure Copy trait’ine sahiptir, dolayısıyla her çağrıldığında iç sayaçlar sıfırdan başlar.

**b)** Dönüş tipi impl Fn(&Log) olarak da yazılabilir çünkü closure yalnızca okuma yapmaktadır.

**c)** Closure, mut olarak çağrılmak zorundadır çünkü error_count, warn_count, info_count değerleri her çağrımda değişir. Bu nedenle dönüş tipi impl FnMut(&Log) olarak belirtilmiştir.

**d)** Kodda move ifadesinin kullanılmasına gerek yoktur. error_count, warn_count, info_count değerleri heap üzerinde tutulmadığı için closure zaten bu değerlere doğrudan erişebilir.

## Soru 7

Aşağıdaki kod parçasını dikkatlice inceleyiniz.

```rust
fn main() {
    create()
}
enum Tree {
    Node(i32, Tree, Tree),
    Empty,
}

pub fn create() {
    let left_child = Tree::Node(1, Tree::Empty, Tree::Empty);
    let right_child = Tree::Node(3, Tree::Empty, Tree::Empty);
    let root = Tree::Node(2, left_child, right_child);
}
```

Yukarıdaki kod derleme zamanında **error[E0391]: cycle detected when computing when `Tree` needs drop** hatası verir. Bu hatanın sebebi şıklardan hangisidir?

**a)** Enum yapısında Empty varyantının bulunması Node varyantını geçersiz kılıp Rust’ın match yapısının kullanmasını gerektirdiği için hata alınır.

**b)** Enum Tree kendi içinde doğrudan kendisini barındırdığı için sonsuz büyüklükte bir veri yapısı oluşur ve bu kontrol edilebilir değildir. Çözüm için Box<Tree> gibi bir Smart Pointer kullanılarak recursive alanların heap’e taşınması sağlanmalıdır.

**c)** Tree yapısı içinde kullanılan i32 türünün Drop trait'ini implemente etmemesi sebebiyle hata oluşur.

**d)** Rust, enum türleri içinde başka enum’ları doğrudan kullanmayı desteklemez. Bu nedenle Tree enum’unun kendi türünde alanları olamaz.

## Soru 8

Rust standart kütüphanesi birçok **smart pointer** türü içerir. Bunlar **Box<T>**, **Rc<T>**, **RefCell<T>**, **Ref<T>** ve **RefMut<T>** şeklinde kategorize edilir. Aşağıda birkaç senaryo belirtilmiştir. Hangi senaryo için hangi smart pointer kullanılmalıdır. Şıklarda doğru sıralamayı seçerek cevaplayınız.

- **I.** Birden fazla sahipliği _(ownership)_ tek bir veri üzerinde sağlamak.
- **II.** Aynı veri üzerinden birden fazla sahiplik sağlamak
- **III.** Aynı veriye farklı iş parçacıkları _(threads)_ erişim söz konusu olduğunda.

**a)** RefMut, RefCell, Ref

**b)** Box/RefCell, Rc, Arc

**c)** Box, Box, Box

**d)** Arc, Mutex, RefCell

## Soru 9

Birden fazla işi farklı iş parçacıklarına _(thread)_ bölerek çalıştırmak mümkündür. Böylece işlemci/çekirdek gücünden efektif şekilde yararlanılabilir. Rust tarafında hem paralel hem de concurrent _(eş zamanlı)_ programlama için birçok hazır enstrüman bulunur. Bir iş parçacığını başlatmak için **thread** modülünün **spawn** metodu kullanılır. Buna göre aşağıdaki kodun çalışma zamanı çıktısı şıklardan hangisidir?

```rust
use std::thread;

fn main() {
    do_something();
    println!("After the thread calling");
}

pub fn do_something() {
    let student_points = vec![30.50, 49.90, 65.55, 90.00, 81.00];
    let handle = thread::spawn(|| {
        println!("Thread is starting...");
        println!("{:?}", student_points);
        println!("Thread completed");
    });
    handle.join().unwrap();
}
```

**a)** Bu kod derlenmez ve error[E0373]: closure may outlive the current function, but it borrows `student_points`, which is owned by the current function mesajını içeren bir hata fırlatır.

**b)**
```text
Thread is starting...
[30.5, 49.9, 65.55, 90.0, 81.0]
Thread completed
After the thread calling
```

**c)**
```text
After the thread calling
Thread is starting...
[30.5, 49.9, 65.55, 90.0, 81.0]
Thread completed
```

**d)**
```text
Thread is starting...
Thread completed
After the thread calling
```

## Soru 10

Aşağıda örnek bir kod parçası verilmiştir.

```rust
use std::thread;

fn main() {
    run_with_error();
    println!("After the thread calling");
}

pub fn run_with_error() {
    let data = vec![
        "Service Red: Task A",
        "Service Blue: Task B",
        "Service Green: Task C",
        "Service Alpha: Task D",
    ];

    let mut handles = vec![];
    for i in 0..2 {
        let handle = thread::spawn(move || {
            for task in &data {
                println!("Thread '{}' is processing '{}'", i, task);
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
```

Kod derleme zamanında aşağıdaki hatayı vermektedir.

```text
error[E0382]: use of moved value: `data`
  --> practices\src\main.rs:18:36
   |
9  |     let data = vec![
   |         ---- move occurs because `data` has type `Vec<&str>`, which does not implement the `Copy` trait
...
17 |     for i in 0..2 {
   |     ------------- inside of this loop
18 |         let handle = thread::spawn(move || {
   |                                    ^^^^^^^ value moved into closure here, in previous iteration of loop
19 |             for task in &data {
   |                          ---- use occurs due to use in closure
```

Aynı anda birden fazla iş parçacığı _(thread)_ ortak bir referansa erişmeye çalışmakta ve referans sahipliği **closure** içerisine alındıktan sonra düşeceği _(drop)_ için bu sorun oluşmaktadır. Problemi çözmek için şıklardan hangisini uygularsınız?

**a)** Data vector yerine referans türlü bir diziye alınırsa sorun çözülür.

**b)** Data Arc _(Atomic Reference Counting)_ smart pointer nesnesi içinde ele alınmalı ve döngüde açılan her bir thread örneği klonlanarak paylaşılmalıdır

**c)** data vektörü her thread::spawn çağrısında yeniden tanımlanmalıdır. Böylece her thread kendi data kopyasına sahip olur ve ownership sorunu yaşanmaz.

**d)** Closure bloğu içine data değişkeni move olmadan geçirilirse, tüm thread’ler ortak referansı kullanabilir.

## Soru 11

Rust programlama dili **thread-safe** bir ortam sağlamak için bazı kuralları devreye alsa da **Deadlock** veya **mutex poisoning** gibi durumlardan **kaçılamayabilir**. Kilit mekanizmalarının hatalı kullanımları **Deadlock** oluşmasına sebep olur. Diğer yandan bir kilit söz konusu iken bulunulan **thread** içerisinde panik oluşması da sorun yaratır ve bu durum **Thread Poisoning** olarak adlandırılır. Aşağıdaki örnek kod parçasında bu problemlerden hangisi meydana gelir?

```rust
use std::sync::{Arc, Mutex};
use std::thread;
fn main() {
    do_something();
    println!("After the thread calling");
}
pub fn do_something() {
    let number = Arc::new(Mutex::new(1));
    let mut handles = vec![];

    for i in 0..10 {
        let number = Arc::clone(&number);
        let handle = thread::spawn(move || {
            println!("For counter is {}", i);
            let mut num = number.lock().unwrap();
            let mut another_num = number.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        println!("Joining handle");
        handle.join().unwrap();
    }

    println!("{:?}", number.lock().unwrap());
}
```

**a)** Thread Poisoning

**b)** Kod problemsiz çalışır

**c)** Deadlock

**d)** Handles değişkeni için derleme zamanında Value Moved Here hatası alınır
