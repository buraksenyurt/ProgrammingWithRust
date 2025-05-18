# Programming With Rust

Uludağ Üniversitesi Bil.Müh/BÖTE bölümlerinde açılan Rust ile Sistem Programlama dersine ait örnek kodlar ve notların
yer aldığı repodur.

- [Rust ile Sistem Programlama](#programming-with-rust)
    - [İsimlendirme Standartları](#isimlendirme-standartları-_naming-conventions_)
    - [Ders 00 - Hello World Uygulaması ve Temel Veri Türleri](./Lesson_00/README.md)
    - [Ders 01 - Metotlar](./Lesson_01/README.md)
    - [Ders 02 - Akış Kontrol İfadeleri ve Döngüler](./Lesson_02/README.md)
    - [Ders 03 - Struct Veri Türü](./Lesson_03/README.md)
    - [Ders 04 - Ownership ve Borrowing Hakkında](./Lesson_04/README.md)
    - [Ders 05 - Enum Veri Türü, Pattern Matching](./Lesson_05/README.md)
    - [Ders 06 - Lifetime(Yaşam Ömrü) Kavramı](./Lesson_06/README.md)
    - [Ara Sınav](MidTermExam.md)
    - [Ders 07 - Generic Türler](./Lesson_07/README.md)
    - [Ders 08 - Traits](./Lesson_08/README.md)
    - [Ders 09 - Closures](./Lesson_09/README.md)
    - [Ders 10 - Smart Pointers](./Lesson_10/Readme.md)
    - [Ders 11 - Threads](./Lesson_11/README.md)
    - [Ders 12 - Channels](./Lesson_12/README.md)
    - [Ders 13 - Macros](./Lesson_13/README.md)
    - [Destekleyici Bölümler]()
        - [Birim Test](UnitTests.md)
        - [Hata Yönetimi](ErrorHandling.md)
        - [Temel I/O İşlemleri](./io.md)
        - [Streams]()
- [Yardımcı Kaynaklar](#yardımcı-kaynaklar)
- [Örnek Uygulamalar](#örnek-uygulamalar)

# İsimlendirme Standartları _(Naming Conventions)_

Rust dilinde **isimlendirme standartları _(Naming Conventions)_** da kod okunurluğu ve genel uyumluluk açısından
önemlidir. Aşağıdaki isimlendirme önerilerine ait bilgilerin yer aldığı bir tablo bulunmaktadır.

| Kategori                | İsimlendirme Standardı    |
|-------------------------|---------------------------|
| Constants               | SCREAMING_SNAKE_CASE      |
| Conversion constructors | from_some_other_type      |
| Crates                  | unclear                   |
| Enum variants           | UpperCamelCase            |
| Features                | unclear but see C-FEATURE |
| Functions               | snake_case                |
| General constructors    | new / init                |
| Lifetimes               | 'a, 'de, 'src             |
| Local variables         | snake_case                |
| Macros                  | snake_case!               |
| Methods                 | snake_case                |
| Modules                 | snake_case                |
| Statics                 | SCREAMING_SNAKE_CASE      |
| Traits                  | UpperCamelCase            |
| Type parameters         | T, K gibi olabilir        |
| Types                   | UpperCamelCase            |

## Yardımcı Kaynaklar

- [Resmi rust kitabı](https://doc.rust-lang.org/book/)
- [Örneklerle Rust](https://doc.rust-lang.org/stable/rust-by-example/)
- [Birlikte Rust Öğrenelim Video Serisi](https://www.youtube.com/playlist?list=PLY-17mI_rla4zcAQtUsolk6G5bfbQAdYZ)
- [Installation](https://www.rust-lang.org/tools/install)
- [Jet Brains Rust Rover](https://www.jetbrains.com/rust/)
- [Visual Studio Code](https://code.visualstudio.com/download)
- [Rust Paket Yönetim Sistemi -Crates.io](https://crates.io/)
- [Rust Core Library](https://doc.rust-lang.org/core/index.html#the-rust-core-library)

## Örnek Uygulamalar

- sysco; Basit bir terminal aracı. Lesson_01 dersinde kullanılan örneğin farklı bir sürümü.
- sysco2; sysco programının daha iyileştirilmiş bir sürümü.
- collector; sysinfo küfesini kullanarak cpu, memory metrikleri toplayan uygulama.
- mocking; Birim testlerde mock nesne kullanımının ele alındığı örnek.
- drone-lab; Konu tekrarı, birim testler, lifetimes kullanımları.
- fops; Temel I/O işlemleri.
- router; trait kullanımını örneklemek için eklenmiş olan uygulama.
- game-engine; trait, thread ve channel kullanımlarının ele alındığı örnektir.
