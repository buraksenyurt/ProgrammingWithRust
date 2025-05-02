# Ders 11: Multi-Thread Programlama

İşletim sistemi tarafında bir uygulama başlatıldığında tekil bir process içerisinde bir ana iş parçacığı _(main thread)_
açılır. Bazı durumlarda program tarafından birden fazla işin eş zamanlı olarak işletilmesi gerekir. Bu durumda
genellikle yeni thread'ler oluşturulur. Birden fazla iş parçacığını aynı anda çalıştırmak performansı artırmak için
idealdir ancak çalışması karmaşıktır. Problemlerden birisi farklı thread'lerin aynı veriye erişip kullanmaya
çalışmasıdır. Thread'ler senkronize edilmediklerinde veriye tutarsız sıralarda erişebilirler. Bu genellikle **Race
Condition** olarak adlandırılan problemin oluşmasına sebebiyet verir. Bir diğer sorun iki iş parçacığının birbirinin
beklemesi durumudur ve Deadlock olarak adlandırılır. Her iki problem de eş zamanlı programlamanın _(Concurrent
Programming)_ thread kullanılan senaryolarda en çok karşımıza çıkanlar arasındadır.

![OS Threads.png](processes.png)
_İşletim Sisteminde Process ve Thread Durumları_

## Thread Oluşturmak

Aşağıdaki kod parçasında ana thread dışında yeni bir thread daha oluşturulması ele alınmaktadır.

```rust
use std::thread;
use std::time::Duration;

fn main() {
    start_a_simple_thread();
    println!("After the thread calling");
}

pub fn start_a_simple_thread() {
    let handle = thread::spawn(move || {
        println!("Starting thread...");
        thread::sleep(Duration::new(3, 0));
        println!("Thread stopped...");
    });
    handle.join().unwrap();
}
```

Bir thread oluşturmak için spawn metodu kullanılır. Bu metot FnOnce trait uygulayan herhangi bir kod bloğunu parametre
olarak alabilir. Dışırdan thread içerisine alınacak değişkenler için move operatörü kullanılır. Ayrıca spawn metodu
geriye bir JoinHandle türü döndürür. Örnekte dikkat edileceği üzere handle üzerinden join çağrısı yapılmıştır. Bu
yapılmadığı takdirde program çalışmakta olan ikinci thread'in işleyişinin bitmesini beklemeden devam eder ve sonlanır.
Yani main thread'in, içeride başlatılan diğer thread'lerin işlerini bitirmeden sonlanmasını engellemek için JoinHandle
nesne takibini yapmak gerekir.

## move Operatörü

Spawn metodu, parametre olarak bir closure kabul eder ve bu kod bloğuna istenirse dış thread üzerinden veri taşınabilir.
Burada move operatörü ile bir bildirim yapılır. move operatörü ile ilgili durumu anlamak için aşağıdaki kod örneğini göz
önüne alalım.

```rust
use std::thread;

fn main() {
    move_keyword_error();
    println!("After the thread calling");
}

pub fn move_keyword_error() {
    let student_points = vec![30.50, 49.90, 65.55, 90.00, 81.00];
    let handle = thread::spawn(|| {
        println!("Thread is starting...");
        println!("{:?}", student_points);
        println!("Thread completed");
    });
    handle.join().unwrap();
}
```

Ana thread içinde tanımlı olan student_points vektörünün yeni başlatılan bir thread içerisinde kullanılması
örnekleniyor. Eğer move operatörünü kullanmazsak derleyici şöyle bir hata mesajı verecektir.

```text
error[E0373]: closure may outlive the current function
, but it borrows `student_points`, which is owned by the current function
```

Sorun, ana thread'in sahiplendiği bir referans türünün haricen açılmış bir thread'in işlettiği closure'a ödünç verilmeye
çalışılmasıdır. Dolayısıyla move keyword'ünü kullanarak derleyici bu konuda bilgilendirmek ve yaşam süresinin
ayarlanmasını sağlamak gerekir. İlginç olan bir durum ise, bu vektörün elemanlarının bir for döngüsü ile ela alınması
halidir. Konuyu daha iyi anlamak için kodu aşağıdaki şekilde değiştirelim.

```rust
use std::thread;

fn main() {
    move_keyword_error();
    println!("After the thread calling");
}

pub fn move_keyword_error() {
    let student_points = vec![30.50, 49.90, 65.55, 90.00, 81.00];
    let handle = thread::spawn(|| {
        println!("Thread is starting...");
        for point in student_points {
            println!("Processing for point {}", point);
        }
        println!("Thread completed");
    });
    handle.join().unwrap();
}
```

Vektör türü ile ifade edilen veriler bilindiği üzere heap üzerinde konuşlandırlır. main thread içinde başlatılan ikinci
thread bu vektörün sahipliğini _(ownership)_ closure ile açılan bloğa taşımak ister. Ancak ikinci thread main thread'den
daha uzun süre çalışabilir ve bu güvenli bir çalışma biçimi değildir _(Memory Safe)_ Bu yüzden rust move operatörü ile
durumun açıkla belirtilmesini bekler.

Rust **thread safe** bir ortam sağlamaya çalışır. Birden fazla thread'in aynı bellek adresine işaret
etmesi data race probleminin oluşmasına yol açabilir. **move** kullanıldığında closure ifadesi, student_points
vektörünün sahipliğini alır. Vektörün sahipliği artık ana thread'e değil, closure bloğuna aittir. Rust
thread’ler arası sahiplik sorununu bu şekilde çözer ve data race durumunun önüne geçilir.

Peki vektör elemanlarını teker teker dolaştığımızda **move** kullanılmaması neden bir probleme sebebiyet vermez? Bunun
nedeni closure bloğunun vektörün kendisini değil elemanlarını kullanmasıdır. Zira bu örnekte vektör elemanları boyutu
belli olan primitive tiplerden **f32** türündendir ve dolayısıyla closure ifadesine kopyalanarak taşınabilirler. Bir
başka deyişle closure bu kopyalanan veriler üzerinde çalışır ki orjinal vektörün sahiplenilmesi veya borç
olara alınması söz konusu olmaz.

Kurguya daha işe yarar bir senaryo haline getirelim. Bir sayı dizisindeki elemanların faktöryel değerlerini örneğin 4
farklı iş parçacığında ele almaya çalışalım. Aşağıdaki örnek kod parçasında bu durum ele alınmaktadır.

```rust
use std::thread;

fn main() {
    multiple_threads_sample();
    println!("After the thread calling");
}

fn calc_factorial(n: u64) -> u64 {
    (1..=n).product()
}
pub fn multiple_threads_sample() {
    let numbers = vec![10, 3, 5, 13, 8, 9, 1, 2, 17, 11, 7, 6];
    let threads_count = 4;
    let mut handles = vec![];
    let chunk_size = numbers.len() / threads_count;

    for i in 0..threads_count {
        let chunk = numbers[i * chunk_size..(i + 1) * chunk_size].to_vec();
        handles.push(thread::spawn(move || {
            let mut results = vec![];
            for num in chunk {
                println!("Thread {} processing for {}", i, num);
                results.push((num, calc_factorial(num)));
            }
            results
        }));
    }

    let mut final_results = vec![];

    for handle in handles {
        final_results.push(handle.join().unwrap());
    }

    println!("{:?}", final_results);
}
```

Numbers isimli vektörün 12 elemanı bulunmaktadır. **threads_count** değişkeni ile belirtilen sayı kadar **thread**
açılır ki örneğimize göre ana thread haricinde 4 farklı thred açılması söz konusudur. Sayı dizisindeki elemanları dört
parçaya bölünür ve her blok açılan thread'lerden birisine işlenmek üzere gönderilir. Birden fazla thread söz konusu
olduğundan her birinin işini bitirmesinin beklenmesi gerekir. Bu yüzden **spawn** çağrılarındaki **JoinHandle**
nesneleri de bir vektörde toplanır ve son aşamada tamamının sonuçları üretilene kadar beklenir.

## Atomic Reference Counting

// todo@buraksenyurt Not Implemented Yet

## Thread Poisoning

// todo@buraksenyurt Not Implemented Yet

## Concurrency vs Parallel Programming

// todo@buraksenyurt Not Implemented Yet