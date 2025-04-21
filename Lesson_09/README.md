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

Takip eden örnekler Game türünden veriler içeren bir vector ile ilişkilidir. Örnek oyun bilgileri
için [repository.rs](./src/repository.rs) dosyasına bakılabilir. Çok basit bir örnekle başlayalım. Oyunları yıllara göre
sıralamak istediğimizi düşünelim. Normalde vector türleri belli bir key değerine göre sıralama işlemi için **sort_by_key
** isimli metodu sağlar. Bu metod **FnMut(&T) -> K** davranışını uygulayan bir ifade bekler. Bir başka deyişle sıralama
için kullanılacak anahtar alanı ele alacağı bir davranışa ihtiyaç duyar. Buna göre aşağıdaki gibi bir örnek
yazılabilir.

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

NotYetImplemented();
