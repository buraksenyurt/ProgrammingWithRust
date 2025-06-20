# Rust ile Sistem Programlama - Final Sınavı Soruları

**Süre :** 75 Dakika

**Soru Sayısı :** 16 _(Sadece istenilen 10 soruyu cevaplayınız)_

## Soru 1

Rust programlama dili **generic** tür kullanımlarını da destekler. Özellikle aynı işlevsellik veya davranışların farklı türler için kod tekrarı yapmaya gerek kalmadan yazılmasında kullanılır. Tür güvenliği _(Type Safety)_ için kısıtlamalar _(Constraints)_ kullanılabilir ve türün belli **Trait** davranışlarına sahip olması şart koşulabilir. Aşağıdaki **Point** isimli veri yapısı _(struct)_ generic tip kullanmaktadır.

```rust
use std::fmt::Debug;
use std::ops::Add;

struct Point<T: Copy + Debug + Add<Output=T>> {
    x: T,
    y: T,
    z: T,
}
```

Buna göre aşağıdakiler hangisi veya hangileri doğrudur?

- **I.** T: Debug: T türü Debug trait’ini uygulamış olmalıdır.
- **II.** T: Copy: T türü kopyalanabilir (Copy trait’ini implemente etmiş) olmalıdır.
- **III.** T: Add<Output = T>: T türü toplama (+) işlemini desteklemelidir.

a) Sadece I

b) I ve II

c) II ve III

d) Hepsi doğrudur

## Soru 2
