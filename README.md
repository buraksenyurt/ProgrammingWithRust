# Programming With Rust

Uludağ Üniversitesi Bil.Müh/BÖTE bölümlerinde açılan Rust ile Sistem Programlama dersine ait örnek kodlar ve notların yer aldığı repodur.

- [x] Lesson 00 : Hello World ve Temel Veri Türleri
- [ ] Lesson 01 : 

# Ders 00: Hello World Uygulaması ve Temel Veri Türleri

Rust dilinin sistemde yüklü olup olmadığını öğrenmek için aşağıdaki komut kullanılabilir.

```shell
rustc --version
```

Eğer rust sistemde yüklü değilse [resmi sitesindeki talimatlardan](https://www.rust-lang.org/tools/install)
ilerlenilebilir.

```shell
# Bir binary oluşturmak için
cargo new hello_world

# Library oluşturmak içinse
cargo new --lib vga_driver
```

### Hello World

Aşağıdaki kodu bir `main.rs` dosyasına yazın ve çalıştırın:

```rust
fn main() {
    println!("System Programming with Rust.");
}
```

Bu kod _println!_ makrosunu kullanarak _(! harfi ile biten fonksiyonlar makro olarak bilinir)_ "System Programming with
Rust." mesajını ekrana yazdırır.

```shell
# uygulama doğrudan rust derleyicisi ile çalıştırılabilir
rustc main.rs
./main

# Proje cargo ile oluşturulduysa (ki bundan sonra cargo ile devam edilecektir)
cargo run
```

# Rust Veri Tipleri

Rust, tür güvenliğini _(type safety)_ sıkı biçimde uygulayan, sistem seviyesinde bellek güvenliğini sağlamayı amaçlayan
modern bir programlama dilidir. Güçlü bir tip sistemine sahiptir.

## Değişkenler ve Atama

Rust'ta değişkenler varsayılan olarak **immutable** (değiştirilemez) olarak tanımlanırlar. Değişkenler **mut** ile
işaretlenmedikleri sürece işaret ettikleri değerler değiştirilemez.

```rust
fn main() {
    let last_score = 90; // Immutable değişken
    println!("Last score is {}", last_score);

    let mut last_score = 80; // Mut değişken
    println!("Last score is {}", last_score);
    last_score = 60;
    println!("Last score is {}", last_score);
}
```

## Sayısal Veri Türleri

Rust, **tamsayılar** ve **kayan noktalı** sayılar için geniş bir tür yelpazesi sunar. Tamsayılar işaretli _(i8, i16,
i32, i64, i128)_ ve işaretsiz _(u8, u16, u32, u64, u128)_ olarak iki ana kategoriye ayrılırlar. Kayan noktalı sayılar
ise _f32_ ve _f64_ türlerinden birine sahip olabilir. Veri türleri ile ilgili daha detaylı bilgi
için [Rust resmi dokümanına](https://doc.rust-lang.org/beta/book/ch03-02-data-types.html) bakılabilir.

```rust
fn main() {
    let delta_time: f32 = 1.25;
    println!("Delta time is {}", delta_time);
}
```

Rust, sayısal değerlerin farklı tabanlarda gösterimini de destekler. Yani sayıların on altılık _(Hexadecimal)_, sekizlik
_(octal)_ ve ikilik _(binary)_ tabanlarda gösterimi de mümkündür. Sadece sayının başına 0x, 0o, 0b gibi tanımlayıcılar
getirilmesi yeterli olacaktır. String gösterimlerinde de {:x}, {:o}, {:b} tanımlayıcıları kullanılır.

```rust
fn main() {
    let color_in_hex = 0xFF0032; // Onaltılık taban
    println!("Color is {color_in_hex} or {:x}", color_in_hex);

    let file_permission: u32 = 0o755; // Sekizlik taban
    println!("File permission is {file_permission} or {:o}", file_permission);

    let some_flags: u8 = 0b1010_0110; // İkilik taban
    println!("Flags is {some_flags} or {:b}", some_flags);
}
```

## Boolean ve Karakter Türleri

Boolean türü yalnızca **true** ve **false** değerlerini alabilir. Rust'ta karakterler **Unicode** desteğine de sahiptir
ve tek tırnak içinde tanımlanarak kullanılabilirler.

```rust
fn main() {
    let is_valid = true;
    println!("Is valid? {}", is_valid);

    let up_button = 'w';
    println!("Button is {}", up_button);
}
```

## Sabitler _(Constants)_ ve Değişmezler _(Immutables)_

Rust'ta sabit değerler **const** anahtar kelimesi ile tanımlanır. Bu değerler doğrudan stack bellek bölgesinde
konumlanır ve programın çalışma süresi boyunca değiştirilmezler.

```rust
const MAX_HEIGHT: u16 = 1280;
const BACKGROUND_COLOR: (u8, u8, u8) = (255, 255, 0);

fn main() {
    println!("Max Height is {}", MAX_HEIGHT);
    println!("Background color is {:?}", BACKGROUND_COLOR);
}
```

## Bileşik _(Compound)_ Veri Yapıları

Bazı senaryolardan birden fazla veri türünün bir araya gelerek tek bir türü tanımlaması gerekebilir. Örneğin bir IP
adresinin basit gösterimi, RGB renk kodlarının temsili, belli sayıda elemandan oluşan koordinat bilgileri vb Rust temel
olarak iki önemli bileşik veri türü sağlar.

### Tuple _(Demetler)_

Tuple'lar birden fazla ve farklı türde değişkeni tek bir yapı altında toplayabilir. Özellikle fonksiyon dönüşlerinde
veya parametre yapılarında bu avantajından yararlanılabilir.

```rust
fn main() {
    let arguments = (640, 480, String::from("Game Title"), false);
    println!("Arguments is {:#?}", arguments);
    println!("Screen width and height {}x{}", arguments.0, arguments.1);
}
```

### Array _(Diziler)_

Diziler yalnızca **aynı türden** veriler içerebilir ve sabit uzunluktadır. Dolayısıyla tanımlanırken belli bir boyuta
sahip olmaları beklenir.

```rust
fn main() {
    let points = [9.0, 8.5, 7.2, 8.4, 9.1];
    println!("Points is {:#?}", points);
    println!("First value is {}", points[0]);
}
```

Diziler değiştirilebilir _(mutable)_ olarak da tanımlanabilir ve böylece üyelerinde değişiklikler yapılabilir. Bu
durum yukarıdaki tuple türü için de geçerlidir.

```rust
fn main() {
    let mut numbers = [1, 2, 3, 4, 5];
    numbers[2] = 8;
    println!("Mutated second value is {}", numbers[2]);
}
```

## Vektörler _(Vectors)_

Rust'ta **Vektörler**, dinamik olarak büyüyebilen veri yapılarıdır ve heap üzerinde bellek tahsis ederler. Dolayısıyla
dizi _(array)_ türünden farklı olarak sabit boyutlu değillerdir. Özellikle programın çalışma zamanında dinamik olarak
manipüle edilebilen _(eleman eklenen, çıkartılan vb)_ belli türde nesne koleksiyonlarını kullanmak istediğimiz durumlar
için idealdir. Aşağıdaki kod parçasında vektörlerin temel kullanımına ait örnekler yer almaktadır.

```rust
fn main() {
    let mut points = vec![3.14, 2.67, 1.24, 9.80];
    points.push(7.5);
    println!("{:?}", points);
    println!("First point is {}", points[0]);

    let last_in = points.pop();
    println!("{:?}", last_in);
    println!("Current vector elements {:#?}", points);

    let mut names = Vec::new();
    names.push(String::from("Bob"));
    names.push(String::from("Frank"));
    names.push(String::from("Orange"));
    names.push(String::from("Mary"));
    names.reverse();
    println!("Players vector capacity is {:?}", names.capacity());
    println!("Players :\n {:#?}", names);

    let codes: Vec<u8> = (50..=60).collect();
    println!("Looking codes are, {:?}", codes);
}
```

Vektörler **dilimler (slices)** ile de birlikte kullanılabilirler. Bir **slice** ile bir vektörün belli bir aralığı
referans edilebilebilir. Bazı durumlarda bir vektörün tamamını işleme almak yerine sadece gerekli parçalarını ele almak
isteyebiliriz.

```rust
fn main() {
    let codes: Vec<u8> = (0..100).collect();
    let first_ten = &codes[0..=10];
    println!("First ten: {:#?}", first_ten);
}
```

Vektörler, Rust programlama dilinde esnek ve güçlü veri yapılarını yönetmek için önemli bir bileşendir.

## String Veri Türü

Rust'ta metinleri yönetmek için genellikle **String** ve **&str** türleri kullanılır. **String** heap üzerinde dinamik
olarak büyüyebilen bir veri türüdür, diğer yandan **&str** ise değişmezdir _(immutable)_ ve genellikle string literaller
için kullanılır. &str dilimlenmiş metinsel içerik olarak da düşünülebilir. Büyük bir metinsel parçanın tamamının
operasyona dahil edilmesi yerine sadece değerlendirilecek kısmının ele alınmasında oldukça kullanışlıdır.

```rust
fn main() {
    let hero_name = "Can Cloud Van Dam".to_string();
    println!("{}", hero_name);

    let position = String::from("Quarter back");
    println!("{}", position);

    let short_name = hero_name.replace("Can Cloud Van Dam", "CCVD");
    println!("{}", &short_name);
}
```

### String Slice _(&str)_

**&str** türden bir değişken ile bir **String** değişkeninin belirli bir kısmı _(dilimi)_ referans edilebilir. Böylece
heap üzerinde yeni bir bellek tahsis edilmesine gerek kalmadan işlemler yapılabilir.

```rust
fn main() {
    let greetings = "Greetings dear young and crazy brains".to_string();
    let first_word = greetings.get(0..10).unwrap();
    println!("{}", first_word);
}
```

String türlerin birbirleriyle kıyaslanmaları ile ilgili olarak aşağıdaki örnek kod parçasını ele alabiliriz.

```rust
fn main() {
    let word_aloha = "Aloha!";
    let word = word_aloha.to_string();
    let word_ref = &word;
    println!("{}", word_aloha);
    println!("{}", word);
    println!("{}", word_ref);

    println!("Words are equal: {}", word_aloha.to_lowercase() == "aloha!");
}
```

Rust ayrıca **Unicode** karakterleri de destekler. Örneğin aşağıdaki ifade ile ekrana Japonca Konnicihiwa
yazılır _(Test ediniz)_

```rust
fn main() {
    let konnichiwa = "\u{3053}\u{3093}\u{306B}\u{3061}\u{306F}";
    println!("{}", konnichiwa);
}
```

Bu özellik sayesinde Rust dilini her tür ulusal dil lehçesi ile birlikte kullanmak mümkün hale gelir.

## Sonuç

Bu derste Rust dilinin temel veri türlerine yer verilmiş olup aşağıdaki konular ele alınmıştır:

- **Değişkenler:** Immutable ve mutable değişkenler
- **Sayısal türler:** Integer ve floating-point türleri
- **Mantıksal ve karakter türleri:** Boolean ve karakterler
- **Sabitler(Constants):** const anahtar kelimesi
- **Bileşik(Compound) veri türleri:** Tuple ve array kullanımı
- **Vektörler(Vectors)**: Dinamik büyüyebilen veri yapıları ve dilimler _(slices)_
- **String Veri Türleri:** String ve &str kullanımı, Unicode desteği

## Tavsiye Okumalar

- [Rust ve Güvenli Bellek Yönetimi Hakkında](https://www.buraksenyurt.com/post/rust-ve-guvenli-bellek-yonetimi-hakkinda)
