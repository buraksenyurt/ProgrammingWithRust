# I/O Temelleri

Bilgisayar programları çoğu zaman çevresel enstrümanlarla iletişim kurar. Konsol panelinden bilgi okumak, konsol
ekranına bir şeyler yazdırmak, dosya hazırlamak, dosya içeriklerini doldurmak veya okumak, ağ ortamına paket yollarken
girdi/çıktı kullanmak bu operasyonlar arasında sayılabilir. Rust genel olarak Input/Output işlemleri için std::io
modülünü kullanır. I/O operasyonları hataya son derece açıktır. Örneğin var olmayan bir dosyaya ilave yapmaya çalışmak,
yazma yetkisi olmayan bir dosyaya bilgi yazmak gibi birçok durum hataya sebebiyet verir. Dolayısıyla tüm I/O
operasyonları Result<T,E> türünü döner. I/O işlemleri ayrıca blocking ve non-blocking olarak iki şekilde de ele alınır.
Blocking türünde işlemler tamamlanana kadar bekeleme yapılır. Diğerinde ise işlemin tamamlanması beklenmez.

Aşağıda temel dosya girdi çıktı işlemlerine ait basit örnekler yer almaktadır. Örneklerde temel olarak Game isimli bir
veri yapısı da kullanılmıştır. Bir Game nesnesinin string dönüşümünü kolaylaştırmak ve böylece I/O işlemlerinde
kullanabilmek için Display davranışı ile donatılmıştır.

```rust
use std::fmt::Display;

#[derive(Debug)]
pub struct Game {
    pub title: String,
    pub year: u16,
    pub popularity: f32,
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}|{}|{}", self.title, self.year, self.popularity)
    }
}
```

## Standart Giriş-Çıkış İşlemleri (stdin, stdout, stderr)

`stdin` _(Standard Input)_, `stdout` _(Standard Output)_ ve `stderr` _(Standard Error)_, UNIX tabanlı sistemlerden beri
kullanılan giriş-çıkış standartlarıdır.

- **stdin**: Kullanıcıdan veya başka bir programdan veri almak için kullanılır.
- **stdout**: Programın normal çıktısını kullanıcıya veya başka bir programa iletmek için kullanılır.
- **stderr**: Hata mesajlarını ayrı tutmak için kullanılır.

Terminal operatörleri ve kullanımları genel olarak aşağıdaki gibi ifade edilebilir.

- **Pipe (`|`)**: Bir programın **stdout** çıktısını başka bir programın **stdin** girişine yönlendirir.
  ```sh
  ls | grep ".rs"
  ```

- **Redirect (`>`)**: **stdout** çıktısını dosyaya yönlendirir.
  ```sh
  ls > files.txt
  ```

- **Append (`>>`)**: **stdout** çıktısını mevcut dosyaya ekleyerek yönlendirir.
  ```sh
  echo "Some thoughts about you" >> memories.txt
  ```

### Terminalden Veri Almak

Aşağıdaki örnekte kullanıcının terminalden girdiği bilgiler okunur ve tekrar geri yazdırılır. Bazı sistemlerin **echo**
komutları bu prensibe göre çalışır.

```rust
use std::io;

fn write_to_file() -> io::Result<()> {
    let mut input = String::new();
    println!("Please enter some text:");

    io::stdin().read_line(&mut input)?;
    println!("Your text is: {}", input.trim());

    Ok(())
}
```

Farklı bir örnekle devam edelim. Yine stdin ile ekrandan iki değer okuyoruz ancak bu sefer birde parse operasyonu var.
I/O operasyonlarında hatalar oluşması muhtemel. Örneğin girilen değer sayısal bir değere dönüştürülemeyebilir.
Dönüştürme hatası parse metodunu takiben ele alınır. read_line operasyonunda da hatalar olması muhtemeldir. Farklı
kullanımları göstermek amacıyla read_line için ? operatörü kullanılmıştır.

```rust
fn sum() -> io::Result<i32> {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Please enter the first number:");
    io::stdin().read_line(&mut input1)?;

    println!("Second number:");
    io::stdin().read_line(&mut input2)?;

    let x: i32 = input1.trim().parse().expect("Please enter a number!");
    let y: i32 = input2.trim().parse().expect("Please enter a number!");

    Ok(x + y)
}
```

Sıradaki kod parçasında ise terminalden sürekli çıktı alınması örneklenmiştir. Döngü, terminalden bilgi yazıldıkça işler
ve kullanıcı Ctrl+Z gibi bir kesme gönderene kadar da çalışmaya devam eder.

```rust
use std::io::{self, BufRead};

fn read() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();

    println!("Please enter some text (Ctrl+Z for exit):");
    for line in reader.lines() {
        let line = line?;
        println!("Input: {}", line);
    }

    Ok(())
}
```

### Pipe'dan Gelen Veriyi Okumak

Pipe operatörü kullanılarak okunan bir verinin devam eden ifadeye stdin ile aktarılması sağlanabilir. Bunun için
aşağıdaki terminal komutu kullanılabilir. Buna göre logs.dat içeriği cargo run komutu sonrası ilgili programa aktarılır.

```sh
cat logs.dat | cargo run
```

Program koduna göre stdin ile gelen içerik _(ki örnekte logs.dat dosyasını açan cat programıdır)_ bir reader değişleni
üzerinden okunarak ekrana yazdırılır.

```rust
use std::io::{self, BufRead};

fn read_from_pipe() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();

    println!("Data is retrieving...");
    for line in reader.lines() {
        let line = line?;
        println!("Data: {}", line);
    }

    Ok(())
}
```

### stdout'u Dosyaya Yönlendirme

Çıktıyı **stdout** üzerinden bir dosyaya yönlendirmek için aşağıdaki komut kullanılabilir.

```sh
cargo run > logs.txt
```

Bu sefer **cargo run** ile çalıştırılan program çıktıyı logs.txt dosyasın yönlendirir. Kod içerisinde kullanılan *
*writeln!** makrosu, ilk parametre olarak gelen handle değişkenine doğru yazma işlemi gerçekleştirir. handle değişkeni
stdout'u kullanır.

```rust
use std::io::{self, Write};

fn write_log() -> io::Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    writeln!(handle, "This will be written to a file.")?;

    Ok(())
}
```

### Pipe ve Redirect Kullanımı (stdin → stdout)

Pipe sonrasında redirect operatörü kullanılarak da bir süreç tasarlanabilir. Aşağıdaki terminal komutuna göre logs.dat
dosya içeriği cat programı ile rust koduna gönderilir ve rust kodu da bunu output_logs.txt dosyasına filtreleyerek
aktarabilir.

```sh
cat logs.dat | cargo run > output_logs.txt
```

Örnekte, stdin üzerinden gelen içerik reader isimli handler ile okunur ve stdout kullanan writeln! makrosu üzerinden
çıktı olarak dışarıya verilir.

```rust
use std::io::{self, BufRead, Write};

fn double_usage() -> io::Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();

    let reader = stdin.lock();
    let mut writer = stdout.lock();

    for line in reader.lines() {
        let line = line?;
        writeln!(writer, "Data received {}", line)?;
    }
    Ok(())
}
```

Buraya kadar ele aldığımız stdin ve stdout kullanımlarına ilişkin şunları söyleyebiliriz.

- **stdin** kullanarak terminalden veya başka programlardan veri okunabilir.
- **stdout** kullanarak terminal ekranına veya başka programlara veri yazdırılabilir.
- **stderr** hata ve uyarı mesajlarını standart çıktıdan ayırmak için kullanılır.
- **Unix/Linux** ortamlarında `|`, `>` ve `>>` operatörleri standart giriş-çıkış işlemlerinde, verilerin akışını
  yönetmek için kullanılır.

---

## Temel Dosya I/O İşlemleri

Aşağıdaki örneklerde dosya okuma ve yazma işlemleri farklı şekillerde ele alınmaktadır.

### Dosya Oluşturma ve İçerik Yazma _(create)_

Aşağıdaki fonksiyon parametre olarak Game türünden bir dizi alır. Bu dizinin her bir elemanı için satır satır akan bir
String içerik üretilir. Söz konusu içerik contents isimli değişkende toplanır. f değişkeni games.dat isimli bir dosyayı
temsil eder. Dosya create metodu ile oluşturulur. Create metodu dosya yoksa yeni bir tane oluşturur ama varsa truncate
işlemini icra eder, bir başka deyişle içeriğini sıfırlar.

```rust
pub fn write_games_to_file(games: &[Game]) -> io::Result<()> {
    let mut contents = String::new();
    for g in games {
        contents.push_str(&g.to_string());
        contents.push_str("\n");
    }
    let mut f = File::create("games.dat")?;
    f.write_all(contents.as_bytes())?;
    Ok(())
}
```

### Dosya İçeriğini Okuma

Aşağıdaki örnek kod **games.dat** isimli dosyanın içeriğini satır satır okuyarak **String** türünden bir vector'de
toplar. Burada satır bazında okuma işlemi yapmak için **BufReader** nesnesi kullanılmıştır. **BufReader** esasında bu
örnek için oldukça maliyetlidir. Genel olarak **TCP Stream**'lerin okunması gibi işlemlerde **BufReader** kullanmak daha
mantıklıdır.

```rust
pub fn read_games_from_file() -> io::Result<Vec<String>> {
    let mut games = Vec::new();
    let f = File::open("games.dat")?;
    for line in BufReader::new(f).lines() {
        games.push(line?);
    }
    Ok(games)
}
```

Yukarıdaki fonksiyondan yararlanılarak dosya içerisinde yer alan oyun bilgilerinin **|** işaretine göre ayrıştırılıp
**Game** türünden bir **vector** halinde ele alınması da mümkündür. Bunun için aşağıdaki gibi bir fonksiyondan
yararlanılabilir.

```rust
pub fn read_games_to_vec() -> io::Result<Vec<Game>> {
    let mut games = Vec::new();

    for line in read_games_from_file()? {
        let cols: Vec<&str> = line.split('|').collect();
        if cols.len() != 3 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Beklenmeyen sütun sayısı: `{}`", line),
            ));
        }

        let title = cols[0].to_string();
        let year = cols[1]
            .parse::<u16>()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        let popularity = cols[2]
            .parse::<f32>()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        games.push(Game {
            title,
            year,
            popularity,
        });
    }

    Ok(games)
}
```

Yukarıdaki iki operasyon tek bir metot haline de getirilebilir. İlk olarak path parametresi üzerinden gelen dosya
açılmaya çalışılır. Operasyon sonrasında hata olması durumu söz konusudur ve bu **?** operatörü ile ele alınarak **error
propagation** ile çağıran yere doğru gönderilir. Dosya içeriğini satır bazından okumak için **BufReader** nesnesi
kullanılır. Bu nesne oluşturulurken bir file nesnesi aldığına dikkat edilmelidir. **BufReader** üzerinden ulaşılan lines
metodu satır bazında okuma yapılmasını sağlar. Döngünün her iterasyonunda dosyadan bir satır okunur. Bu işlem okunabilir
satır kalmayıncaya kadar devam eder. İlgili kontrol is_empty çağrısı ile gerçekleştirilmektedir. Game nesnesnin dosya
içerisindeki tutuluş biçimine göre **|** işaretleri ile ayrılmış 3 kolon olması gerekmektedir. Bu durum kontrol edilir
ve hatalı kolon olması halinde geriye bir Error döndürülür. Buradaki akış tamamen stratejiye bağlıdır. Hatalı kolonların
olduğu satırları atlayarak devam etmek de bir seçenektir.

Kolonlar elde edilikten sonra bazı dönüştürme işlemleri de icra edilir ve bunlarda da **error propagation** tekniği
kullanılır.

```rust
pub fn read_games_buffered_into_vec(path: &str) -> io::Result<Vec<Game>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut games = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if !line.is_empty() {
            let cols: Vec<&str> = line.split('|').collect();
            if cols.len() != 3 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Beklenmeyen sütun sayısı: `{}`", line),
                ));
            }
            let title = cols[0].to_string();
            let year = cols[1]
                .parse::<u16>()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
            let popularity = cols[2]
                .parse::<f32>()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

            games.push(Game {
                title,
                year,
                popularity,
            });
        }
    }
    Ok(games)
}
```

### Dosyaya Veri Yazma

Bir dosyaya veri yazma işlemi aslında içeriğin bir byte array olarak aktarılmasından ibarettir. Aşağıdaki örnek
fonksiyonu ele alalım.

```rust
pub fn write_games_to_file(games: &[Game]) -> io::Result<()> {
    let mut contents = String::new();
    for g in games {
        contents.push_str(&g.to_string());
        contents.push_str("\n");
    }
    let mut f = File::create("games.dat")?;
    f.write_all(contents.as_bytes())?;
    Ok(())
}
```

Bu fonksiyon Game nesnelerinden oluşan bir diziyi parametre olarak alır. Her bir oyun değişkeni için içeriği | ile
ayıran bir string üretilir ve bunlar contents isimli String değişkende toplanır. Satır bazında ayrıştırılarak tutulan
içerik as_bytes metodu ile byte array'a çevrilip tek seferde games.dat isimli dosyaya yazdırılır.

Yazma işlemi BufWriter enstrümanını ile de gerçekleştirilebilir. Aşağıdaki kod parçasında bu durum ele alınmaktadır. Çok
büyük blokların tek seferde yazılmasından ziyade in-memory olarak tutulan içeriklerin küçük bloklar halinde yazılması
adına daha verimlidir. Yazma operasyonu aynı kaynağa doğru ele alınır. Bir dosya veya network'e yazma en çok kullanılan
senaryolardandır. Yazma işlemi tamamlandığında bellekte kalmış olabilecek veri kalıntılarının da tamamen aktarıldığından
emin olmak gerekir. Bunun için flush komutu kullanılır.

```rust
pub fn write_games_buffered(path: &str, games: &[Game]) -> io::Result<()> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    for game in games {
        writeln!(writer, "{}", game)?;
    }
    writer.flush()?;
    Ok(())
}
```

### Var Olan Dosya İçeriklerine Ek Yapmak

Çoğu zaman var olan dosya içeriklerine ilaveler yapılır. Söz gelimi log biriktiren dosyalar veya oyunun son durumunu
tutan dosyalar bunlara örnek olarak verilebilir. Bu gibi senaryolarda **OpenOptions** türünü kullanarak dosyanın hangi
modda oluşturulacağı belirtilebilir. Aşağıdaki örnekte path değişkeni üzerinden gelen dosyanın **append** modda
açılacağı belirtilir. Buna göre dosyanın sonuna ekleme yapılacağı söylenir. Yukarıda gerçekleştirilen birçok operasyonda
doğrudan File nesnesine erişmek yerine **OpenOptions** enstrümanı ile de ilerlenebilir.

```rust
pub fn append_game_to_file(path: &str, game: &Game) -> io::Result<()> {
    let mut file = OpenOptions::new().append(true).create(true).open(path)?;
    writeln!(file, "{}", game)?;
    Ok(())
}
```

### Iterator Fonksiyonlarını Kullanmak

Rust'ın güçlü özelliklerinden birisi de Zero Cost Abstraction sağlayan Higher-Order Function setidir. Genellikle
fonksiyonel dillerde fonksiyonları parametre olarak alan veya döndüren fonksiyonlar yaygın olarak kullanılır. Rust'ın
iterasyon metotları sonrasında gelen birçok fonksiyon bu tanıma uyar. Dolayısıyla Rust'ın da Higher-Order Function
desteği sağladığını söyleyebiliriz.

Aşğıdaki fonksiyon games.dat dosyasından okuma ve Game türünden vektöre dönüştürme işlevini iterasyon fonksiyonlarını
kullanarak gerçekleştirir. Örnekte bunlar map ve collect çağrılarıdır. Bu metotların çalışma zamanı maliyetleri yoktur.

```rust
pub fn read_games_to_vec_with_hof() -> io::Result<Vec<Game>> {
    read_games_from_file()?
        .into_iter()
        .map(|line| {
            let cols: Vec<&str> = line.split('|').collect();
            if cols.len() != 3 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Beklenmeyen sütun sayısı: `{}`", line),
                ));
            }

            let title = cols[0].to_string();
            let year = cols[1]
                .parse::<u16>()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
            let popularity = cols[2]
                .parse::<f32>()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

            Ok(Game {
                title,
                year,
                popularity,
            })
        })
        .collect()
}
```

Bu örnekten hareketle file_io_ops modülündeki diğer metotlarda da benzer kullanımlar icra edilebilir. Örneğin yazma
işlemini ele aldığımız fonksiyonu aşağıdaki gibi değiştirebiliriz.

```rust
pub fn write_games_buffered_with_hof(path: &str, games: &[Game]) -> io::Result<()> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);

    games
        .iter()
        .try_for_each(|game| writeln!(writer, "{}", game))?;

    writer.flush()
}
```