# Ders 08: Trait Kullanımı

**trait** türü belirli bir davranış kümesini tanımlayan ve bunu bir veya birden fazla veri yapısına uygulayabilen bir
enstrüman olarak düşünülebilir. C# tarafındaki **interface** türüne ve kullanım amacına benzer olduğunu söyleyebiliriz.
Özellikle **generic** yapılarla kullanıldığında büyük esneklik sağlar _(ki Generics bölümünde trait'lerin dinamik
kullanımına değinilmektedir)_ Rust standart kütüphanesi ile gelen bir çok faydalı ve kullanışlı trait de vardır.
**Debug**, **Copy**, **Clone**, **Add**, **Display**, **From** ve benzerleri bunlara örnek olarak gösterilebilir.

## Trait Tanımlama ve Kullanımı

Aşağıdaki örnekte **Service** adında bir trait tanımlanmıştır. **activate**, **deactivate** ve **status** olmak üzere üç
metot bildirimi/davranış tanımı içermektedir.

```rust
trait Service {
    fn activate(&mut self);
    fn deactivate(&mut self);
    fn status(&self) {
        println!("Service status");
    }
}
```

Bu trait aşağıdaki gibi Redis ve HeathCheck isimli veri yapılarına uygulanabilir.

```rust
struct Redis;
struct HealthCheck {
    is_online: bool,
}

impl Service for Redis {
    fn activate(&mut self) {
        println!("Activating Redis");
    }
    fn deactivate(&mut self) {
        println!("Deactivating Redis");
    }
}

impl Service for HealthCheck {
    fn activate(&mut self) {
        self.is_online = true;
        println!("Activating HealthCheck");
    }
    fn deactivate(&mut self) {
        println!("Deactivating HealthCheck");
    }
    fn status(&self) {
        println!("Service status: {}", self.is_online);
    }
}
```

Dikkat edilmesi gereken noktalardan birisi Redis veri yapısında status isimli davranışın yazılmamasıdır. Dolayısıyla
default trait davranışının uygulandığını ifade edebiliriz.

## Fonksiyon Parametresi Olarak Trait Kullanımı

Bir trait'i parametre olarak alan fonksiyonlar da yazılabilir ve böylece söz konusu trait'i uygulayan herhangi bir yapı
ilgili fonksiyonda kullanılabilir. Trait'leri fonksiyonlara almanın iki farklı yolu vardır.

- **impl Trait Kullanımı**

```rust
fn start_sample(service: &mut impl Service) {
    service.activate();
    println!("{:?}", service.status());
    service.deactivate();
    println!("{:?}", service.status());
}
```

- **Generic Trait Kullanımı**

```rust
fn start_sample2<T: Service>(service: &mut T) {
    service.activate();
    println!("{:?}", service.status());
    service.deactivate();
    println!("{:?}", service.status());
}
```

Buraya kadar anlatılan örnekler aşağıdaki kod parçasındaki gibi denenebilirs.

```rust
fn main() {
    let mut redis_instance = Redis;
    let mut doctor = HealthCheck { is_online: false };

    start_sample(&mut redis_instance);
    start_sample(&mut doctor);
    start_sample2(&mut redis_instance);
}
```

## Varsayılan Trait Metodu

Bazı durumlarda bir trait içinde **varsayılan** işlevsellik sunan bir davranış tanımlanabilir. Bu metot, trait'i
uygulayan yapı tarafından **ezilmek/override zorunda değildir**. Aşağıdaki kod parçasını göz önüne alalım. Örnekte *
*Payment** isimli bir trait tanımlanmıştır. **pay** metodu varsayılan olarak nakit ödeme işlemini icra eder ama
istenirse farklı bir veri yapısında bu davranış farklı şekilde uygulanabilir.

```rust
trait Payment {
    fn pay(&self, amount: f64) {
        println!("Paid {:.2} with cash.", amount);
    }
}
```

Farklı ödeme türleri için söz konusu trait'in aşağıdaki kod parçasında olduğu gibi uygulandığını düşünelim. Dikkat
edileceği üzere kredi kartı ödemeleri için aynı davranışın farklı bir işleyişi tasarlanmıştır.

```rust
struct DebitCard;
impl Payment for DebitCard {}

struct CompanyAccount;
impl Payment for CompanyAccount {}

struct CreditCard;
impl Payment for CreditCard {
    fn pay(&self, amount: f64) {
        let amount_with_commision = amount * 1.1;
        println!("Paid {:.2} with credit card.", amount_with_commision);
    }
}

fn main() {
    let debit_payment = DebitCard;
    let company_payment = CompanyAccount;
    let card_payment = CreditCard;

    debit_payment.pay(100.0);
    card_payment.pay(100.0);
    company_payment.pay(100.0);
}
```

## Trait Object

Bazı durumlarda trait'i implemente eden nesneleri **dinamik olarak** saklamak isteyebiliriz. Bu gibi durumlarda **Trait
Object** kullanılır. Çizim davranışını tarifleyen **Draw** isimli bir trait nesnesini ele alalım.

```rust
trait Draw {
    fn draw(&self);
}
```

ve bu trait'i implemente eden bazı veri yapıları oluşturalım.

```rust
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

Bir vektör değişkeninde **Draw trait**' ini uygulayan **farklı nesneleri** tutabilir ve toplu işlemler
gerçekleştirebiliriz. Özellikle aynı davranışı farklı şekillerde uygulayan ama sayıları çalışma zamanında belli olan
heap odaklı nesne toplulukları göz önüne alındığında bu teknik oldukça kullanışlıdır.

```rust
fn draw_shapes(shapes: &Vec<&dyn Draw>) {
    for shape in shapes.iter() {
        shape.draw();
    }
}
```

Örnek bir kullanım,

```rust
fn main() {
    // Trait Object kullanımı
    let red_ball = Circle;
    let blue_ball = Circle;
    let wall = Square;
    let warrior = Player;
    let level_1: Vec<&dyn Draw> = vec![&red_ball, &blue_ball, &wall, &warrior];
    draw_shapes(&level_1);
}
```

## Rust'ta Yaygın Kullanılan Trait'ler ve Kullanım Örnekleri

Rust programlama dili, güçlü tür sistemine sahip olmasıyla da bilinir. Bu sistemin önemli bir parçası da **trait**
yapılarıdır. Trait'ler, bir türün belirli bir davranışı nasıl uygulayacağını belirten arayüzlerdir. Rust standart
kütüphanesi birçok yerleşik trait sunar. Var olan trait'lerin bir kısmı **derive** direktifi ile implemente edilirken
bazıları da açıkça yazılır ve tanımladıkları fonksiyonlar ezilir. Aşağıdaki bölümlerde sık kullanılan trait örneklerine
yer verilmiştir.

### Debug

Rust'ta bir yapıyı **Debug** trait'ini uygulamadan **println!** makrosu ile **{:#?}** veya **{:?}** formatında yazdırmak
mümkün değildir. **Debug** trait'ini etkinleştirmek için **#[derive(Debug)]** özelliği kullanılmalıdır.

```rust
#[derive(Debug)]
struct Player {
    name: String,
    level: u8,
    last_score: f32,
}
```

### Display

**Display** trait, bir yapının _(struct)_ veri içeriğinin istenilen şekilde yazdırılmasını/okunmasını sağlar. Bu trait'i
uygulamak için **std::fmt::Display** trait'inin **fmt** metodu override edilmelidir. Aşağıdaki örnekte **Player** veri
yapısının örneğin bir println makrosunda kullanılması halinde ekrana ne yazılacağı programlanmıştır. Bu yaklaşım sadece
println ile değil nesnenin String ifadelerinde de kullanılabilir. Örneğin bir CSV içeriğini hazırlarken de ele
alınabilir, log bilgisi olarak bir kaynağa gönderilirken de.

```rust
use std::fmt::{Display, Formatter};

impl Display for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}({}) last score is {}",
            self.name, self.level, self.last_score
        )
    }
}

fn main() {
    let max_payne = Player {
        name: String::from("Max Payne"),
        level: 80,
        last_score: 167.58,
    };
    // Debug trait'ini uygulamazsak
    // `Player` does not implement `Debug` (required by `{:#?}`): E0277
    println!("{:#?}", max_payne);

    // Aşağıdaki kullanım ise Display trait'ini uygulamayı gerektirir
    println!("{}", max_payne);
}
```

### Clone ve Copy

Rust'ta veri sahipliği _(ownership)_ önemli bir konudur. **Clone** trait'i, bir nesnenin kopyasını oluşturmayı sağlar.
**Copy** trait'i ise küçük ve taşınabilir türler için tercih edilir. **Shallow Copy** durumlarında **Copy** trait ön
plana çıkarken, **Deep Copy** operasyonlarında **Clone** trait ele alınabilir. **Velocity** isimli struct'ı ve bu türü
parametre olarak kullanan **accelarate** fonksiyonunu göz önüna alalım. Velocity veri yapısı Debug, PartialEq, Copy ve
Clone trait'lerini implemente etmektedir.

```rust
#[derive(Debug, PartialEq, Copy, Clone)]
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
    let max_payne = Player {
        name: String::from("Max Payne"),
        level: 80,
        last_score: 167.58,
    };

    // Clone trait uygulaması
    let mut red_one = max_payne.clone();
    red_one.name = String::from("Red Leader One");
    println!("{:#?}", red_one);

    // PartialEq ve Copy, Clone örnekleri
    let my_speed = Velocity { x: 10.0, y: 0.0 };
    let your_speed = Velocity { x: 10.0, y: 0.0 };
    if my_speed == your_speed {
        println!("{my_speed:?} is equal to {your_speed:?}")
    }
    accelerate(your_speed);
    // Eğer Velocity için Copy trait kullanmazsak
    // Value used after being moved [E0382] derleme hatası oluşur
    if my_speed == your_speed {
        println!("{my_speed:?} is equal to {your_speed:?}")
    } else {
        println!("{my_speed:?} is not equal to {your_speed:?}")
    }
}
```

Bir önceki örnekte **PartialEq** trait'i derive direktifi ile implemente edilmiştir. Bu durumda rust
varsayılan trait davranışını Velocity için uygular ve içerdiği tüm primitive tipler karşılaştırmaya dahil edilir. Ancak
bazı durumlarda sadece belli alanlar üzerinden karşılaştırma yapılmak istenebilir. LogStamp isimli aşağıdaki yapıyı ele
alalım. Sadece id değerleri üzerinden iki değişkenin eşit olup olmadığı kontrol edilmek isteniyorsa **PartialEq**
trait'i LogStamp türü için implemente edilebilir.

```rust
#[derive(Debug)]
struct LogStamp {
    id: i32,
    time: String,
    content: String,
}

impl PartialEq for LogStamp {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

fn main() {
    let log_1 = LogStamp {
        id: 1234,
        time: String::from("2024-10-20T09:56-PM"),
        content: String::from("Check with ping"),
    };
    let log_2 = LogStamp {
        id: 1234,
        time: String::from("2024-10-20T09:58-PM"),
        content: String::from("Check with ping"),
    };
    if log_1 == log_2 {
        println!("Same logs\n{log_1:?}\n{log_2:?}");
    }
}
```

### Default

**Default** trait, bir struct türünün varsayılan değerlerle oluşturulmasında kullanılır. Aşağıdaki örnekte yer alan Game
yapısı Default trait ile oluşturulurken içeriğine bazı ilk değerler verilmektedir. **Display** trait eğer derive
direktifi ile kullanılırsa türün içeriğindeki alanlar içinde varsayılan değerler set edilir ancak aşağıdaki örnekte
olduğu gibi içeriğin özel verilerle oluşturulması da sağlanabilir.

### Örnek:

```rust
#[derive(Debug)]
struct Game {
    fps: u8,
    title: String,
    screen_width: u32,
    screen_height: u32,
}

impl Default for Game {
    fn default() -> Self {
        Game {
            fps: 30,
            title: String::from("A Simple Game"),
            screen_width: 1280,
            screen_height: 960,
        }
    }
}

fn main() {
    let my_game = Game::default();
    println!("{:#?}", my_game);
    println!("{}", my_game);
}
```

### FromStr

**FromStr** trait, metinsel bir ifadenin belirli bir veri türüne dönüştürülmesinde kullanılır. Aşağıdaki örnekte
**SystemColor** isimli veri yapısının aralarında **:** işaret bulunan bir RGB(Red Green Blue) formatından üretilmesi ele
alınmaktadır. Pek tabii dönüştürme operasyonunun başarısız olması ihtimali **Result** türü ile kontrol altında
tutulmaktadır.

```rust
use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Debug, Copy, Clone)]
struct SystemColor {
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Debug)]
enum ColorParseError {
    WrongArgumentCount,
    ParseError(ParseIntError),
}

impl FromStr for SystemColor {
    type Err = ColorParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 3 {
            return Err(ColorParseError::WrongArgumentCount);
        }

        let r = parts[0].parse::<u8>().map_err(ColorParseError::ParseError)?;
        let g = parts[1].parse::<u8>().map_err(ColorParseError::ParseError)?;
        let b = parts[2].parse::<u8>().map_err(ColorParseError::ParseError)?;

        Ok(SystemColor { r, g, b })
    }
}

fn main() {
    let valid_color = "125:100:100".parse::<SystemColor>();
    match valid_color {
        Ok(color) => println!("Parsed color: {:?}", color),
        Err(e) => println!("Error parsing color: {:?}", e),
    }

    let invalid_color = "red:100:100".parse::<SystemColor>();
    match invalid_color {
        Ok(color) => println!("Parsed color: {:?}", color),
        Err(e) => println!("Error parsing color: {:?}", e),
    }
}
```

### Drop

**Drop** trait, bir nesne bellekten kaldırılırken belirli işlemlerin gerçekleştirilmesi istendiği durumlar için
idealdir. Harici kaynaklar kullanan bir nesnenin kullandığı kaynaklarla ilgili işlemler yapmak buna örnek
gösterilebilir.

```rust
#[derive(Debug)]
struct Service {
    title: String,
    address: String,
}

impl Service {
    fn call(&self) {
        println!("Service call");
    }
}

impl Drop for Service {
    fn drop(&mut self) {
        println!("Closing temp connections and releasing resources.");
    }
}

fn main() {
    {
        let author_service = Service {
            title: String::from("Author service"),
            address: String::from("https://azon/services/author"),
        };
        println!("{:#?} is ready.", author_service);
        author_service.call();
    } // Scope sonu, drop trait tetiklenir
}
```

## Özet

| Konu                        | Açıklama                                                       |
|-----------------------------|----------------------------------------------------------------|
| **Trait Tanımlama**         | `trait TraitAdı { ... }` şeklinde tanımlanır.                  |
| **Trait Implementasyonu**   | `impl TraitAdı for StructAdı { ... }`                          |
| **Generic Trait Kullanımı** | `fn func<T: TraitAdı>(param: &T) { ... }`                      |
| **Trait Object Kullanımı**  | `&dyn TraitAdı` şeklinde dinamik nesne tutulur.                |
| **Varsayılan Trait Metodu** | Trait içinde metod için varsayılan implementasyon yazılabilir. |
