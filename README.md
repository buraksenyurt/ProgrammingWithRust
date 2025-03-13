# Programming With Rust

Uludağ Üniversitesi Bil.Müh/BÖTE bölümlerinde açılan Rust ile Sistem Programlama dersine ait örnek kodlar ve notların
yer aldığı repodur.

- [Rust ile Sistem Programlama](#programming-with-rust)
    - [İsimlendirme Standartları](#isimlendirme-standartları-_naming-conventions_)
    - [Ders 00 - Hello World Uygulaması ve Temel Veri Türleri](./Lesson_00/README.md)
    - [Ders 01 - Metotlar](./Lesson_01/README.md)
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

