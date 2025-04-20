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

## Temel Dosya I/O İşlemleri

Aşağıdaki örneklerde dosya okuma ve yazma işlemleri farklı şekillerde ele alınmaktadır.

_NotYetImplemented();_