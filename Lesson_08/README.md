# Ders 08: Trait Kullanımı

**trait** türü belirli bir davranış kümesini tanımlayan ve bunu bir veya birden fazla veri yapısına uygulayabilen bir
enstrüman olarak düşünülebilir. C# tarafındaki **interface** türüne ve kullanım amacına benzer olduğunu söyleyebiliriz.
Özellikle **generic** yapılarla kullanıldığında büyük esneklik sağlar _(ki Generics bölümünde trait'lerin dinamik
kullanımına değinilmektedir)_ Rust standart kütüphanesi ile gelen bir çok faydalı ve kullanışlı trait de vardır. **Debug
**, **Copy**, **Clone**, **Add**, **Display**, **From** ve benzerleri bunlara örnek olarak gösterilebilir.

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

## Özet

| Konu                        | Açıklama                                                       |
|-----------------------------|----------------------------------------------------------------|
| **Trait Tanımlama**         | `trait TraitAdı { ... }` şeklinde tanımlanır.                  |
| **Trait Implementasyonu**   | `impl TraitAdı for StructAdı { ... }`                          |
| **Generic Trait Kullanımı** | `fn func<T: TraitAdı>(param: &T) { ... }`                      |
| **Trait Object Kullanımı**  | `&dyn TraitAdı` şeklinde dinamik nesne tutulur.                |
| **Varsayılan Trait Metodu** | Trait içinde metod için varsayılan implementasyon yazılabilir. |
