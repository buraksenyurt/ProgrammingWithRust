# Ders 09: Closures

**Closure** terimi çoğu zaman anonim fonksiyon olarak da ifade edilir. Özellikle fonksiyonel dillerde yaygın olarak
kullanılan closure'lar, bir değişkene atanabilir ve bu sayede fonksiyonlara parametre olarak kod bloklarının taşınması
sağlanabilir. Benzer şekilde fonksiyonlardan dönüş türü olarak da kullanılabilir. Aşağıdaki basit kod parçasında en
temel haliyle bir closure kullanımı yer almaktadır.

```rust
fn main() {
    let square = |x| x * x;
    println!("{}", square(5));
}
```

Örnekte tanımlanan square isimli değişken x değerlerinin çarpımını hesap eden bir kod bloğunu ifade eder. square
değişkeni normal bir fonksiyon gibi çağırılabilir. Rust programlama dili built-in gelen üç farklı trait ile closure
desteği sağlar.

- **Fn:** Closure, dışarıdan yakaladığı değişkenleri salt okunur _(read only)_ şekilde kullanır.
- **FnMut:** Closure, dış değişkenleri değiştirerek _(mutable)_ kullanabilir.
- **FnOnce:** Closure, dış değişkenleri sahiplenir (move eder) ve yalnızca bir kez çağrılabilir.

Özellikle nesne toplulukları üzerinden hareket eden iteratif fonksiyonlar bu ön tanımlı closure'ları sıklıkla kullanır.
Bu trait'ler bir anlamda C# tarafından gelenler için delegate türüne de benzetilebilir.

## Filtreleme ve Sıralama İşlemleri

Takip eden örnekler farklı veri türlerini kullanır ve bunlar [models.rs](./src/models.rs) dosyasında yer almaktadır.
Örnek oyun bilgileri içinse [repository.rs](./src/repository.rs) dosyasına bakılabilir. Çok basit bir örnekle
başlayalım. Oyunları yıllara göre sıralamak istediğimizi düşünelim. Normalde vector türleri belli bir key değerine göre
sıralama işlemi için **sort_by_key** isimli metodu sağlar. Bu metod **FnMut(&T) -> K** davranışını uygulayan bir ifade
bekler. Bir başka deyişle sıralama için kullanılacak anahtar alanı ele alacağı bir davranışa ihtiyaç duyar. Buna göre
aşağıdaki gibi bir örnek yazılabilir.

```rust
mod repository;

use repository::*;

fn year_sorter(game: &Game) -> u16 {
    game.year
}

fn print_games(games: &Vec<Game>) {
    for game in games {
        println!("{}, {}", game.year, game.title);
    }
}

fn main() {
    let games = repository::load_games();

    let mut games = repository::load_games();

    games.sort_by_key(year_sorter);
    print_games(&games);
}
```

Diğer yandan aynı işlev year_sorter fonksiyonunu yazmaya gerek kalmadan **sort_by** metoduna **FnMut** ile taşınabilecek
bir kod bloğu gönderilerek de gerçekleştirilebilir. Hatta farklı sıralama ve filtreleme kritleri de aynı metodoloji ile
uygulanabilir.

```rust
fn main() {
    let games = repository::load_games();
    let mut games = repository::load_games();

    // Closure ile artan yıl sıralaması
    games.sort_by(|g1, g2| g1.year.cmp(&g2.year));
    println!("Yıla göre artan sıralama:");
    print_games(&games);

    // Closure ile azalan yıl sıralaması
    games.sort_by(|g1, g2| g2.year.cmp(&g1.year));
    println!("\nYıla göre azalan sıralama:");
    print_games(&games);

    // Popülaritesi 2.0'den yüksek olan oyunlar
    let popular_games: Vec<Game> = games.into_iter().filter(|g| g.popularity > 2.0).collect();
    println!("\nPopüler oyunlar (popularity > 2.0):");
    print_games(&popular_games);
}
```

**sort_by** ve **into_iter** çağrısı sonrası erişilen **filter** metoduna parametre olarak **closure** ifadeleri
gönderilmiştir. Buna göre tüm nesne koleksiyonu üzerinde closure ifadesi ile gelen kod bloğu çalıştırılır. Örneğin
popülerlik değeri 2.0 üzerinden olanları çekmek için aşağıdaki **closure** kullanılmıştır.

```text
|g| g.popularity > 2.0
```

## Metot Parametresi Olarak Closure Kullanımı

Bazı durumlarda bir nesne topluluğunun çalışma zamanında neye göre filtreleneceği bilinmez. Programcı söz konusu nesne
topluluğu üzerinde işletmek istediği kod bloklarını bir fonksiyon gibi geçebilmelidir. Böylece dinmaik ve esnek bir
işlevsellik kullanabilir. Closure ifadeleri bunu gerçekleştirmek için idealdir. **Entity Component System** temelli
basit bir oyun motoru tasarladığımızı düşünelim. Deneysel olarak da aşağıdaki veri modellerini kullandığımızı
varsayalım.

```rust
#[derive(Debug)]
struct Player {
    id: u32,
    position: (f32, f32),
    velocity: (f32, f32),
    score: u32,
}

#[derive(Debug)]
struct GameWorld {
    players: Vec<Player>,
}
```

Basitçe oyun sahasındaki oyuncuları tanımlayan ve nesne topluluğu olarak ele alan iki veri yapısı mevcut. Sahadaki tüm
oyuncular için farklı işlevleri işletecek farklı sistemler tasarlanabilir. Söz gelimi tüm oyuncların pozisyon
bilgilerini değiştirecek tek bir fonksiyon yazılabilir veya oyunculardan belli kriterlere uyanların skorlarında
değişiklik yapacak bir başka sistem fonksiyonu da geliştirilebilir. Burada anahtar nokta sistem fonksiyonlarının
işletecekleri kodun ne olacağını bilmemeleridir. Eğer bu esnekliği framework ilkeleri çerçevesinde sağlayabilirsek genel
kullanıma uygun bir oyun motoru tasarlayabiliriz. Aşağıdaki kod parçası **Fn** ve **FnMut** trait'lerinin örnek
kullanımlarını içermektedir.

```rust
fn update_players_system<F>(world: &mut GameWorld, mut f: F)
where
    F: Fn(&mut Player),
{
    for p in &mut world.players {
        f(p);
    }
}

fn update_score_system<F>(world: &GameWorld, mut f: F)
where
    F: FnMut(&Player),
{
    /*
      Burada FnMut yerine Fn kullanıp oluşan hata mesajı incelenebilir.

      error[E0594]: cannot assign to `total_team_score`, as it is a captured variable in a `Fn` closure
      change this to accept `FnMut` instead of `Fn`
   */
    for p in &world.players {
        f(p);
    }
}

pub fn main() {
    let mut world = GameWorld {
        players: vec![
            Player {
                id: 1,
                position: (0.0, 0.0),
                velocity: (2.0, 0.0),
                score: 0,
            },
            Player {
                id: 2,
                position: (100.0, 0.0),
                velocity: (8.0, 0.0),
                score: 0,
            },
        ],
    };

    let apply_gravity = |entity: &mut Player| {
        entity.position.0 += entity.velocity.0 * 0.9;
        entity.position.1 += entity.velocity.1 * 0.9;
    };

    println!("Before Update: {:?}", world.players);
    update_players_system(&mut world, apply_gravity);
    // update_players_system(&mut world, |entity| {
    //     entity.position.0 += entity.velocity.0 * 0.9;
    //     entity.position.1 += entity.velocity.1 * 0.9;
    // });
    println!("After Update: {:?}", world.players);

    // FnMut kullanımı ile ilgili bir örnek
    let mut total_team_score = 0;

    println!("Total score before update: {}", total_team_score);
    update_players_system(&mut world, |p| p.score += 2);
    update_score_system(&world, |p: &Player| {
        total_team_score += p.score;
    });
    println!("Total score after update: {}", total_team_score);
}
```

Dikkat edileceği üzere fonksiyonlar parametre olarak gelen kod bloklarının world nesnesi üzerinden ulaşılan tüm player
değişkenleri için icra eder _(Gerçek bir oyun motorunda Player herhangi bir tür olabilir ve motor kendisine bildirilen
sistemleri belli bir takvim planına uygun olarak frame by frame işletir)_ **update_player_system** fonksiyonunda
kullanılan **f** değişkeni generic bir tür olarak belirtilmiştir ve **Fn** trait'ini uygulaması beklenmektedir. Kısaca f
yerine **Fn** trait'ine uygun bir **closure** ifadesi gelebilir. Örneğin sistem fonksiyonuna **apply_gravity** değişkeni
ile tanımlı fonksiyonu atanabilir ya da closure ifadesi ile doğrudan bir blok gönderilebilir. **update_score_system**
fonksiyonunda FnMut trait'ini uygulayan bir closure ifadesi beklenir. Bu örnek main fonksiyonunda yer alan
**total_team_score** değişkeni üzerinde değişiklik yapar. **FnMut** olarak tanımlanmasının bir sebebi de bulunduğu scope
dışındaki bir değişken üzerinde değişiklik yapıyor olmasıdır.

## FnOnce Senaryosu

**FnOnce**, kullandığı değerleri sahiplenen ve bir sefer çalıştırılması istenen kodların kullanımı için idealdir. Daha
çok thread'lerin kullanıldığı durumlarda ele alınabilir. Aşağıdaki örnek kod parçasını bu anlamda ele alabiliriz.

```rust
fn main()
{
    // FnOnce Örneği
    let message = Some(String::from("You have unlocked a new level!"));
    let show_message = || {
        if let Some(msg) = message {
            println!("{}", msg);
        } else {
            println!("Message already shown.");
        }
    };

    show_message();
    // show_message(); // Burada 'value used here after move' hatası oluşur
    /*
       Henüz erken olsa da thread açmak FnOnce kullanımı için iyi bir örnek olabilir.
       thread::spawn yeni bir thread başlatırken FnOnce türünden bir closure alır. Dışarıdan
       değerler closure içerisine taşınır ve thread sonlanana kadar closure sahip olduğu tüm
       değerleri tüketir. Bu tek sefer çalıştırılması gereken bir closure olarak düşünülebilir.
    */
    let message = String::from("Inside a thread!");
    let handle = thread::spawn(move || {
        println!("{}", message);
    });
    // println!("{}", message); // value borrowed here after move
    handle.join().unwrap();
}
```

## Fonksiyonlardan Closure Döndürülmesi

Pek tabii fonksiyonlardan Fn, FnMut veya FnOnce davranışlarını uyarlayan fonksiyonlar da döndürülebilir. Örneğin
sistemde üretilen log bilgilerini türlerine göre anında sayan bir senaryomuz olduğunu düşünelim. Log içeriğine göre bunu
kullanan bir fonksiyon aşağıdaki gibi tasarlanabilir.

```rust
use crate::models::{Level, Log};
use std::io::{Write, stdout};

pub fn log() -> impl FnMut(&Log) {
    let mut error_count = 0;
    let mut warn_count = 0;
    let mut info_count = 0;
    move |l| {
        stdout()
            .write(format!("{}\n", l.to_string()).as_bytes())
            .unwrap();
        match l.level {
            Level::Error => error_count += 1,
            Level::Warn => warn_count += 1,
            Level::Info => info_count += 1,
            _ => {}
        }
        stdout()
            .write(
                format!(
                    "Log Tracker: {} errors, {} warnings, {} infos\n",
                    error_count, warn_count, info_count
                )
                    .as_bytes(),
            )
            .unwrap();
    }
}
```

log metodu geriye Log türünden referanslar alabilen **FnMut** türünden bir trait döndürmektedir. Fonksiyon ekrana log
mesajını basarken bir yandan da seviyelere göre toplamları hesaplar. Bu fonksiyon aşağıdaki gibi kullanılabilir.

```rust
fn main()
{
    let mut logger = log();

    logger(&Log::new(Level::Info, "Authentication Success".to_string()));
    logger(&Log::new(Level::Error, "File Not Found".to_string()));
    logger(&Log::new(Level::Error, "Login failed".to_string()));
    logger(&Log::new(
        Level::Warn,
        "Response Time Decreasing".to_string(),
    ));
}
```

Fonksiyonlardan closure döndürme ile ilgili olarak farklı bir örnekle devam edelim. Bu örnekte metinsel ifadeler
üzerinde doğrulama işlemlerini ele alan fonksiyonel bir tasarım söz konudur. Tüm metotlar dikkat edileceği üzere Fn
türünden bir trait döndürmektedir.

```rust
#[allow(dead_code)]
fn min_length(length: usize) -> impl Fn(&str) -> bool {
    move |input| input.len() > length
}

#[allow(dead_code)]
fn is_digits_only() -> impl Fn(&str) -> bool {
    |input| input.chars().all(|c| c.is_ascii_digit())
}

#[allow(dead_code)]
fn illegal_chars(illegals: Vec<char>) -> impl Fn(&str) -> bool {
    move |input| input.chars().any(|c| illegals.contains(&c))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_length_test() {
        let validator = Box::new(min_length(10));
        let input = "Jan Claud Van D@me!";
        assert!(validator(input));
    }

    #[test]
    fn only_digits_test() {
        let validator = Box::new(is_digits_only());
        let input = "12345";
        assert!(validator(input));
    }

    #[test]
    fn illegal_chars_test() {
        let validator = Box::new(illegal_chars(vec!['=', ';']));
        let input = "Select * from Products; 1=1; Select * from sys";
        assert!(validator(input));
    }
}
```

## move Keyword Kullanımı

Bazı fonksiyonlarda **move** operatörü kullanıldığı gözden kaçırılmamalıdır. Bu operatör dışarıdan gelen bir değişken
varsa onun sahipliğinin **closure** bloğuna taşınmak için kullanılır. Özellikle closure ifadelerinin bir thread
içerisinde kendi başlarına yaşamaya devam etmeleri gerekiyorsa bu bildirim zorunludur. Elbette sahiplik alınması için
gerekli koşullar söz konusu ise move kullanılır. **String** ve **Vec** gibi türlerdede Copy trait implementasyonu
olmadığında sahipliğin bilinçli olarak taşınacağı belirtilmelidir. **move** keyword kullanımı zaman zaman kafa
karıştırıcı olabilir. Derleme zamanı her ne kadar uyarsa da hangi durumlarda gerekli olduğunu bilmek önemlidir. Bunun
için aşağıdaki özet tablodan yararlanılabilir.

| Kullanım Durumu                                                                   | move Gerekir mi? |
|-----------------------------------------------------------------------------------|------------------|
| Closure bir thread’e yollanacaksa (Örn, thread::spawn üzerinden)                  | Evet             |   
| Closure dışarıdan gelen String, Vec, Box gibi sahipliği alan türleri kullanıyorsa | Evet             |  
| Döndürülen closure kendi bloğunda harici bir veriye ihtiyaç duyuyorsa             | Evet             | 
| Closure sadece referansla çalışıyor veya Copy Trait türevlerini kullanıyorsa      | Hayır            |

Kullanımla ilgili bazı örnekler;

```rust
fn main() {
    let message = String::from("Hello Rust");
    let _ = || println!("{}", message);
    println!("{}", message);
}
```

Yukarıdaki kullanımda message isimli değişkenin closure tanımı sonrası da kullanılabildiği görülür. Ancak burada move
operatörü ile bilinçli şekilde sahipliği closure içerisine aktarırsak bir hata alırız.

```rust
fn main() {
    let message = String::from("Hello Rust");
    let _ = move || println!("{}", message);
    println!("{}", message);
}
```

Bu durumda aşağıdaki hata üretilir.

```text
let message = String::from("Hello Rust");
   |         ------- move occurs because `message` has type `String`, which does not implement the `Copy` trait
88 |     let _ = move || println!("{}", message);
   |             -------                ------- variable moved due to use in closure
   |             |
   |             value moved into closure here
89 |     println!("{}", message);
   |                    ^^^^^^^ value borrowed here after move
```

Burada String türünün kullanılması taşıma hatasına sebebiyet vermektedir. Zira **Copy trait** uygulayan bir türün
kullanılırsa sorun oluşmayacaktır.

```rust
fn main() {
    let value = 23;
    let closure_1 = || println!("{}", value);
    let closure_2 = move || println!("{}", value);
    closure_1();
    closure_2();
}
```

