# Ders 07: Generic _(Genel)_ Türler

Rust programlama dili, **generic** türleri destekleyerek **yeniden kullanılabilir** ve **tür bağımsız** kodlar yazmamıza
olanak tanır. Generic kullanımı sayesinde, aynı fonksiyon veya veri yapısını **farklı türler** için ayrı ayrı tanımlamak
yerine **tek bir tanım** altında esnek hale getirebiliriz. Rust'taki **generic** mekanizması, C#, Java ve Go gibi
dillerdeki generic yapılarla benzerlik gösterir ancak **Rust’ın güçlü tür sistemi ve trait tabanlı kısıtlama yapısı
sayesinde daha güvenlidir.**

## Generic Fonksiyonlar

Bir fonksiyonun parametelerini veya dönüş değerlerini generic hale getirebiliriz. Bazı hallerde bu generic türler için
kısıtlamalar da eklenebilir. Aşağıdaki kod parçasında yer alan **log_any** fonksiyonu, **her türden veriyi** işleyebilir
ancak gelen türün **Debug** trait ile belirtilen bir davranışı desteklemesi gerekir. Bunun nedeni, fonksiyon içinde
println! makrosunun **{:?}** format belirleyicisiyle kullanılmasıdır. **{:?}** kullanımı söz konusu parametrenin Debug
trait' ini implemente etmesini gerektirir.

```rust
use std::fmt::Debug;

fn log_any<T: Debug>(object: T) {
    println!("Logged value is '{:?}'", object);
}

fn main() {
    log_any(3.14f32);
    log_any("TCP connection established");
    log_any(State::InProgress);
    log_any(false);
}
```

Örnekte yer alan Staten enum türününün log_any ile kullanılabilmesi için **Debug** davranışı ile donatılmış olması
beklenir.

```rust
#[derive(Debug)]
enum State {
    InProgress,
    Done,
    Error,
}
```

**Not:** Eğer Debug trait’i belirtilmezse log_any(State::InProgress); çağrısı derleme hatası verecektir.

## Generic Veri Yapıları _(Struct)_

Rust’ta **generic struct'lar** tanımlayarak farklı türlerle kullanılabilecek veri modelleri oluşturabiliriz. Aşağıdaki
örnek kod parçasında yer alan Point<T> isimli struct **3 boyutlu koordinatları** temsil etmekte için tasarlanmıştır.

```rust
use std::fmt::Debug;
use std::ops::Add;

struct Point<T: Copy + Debug + Add<Output=T>> {
    x: T,
    y: T,
    z: T,
}
```

Point ile çalışabilecek generic T türleri için birden fazla kısıtlama _(Constraint)_ tanımlanmıştır.

- T: Debug: T türü Debug trait’ini implemente etmelidir.
- T: Copy: T türü **kopyalanabilir _(Copy trait’ini implemente etmiş)_ olmalıdır.**
- T: Add<Output = T>: T türü **toplama (+) işlemini** desteklemelidir.

Aşağıdaki örnek kod parçasında T türü için gerekli kriterlere göre çalışacak bazı metotlar dahil edilmiştir. new metodu
ile yeni bir Point değişkeni tanımlanabilir, info metodu ile veri içeriği String türden geri döndürülür. add metodu ise
yine generic T türünden olan başka bir Point değişkeni ile toplama işlemini icra eder. Generic kısıtları daha iyi
anlamak için Add trait kuralını kaldırıp programı derlemeyi deneyiniz.

```rust
impl<T: Copy + Debug + Add<Output=T>> Point<T> {
    fn new(x: T, y: T, z: T) -> Self {
        Point { x, y, z }
    }

    fn info(&self) -> String {
        format!("({:?}, {:?}, {:?})", self.x, self.y, self.z)
    }

    fn add(self, other: Point<T>) -> Point<T> {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

fn main() {
    let game_point = Point::<i32>::new(10, 20, 10);
    println!("{}", game_point.info());

    let vehicle_position: Point<f32> = Point::new(5.5, 3.14, 2.20);
    println!("Vehicle Position {}", vehicle_position.info());

    let vehicle_new_position: Point<f32> = vehicle_position.add(Point::new(1.0, 1.0, 2.0));
    println!("New position after move {}", vehicle_new_position.info());
}
```

Not: Generic kısıtlar bazen çok uzun olabilir. Okunabilirliği artırmak için where ifadesi de ele alınabilir. Yukarıdaki
örnekte yer alan implementasyon kısmı için aşağıdaki gibi bir tanımlama da yapılabilir.

```rust
impl<T> Point<T>
where
    T: Copy + Debug + Add<Output=T>,
{
    // Codes
}
```

## Generic Tür Kullanmanın Avantajları

Generic türler bazı avantajları da beraberinde getirirler.

- **Kod tekrarını azaltır**: Aynı işlemi farklı türler için tekrar tekrar yazmaktan kurtuluruz.
- **Esneklik sağlar**: Farklı veri türlerini tek bir yapı içinde kullanabiliriz.
- **Performans açısından güvenlidir**: Rust’ın monomorphization _(tip uyarınca özelleştirme)_ kabiliyeti sayesinde,
  generic türler derleme zamanı optimizasyonları ile çağırıldığı tipe uygun makine koduna dönüştürülür.

## Monomorphization Mekanizması

Belirli koşullarda generic türler çalışma zamanında tip dönüşümüne tabi olmazlar. Bunun yerine, **derleme zamanında**
her farklı tür için **ayrı bir versiyon oluşturulur**. Bu işlem **monomorphization** olarak da adlandırılır. Örneğin,
yukarıdaki kod parçasına göre hem **i32** hem de **f32** türleri için **derleme zamanında ayrı ayrı fonksiyonlar
üretilir**. Bu durumu aşağıdaki gibi düşünebiliriz.**

```rust
impl Point<i32> {
    fn new(x: i32, y: i32, z: i32) -> Self { ... }
    fn add(self, other: Point<i32>) -> Point<i32> { ... }
}

impl Point<f32> {
    fn new(x: f32, y: f32, z: f32) -> Self { ... }
    fn add(self, other: Point<f32>) -> Point<f32> { ... }
}
```

Böylece **performans kaybı olmadan** generic kullanımının avantajlarından faydalanabiliriz.

## Dynamic Dispatch

Yukarıdaki senaryo gereği derleme zamanında **monomorphization** süreci işler ve her tür için ayrı bir versiyon
üretilir. Lakin **dynamic dispatch** kullanılması halinde bu durum değişir. Örneğin, **Box<dyn Trait>** gibi bir
kullanım söz konusu olduğunda **runtime polymorphism** devreye girer ve **monomorphization** mekanizması çalışmaz.
Durumu daha iyi anlamak için örnek üzerinden ilerleyelim. İlk senaryomuzda **static dispatch _(monomorphization)_**
uygulanan yapılar konusu.

```rust
trait Drawable {
    fn draw(&self);
}

struct Circle;
struct Square;

impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing a Circle");
    }
}

impl Drawable for Square {
    fn draw(&self) {
        println!("Drawing a Square");
    }
}

fn draw_static<T: Drawable>(shape: &T) {
    shape.draw();
}

fn main() {
    let c = Circle;
    let s = Square;

    draw_static(&c); // `draw_static::<Circle>` derleme zamanında oluşturulur
    draw_static(&s); // `draw_static::<Square>` derleme zamanında oluşturulur
}
```

Generic olarak tanımlanmış draw_static fonksiyonunun Circle ve Square kullanımlarına istinaden derleyici iki farklı
metot oluşturacaktır. Eğer runtime polymorphism kullanmak istiyorsak trait nesnesini dynamic tanımlamak gerekir.
Aşağıdaki kod parçasında bu durum örnekleniyor. Dikkat edileceği üzere Drawable davranışı &dyn operatörü ile birlikte
donatılmıştır.

```rust
fn draw_dynamic(shape: &dyn Drawable) {
    shape.draw();
}

fn main() {
    let c = Circle;
    let s = Square;

    draw_dynamic(&c);
    draw_dynamic(&s);
}
```

Dynamic Dispatch _(dyn Trait)_ kullanıldığında tek bir fonksiyon oluşturulur ve vtable _(sanal tablo)_ üzerinden
çağrım işlemleri gerçekleştirilir. Tahmin edileceği üzere bunun bir runtime maliyeti vardır zira her çağrı sırasında
**vtable üzerinden bir look-up** operasyonu gerçekleştirilir.

Box<dyn Trait> enstrümanı ile de generic türlerin dinamik hale gelmesi sağlanabilir. Aşağıdaki kod parçasını göz
önüne alalım.

```rust
fn draw_boxed(shape: Box<dyn Drawable>) {
    shape.draw();
}

fn main() {
    let c = Box::new(Circle);
    let s = Box::new(Square);

    draw_boxed(c); // Tek bir fonksiyon çağrısı, runtime dispatch
    draw_boxed(s);
}
```

Bu durumda generic parametreler monomorphization yerine tek bir dyn Trait referansına dönüşür. Fonksiyon her tür
için tekrar tekrar üretilmez, aksine runtime lookup kullanılır.

_Not: Eğer generic türler T: Trait olarak kullanılırsa, Rust monomorphization yapar ve her tür için ayrı versiyonlar
oluşturur. Eğer Box<dyn Trait> veya &dyn Trait kullanılırsa, runtime dispatch söz konusu olur ve tek bir fonksiyon
çalıştırır. Dynamic dispatch kullanımı monomorphization avantajını kaybettirir zira esneklik sağlar._

Rust, performans açısından mümkün olduğunca monomorphization kullanır ancak dinamik dispatch gerektiğinde runtime lookup
mekanizmasını devreye sokar.

## Enum ile Kullanım

Enum veri yapılarında da generic türler kullanılabilir. Aşağıdaki kod parçasında bu durum ele alınmaktadır.

```rust
#[derive(Debug)]
enum ApiResponse<T> {
    Success(T),
    Error(String),
}

fn main() {
    let res_1: ApiResponse<i32> = ApiResponse::Success(200);
    let res_2: ApiResponse<&str> = ApiResponse::Error("404 Bad Request".to_string());

    println!("{:?}", res_1);
    println!("{:?}", res_2);
}
```

## Sonuç

- Rust’ta **generic türler** sayesinde, aynı kodu farklı türler için yeniden yazmaya gerek kalmaz.
- **Trait Bound (Kısıtlama)** ekleyerek generic türlerin **hangi işlemleri destekleyeceğini** belirleyebiliriz.
- **Monomorphization** derleme zamanında **generic türler için özel kod üretilmesini** sağlayarak **runtime
  maliyetlerini minimize eder**.
