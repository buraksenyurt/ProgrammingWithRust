# Vize Sınav Soruları

**Süre :** 75 Dakika  
**Soru Sayısı :** 10

## Soru 1

Rust, bazı programlama dillerinde olan **Garbage Collector** gibi bir mekanizmaya sahip değildir. Bellek yönetiminde çok titiz davranır ve olası açıklara sebebiyet verecek birçok hatayı derleme aşamasında keşfederek önler. Rust’ın bellek yönetimi hem derleme hem de çalışma zamanında **Ownership, Borrow Checker, Lifetimes** ve **RAII** _(Resource Acquisition is Initialization)_ gibi mekanizmalarla sağlanır. Bunu göz önüne aldığınızda aşağıdaki kod parçasındaki problem şıklardan hangisidir?

```rust
fn main() {  
    let msg = String::from("Hello Ferris…");
    do_something(msg);
    println!("{}", msg);
}

fn do_something(some_string: String) {
    println!("{} How Are You?", some_string);
}
```

**A)** Program önce ekrana **"Hello Ferris…How Are You?"** yazar. Ardından sadece **"Hello Ferris…"** yazar ve sonlanır.  
**B)** **msg** değişkeninin sahipliği **do_something** fonksiyonuna geçer ve devamında main fonksiyonunda artık kullanılamaz çünkü do_something tamamlandığında **scope** dışında kalır ve bellekten düşer. Bu sahiplik ilkesinin ihlalidir ve kod derlenmez.  
**C)** **{}** ifadesi **{:?}** şeklinde yazılmadığı için derleme zamanında hata alınır.  
**D)** **msg** değişkeni **mutable** olarak belirtilmediği için çalışma zamanında panic oluşur.

## Soru 2

Aşağıda örnek bir kod parçası yer almaktadır.

```rust
fn main() {  
    let mut x = 8;  
    let y = &mut x;  
    *y += 1;  
    println!("{}", x);  
}
```

Sizce bu kodun çıktısı şıklardan hangisidir?

**A)** 9  
**B)** 8  
**C)** y değerinin sahipliği alınıp x **println** metoduna gönderilmeye çalışıldığında value moved hatası oluşur ve kod derlenmez.  
**D)** x değişkeninin bellekteki bir sonraki adresi ekrana yazdırılır.

## Soru 3

Aşağıda derlenmeyen bir kod parçası yer almaktadır.

```rust
fn main() {  
    let x = 10;  
    let y = &mut x;  
    println!("{}", y);  
}
```

Bu kodun derlenmeme sebebi şıklardan hangisidir?

**A)** y değişkeninin yaşam süresi **(lifetime)** çok kısadır.
**B)** x değişkeni **move** edildiği için erişilemez.  
**C)** x değişkeni mutable olarak tanımlanmadığı halde **mutable** referans olarak alınmaya çalışılmaktadır.  
**D)** x değişkeninin değeri **heap** bellek bölgesi yerine **stack** alanında saklandığı için bu hata oluşur.

## Soru 4

Geliştirmekte olduğumuz uygulamada sistemdeki **CPU**, **Memory** gibi unsurların kullanım değerlerini toplayıp seviyelerine göre derecelendiren ve farklı bir sisteme log olarak bırakan bir modül yazılması istenmiştir. Bu amaçla aşağıdaki **enum** ve **struct** veri yapıları tasarlanmıştır.

```rust
#[derive(Debug, PartialEq)] 
pub enum Level {  
    NORMAL,  
    WARM,  
    BURNING,  
    ALARM,  
    UNKNOWN,  
}

#[derive(Debug, PartialEq)]  
struct SystemLog {  
    id: u32,  
    time_stamp: Option<DateTime<Utc>>,  
    level: Level,  
    description: String,  
}
```

Buna göre aşağıdaki kod parçalarından hangisi **SystemLog** yapısına uygun şekilde yeni bir değişken oluşturur?

**A)**

```rust
let log = SystemLog {  
    id: 1,  
    time_stamp: None,  
    level: Level::BURNING,  
    description: "CPU 85%".to_string(),  
};
```

**B)**  

```rust
let log = SystemLog {  
    id: 1,  
    level: Level::BURNING,  
    description: String::from("CPU 85%"),  
};
```

**C)**

```rust
let log = SystemLog {  
    id: 1,  
    time_stamp: Some(Utc::now()),  
    level: "BURNING",  
    description: String::from("CPU 85%"),  
};
```

**D)**  

```rust
let log = SystemLog::new(1, Level::BURNING, "CPU 85%");
```

## Soru 5

Geliştirmekte olduğumuz uygulamada system loglarının seviyelerini aşağıdaki **enum** yapısı ile tutmayı planlıyoruz.

```rust
#[derive(Debug, PartialEq)]  
pub enum Level {  
    NORMAL,  
    WARM,  
    BURNING,  
    ALARM,  
    UNKNOWN,  
}
```

Ayrıca bu enum yapısına **to_string** isimli bir metot ekleyip seviyeye göre şu ifadelerin döndürülmesini istiyoruz.

```text
Level::BURNING → "Aşırı kullanım!"  
Level::NORMAL → "Her şey yolunda."  
Diğer tüm durumlar → "Dikkat edilmesi gerekiyor."
```

Bu durumları ele almak için **pattern match** ifadesi kullanmak biçilmiş kaftan. Bu nedenle **to_string** metodu içerisinde **pattern match** kullanmaya karar verdiniz. Aşağıdaki şıklardan hangisi doğru ve idiomatic bir kullanım şeklidir.

**A)**

```rust
if level == Level::BURNING {  
    println!("Aşırı kullanım!");  
} else if level == Level::NORMAL {  
    println!("Her şey yolunda.");  
} else {  
    println!("Dikkat edilmesi gerekiyor.");  
}
```

**B)**

```rust
match level {  
    Level::NORMAL => println!("Her şey yolunda."),  
    Level::WARM => println!("Dikkat edilmesi gerekiyor."),  
    Level::BURNING => println!("Aşırı kullanım!"),  
    Level::ALARM => println!("Dikkat edilmesi gerekiyor."),  
    Level::UNKNOWN => println!("Dikkat edilmesi gerekiyor."),  
}
```

**C)**  

```rust
match level {  
    if Level::NORMAL => println!("Her şey yolunda."),  
    If Level::BURNING => println!("Aşırı kullanım!"),  
    _ => println!("Dikkat edilmesi gerekiyor."),  
}
```

**D)**

```rust
match level {  
    Level::NORMAL => println!("Her şey yolunda."),  
    Level::BURNING => println!("Aşırı kullanım!"),  
    _ => println!("Dikkat edilmesi gerekiyor."),  
}
```

## Soru 6

Elimizde yine **SystemLog** yapısı ve **Level struct**’ı var. _(4ncü sorudaki tanımlamalar bu soru içinde geçerlidir)_ Sistemde biriken loglar içeride bir vektör üzerinde toplanmaktalar. Bir metot ile parametre olarak gelen vector içerisindeki loglardan sadece **ALARM** seviyesinde olanları yeni bir Vector ile geri döndürmek istediğimizi düşünelim. Aşağıdaki kod parçalarından hangisi bu ihtiyacı doğru şekilde karşılar.

**A)**  

```rust
fn filter_alarm(logs: &Vec<SystemLog>) -> Vec<&SystemLog> {  
    let mut result = Vec::new();

    for log in logs {  
        if log.level == Level::ALARM {  
            result.push(log);  
        }  
    }

    result  
}
```

**B)**

```rust
fn filter_alarm(logs: Vec<SystemLog>) -> Vec<&SystemLog> {  
    let mut result = Vec::new();

    for log in logs {  
        if log.level == Level::ALARM {  
            result.push(&log);  
        }  
    }

    result  
}
```

**C)**  

```rust
fn filter_alarm(logs: &Vec<SystemLog>) -> Vec<SystemLog> {  
    let mut result = Vec::new();

    for log in logs {  
        if log.level == Level::ALARM {  
            result.push(*log);  
        }  
    }

    result  
}
```

**D)**

```rust
fn filter_alarm(logs: Vec<SystemLog>) -> Vec<SystemLog> {  
    let mut result = Vec::new();

    for log in logs {  
        if log.level == Level::ALARM {  
            result.push(log);  
        }  
    }

    result  
}
```

## Soru 7

Çalıştırılan yük testleri sonrasında 6ncı soruda belirtilen ihtiyacı karşılayan filtreleme fonksiyonunun bellekte çok fazla vektör alanı oluşmasına neden olduğu tespit edilmiştir. Bu nedenle **filter_alarm** fonksiyonundan aynı **SystemLog** nesnelerinin farklı kopyalarını içeren bir vektör yerine, orjinal vektörün **ALARM** türevli log değişkenlerinin **referansını** içeren bir vektör dönülmesine karar verilmiştir. Bunu karşılayacak en doğru fonksiyon aşağıdakilerden hangisidir?

**A)**

```rust
fn filter_alarm(logs: Vec<SystemLog>) -> Vec<&SystemLog> {  
    let mut result = Vec::new();

    for log in logs {  
        if log.level == Level::ALARM {  
            result.push(&log);  
        }  
    }

    result  
}
```

**B)**

```rust
fn filter_alarm(logs: &Vec<SystemLog>) -> Vec<&SystemLog> {  
    let mut result = Vec::new();

    for log in logs {  
        if log.level == Level::ALARM {  
            result.push(log);  
        }  
    }

    result  
}
```

**C)**  

```rust
fn filter_alarm(logs: &Vec<SystemLog>) -> Vec<&SystemLog> {  
    let mut result = Vec::new();

    for &log in logs {  
        if log.level == Level::ALARM {  
            result.push(log);  
        }  
    }

    result  
}
```

**D)**  

```rust
fn filter_alarm(logs: &Vec<SystemLog>) -> Vec<&SystemLog> {  
    let result = Vec::new();

    for log in logs {  
        if log.level == Level::ALARM {  
            result.push(log);  
        }  
    }

    result  
}
```

**Dikkat!** _Takip eden iki soru aşağıda tamamı verilen program kodları ile ilgilidir. Cevaplamadan önce program kodunu okuyup anlamaya çalışınız._

```rust
use std::{env, process};

fn main() {  
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {  
        show_help();  
        process::exit(1);  
    }

    match args[1].as_str() {  
        "-e" | "-env" => display_env(),  
        "-w" | "-cwd" => display_cwd(),  
        "-u" | "-usr" => display_user(),  
        "-h" | "-help" => show_help(),  
        _ => {  
            eprintln!("Invalid option. Use '-help' for more information");  
            process::exit(1);  
        }  
    }  
}

fn display_env() {  
    println!("System Environment Variables:");  
    for (k, v) in env::vars() {  
        println!("{}: {}", k, v);  
    }  
}

fn display_cwd() {  
    match env::current_dir() {  
        Ok(path) => println!("Current Working Directory: {}", path.to_string_lossy()),  
        Err(e) => {  
            eprintln!("Error retrieving current directory: {}", e);  
            process::exit(1);  
        }  
    }  
}

fn display_user() {  
    match env::var("USERNAME") {  
        Ok(user) => println!("Current User: {}", user),  
        Err(e) => eprintln!("Could not retrieve user information {}", e),  
    }  
}

fn show_help() {  
    println!("SysCo - A lightweight system tool\n");  
    println!("Usage: sysco <option>\n");  
    println!("Valid Options");  
    println!("  -h, -help : Display usage");  
    println!("  -e, -env  : Show env values");  
    println!("  -w, -cwd  : Show the current working directory");  
    println!("  -u, -usr  : Current root user\n");  
    println!("For details azon.com/sysco/help/readme.md");  
}
```

## Soru 8

Yukarıdaki örnek program derlenip çalıştırıldığında komut satırından sadece **sysco** yazılıp hiçbir parametre verilmezse _(yani sadece cargo run gibi çalışıtırılısa)_, programın çıktısı ne olur?

**A)** Program hata verir çünkü **args[1]** değişkeni gönderilmemiştir.  
**B)** **show_help()** fonksiyonu çağrılır, kullanım şekli ekrana bastırılır ve ardından **process::exit(1)** ile programdan çıkılır.  
**C)** Program **"Invalid option"** hatası gösterir.  
**D)** Program çalışır ama hiçbir çıktı üretmeden sonlanır.

## Soru 9

Programın çalıştığı sistemdeki kullanıcı bilgisini göstermek için **sysco -u** şeklinde çalıştırılması yeterlidir. Eğer ortam değişkenlerinde **USERNAME** bilgisi yoksa program nasıl bir tepki verir?

**A)** **unwrap()** metodu için hiçbir şey yazdırılmaz.  
**B)** Varsayılan kullanıcı adı olan **"guest"** yazdırılır.  
**C)** Terminale bir hata mesajı bastırılır ama program çalışmaya devam eder.  
**D)** **Program process::exit(1)** çağrılarak sonlandırılır.

## Soru 10

Vizenin gerçekleştirildiği tarih itibariyle **Rust** programlama dilinin birçok temel özelliğini inceleme fırsatı bulduk. Temel veri türleri, **vector**’ler, **tuple**, **if…else** ifadeleri, döngüler, **pattern matching**, **struct**, **enum**, iterasyon fonksiyonları, **ownership** ve **borrow checker** mekanizması vs Bu sınavı siz yapıyor olsaydınız nasıl bir soru sorardınız, şıkları neler olurdu aşağıya doğru olan şık ve doğru olma sebebi ile birlikte yazınız.

---

## Cevap Anahtarı

Tüm sorular 10 puandır.

1) B  
2) A  
3) C  
4) A  
5) D  
6) D  
7) B  
8) B
9) C  
10) Açık soru (10 puan)
