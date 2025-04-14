# Hata Yönetimi (Error Handling)

Programlar çalışma zamanlarında çeşitli sebeplerden hatalar üretebilirler. Bu hataların yönetimi dilden dile farklılık
gösterebilir ancak özünde kontrol edilebilen, öngörülebilen hataların yönetimini ele almak daha az sorunla karşılaşmak
için dikkate alınması gereken bir konudur. Rust dilinde hataları iki kategoride ele alabiliriz.

1. **Recoverable Errors _(Telafi Edilebilir Hatalar)_**: Programın uygun bir şekilde ele alabileceği hatalardır.
   Örneğin bir dosyanın var olup olmaması, bir peripherale sinyal gönderilip gönderilememesi, bir veri tabanı ile
   bağlantı sağlanıp sağlanaması gibi durumlar bu kategoride değerlendirilir zira söz konusu hataların olması
   muhtemeldir ve önceden tedbir alınabilir **Result<T, E>** türü ile temsil edilirler.
2. **Unrecoverable Errors (Telafi Edilemez Hatalar)**: Programın devam etmesini imkansız kılan hatalardır. Sözgelimi bir
   dizinin sınırlarının dışına çıkılması, unsafe modda çalışırken gerçekleşebilecek taşma hataları ve durumlarla oluşan
   **panic!** çağrıları bu kategoriye girer.

## Unrecoverable Errors

Bazı hallerde kasıtlı olarak programın sonlanmasını _(crash olmasını)_ isteyebiliriz. Bu gibi durumlarda **panic!**
makrosu kullanılabilir. Bu makro programın çalışmasını anında durdurur ve bir hata mesajı ile bir geri izleme _(
backtrace)_ sağlar. Aşağıdaki kod parçasını göz önüne alalım. Burada numbers isimli dizinin sınırlarını aşmaya
çalıştığımız için çalışma zamanında bir panik durumu oluşur . Panik durumlarında istersek Backtrace içeriğine de
bakabilir ve hata ile ilgili daha detayalı ipuçları yakalayabiliriz. Bunun için terminalden **$env:RUST_BACKTRACE = 1**
şeklinde komut çalıştırmak yeterlidir.

```rust
fn main() {
    let numbers = vec![1, 4, 7, 0, 9];
    // let some_number= numbers[99];  // Bu satır aktif edilirse panic oluşur.
}
```

Rust'ta bir panik durumu oluştuğunda varsayılan olarak *unwinding* _(geri sarma)_ yöntemi kullanılır. Bu akış programın
anında sonlandırılmasını geciktirebilir. Diğer yandan derleme zamanında bu strateji **abort** bildirimi ile
değiştirilerek hata durumunda programın daha hızlı sonlandırılması sağlanabilir.

```toml
[profile.release]
panic = 'abort'
```

## Recoverable Errors

Rust telafi edilebilir hataların yönetimi için **Result<T, E>** türünü sağlar. **Result** türü aşağıdaki iki durumu
temsil eder.

- **Ok(T)**: İşlemin başarılı olduğunu gösterir. İşlem sonucu dönen veri T üzerinden okunabilir.
- **Err(E)**: Bir hata meydana geldiğini gösterir. E ile hata detayını içeren nesneye ulaşılabilir.

Aşağıdaki kod parçasını göz önüne alalım. Burada bir dosya açılmaya çalışılmaktadır. Olası tüm durumlar open metodundan
dönen Result türüne göre pattern match ile alınır.

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("there_is_no_spoon.dat");
    let file = match f {
        Ok(file) => file,
        Err(e) => panic!("Hata oluştu: {:?}", e),
    };
}
```

Yukarıdaki yaklaşıma alternatif olarak, **unwrap_or_else** kullanılarak hata yönetimi daha esnek hale getirilebilir.
Aşağıdaki kod parçasında bu durum ele alınmaktadır.

```rust
fn main() {
    let file = f.unwrap_or_else(|e| match e.kind() {
        ErrorKind::NotFound => match File::create("there_is_no_spoon.dat") {
            Ok(fc) => fc,
            Err(e) => panic!("Dosya oluşturulamadı: {:?}", e),
        },
        _ => panic!("Dosya açılamadı: {:?}", e),
    });
}
```

## unwrap ve expect Metotları

Sadece geliştirme aşamasında veya bir şeyleri denerken kullanılması önerilen **unwrap** metodu eğer işlem sonucunda
problem varsa bir panik oluşturur. **Unwrap** yerine **expect** metodu da kullanılabilir ve bu şekilde hata hakkında
çalışma zamanına ekstra bilgi sağlanabilir. Özetle;

- **unwrap():** Ok durumunda değeri döndürür, Err durumunda panic! makrosunu tetikler.
- **expect(msg):** unwrap gibi çalışır ancak özel bir hata mesajı döndürülmesini sağlar.

```rust
fn main() {
    let file = File::open("there_is_no_spoon.dat").expect("Are you sure? There is no file I think.");
}
```

## Error Propagation (Hata Yayılımı)

Bir fonksiyon içerisinde hata meydana geldiğinde, söz konusu hatayı çağıran koda iletmek için **?** operatörü
kullanılabilir. Bir başka deyişle hata durumu, oluştuğu fonksiyondan çağırım yönüne göre yukarıya doğru iletilebilir.
Aşağıdaki kod parçasında kredi başvuru operasyonunu ele alan fonksiyonda bir hata oluşursa bu hatanın üst fonksiyonlara
nasıl taşınacağı ele alınmaktadır _(Örnekte rastgele sayı üretimleri için **rand crate'i** kullanılmıştır)_

```rust
use rand::Rng;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
enum CreditScoreError {
    NotFound(String),
    ServiceUnavailable,
    LowScore(i32),
}

impl Display for CreditScoreError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            CreditScoreError::NotFound(ref account_id) => write!(f, "Credit score not found. Account Id {}", account_id),
            CreditScoreError::ServiceUnavailable => write!(f, "Credit service is currently unavailable"),
            CreditScoreError::LowScore(score) => write!(f, "Low credit score. Score is {}", score),
        }
    }
}

fn fetch_credit_score(account_owner: &str) -> Result<i32, CreditScoreError> {
    let mut generator = rand::thread_rng();
    let simulation_number = generator.gen_range(0..=3);
    let score = generator.gen_range(300..=700);
    match simulation_number {
        0 => Ok(score),
        1 => Err(CreditScoreError::NotFound(String::from(account_owner))),
        _ => Err(CreditScoreError::ServiceUnavailable),
    }
}

fn approve_application(account_owner: &str) -> Result<(), CreditScoreError> {
    let score = fetch_credit_score(account_owner)?;
    if score < ELIGIBLE_CREDIT_SCORE {
        Err(CreditScoreError::LowScore(score))
    } else {
        Ok(())
    }
}

const ELIGIBLE_CREDIT_SCORE: i32 = 600;

fn main() {
    match approve_application("10001") {
        Ok(_) => println!("Application approved."),
        Err(e) => println!("Error occurred: {}", e),
    }
}
```

Dikkat edileceği üzere **approve_application** fonksiyonu içerisinde yapıan **fetch_credit_score** çağrısında **?**
operatörü kullanılmıştır. Buna göre **fetch_credit_score** eğer bir hata dönerse bu otomatik olarak *
*approve_application** fonksiyonunun da dönüşü olur ve main içerisindeki pattern matching ile ele alınabilir. Bu kod,
kredi skoru düşük olan veya erişilemeyen hesap sahiplerine hata mesajı döndürerek başvurunun reddedilmesine neden olur.
Bir nevi business anlamı olan bir hata yönetim senaryosu olarak düşünülebilir.

## Sonuç

Rust'ta hata yönetimi, **panic!** ve **Result<T, E>** türleri ile güçlü bir şekilde ele alınabilir. **panic!**, geri
döndürülemez hatalar için kullanılırken **Result<T, E>** ile hatalar programcı kontrolünde işlenebilir ve yayılabilir _(
propogate)_

