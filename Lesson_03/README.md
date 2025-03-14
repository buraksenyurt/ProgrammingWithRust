# Ders 03: Struct Veri Türü

Struct _(yapı)_, Rust'ta verileri gruplamak ve ilgili fonksiyonellikleri kapsamak _(encapsulation)_ için kullanılan
temel veri türlerinden birisidir. Nesne yönelimli programlama _(Object Oriented Programming)_ pratiklerinin
uygulanmasında da önemli bir rolü vardır. Rust dilinde kullanılan üç tür struct vardır.

1. **Named Field Struct** – Alan isimleri ile tanımlanan yapılar.
2. **Tuple Struct** – İçeriklerin Tuple veri türü ile saklandığı yapılar.
3. **Unit Struct** – Herhangi bir alan içermeyen yapılar.

Şimdi bu türleri sırasıyla ele alalım.

## Named Field Struct

Named Field Struct, içindeki alanları adlandırarak verileri saklayan bir struct türüdür. Genellikle gerçek hayat
modellerinin nesnel olarak ifade edilmesinde sıklıkla kullanılır. Komposit bir blok olarak düşünüldüğünde çeşitli
verilerle tanımlanabilen bir değişkeni ifade eder.

Örneğin bir oyuncunun adını, aktiflik durumunu, seviyesini ve sahadaki koordinatlarını tutmak istediğimiz bir değişken
tanımı yapalım. Aşağıdaki örnek kod parçasında olduğu gibi Player ve Position isimli struct türlerinden
yararlanabiliriz.

```rust
struct Position(f32, f32);

struct Player {
    name: String,
    is_active: bool,
    level: u8,
    position: Position,
}

fn main() {
    let marlon = Player {
        name: String::from("Marlon Singer"),
        is_active: false,
        level: 100,
        position: Position(1.0, 0.0),
    };
    println!("{} level is {}", marlon.name, marlon.level);
}
```

## Tuple Struct

Tuple Struct modelinde alanlara belirli isimler vermek yerine, ilgili değerlere sıralı şekilde ulaşılmasını sağlayan
parametreler kullanılır. Neyi ifade ettiği çok belli olan bir tuple serisinin domain için anlamlı bir nesne olarak
kullanılması istendiği durumlar için idealdir bir varsayondur. Söz gelimi x,y,z koordinatlarının matemtikteki vektör
operatörü ile ifade edildiği durumları ele alalım. Bu senaryoda Vector3D isimli bir **tuple struct** tanımlanabilir.
Aşağıdaki örnekte x,y koordinatlarını temsil eden bir struct tanımı söz konusudur.

```rust
struct Position(f32, f32);

fn main() {
    let position = Position(9.0, 12.0);
    println!("Some position at the galaxy ({}:{})", position.0, position.1);
}
```

## Unit Struct

Unit Struct, herhangi bir alan içermeyen ve genellikle marker _(işaretleme)_ için kullanılan bir yapı türevidir. Örneğin
Entity Component System üzerine oturan oyun motorlarında sıklıkla kullanılır ya da belli davranışları _(behavior)_
içeren türlerin ortak tanımlarında ele alınabilir.

```rust
struct Entity;

fn main() {
    let _e = Entity;
    println!("Entity struct initialized");
}
```

## Struct' larda Metot Kullanımı

Struct'lar kendileri ile alakalı iş yapan metotlar barındırabilir. Bunun için **impl** bloğu kullanılır. Örneğin dörtgen
şekilleri ifade edecek Rectangle isimli bir struct tanımlayıp alan ve çevre hesaplamalarını yapan metotları bu veri
yapısı ile ile ilişkilendirebiliriz. Aşağıdaki örnek kod parçasında bu durum ele alınmıştır.

```rust
struct Rectangle {
    width: f32,
    height: f32,
}

impl Rectangle {
    fn new(width: f32, height: f32) -> Self {
        Rectangle { width, height }
    }

    fn area(&self) -> f32 {
        self.width * self.height
    }

    fn perimeter(&self) -> f32 {
        (self.width + self.height) * 2.0
    }

    fn resize(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
    }
}

fn main() {
    let mut rectangle = Rectangle::new(10.0, 20.0);
    println!("Area is {}", rectangle.area());
    println!("Perimeter is {}", rectangle.perimeter());

    rectangle.resize(10.0, 10.0);
    println!("Updated Area is {}", rectangle.area());
    println!("Updated Perimeter is {}", rectangle.perimeter());
}
```

Yukarıdaki örnekte Rectangle yapısı ve metotları main modülünde yer alamktadır. Bu tip veri yapılarının farklı
modüllerde oluşturulması halinde genel erişim için **pub** erişim belirleyicileri ile tanımlanmaları gerekebilir. Public
anlamına gelen pub keyword uygulandığı enstrümanın her yerden erişilebilir olmasını sağlar.

## Metot Dönüşlerinde Struct Kullanımı

Metotlardan geriye kendi tanımladığımız Struct değişkenlerini döndürülebiliriz. Aşağıdaki fonksiyon bir Player
veri yapısı örnekler ve geriye döndürür.

```rust
fn spawn_random_user(name: String) -> Player {
    Player {
        name,
        is_active: true,
        level: 100,
        position: Position(10.0, 20.40),
    }
}

fn main() {
    let durden = spawn_random_user(String::from("Tyler Durden"));
    println!("{}({}) on ({}:{})", durden.name, durden.is_active, durden.position.0, durden.position.1);
}
```

Burada dikkat edilmesi gereken hususlardan birisi de spawn_random_user isimli fonksiyonun yeni bir Player nesnesi
oluşturup döndürmesidir. Farklı senaryolar ve koşullarda struct'ları parametre alan veya döndüren fonksiyonlarda
referans kullanımı tercih edilebilri _(& ile birlikte)_ Ancak bu durumlarda dikkat edilmesi gereken birçok husus
karşımıza çıkar. Sahiplik _(ownership)_ , ödünç alma _(borrow)_, yaşam ömrü _(lifetime)_, referanslar için mutable olma
ilkeleri vs gibi bellek seviyesinde güvenli kullanımı gerektiren birçok unsurun dikkate alınması gerekir.

## Sonuç

Bu bölümde, Rust dilinde struct kullanımı incelenmiş ve aşağıdaki konular ele alınmıştır;

- **Named Field Struct** kullanımı
- **Tuple Struct** yapıları ve örnekleri
- **Unit Struct** tanımları
- **Struct'lara metot ekleme**
- **Fonksiyonlardan struct döndürme**

Struct enstrümanı veri modelleme ve nesneye yönelik programlama mantığını takip eden sistemler geliştirmek için
oldukça önemlidir.
