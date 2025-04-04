# Ders 05: Enum Veri Türü, Pattern Matching

Enum _(enumeration)_ türü, farklı veri türlerini ve ilişkili durumları tek bir çatı altında toplamak için kullanılır.
Sadece bir durumun mevcut olduğu durumları değil, aynı zamanda her bir duruma ait farklı verileri de modelleyebilir.
Aşağıdaki örnekte bir **Status** enum türü tanımlanmıştır. Bu **enum**, bir işlemin durumunu ifade etmek için
kullanılır.
Başarılı işlem, devam eden işlem ve detay bilgisi ile birlikte hata durumu.

```rust
enum Status {
    Success,
    InProgress,
    Error(String),
}
```

Bir Enum türüne aynen struct türünde olduğu gibi metotlar eklenebilir. Örneğin bir Status değeri için daha
anlamlı bir mesaj yazdırmak istersek **get_info** gibi bir metodu ilgili **enum** türüne bağlayabiliriz _(İlerleyen
kısımlarda Display trait implementasyonu bu işlevi üstlenecektir)_

```rust
impl Status {
    fn get_info(&self) -> &str {
        match self {
            Status::Success => "Success",
            Status::InProgress => "InProgress",
            Status::Error(detail) => detail,
        }
    }
}
```

## Metotlardan enum Döndürmek

Aşağıdaki kod parçasında yer alan **create_report** metodu bir **Status** bilgisi dönülmektedir. **title** bilgisinin
boş olması halinde **Status::Error** değeri dönülür. **Error** alanının hata ile ilgili detay bilgi içerdiği gözden
kaçmamalıdır. Diğer haller tamamen rastlantısal olarak deneysel amaçlı elde edilir.

```rust
use rand::Rng;

fn create_report(title: String) -> Status {
    if title.is_empty() {
        return Status::Error("Title cannot be empty".to_string());
    }

    let mut randomizer = rand::thread_rng();
    let value = randomizer.gen_range(0..=3);

    if value % 3 == 0 {
        return Status::InProgress;
    }

    Status::Success
}

fn main() {
    let result = create_report(String::new());
    println!("{}", result.get_info());

    let result = create_report(String::from("Last Sales"));
    println!("{}", result.get_info());
}
```

Farklı bir örnek ile devam edelim. Bu sefer bir oyuncunun seviyesini **Level** isimli aşağıdaki enum türü ile ifade
ediyoruz.

```rust
#[derive(Debug)]
enum Level {
    Rookie,
    Pro,
    Elit(f32),
}
```

**Enum** veri türünü farklı veri türlerinde bir alan _(field)_ olarak da kullanabiliriz. Örneğin oyuncuların seviyleri *
*Level enum** türü ile aşağıdaki gibi ifade edilebilir.

```rust
#[derive(Debug)]
struct Player {
    title: String,
    level: Level,
    is_active: bool,
}

impl Player {
    fn new(title: String, level: Level) -> Self {
        Player {
            title,
            level,
            is_active: false,
        }
    }
    fn change_level(&mut self, level: Level) {
        self.level = level;
    }
    fn activate(&mut self) {
        self.is_active = true;
    }
    fn deactivate(&mut self) {
        self.is_active = false;
    }
}
```

Bu ikilinin bir örnek kullanımı aşağıdaki gibidir.

```rust
fn main() {
    let mut mario = Player::new(String::from("mario"), Level::Rookie);
    println!("{:?}", mario);
    mario.activate();
    mario.change_level(Level::Pro);
    println!("{:?}", mario);
    mario.change_level(Level::Elit(300.59));

    match mario.level { // Enum veri türü sıklıkla Pattern Matching ifadeleri ile birlikte kullanılır
        // Elit değeri f32 türünden birde parametre barındırabiliyor
        Level::Elit(coin) => {
            println!("Now Mario is on elit league with {coin} coin", );
        }
        _ => {} // Diğer durumları ele almak istemiyoruz
    }
    mario.deactivate();
    println!("{:?}", mario);
}
```

## Enum Sabitlerinin Farklı Veri Türlerini Barındırması

Bu durumu ele almak için aşağıdaki örneği kullanabiliriz. Örnekte sistemdeki bir kullanıcının aktif ve pasif olma
durumları **enum** türü kullanılarak ele alınmaktadır.

```rust
use chrono::{DateTime, Utc};

#[derive(Debug)]
enum User {
    Inactive {
        name: String,
    },
    Active {
        name: String,
        active: bool,
        activation_date: DateTime<Utc>,
    },
}
```

- **Inactive:** Kullanıcının sadece adını içeren bir durumdur.
- **Active:** Kullanıcının adı, aktif olup olmadığı ve aktivasyon tarihi gibi bilgileri de içerir.

Bir kullanıcının aktif hale getirilmesi için **pattern matching** kullanılan basit bir fonksion da yazabiliriz.

```rust
impl User {
    fn activate(&self, activation_date: DateTime<Utc>) -> Option<User> {
        match self {
            User::Inactive { name } => {
                let created = User::Active {
                    name: name.clone(),
                    active: true,
                    activation_date,
                };
                Some(created)
            }
            User::Active { .. } => None,
        }
    }
}

fn main() {
    let mario = User::Inactive {
        name: "Can Cey Mario".to_string(),
    };
    println!("{:#?}", mario);

    if let Some(m) = mario.activate(Utc::now()) {
        println!("Mario etkinleştirildi");
        println!("{:#?}", m);
    }
}
```

Bu metot **Option<User>** türünden bir değer döndürmektedir. Eğer kullanıcı zaten aktifse **None**, değilse yeni bir *
*User::Active** örneği döndürülür.

## Enum Türünden Option ve Result Kullanımı

Daha önceden de bahsettiğimiz üzere Rust dili, **Option<T>** ve **Result<T, E>** isimli iki güçlü **generic enum**
türünü built-in olarak sunmaktadır. Aşağıda bu iki türün kullanımına ait basit örnekler yer almaktadır.

```rust
fn main() {
    let value: Option<i32> = Some(42);
    let empty: Option<i32> = None;

    if let Some(v) = value {
        println!("Value: {}", v);
    }
}
```

**Result<T, E>** enum türü daha çok recoverable error _(Kurtarılabilir Hata)_ yönetimi senaryolarında kullanılır. Bir
operasyon sonucunun başarılı olma veya hatalı ise sebebini bildirme gibi durumlar için ideal bir enstrümandır. Aşağıdaki
örnek kod paröasında 0'a bölme hatası Result türü ile kontrollü bir şekilde ele alınır.

```rust
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        return Err("Division by zero".to_string());
    }
    Ok(a / b)
}
```

---

## Pattern Matching (Desen Eşleme)

Pattern Matching karar yapılarını örgülemek için yaygın olarak kullanılan bir enstrümandır. Rust tarafında **match** ve
**if let** gibi ifadeler yardımıyla özellikle **Option** ve **Result** gibi enum yapılarını ele almak oldukça kolay hale
gelir. Konuyu örnekleri ile inceleyelim.

### match Kullanımı

Rust'ta **match** yapısı bir değişkenin farklı durumlarının ele alınmasını sağlar. Burada kritik olan nokta olası tüm
durumların ele alınmak zorunda olmasıdır. Aşağıdaki örnekte belirtilen **HttpStatus** enum değeri bir servisin
döndürebileceği **HTTP** yanıtlarını temsil eder. **check_status** isimli fonksiyon ile olası tüm durumlar ele alınır ve
buna göre ekrana bir mesaj basılır.

```rust
use rand::Rng;

fn main() {
    check_status();
}

enum HttpStatus {
    Ok,
    Accepted,
    NotFound,
    BadRequest,
    InternalServerError,
}

fn ping(service_address: &str) -> HttpStatus {
    let mut generator = rand::thread_rng();
    let lucky_number = generator.gen_range(0..=10);
    println!("Pinging the {service_address}");
    // lucky_number değerine göre bir Status değeri üretilir
    match lucky_number {
        1 => HttpStatus::Ok,                  // 1 ise
        2..=4 => HttpStatus::Accepted,        // 2, 3 ve 4 dahil ise
        5 => HttpStatus::BadRequest,          // 5 ise
        8 | 10 => HttpStatus::NotFound,       // 8 veya 10 ise
        _ => HttpStatus::InternalServerError, // 1,2,3,4 dışındaki tüm durumlar için
    }
}

fn check_status() {
    let call_response = ping("http://localhost:3456/ping");
    match call_response {
        HttpStatus::Ok => println!("Http Status is OK(200)"),
        HttpStatus::Accepted => println!("Http Status is ACCEPTED(201)"),
        HttpStatus::NotFound => println!("Http Status is NOT FOUND(404)"),
        HttpStatus::BadRequest => println!("Http Status is BAD REQUEST(400)"),
        HttpStatus::InternalServerError => println!("Http Status INTERNAL ERROR(500)"),
    }
}
```

Örnekte `match` kullanılarak **HTTP** durum kodları kontrol edilmekte ve farklı senaryolar için ekrana farklı mesajlar
yazdırılmaktadır.

---

Bir başka örnekle devam edelim. **Match** ifadelerinde değer aralıkları da kullanılabilir. Aşağıdaki fonksiyon, bir
öğrencinin notuna göre başarı durumunu yazdırır.

```rust
fn check_exam(student: Student) {
    match student.grade {
        0..=49 => println!("[{}]{} failed.", student.id, student.full_name),
        50..=79 => println!("[{}]{} passed.", student.id, student.full_name),
        80..=100 => println!("[{}]{} passed with congrats.", student.id, student.full_name),
        _ => println!("Invalid grade score"),
    }
}

fn main() {
    check_exam(Student {
        id: 1,
        full_name: String::from("Burak De La Fuante Dos Selimos"),
        grade: 44,
    });
}
```

Bilindiği üzere enum sabitleri **primitive** tipler veya kullanıcı tanımlı komposit tiplerle kullanabilir. Bu durumda
kullanılan değerlere **match** ifadeleri üzerinden erişilmesi de mümkündür. Aşağıdaki örnek kod parçasında müşterilere
ait finansal hareketleri temsil eden **CustomerTransction** enum sabiti üzerinden pattern matching kullanılmış ve f64
türünden olan parametreler işleme katılmıştır.

```rust
enum CustomerTransaction {
    Deposit(f64),
    Withdraw(f64),
}

fn process_transaction(balance: &mut f64, transaction: CustomerTransaction) {
    match transaction {
        CustomerTransaction::Deposit(amount) => {
            *balance += amount;
            println!("Deposited ${}\nNew balance: ${}.", amount, balance);
        }
        CustomerTransaction::Withdraw(amount) => {
            if *balance >= amount {
                *balance -= amount;
                println!("Withdraw ${}\nNew balance: ${}.", amount, balance);
            } else {
                println!("Insufficient funds.");
            }
        }
    }
}

fn main() {
    let mut balance = 1000.0;
    process_transaction(&mut balance, CustomerTransaction::Deposit(400.0));
    process_transaction(&mut balance, CustomerTransaction::Withdraw(50.0));
}
```

### Option ve if let Kullanımı

**Option<T>** enum türünü kullanarak bir değişkenin **Some(value)** veya **None** olup olmadığını kontrol edebiliriz. Bu
yaklaşım hata kontrolünden farklıdır. Yani işlemler sonucunda Result türünde olduğu gibi bir hata durumu kontrolünden
ziyade işlem sonucunda ortada bir değer olup olmadığı yorumlanır.

```rust
struct User {
    id: u32,
    title: String,
    email: Option<String>,
}

impl User {
    fn info(&self) -> String {
        match &self.email {
            Some(em) => format!("{}-{} ({})", self.id, self.title, em),
            None => format!("{} ({})", self.id, self.title),
        }
    }
}

fn main() {
    let klyde = User::new(19, String::from("Jhony Klyde"), None);
    println!("{}", klyde.info()); // İlerde info yerinde Display Trait implementasyonu kullanılır
    let zee = User::new(
        23,
        String::from("Zee"),
        Some(String::from("zee@somewhere.abc")),
    );
    println!("{}", zee.info());
}
```

Yukarıdaki örnek kod parçasında User veri yapısındaki email bilgisi **Option<String>** tipi şeklinde tutulmaktadır.
User veri yapısına eklenmiş info metodu email bilgisini match ifadesi ile kontrol ederek bir sonuç döndürür. Email
bilgisi varsa bu çıktıya dahil edilir. Alternatif bir yöntem olarak **if let** operatörünü kullanarak bazi hallerde daha
sade bir sözdizimine ulaşabiliriz. Örneğin bir hesap bilgisinin sistemde olup olmadığının kontrolünde her iki yaklaşımı
da ele alarak karşılaştıralım.

```rust
struct Account {
    id: u32,
    holder_name: String,
    balance: f64,
}
fn find_account(accounts: &Vec<Account>, id: u32) -> Option<&Account> {
    accounts.iter().find(|acc| acc.id == id)
}

fn load_accounts() -> Vec<Account> {
    vec![
        Account {
            id: 1001,
            holder_name: "Nora Min".to_string(),
            balance: 1000.0,
        },
        Account {
            id: 1002,
            holder_name: "Agnis Yang".to_string(),
            balance: 750.0,
        },
        Account {
            id: 1003,
            holder_name: "Valeri Mora".to_string(),
            balance: 850.0,
        },
        Account {
            id: 1004,
            holder_name: "Monti Konti".to_string(),
            balance: 275.0,
        },
    ]
}

fn main() {
    let accounts = load_accounts();
    let result = find_account(&accounts, 1003);
    match result {
        Some(account) => println!(
            "Account for '{}' found: {} with balance ${}",
            account.id, account.holder_name, account.balance
        ),
        None => println!("Account not found."),
    }
    // Bazen yukarıdaki gibi bir match ifadesi yerine 'if let' söz dizimi de kullanılabilir
    if let Some(account) = find_account(&accounts, 1002) {
        println!(
            "Account for '{}' found: {} with balance ${}",
            account.id, account.holder_name, account.balance
        );
    }
}
```

## Sonuç

Enum veri türü oldukça zengin bir veri modeli sağlar ve ağırlıklı olarak aşağıdaki durumlarda tercih edilir.

- Birden fazla olası değeri temsil eden türler oluşturmak.
- Duruma _(State)_ bağlı ek veri saklamak.
- Pattern Matching ile kodu daha okunaklı hale getirmek.
- Hata yönetimi _(Result<T, E>)_ ve opsiyonel değerler _(Option<T>)_ ile ilgili durumlar.

## Tavsiye Kaynaklar

- [Enum Veri Türünün Rust Tarafında Etkili Bir Kullanımı](https://www.buraksenyurt.com/post/enum-veri-turunun-rust-tarafinda-etkili-bir-kullanimi)
