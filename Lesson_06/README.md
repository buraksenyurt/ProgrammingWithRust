# Ders 07: Lifetime (Yaşam Ömrü) Kavramı

Rust, bellek güvenliğini sağlarken **ownership** konusuna dikkat eder.Bazı durumlarda referanslar belirli bir kapsam
dahilinde yaşam ömrüne sahiptir ve ne kadar süre yaşayacaklarının açık bir şekilde belirtilmesi gerekir. **Lifetime (
yaşam ömrü)** kavramı referansların ne kadar süre yaşayabileceğini belirlemeye yardımcı olur ve **dangling reference** (
geçersiz referans) oluşmasını engeller.

## Dangling Pointer Sorunu ve Lifetime Kullanımı

Bir referansa, işaret ettiği veri bellekten düştüğü halde hala erişim sağlanmaya çalışılırsa **dangling reference**
durumu oluşur. Rust derleyicisi bu gibi durumları fark eder, hata üretir ve kodu derlemez. Aşağıdaki örnek kod parçasını
ele alalımı. Aşağıdaki örnekte point değişkeni açıkça belirtilen bir scope _(kapsam)_ içinde tanımlanmıştır. Ayrıca
scope dışında tanımlanmış **value** değişkenine referansı atanmıştır. Scope sonlandığından point değişkeni otomatik
olarak bellekten düşer ve içeriği silinir. Bu durumda value var olmayan bir değere referans etmeye devam eder. Ancak
görüldüğü üzere rust derleyicisi bunu tespit ederek hata verir ve kod derlenmez.

```rust
fn main() {
    let value;
    {
        let point = 2.25;
        value = &point; // point, bu bloğun sonunda yok olacak
    }
    println!("{}", value); // Geçersiz referans
}
```

Rust derleyicisi bu kodu derlenmez ve aşağıdaki hata mesajını üretir.

```
error[E0597]: `point` does not live long enough
      --> S08_lifetimes\src/main.rs:13:17
       |
    12 |         let point = 2.25;
       |             ----- binding `point` declared here
    13 |         value = &point;
       |                 ^^^^^^ borrowed value does not live long enough
    14 |     }
       |     - `point` dropped here while still borrowed
    15 |     println!("{}", value);
       |                    ----- borrow later used here

    For more information about this error, try `rustc --explain E0597`.
```

Burada açık bir şekilde referansın gereğinden az yaşaması sebebiyle oluşan bir hata durumu söz konusudur ancak bazı
durumlarda Rust, hangi referansın daha uzun yaşaması gerektiğini bilemez. Bu gibi durumlarda **lifetime** belirteçleri
devreye girer. Durumu aşağıdaki diagramla da irdeleyebiliriz.

```text
┌──────────── Scope Başlangıcı ─────────────┐
│
│   let point = 2.25;       <-- point burada oluşturuldu
│   value = &point;         <-- value, point'e referans aldı
│
└──────────── Scope Sonu ───────────────────┘
                                ↓
                    point drop edildi (geçersiz oldu)
                                ↓
         value hala point'i referans etmeye çalışıyor
```

Aşağıdaki kod örneğinde **Account** isimli bir **struct** tanımlanmış ve **literal string** olarak **customer_name**
alanı kullanılmıştır. Burada **customer_name** ile işaret edilen referansın yaşam ömrünün **Account** referanslarının
yaşam ömrü ile aynı olması gerekir.

```rust
struct Account<'a> {
    customer_name: &'a str, // Lifetime belirtilmezse derleyici hata verecektir
    balance: f32,
}

fn main() {
    let name = String::from("Jan Klot Van Dam");
    let van_dam_account = Account {
        customer_name: &name,
        balance: 1000f32,
    };
    println!(
        "{} has a {} coin",
        van_dam_account.customer_name, van_dam_account.balance
    );
}
```

Eğer **customer_name** için bir lifetime annotation kullanılmazsa derleyici aşağıdaki hatayı üretir.

```
error[E0106]: missing lifetime specifier
          --> S08_lifetimes\src/main.rs:66:20
           |
        66 |     customer_name: &str,
           |                    ^ expected named lifetime parameter
           |
        help: consider introducing a named lifetime parameter
           |
        65 ~ struct Account<'a> {
        66 ~     customer_name: &'a str,
           |
```

## 'static Lifetime

Bazı referans değerlerin programın tüm çalışma zamanı boyunca yaşaması istenebilir. Bu tip değerlerin yaşam ömrü **'
static lifetime** kullanarak belirtilir. Genellikle sabit metinsel ifadeler _(string literals)_ gibi veriler
için ele alınır.

Aşağıdaki örnekte **ApplicationSettings** isimli bir yapı tanımlanmıştır. Bu yapıda yer alan **server_address** ve *
*connection_string** bilgileri esasında tüm program boyunca herhangi bir yerden erişilip kullanılacak türdendir.
Dolayısıyla bu bilgileri static bir yaşam süresi belirtilerek kullanılabilir.

```rust
struct ApplicationSettings<'a> {
    server_address: &'a str,
    connection_string: &'a str,
}

fn get_app_settings() -> ApplicationSettings<'static> {
    ApplicationSettings {
        server_address: "localhost",
        connection_string: "data source=Northwind;server:127.0.0.1;",
    }
}
```

Yukarıdaki fonksiyon tanımlarında dikkat edileceği üzere **'static** lifetime açıkça belirtilmiştir zira *
*server_address** ve **connection_string** sabit metinsel ifadelerdir ve tüm program boyunca var olacaklardır.

```text
      "localhost"           "data source=...;"
         ↓                           ↓
┌───────────────┐      ┌────────────────────────┐
│ string literal│      │ string literal         │
│ (lifetime: 'static)  │ (lifetime: 'static)    │
└───────┬───────┘      └────────────┬───────────┘
        │                           │
        └──> ApplicationSettings<'static>
```

**'static** yaşam ömrü mümkün mertebe az kullanılmalıdır ziraz nesnelerin yaşam ömrüne doğru süreleri vermek bellek
optimizasyonu açısından daha verimlidir. Büyük sistemlerde bu optimizasyon çok fark yaratmazasa genellikle kapasite
olarak daha düşük konfigurasyonda olan gömülü sistemlerde veya yüksek saat hızı gerektiren problemlerde önem arz
etmektedir.

## Fonksiyon Parametrelerinde Lifetime Kullanımı

Bir fonksiyon, referans türünden parametreler ile çalışıyorsa **lifetime annotation** kullanılması gerekebilir.
Aşağıdaki kod parçasında yer alan **longest** fonksiyonunu göz önüne alalım. Bu fonksiyon parametre olarak iki literal
string alır ve hangisinin daha uzun olduğunu bulur. Literal string kullanımı söz konusu olduğundan ilgili parametrelerin
hangi scope'larda hayatta kalması gerektiğinin açıkça tanımlanması ve fonksiyonları kullanacak scope'lar tarafından da
ele alınması gerekir.

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let s1 = String::from("Hello");
    let s2 = String::from("Rustaceans");
    let result = longest(&s1, &s2);
    println!("Longest string: {}", result);
}
```

Burada, **'a** lifetime belirteci, parametre olarak gelen x ve y argümanları ile geriye döndürülen referansın aynı
yaşam ömrüne sahip olmasını garanti eder.

Bazı durumlarda parametre yapıları veya dönüş türleri farklı yaşam süresi belirteçlerine sahip olabilirler. Aşağıdaki
kod parçasında bu durum basitçe özetlenmiştir. İlk olarak aşağıdaki kod parçasını ele alalım.

```rust
fn select_first(x: &str, y: &str) -> &str {
    x
}

fn main() {
    let s1 = String::from("Black");
    let s2 = String::from("Smith");
    let result = select_first(&s1, &s2);
    println!("First Word: {}", result);
}
```

Tahmin edileceği üzere bu kod derlenmeyecek ve lifetime hatası alınacaktır. Hatta rust derleyicisi bir çözüm de
önerecektir

```text
error[E0106]: missing lifetime specifier                                                                                                                                             
 --> Lesson_06\src\main.rs:1:38
  |
1 | fn select_first(x: &str, y: &str) -> &str {
  |                    ----     ----     ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`
help: consider introducing a named lifetime parameter
  |
1 | fn select_first<'a>(x: &'a str, y: &'a str) -> &'a str {
  |                ++++     ++          ++          ++
```

Fonksiyonda kullanılan parametreler ve dönüş türünün aynı yaşam süresine sahip olması sağlanabilir ama bazı hallerde
farklı yaşam sürelerinin belirtilemeleri de gerekebilir. Örnekte a ve b parametrelerinin farklı yaşam sürelerine sahip
olması gerektiğini ve dönüş referansının da ilk parametre ile aynı yaşama döngüsünde olması gerektiğini farz edelim. Bu
durumda fonksiyon tanımı aşağıdaki gibi değiştirilmelidir.

```rust
fn select_first<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    x
}

fn main() {
    let s1 = String::from("Black");
    let s2 = String::from("Smith");
    let result = select_first(&s1, &s2);
    println!("First Word: {}", result);
}
```

Yukarıdaki durumu aşağıdaki diagramla özetleyebiliriz.

```text
x (yaşam süresi 'a) ────────────────┐
                                   │
                                   ├───> return: &'a str
                                   │
y (yaşam süresi 'b) ─────┐         │
                        └────────┘_(y kullanıldı ama dönüş değeriyle ilişkisi yok)_
```

## Lifetime Elision

Rust'ın kendi standart kütüphanesinde **lifetime** kullanımı oldukça yaygındır ancak çoğu durumda söz konusu bileşenleri
kullanırken açıkça **annotation** belirtilmediğini görürüz. Rust çoğu durumda **lifetime elision** olarak da bilinen
kuralı uygular ve biz belirtmesekte kendisi **lifetime** sürelerini tespit eder. Bu tespitte genellikle fonksiyonların
parametre sayısı ve türleri de önemlidir. Söz gelimi tek parametresi olan fonksiyonlar parametre olarak gelen referansın
yaşam ömrünü ele alır. Parametre sayısı 1den fazla ve biri &self ise durum değişir. Self varsa yaşam ömrü &self
kadardır, aksi durumda lifetime belirtmemiz gerekebilir.

```text
   Fonksiyonda kaç adet referans türünden parametre var?
                           │
              ┌────────────┴────────────┐
              │                         │
           Sadece 1                 1den fazla
              │                         │
              ▼                         ▼
     return lifetime,           Parametrelerden biri
     bu parametreye             &self veya &mut self mı?
     eşit kabul edilir             │
                                   ├─────────────┐
                                   │             │
                                 Evet          Hayır
                                   │             │
                                   ▼             ▼
                return lifetime,   │     Açıkça lifetime belirtmek
                &self ile eşdir    │     gerekir!
                                   ▼
```

## Sonuç

- Rust'ta **lifetime annotation**, referansların ne kadar yaşayacağını ve dolayısıyla hangi scope'larda geçerli
  olacağını belirlemek için kullanılır.
- Dangling reference _(geçersiz referans)_ oluşmasını engeller.
- 'static lifetime, programın tüm kapsamlarında ve çalıştığı süre boyunca yaşaması istenen referanslar için kullanılır.
- Fonksiyonların referans alan parametrelerinde bazı türler için lifetime belirtmek gerekebilir.
