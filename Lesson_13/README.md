# Ders 13: Macros

Makrolar meta programlamanın temel unsurlarındadır. Meta programlama kod yazan kodlar oluşturulmasını benimseyen bir
yaklaşımdır. Makroları kullanarak kod tekrarını azaltabilir ve daha okunabilir kodların oluşturulmasını sağlayabiliriz.
Makrolar ile delerme zamanında kaynak kod üzerinde manipülasyon da yapılabilir. Ayrıca struct, enum, fn gibi yapıların
derleme zamanında analiz edilip yeni kodların üretilmesini de sağlayabiliriz. Bugüne kadarki örneklerimizde bir çok
makro kullandık. println!, write!, vec! gibi sonu ! ile biten enstrümanlar aslında birer makrodur. Makroları Declarative
_(Bildirime dayalı)_ ve Procedural _(Yönergelere dayalı)_ olmak üzere iki ana kategoriye ayırabiliriz.

## Hello Macro _(Declarative)_

Makrolar belli bir fonksiyonelliğin farklı türevlerinin yazmaktan kaçınmamızı da sağlar. Aşağıdaki basit fonksiyonu göz
önüne alalım.

```rust
pub fn max_of_two(a: i32, b: i32) -> i32 {
    if a >= b {
        return a;
    }
    b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_of_two() {
        assert_eq!(max_of_two(10, 20), 20);
        assert_eq!(max_of_two(30, 20), 30);
    }
}
```

max_of_two fonksiyonu iki integer değerden hangisi büyükse onu geriye döndürmek amacıyla tasarlanmıştır. Ancak n adet
sayının birbiriyle karşılaştırılmasını istediğimiz bir durumda ne yaparız? Bu genellikle yeni bir fonksiyon yazılmasını
gerektirecektir. Bunun yerine bir makro hazırlayıp, sum_of fonksiyon bloğunun gelen koşullara göre üretilmesini de
sağlayabiliriz.

```rust
macro_rules! max_of {
    ($x:expr) => {
        $x
    };
    ($x:expr,$y:expr) => {
        if $x > $y {
            $x
        } else {
            $y
        }
    };
    ($x:expr,$($y:expr),+) => {
        max_of!($x,max_of!($($y),+))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_max_macro() {
        assert_eq!(max_of!(1), 1);
        assert_eq!(max_of!(2, 7), 7);
        assert_eq!(max_of!(10, 0, 6, 19, 20, 3, 2, 7), 20);
        assert_eq!(max_of!(-8, -5, -3, -99), -3);
    }
}
```

Declarative yani bildirime dayalı makrolar yazmak için macro_rules isimli başka bir makro kullanılır. Kendi yazdığımız
makrolar dahil isimlendirmelerde fonksiyon adı ! işareti ile sonlandırılır. Örnekte bazı özel kalıpları kullanılmıştır.

- ($x:expr) => { $x } : Tek bir argüman ile çalışılacağını belirtir. expr ile bir expression türü ifade edilir.
  Dolayısıyla makronun uygulandığı yapıda tek bir sayısal ifade varsa doğrudan döndüren bir kod bloğu üretilecektir.
- ($x:expr, $y:expr) => { ... } : Bu kısımda ise iki argümanla eşleşilen durumu ele alır. Burada if kullanılan bir kod
  bloğunun üretimi sağlanır.
- ($x:expr, $($y:expr),+) => { ... } : Bu kalıpsa iki veya daha fazla argüman için geçerlidir. İkinci söz diziminde yer
  alan + operaötörü en az bir veya daha fazla anlamındadır. Bu durumla karşılaşılması halinde recursive olarak kendisini
  çağıran bir fonksiyon kodu üretilecektir.

## MetaSyntactic Variables

Makrolarda ifadeleri analiz etmek ve eşleşmeleri yakalamak için token'lar kullanılır. Bunlardan en çok kullanılanlar
aşağıdaki tabloda belirtilmektedir.

| Token     | Açıklama                                                              | Örnek                                             |
|-----------|-----------------------------------------------------------------------|---------------------------------------------------|
| `ident`   | Değişken, fonksiyon, struct adı gibi tanımlayıcıyıları temsil edir    | `User`, `my_function`, `x`                        |
| `ty`      | Belirli bir türü temsil eder _(örneğin, `f32`, `String`, `Vec<i32>`)_ | `f32`, `String`, `Option<T>`                      |
| `expr`    | Bir expression anlamına gelir.                                        | `5 + 4`, `"hello world"`, `vec![1, 2, 3, 4, 10]`  |
| `stmt`    | Bir ifade ya da bildirim anlamına gelir.                              | `let range = 50;`, `return 3.14;`                 |
| `path`    | Modül ya da tür yolu için kullanılır                                  | `std::io::Read`, `crate::module::function`        |
| `literal` | Sabit değer anlamına gelir (string, sayı, boolean).                   | `23`, `"rustacean"`, `false`                      |
| `block`   | `{}` bloğunu temsil eder.                                             | `{ let x = 10; x + 1 }`                           |
| `item`    | struct, enum, fn gibi enstrümanları temsil eder.                      | `struct Product;`, `fn send_email() {}`           |
| `meta`    | Bir attribute' u temsil eder.                                         | `#[derive(Debug)]`, `#[cfg(target_os = "linux")]` |
| `tt`      | Herhangi bir "token tree" ağacını temsil eder.                        | Herhangi bir Rust kodu parçası olabilir           |

## Örnekler

Aşağıdaki kod parçalarında farklı senaryoların ele alındığı procedural makrolar yer almaktadır. İlk örnek bir model
nesnesi için gerekli struct'ı kolayca oluşturmak için kullanılır.

```rust
macro_rules! crud {
    ($struct_name:ident, $($field_name:ident: $field_type:ty),*) => {
        #[derive(Debug)]
        struct $struct_name {
            $(
                $field_name: $field_type,
            )*
        }

        impl $struct_name {
            fn new($($field_name: $field_type),*) -> $struct_name {
                $struct_name { $($field_name),* }
            }
        }
    };
}

crud!(Product, id: i32,title: String,list_price:f32,category: String);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_crud_macro() {
        let c64 = Product::new(
            1,
            "C64 monitor 14.1inch".to_string(),
            999.99,
            "Retro Computer".to_string(),
        );
        assert_eq!(c64.id, 1);
        assert_eq!(c64.title, "C64 monitor 14.1inch".to_string());
        assert_eq!(c64.list_price, 999.99);
        assert_eq!(c64.category, "Retro Computer".to_string());
    }
}
```

Örneğin Product, Customer, Order, Category ve benzeri entity nesnelerinin yer aldığı bir senaryoda her birisi için ayrı
ayrı struct geliştirmek yerine bir makro ile kod tekrarlarının önüne geçebilir, veri yapılarını basitçe
tanımlayabiliriz. crud isimli makro argüman olarak gelen identifier ve type bilgilerini kullanarak struct'ın temel
halini inşa eder ve aynı zamanda new metodunu otomatik olarak implemente eder.