# Birim Test (Unit Test)

Programları en küçük birimi olan metotların oluşan koşullara göre beklendiği şekilde çalışacağını garanti etmek için
başvurulan yollardan birisi birim test yazmaktır. Birim testler kodun beklendiği şekilde çalışacağını ölçümlemek dışında
kod kalite metriklerinin önemli bir parçası olan **Code Coverage** değerlerini doğrudan etkiler. Satır bazında farklı
durumlara göre testleri yazılmış bir birim metodun Code Coverage değeri yükselir ve kodun kalitesi artar. Test yazmak
aynı zamanda kodu iyileştirmek ve **Clean Code** prensiplerine uymak açısından da önemlidir. Müfredatın dışında olsa da
bu anlamda Test Driven Development metodolojisi incelenebilir. Rust'ta test yazmak oldukça kolaydır. assert makroları
herhangi bir yerde test koşumu yapılmasına olanak sağlar.

Rust, güvenli sistem programlama dili olarak geliştirilmiştir ve test yazımı dilin **first-citizen** birimi olarak kabul
gören konularındandır. Test metotları varsayılan olarak her **cargo test** çalıştırıldığında otomatik olarak derlenip
işletilir.

Bir test fonksiyonu, aşağıdaki özellikleri taşır:

- Test metotları **#[test]** özniteliği ile işaretlenir.
- Test metotları **#[cfg(test)]** modülü içinde toplanır.
- **assert_eq!**, **assert!**, **assert_ne!** gibi makrolarla olası sonuçlar doğrulanır.

Aşağıda çok basit bir test metodu görülmektedir. Özellikle **cargo new --lib** şeklide bir rust kütüphanesi
oluşturulduğunda içerisinde varsayılan olarak test modül gelir.

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_plus_two_is_four() {
        assert_eq!(2 + 2, 4);
    }
}
```

## Test Komutları

Proje testlerini çalıştırmak için cargo aracından yararlanılır.

```bash
cargo test                      # Tüm testleri çalıştırır
cargo test function_name        # Belirli bir test çalıştırılır
cargo test module_name::name    # Modül bazlı test çalıştırılır
cargo test -- --test-threads=4  # 4 iş parçacığı ile testleri paralel koşturur
cargo test --help               # Yardım ekranı
```

## Örnek 1 : accounting.rs

accounting.rs isimli modülde birim fiyat ve indirim oranına göre hesaplama yapan bir fonksiyon yer alır. Fonksiyon
geçersiz değerler için **Result** ile hata döndürür.

```rust
fn get_discount_amount(unit_price: f64, rate: f64) -> Result<f64, AccountingError> {
    if unit_price < 0.0 {
        return Err(AccountingError::InvalidUnitPrice(
            "Unit price cannot be negative.",
        ));
    }
    if rate < 0.0 || rate > 1.0 {
        return Err(AccountingError::InvalidRate(
            "Discount rate must be between 0.0 and 1.0.",
        ));
    }
    Ok(unit_price * (1.0 - rate))
}

#[derive(Debug, PartialEq)]
enum AccountingError {
    InvalidUnitPrice(&'static str),
    InvalidRate(&'static str),
}
```

### Test Senaryoları

Bu fonksiyon için aşağıdaki gibi test senaryoları düşünebiliriz.

- İndirim oranı 0 olduğunda fiyat değişmemeli.
- İndirim oranı 1 olduğunda fiyat sıfır olmalı.
- Negatif fiyat ya da geçersiz indirim oranı hataya sebep olmalı.

Yukarıdaki durumlar ve diğerleri için aşağıdaki gibi bir test modülü geliştirilebilir.

```rust
#[cfg(test)]
mod accounting_tests {
    use super::*;
    use crate::accounting::AccountingError::{InvalidRate, InvalidUnitPrice};

    #[test]
    fn test_no_discount() {
        let result = get_discount_amount(100.0, 0.0);
        assert_eq!(result.unwrap(), 100.0);
    }
    #[test]
    fn test_two_percent_discount() {
        let result = get_discount_amount(100.0, 0.2);
        assert_eq!(result.unwrap(), 80.0);
    }
    #[test]
    fn test_full_discount() {
        let result = get_discount_amount(100.0, 1.0);
        assert_eq!(result.unwrap(), 0.0);
    }
    #[test]
    fn test_negative_price() {
        let result = get_discount_amount(-50.0, 0.2);
        assert_eq!(
            result,
            Err(InvalidUnitPrice("Unit price cannot be negative."))
        );
    }
    #[test]
    fn test_invalid_discount_greater_than_one() {
        let result = get_discount_amount(100.0, 1.5);
        assert_eq!(
            result,
            Err(InvalidRate("Discount rate must be between 0.0 and 1.0."))
        );
    }
    #[test]
    fn test_invalid_discount_negative() {
        let result = get_discount_amount(100.0, -0.3);
        assert_eq!(
            result,
            Err(InvalidRate("Discount rate must be between 0.0 and 1.0."))
        );
    }
}
```

## Örnek 2: membership.rs

Farklı bir senaryo ile devam edelim. Bu kez modül içerisinde MembershipManager isimli bir veri yapısı yer almaktadır. Bu
yapı için tasarlanmış validate_nickname metodu için farklı test senaryoları yazılabilir.

```rust
struct MembershipManager;

impl MembershipManager {
    fn validate_nickname(&self, nick_name: &str) -> Result<(), UserError> {
        if nick_name.len() < 5 {
            return Err(UserError::InvalidNickname(String::from(
                "Nickname is too short",
            )));
        }
        if nick_name.len() > 15 {
            return Err(UserError::InvalidNickname(String::from(
                "Nickname is too long",
            )));
        }
        if nick_name.contains(' ') {
            return Err(UserError::InvalidNickname(String::from(
                "Nickname contains space",
            )));
        }
        if nick_name.chars().any(|c| !c.is_alphanumeric()) {
            return Err(UserError::InvalidNickname(String::from(
                "Nickname contains alphanumeric",
            )));
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
enum UserError {
    InvalidNickname(String),
    WrongPassword,
}
```

### Test Senaryoları:

- Geçerli rumuz: 5-15 karakter arası, boşluk ve özel karakter içermemeli.
- Hatalı rumuzlar: çok kısa, çok uzun, boşluklu, özel karakterli.

### Test Örnekleri:

```rust
#[cfg(test)]
mod membership_tests {
    use super::*;

    #[test]
    fn test_valid_nick_name() {
        let membership_manager = MembershipManager;
        assert!(membership_manager.validate_nickname("rustropic").is_ok());
    }

    #[test]
    fn test_short_nick_name() {
        let membership_manager = MembershipManager;
        let result = membership_manager.validate_nickname("none");
        assert_eq!(
            result,
            Err(UserError::InvalidNickname(
                "Nickname is too short".to_string()
            ))
        );
    }

    #[test]
    fn test_long_nick_name() {
        let membership_manager = MembershipManager;
        let result = membership_manager.validate_nickname("ToooooLonnnnnnnnngggggggggggNickkkk");
        assert_eq!(
            result,
            Err(UserError::InvalidNickname(
                "Nickname is too long".to_string()
            ))
        );
    }

    #[test]
    fn test_nick_name_with_space() {
        let membership_manager = MembershipManager;
        let result = membership_manager.validate_nickname("The Red Barron");
        assert_eq!(
            result,
            Err(UserError::InvalidNickname(
                "Nickname contains space".to_string()
            ))
        );
    }

    #[test]
    fn test_nick_name_with_special_chars() {
        let membership_manager = MembershipManager;
        let result = membership_manager.validate_nickname("Rust@Nickname!");
        assert_eq!(
            result,
            Err(UserError::InvalidNickname(
                "Nickname contains alphanumeric".to_string()
            ))
        );
    }
}
```

## Ignored Test

Projeler büyüdükçe ve metot sayıları arttıkça metot başına yazılan birim testlerin sayısıda çoğalarak artmaya devam
eder. Özellike **CI/CD** hattına yerleştirilen birim test koşumlarında bazı testleri geçici olarak göz ardı etmek
isteyebiliriz. Zira belli bir feature için kontrol edilmesi gerekmeyen birim testler söz konusu olabilir. Bu gibi
durumlarda **#[ignore]** özniteliği kullanılırarak bir test fonksiyonunun devre dışı bırakılması sağlanabilir.

```rust
#[test]
#[ignore]
fn just_fails() {
    panic!("Failing...");
}
```

Elbette özellikle ignore edilen testlerin ayrıca çalıştırılması da sağlanabilir.

```bash
cargo test -- --ignored
```

---

## Projelerde Test Modül Yapıları

Projenin içeriğine bağlı olarak testlerin modül bazında ayrıştırılması okunurluk ve takip açısından daha kolay olabilir.
Örneğin bir e-ticaret platformunu göz önüne alalım. accounting(içerisinde invoice), membership, orders gibi modüller
içerebilir. Böyle bir senaryoda Test modülleri aşağıdaki gibi dizayn edilebilir.

```text
ecommerce/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── accounting/
│   │   ├── mod.rs
│   │   ├── discount.rs
│   │   ├── invoice.rs
│   │   └── tests/
│   │       ├── discount_tests.rs
│   │       └── invoice_tests.rs
│   ├── membership/
│   │   ├── mod.rs
│   │   ├── auth.rs
│   │   └── profile.rs
│   ├── orders/
│   │   ├── mod.rs
│   │   └── order.rs
│   ├── utils.rs
│   └── tests/
│       └── common.rs  <-- farklı testlerce paylaşılan yardımcı kodlar
├── tests/
│   ├── integration_order.rs  <-- `cargo test` ile koşturulan entegrasyon testleri
│   └── integration_user.rs
```

## Mock Nesne Kullanımı

Geliştirdiğimiz uygulamalar bazen harici process'lerle entegre olur. Örneğin harici bir veritabanına bağlantı kurabilir,
bir servise erişebilirler. Bu tip çağrıları içeren fonksiyonların birim testleri yazılırken neyin test edilmek
istendiğinin açıkça düşünülmesi gerekir. Eğer bir entegrasyon testi söz konusu değilse ve birim fonksiyonun olası
davranışlara göre tepkisinin test edilmesi isteniyorsa, söz konusu dış bağlantıları taklit ederek testin devamlılığını
sağlayacak enstrümanlar kullanılabilir. Bu amaçla genellikle mock adı verilen nesneler ele alınır. Bir Login
operasyonu olduğunu ve kullanıcı doğrulama fonksiyonelliğinin harici bir servisle karşılandığını düşünelim. Amacımız
login operasoyonu için birim test yazmak. Aşağıdaki örnek kod parçası buna göre incelenebilir.

```rust
pub struct User {
    username: String,
    password: String,
}
pub trait OpenAuth {
    fn validate(&self, user: &User) -> bool;
    fn is_user_locked(&self, user: &User) -> bool;
}

pub trait Logger {
    fn log(&self, message: &str);
}

pub fn login<A: OpenAuth, L: Logger>(auth: &A, logger: &L, user: &User) -> Result<String, String> {
    if user.username.len() < 5 {
        return Err("Username too short".to_string());
    }

    if auth.is_user_locked(user) {
        return Err("Account is locked".to_string());
    }

    if auth.validate(user) {
        logger.log(&format!("User {} logged in", user.username));
        Ok(format!("Welcome, {}!", user.username))
    } else {
        Err("Invalid credentials".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockAuth {
        locked: bool,
        valid: bool,
    }

    impl OpenAuth for MockAuth {
        fn validate(&self, _: &User) -> bool {
            self.valid
        }

        fn is_user_locked(&self, _: &User) -> bool {
            self.locked
        }
    }

    struct TestLogger {
        pub messages: std::cell::RefCell<Vec<String>>,
    }

    impl Logger for TestLogger {
        fn log(&self, message: &str) {
            self.messages.borrow_mut().push(message.to_string());
        }
    }

    #[test]
    fn test_successful_login_with_logging() {
        let auth = MockAuth {
            locked: false,
            valid: true,
        };

        let logger = TestLogger {
            messages: std::cell::RefCell::new(vec![]),
        };

        let result = login(
            &auth,
            &logger,
            &User {
                username: "Billi Geyts".to_string(),
                password: "1976".to_string(),
            },
        );
        assert!(result.is_ok());

        let logs = logger.messages.borrow();
        assert_eq!(logs.len(), 1);
        assert!(logs[0].contains("Billi Geyts"));
    }

    #[test]
    fn test_login_fails_for_locked_account() {
        let auth = MockAuth {
            locked: true,
            valid: true,
        };

        let logger = TestLogger {
            messages: std::cell::RefCell::new(vec![]),
        };

        let result = login(
            &auth,
            &logger,
            &User {
                username: "Jan Claud".to_string(),
                password: "1Dome".to_string(),
            },
        );
        assert_eq!(result, Err("Account is locked".to_string()));
        assert!(logger.messages.borrow().is_empty());
    }

    #[test]
    fn test_short_username_rejected_before_auth() {
        let auth = MockAuth {
            locked: false,
            valid: true,
        };

        let logger = TestLogger {
            messages: std::cell::RefCell::new(vec![]),
        };

        let result = login(
            &auth,
            &logger,
            &User {
                username: "r".to_string(),
                password: "secret".to_string(),
            },
        );
        assert_eq!(result, Err("Username too short".to_string()));
    }
}
```

Örneğe ait kodlar [mocking](./mocking/src/lib.rs) isimli proje içerisinde yer almaktadır.

## Ekstra Notlar

- **cargo test** ile hem modüler hem bütünsel testler çalıştırılabilir.
- Hata türlerinin **PartialEq** implementasyonu, testlerde karşılaştırma yapabilmek için gereklidir.
- Fonksiyonlarda **Result<T, E>** tipi kullanıldığında hata kontrolü kolaylaşır.

## Tavsiyeler

- Her fonksiyon için başarı ve hata durumlarını test etmeyi unutmayın.
- Testlerin başarısız olması da öğrenme sürecinin bir parçasıdır.
- Test adlarını verirken neyi test ettiğini açıklayan ifadeler kullanın.
- Test Driven Development pratiklerini kullanarak kod kalitesini yükseltin.
