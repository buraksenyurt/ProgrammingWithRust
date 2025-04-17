/*
    Bu terminal uygulaması ile bazı çevre değişkenlerin(environment variables)
    değerlerini ekrana yazdırmayı planlıyoruz.
*/
use std::{env, process};

fn main() {
    // Command line argümanlarına ulaşmak için args() metodu
    // Sistem çevre değişkenlerine (key,value) çiftleri ulaşmak için vars() metodu

    // Komut satırından argüman alalım
    // --env yazılmışsa tüm çevre değişkenlerini alalım (--env args() üstünden yakalanabilir
    let args = env::args().collect::<Vec<String>>();

    // Programın mutlaka en az bir parametre ile çalışması gerektiğinde küçük bir kontrol ekledik
    if args.len() < 2 {
        print_help();
        process::exit(1); // ve programdan gently bir şekilde çıkalım
    }

    /*
        Kullanılabilecek Environment argümanları düşünüldüğünde, if bloklarının sayısı
        giderek artabilir. Çok fazla if kodun okunurluğunu zorlaştırır.
        Alternatif yollar denenerek kodun daha okunabilir ve yönetilebilir olması sağlanabilir.

        pattern matching kullanılabilir, var() metodundaki çağrılarda değişen sadece
        parametre kısmı. Birden fazla if yerine tek bir metot ile bilinen key değerleri
        çekilebilir vs
    */

    match args[1].as_ref() {
        "-h" | "-help" => {
            print_help();
        }
        "-e" | "-env" => {
            print_environments();
        }
        "-w" | "-cwd" => {
            print_cwd();
        }
        "-v" | "-var" => {
            // Fonksiyon dönüşünde bir Err varsa
            if let Err(_e) = check_key_arguments(&args) {
                print_help();
                process::exit(1);
            }

            match print_variable(&args[2]) {
                Ok(value) => println!("Value is {}", value),
                Err(e) => {
                    match e {
                        SyscoError::WrongArgumentCount => eprintln!("Invalid argument count"),
                        SyscoError::WrongVariable(v) => eprintln!("Invalid variable: {}", v),
                    }
                    // eprintln!("Error {:?}", e);
                    // SysycoError nesnesinin içeriğini eprintln! makrosunda gösterebilmek için
                    // Debug trait'i derive direktifi ile implemente edilmiştir.
                }
            }
            //print_user_name();
        }
        _ => {
            // Yukarıdaki durumlar dışında bir kullanım söz konusu ise
            print_help();
        }
    }

    // // Elde edilen argümanlardan 1nci indistekinin literal string değerinin
    // // --env olup olmadığına bakıyoruz
    // if args[1].as_str() == "-e" || args[1].as_str() == "-env" {
    //     let env_vars = env::vars();
    //     // Vars türünden olan args içeriğini dolaşan for döngüsü
    //     for (key, value) in env_vars {
    //         println!("{}:\t{}", key, value);
    //     }
    // } else if args[1].as_str() == "-h" || args[1].as_str() == "-help" {
    //     print_help();
    // } else if args[1].as_str() == "-w" || args[1].as_str() == "-cwd" {
    //     // // En garanti olmayan yol
    //     // let current_dir = env::current_dir().unwrap();
    //
    //     let current_dir = env::current_dir();
    //     match current_dir {
    //         Ok(dir) => {
    //             println!("Current working directory: {}", dir.display());
    //         }
    //         Err(e) => {
    //             eprintln!("Error : {}", e);
    //         }
    //     }
    //
    //     // if let Ok(dir) = env::current_dir() {
    //     //     println!("Current working directory: {}", dir.display());
    //     // }
    // } else if args[1].as_str() == "-u" || args[1].as_str() == "-usr" {
    //     // let user_name = env::var("USER_NAME").unwrap(); // Unwrap fonksiyonunun hata verme durumu
    //     if let Ok(user_name) = env::var("USERNAME") {
    //         println!("Username: {}", user_name);
    //     }
    // } else {
    //     // eprintln!("Komut anlaşılamadı");
    //     print_help();
    // }
}

/// Aracın nasıl kullanıldığını ekrana yazdıran metot
fn print_help() {
    // Eğer argüman dizisinin eleman sayısı 2den azsa
    // Ekrana programın nasıl kullanıldığını yazdıralım
    println!("SysCo - A lightweight system tool\n");
    println!("Usage: sysco <argument>\n");
    println!("Valid Arguments");
    println!("  -h, -help : Display usage");
    println!("  -e, -env  : Show env values");
    println!("  -w, -cwd  : Show the current working directory");
    println!("  -v, -var  : Show variable's value\n");
    println!("For details azon.com/sysco/help/readme.md");
}

fn print_environments() {
    let env_vars = env::vars();
    // Vars türünden olan args içeriğini dolaşan for döngüsü
    for (key, value) in env_vars {
        println!("{}:\t{}", key, value);
    }
}

fn print_cwd() {
    let current_dir = env::current_dir();
    match current_dir {
        Ok(dir) => {
            println!("Current working directory: {}", dir.display());
        }
        Err(e) => {
            eprintln!("Error : {}", e);
        }
    }
}

// // print_user_name aslında Environment key değerinin karşılı olan value bilgisi gösterecek
// // şekilde hata kontrolünü de ele alan bir fonksiyon haline getirilebilir.
// fn print_user_name() {
//     if let Ok(user_name) = env::var("USERNAME") {
//         println!("Username: {}", user_name);
//     }
// }

// // print_user_name aslında Environment key değerinin karşılı olan value bilgisi gösterecek
// // şekilde hata kontrolünü de ele alan bir fonksiyon haline getirilebilir.
// fn print_variable(key: &str) {
//     if let Ok(value) = env::var(key) {
//         println!("{}\t{}", key, value);
//     } else {
//         eprintln!("Key '{}' not found", key);
//     }
// }

// Bu fonksiyon String türünden bir diziyi parametre olarak kullanır ama bunu referans olarak alır.
// Dolayısıyla bütün dizi kopyalanmaz onun yerine bir pointer gelir.
// Fonksiyon yine bir Result dönmekte. Örneğin argüman sayısı 3 değilse buna uygun bir Error
// nesnesi dönülür. Her şey yolunda ise boş bir Ok dönülmektedir.
fn check_key_arguments(args: &[String]) -> Result<(), SyscoError> {
    if args.len() != 3 {
        return Err(SyscoError::WrongArgumentCount);
    }
    Ok(())
}

// Fonksiyonlardan geriye değer döndürebiliriz ancak garanti senaryolar için Result veya Option
// gibi türlerden de yararlanabiliriz.
// Aşağıdaki fonksiyon eğer variable varsa değerini yoksa kendi tasarladığımız bir Error
// nesnesini döndürür.
// Dolayısıyla fonksiyonun kullanıldığı yerde mutlak suretle her iki durumun ele alınması gerekir.
fn print_variable(key: &str) -> Result<String, SyscoError> {
    if let Ok(value) = env::var(key) {
        Ok(value) // İşlem başarılı ise Ok(T) şeklinde dönülür
    } else {
        Err(SyscoError::WrongVariable(key.to_string())) // Başarılı değilse Err(ErrorType) şeklinde dönülür
    }
}

#[derive(Debug)]
// #[allow(dead_code)]
enum SyscoError {
    WrongVariable(String),
    WrongArgumentCount,
}
