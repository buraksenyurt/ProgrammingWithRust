# Ders 01: Fonksiyonlar

Pek çok programlama dilinde olduğu gibi Rust dilinde de fonksiyonlar kodun tekrar kullanılabilirliğini artıran önemli
enstrümanlar arasındadır. Rust fonksiyonları **fn** kelimesi ile tanımlanır, dönüş türüne ve parametrelere sahip
olabilirler.

## Temel Fonksiyon Kullanımı

Bir fonksiyon en basit haliyle aşağıdaki gibi tanımlanabilir. print_greetings isimli fonksiyon herhangi bir parametre
almaz veya bir
sonuç döndürmez. Çağırıldığında sadece bünyesinde ihtiva ettiği işlemleri gerçekeştirir.

```rust
fn print_greetings() {
    println!("Welcome to my new world!");
}

fn main() {
    print_greetings();
}
```

## Parametre Alan Fonksiyonlar

Çok doğal olarak fonksiyonlara parametre/ler gönderebiliriz. Örneğin aşağıdaki fonksiyon bir **string slice** almakta _(
&str)_ ve ekrana yazdırmaktadır.

```rust
fn greet(name: &str) {
    println!("Hello {}! How are you?", name);
}

fn main() {
    let name = "Can Cloud Van Dam";
    greet(name);
}
```

## Geri Dönüş Değerine Sahip Fonksiyonlar

Bir fonksiyonun geriye değer döndürmesi için **->** işareti kullanılır. Fonksiyonlardan herhangi bir tür dönebilir.
Dönüş değerlerinin önmeli veya hata yönetiminin kritik olduğu durumlarda genellikle **Result** ve **Option** gibi
enstrümanlardan yararlanılır.

```rust
fn sum(value_1: i32, value_2: i32) -> i32 {
    value_1 + value_2
}

fn main() {
    println!("4 + 5 = {}", sum(4, 5));
}
```

## Fonksiyonlarda Vector Kullanımı

Fonksiyonlarda parametre ve dönüş değeri olarak vektörleri kullanılabilir. Aşağıdaki örneklerde yer alan
fonksiyonlar parametre olarak vektör referansları kullanırlar. Yani metotların çağırıldığı yerdeki vektör içeriğinin
kendisi tüm içeriği ile birlikte kopyalanarak fonksiyona taşınma, bunun yerine referans bilgisi taşınır. Bu bellekte
yapılacak gereksiz kopyalama işini engeller ve çalışam zamanı maliyetini de azaltır.

_Fonksiyonlara parametre geçilirken Rust'ın **ownership**, **borrow checker**, **lifetime** gibi mekanizmaları devreye
girer ve olası hataların önüne geçmek için derleme zamanını kontrol altına alır. Bu önemli konuya ilerleyen bölümlerde
yer vereceğiz._

```rust
fn get_evens(numbers: &Vec<i32>) -> Vec<i32> {
    let mut evens: Vec<i32> = Vec::new();
    for number in numbers {
        if *number % 2 == 0 {
            evens.push(*number);
        }
    }
    evens
}

fn get_odds(numbers: &[i32]) -> Vec<i32> {
    numbers
        .iter()
        .filter(|&&i| i % 3 == 0)
        .cloned()
        .collect::<Vec<i32>>()
}
```

## 5. Tuple Dönüşü Yapan Fonksiyonlar

Fonksiyonlardan birden fazla değer döndürmek istediğimiz durumlarda ele alabileceğimi alternatiflerden birisi de **Tuple
** türüdür. Pek tabii daha sonradan göreceğimiz gibi kendi veri yapılarımızı da dönüş türü olarak kullanabiliriz.
Aşağıdaki örnek fonksiyon aldığı parametrelere göre yeni koordinat noktası için bir öteleme yapar ve x,y değerlerini bir
tuple olarak döndürür. Tuple ile dönen nesne içeriklerine hatırlanacağı üzere indis değerleri üzerinden erişilebilir _(
.0, .1 gibi)_

```rust
fn move_position(mut x: f32, mut y: f32, acceleration: f32) -> (f32, f32) {
    x += acceleration;
    y += acceleration;
    (x, y)
}
```

## 6. Farklı Fonksiyon Kullanım Örnekleri

Aşağıdaki fonksiyon parametre olarak gelen string dilimdeki her bir değeri büyük harfe dönüştürerek ekrana
basar.

```rust
fn print_all_to_upper(names: Vec<&str>) {
    for name in names {
        println!("{}", name.to_uppercase());
    }
}
```

Aşağıdaki örnekte recursive bir fonksiyon kullanılarak faktöryel hesabı yapılmaktadır.

```rust
fn factorial(n: u32) -> u32 {
    if n == 0 { 1 } else { n * factorial(n - 1) }
}

fn main() {
    println!("5! = {}", factorial(5));
}
```

Bu fonksiyon ise 0 ile belli bir değer arasındaki sayıların toplamını bulmak için **Gauss** tekniğini icra eder.

```rust
fn gauss_sum(n: u32) -> u32 {
    (n * (n + 1)) / 2
}

fn main() {
    println!("Gauss sum of 100: {}", gauss_sum(100));
}
```

## 7. Result ve Option Türleri Hakkında

Önceden de belirttiğimiz üzere bazı hallerde metotların dönüş değerlerinin kontrollü şekilde ele alınması gerekir. Hata
durumlarını ele aldığımız hallerde [Result](https://doc.rust-lang.org/std/result/) veya Some/None
döndüren [Option](https://doc.rust-lang.org/std/option/) türleri kullanılabilir. Aşağıdaki iki örnek kod parçasını bu
anlamada ele alabiliriz. **safe_divide** isimli fonksiyon sıfıra bölme durumunu bir **Err** nesnesi ile ele alır _(Err
nesnesi bu örnekte sadece String dönecek şekilde kullanılmıştır. Gerçek hayat senaryolarında hatayı bir
Enum türünde veya farklı bir composit türde ele almak gerekir)_ İlgili main fonksiyonunda, safe_divide çağrısından dönen
Result türünün **pattern matching** ile nasıl ele alındığına dikkat edelim.

```rust
fn safe_divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Division by zero is not allowed!"))
    } else {
        Ok(a / b)
    }
}

fn main() {
    match safe_divide(10.0, 2.0) {
        Ok(result) => println!("Result: {}", result),
        Err(error) => println!("Error: {}", error),
    }
}
```

Bazı durumlarda hata durumunu ele almak yerine **Some** veya **None** ile dönüş değerinin ele alınmasını da
sağlayabiliriz. Bu durumda aşağıdaki örnekte olduğu gibi **Option<T>** türünden yararlanırız. **get_element** fonksiyonu
numbers dizisinden index ile gelen sıradaki elemanın bir klonunu geriye döndürür. **cloned** fonksiyonu bunu **Option<T>
** türü ile sağlar. Dolayısıyla main fonksiyonunda bu dönüş değerinin var olup olmadığı bir önceki örnekte olduğu gibi *
*pattern matching** ile kontrol edilebilir.

```rust
fn get_element(numbers: &[i32], index: usize) -> Option<i32> {
    numbers.get(index).cloned()
}

fn main() {
    let numbers = vec![10, 20, 30, 40, 50];
    match get_element(&numbers, 2) {
        Some(value) => println!("Found: {}", value),
        None => println!("Index out of bounds"),
    }
}
```

_Not: Rust dilinde null, nil gibi kavramlar yoktur. Buna göre dönüş türlerinin işe yarar kılınması ve olası tüm
durumların ele alınması garanti edilir._

## Sonuç

Bu çalışmada Rust dilinde fonksiyon kullanımı incelenmiş ve aşağıdaki konular ele alınmıştır.

- **Temel fonksiyon tanımlama**
- **Parametre alan, dönüş türlü kullanan fonksiyonlar**
- **Vektör işleme fonksiyonları**
- **Tuple döndüren fonksiyonlar**
- **String manipülasyon fonksiyonları**

## Tavsiye Yazılar

- [Rust Pratikleri - Değişkenleri Kopyalayarak veya Referans Olarak Taşımak](https://www.buraksenyurt.com/post/rust-pratikleri-degiskenleri-kopyalayarak-veya-referans-olarak-tasimak)
