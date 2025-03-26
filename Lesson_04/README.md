# Ders 04 - Ownership ve Borrowing Hakkında

Rust bellek yönetimi konusunda oldukça hassastır ve katı kurallar içerir. Bellek güvenliğini Garbage Collector veya farklı bir ortama ihtiyaç duymadan karşılarken performansı kayıplarının da önüne geçmeye çalışır. Bellek yönetiminde **scope** kullanımı, **ownership**, **borrow checker** ve **lifetime** konuları öne çıkar ve bunları iyi bilmek gerekir.

**Ownership _(sahiplik)_** ilkesi değişkenlerin işaret ettikleri verilere yönelik sahiplikleri ile ilgilidir.

## Temel Ownership Kuralları

1. Her değerin t anında yalnızca bir sahibi _(owner)_ olabilir bir başka deyişle bir değer _(value)_ aynı anda sadece bir değişken _(variable)_ tarafından sahiplenebilir.
2. Bir değişken başka bir değişkene atandığında sahiplik taşınır _(move edilir)_ ve ilk değişken artık o değere erişemez.
3. Bir değerin _(value)_ sahipliği kaybolduğunda _(değişken kapsam-scope dışına çıktığında)_ bellekten temizlenir _(değer için tahsis edilen bellek bölgesi de serbest kalır)_

### Scope ve Sahiplik

Bir değişken kapsam dışına çıktığında hem veri hem de sahipliği otomatik olarak düşer. Aşağıdaki kod parçasını ele alalım. **main** fonksiyonuna bilinçli olarak **{ }** bloğu açılmıştır. İçeride tanımlı **player_score** değişkeni scope sonuna gelindiğinde otomatik olarak bellekten düşer. Bu nedenle tekrar kullanılmak istenirse olmayan bir referansın kullanımı söz konusu olur. Bu durum derleme zamanında yakalanan bir hata olarak karşımıza çıkar.

```rust
fn main() {
    {
        let player_score = 90;
        println!("Player Score is {}", player_score);
        let _title = String::from("Programming with Rust");
        let _size = (640_f32, 480_f32);
    } // Scope bittiğinde player_score ve diğer değişkenler bellekten düşer.

    // println!("Player Score is {}", player_score); // Error: player_score artık erişilebilir değil.
}
```

## Move ve Copy Kuralları

Değişkenler arası atamalarda sahiplik el değiştirir ancak veri türüne göre atamalar farklı şekillerde ele alınır. Örneğin **stack** odaklı türler kopyalama davranışına sahiptir ve bu tip atamalar sonrasında sahiplikler kaybolmaz. Bu durum genellikle Copy trait'i destekleyen türler için geçerlidir. _(Copy Trait'ini destekleyen i32, u8, f32, bool vb türler **otomatik olarak kopyalanırlar**)_

```rust
fn main() {
    let number = 32u8;
    let other_number = number; // Kopyalama yapılır, sahiplik kaybolmaz.
    println!("Number is {}", number);
    println!("Other number is {}", other_number);
}
```

Ancak birde **Move edilen türler** vardır. **String, Vec<T>** gibi heap'te veri saklayan türler, başka bir değişkene atandığında **veri sahiplikleri bu yeni değişkene taşınır.** Konuyla ilgili olarak aşağıdaki kod parçasını göz önüne alabiliriz.

```rust
fn main() {
    let first_name = String::from("Burak");
    let full_name = first_name + " Selim Şenyurt"; // first_name artık kullanılamaz.
    println!("Full name is {}", full_name);
    // println!("First name is {}", first_name); // Value used after being moved [E0382]
}
```

Örnek kod parçasını yorum satırını açarak deneyebiliriz.

## Fonksiyon Parametrelerinde Sahiplik Durumları

Bir değişken bir fonksiyona parametre olarak gönderildiğinde, değişken sahipliği de bu fonksiyona geçer. Aşağıdaki örnek kod parçasını ele alalım. **main** fonksiyonunda tanımlanan **book_title** değişkeni **referans** türlüdür. **search_book** fonksiyonu da **String** türünden bir değişken alır. **book_title** bu fonksiyona aktarıldığında değişken sahipliği **search_book** fonksiyonuna geçer ve fonksiyon işleyişini tamamlandığında bu sahiplik ile işaret ettiği değer bellekten düşer. Dolayısıyla **main** fonksiyonu devamında artık **book_title** değişkeni kullanılamaz zira artık bellekte yoktur.

```rust
fn search_book(title: String) {
    println!("Searching for '{}'", title);
} // title burada bellekten düşer

fn main() {
    let book_title = String::from("Programming with Rust");
    search_book(book_title);
    // println!("Book title is {}", book_title); 
    // Error: Ownership fonksiyona geçtiği için artık book_title kullanılamaz.
}
```

Sahipliğin fonksiyonlara taşınmasında yukarıdaki gibi sorunların önüne geçmek için bellek güvenli bazı yöntemler kullanılabilir. Örneğin sahipliği tekrar geri almak için fonksiyon geri dönüş değeri olarak yeni bir değişken kullanılabilir ya da **&** operatörü ile _(yani referansı ödünç verme usülü ile)_ aktarım gerçekleştirilebilir. Aşağıdaki örnek kod parçasında yer alan check_department fonksiyonu parametre olarak gelen String değeri kullandıktan sonra yeni bir String değişken olarak geriye dönmektedir.

```rust
fn check_department(department: String) -> String {
    println!("Checking department: {}", department);
    department // Sahipliği geri döndür
}

fn main() {
    let department = String::from("IT");
    let department = check_department(department);
    println!("Department: {}", department); // Ownership tekrar alındı, kullanılabilir.
}
```

## Borrowing _(Ödünç Alma)_

Fonksiyonlara sahipliği taşımadan da değişken değerlerini gönderebiliriz. Ödünç verme _(borrowing)_ olarak da
adlandırılan bu süreçte **&** ile referans usülü taşımalar söz konusudur. Aşağıdaki örnek kod parçasında yer alan **load_level**
fonksiyonu **name** isimli **String** bir değişkeni referans olarak alır. Dolayısıyla main ile gönderilen değer aslında **level_name** değişkeninin referansıdır. Dolayısıyla sahiplik taşınması yerine referansın aktarımı söz konusudur. load_level fonksiyonu sonlansa dahi kendisine ödünç verilen referans yaşamına devam eder.

```rust
fn load_level(name: &String) {
    println!("Loading level {}", name);
}

fn main() {
    let level_name = String::from("Casino Royal");
    load_level(&level_name); // & işareti ile sahiplik fonksiyona verilmez, sadece referans gönderilir.
    println!("Now game at level {}", level_name); // level_name halen kullanılabilir.
}
```

Bu türden ödünç vermelerde referansın değiştirilebilir _(mutable)_ olarak geçirilmesi de mümkündür. Dolayısıyla ödünç
verilen referans üzerinde değişiklikler yapılabilir. Aşağıdaki kullanımda **mutable referans kullanımı** örneklenmiştir.

```rust
fn trim_last(context: &mut String) {
    context.pop();
}

fn main() {
    let mut some_words = String::from("Some words !.");
    trim_last(&mut some_words);
    println!("{}", some_words);

    /*
         Aşağıdaki kullanım da mümkündür.
         some_words değişkeninin sahipliği mutable referans olarak trim_last fonksiyonuna taşınır.
    */
    let mut some_words = String::from("Some words !.");
    trim_last(&mut some_words);
    println!("{}", some_words);
    trim_last(&mut some_words);
    println!("{}", some_words);
}
```

## Birden Fazla Mutable Olma Hali

Referans ödünç alma _(borrowing)_ ile ilgili iki ana kural vardır;

- t anında bir değerin yalnızca bir değiştirilebilir _(mutable)_ referansı olabilir.
- Bir değerin hem değiştirilebilir hem de değişmez _(immutable)_ referansları aynı anda var olamaz. Lakin n tane
  immutable referans söz konusu olabilir.

Paylaşımlı veri kullanan iş parçacıkları _(threads)_ söz konusu olduğunda ise **borrowing** konusu biraz daha karmaşıklaşabilir ve **Smart Pointer** türlerinin kullanımını gerektirebilir. Bu konuyu ileride ele alacağız.

```rust
fn main() {
    only_one_mutable();
}

fn only_one_mutable() {
    let mut greetings = String::from("Hello HAL! How are you doing?");

    let ref_1 = &greetings; // Immutable referans
    let ref_2 = &greetings; // Diğer immutable referansımız

    // let mut_ref = &mut greetings;  // Mutable referans oluşturmaya çalışıyoruz

    /*
        Hem immutable hem de mutable referansı aynı anda kullanmaya çalışmak hataya neden olacaktır.

        error[E0502]: cannot borrow `greetings` as mutable because it is also borrowed as immutable
       --> S06_ownership\src/main.rs:163:19
        |
    160 |     let ref_1 = &greetings;  // Immutable referans
        |                 ---------- immutable borrow occurs here
    ...
    163 |     let mut_ref = &mut greetings;  // Mutable referans oluşturmaya çalışıyoruz
        |                   ^^^^^^^^^^^^^^ mutable borrow occurs here
    ...
    169 |     println!("{}, {}", ref_1, ref_2);
        |                        ----- immutable borrow later used here

     */
    println!("{}, {}", ref_1, ref_2);
    // println!("{}", mut_ref);
}
```

## Shallow Copy vs Deep Copy

**Stack** bazlı veri türleri _(örneğin, i32, f32, char)_ **Copy trait**’ine sahiptir _(Kopylama davranışı sergilerler)_ . Bu türler doğrudan stack üzerinde tutulur
ve kopyalandıklarında yeni bir bellek tahsisine gerek kalmaz. Yani, bir fonksiyona geçirilirken veya bir değişkenden başka bir değişkene atanırken **Shallow Copy** operasyonu gerçekleşir. Bellek açısından hızlıdır ve sahipliğin el değiştirmesi söz konusu değildir.

```rust
fn main() {
    let amount = 955.50f32;
    send_money(amount);
    println!("{} coin sent", amount);
}

fn send_money(money: f32) {
    println!("{} coin is sending...", money);
}
```

**Heap** üzerinde tutulan ve **Copy** trait’ini implemente etmeyen veri türleri **Clone** trait’i kullanılarak **Deep Copy** işlemine tabi tutulabilir. Örneğin, **String, Vec<T>, Box<T>** gibi heap tahsisi gerektiren veri türleri varsayılan olarak taşınır _(moved)_ nesnelerdir. Bununla birlikte .clone() fonksiyonu kullanılarak açık bir şekilde **Deep Copy** işlemi gerçekleştirilebilir.

```rust
fn main() {
    let message = String::from("System programming with rust!");
    let cloned_message = message.clone(); // Heap üzerinde yeni bir kopya oluşturur.
    send_message(cloned_message); // cloned_message fonksiyona geçer.
    println!("Original message: {}", message); // Orijinal veri hala kullanılabilir.
}

fn send_message(msg: String) {
    println!("Sending message: {}", msg);
}
```

Durumu daha iyi irdelemek adına yukarıdaki kod parçasında **clone** metodu kullanmadan ilerlemeyi deneyebiliriz.

Özetle,

- **Copy**
    - Copy özelliği, i32, f64, bool, char gibi küçük ve sabit boyutlu türlerde kullanılır.
    - Bir türün Copy davranışına sahip olması, onun bellekte kopyalanmasının çok hızlı ve maliyetsiz olduğunu da gösterir. (Shallow Copy) işlemi de diyebiliriz.
    - Copy işlemi, sadece basit ve küçük veri türleri için geçerlidir. String veya Vec gibi büyük veri yapıları tutmaya aday yapılar Copy davranışı sergilemezler, çünkü bunların derin kopyalanması (Deep Copy) gerekir.
- **Clone**
    - Clone, bir veri yapısının derin kopyasını (Deep Copy) oluşturur. Verinin içeriği kopyalanarak yeni bir alan yaratılır.
    - Clone operasyonu daha maliyetlidir çünkü veri yapısının tamamının kopyalanmasını gerektirir.
    - String ve Vec gibi heap üzerinde veri tutan yapılar Clone trait implementasyonu kullanır (String türünün koduna bakılabilir).

## Sonuç

Bu bölümde, Rust'ta sahiplik sistemini anlamak için aşağıdaki başlıklar ele alınmıştır:

- **Scope ve ownership ilişkisi**
- **Move ve Copy mekanizması**
- **Fonksiyonlara sahiplik transferi ve geri alma**
- **Borrowing _(ödünç alma)_ ve mutable referanslar**
- **Mutable ve immutable referansların aynı anda kullanımı**
- **Clone kullanımı ve maliyeti**

## Tavsiye Yazılar

- [Rust Pratikleri - Value Moved Here Hatası](https://www.buraksenyurt.com/post/rust-pratikleri-value-moved-here)
