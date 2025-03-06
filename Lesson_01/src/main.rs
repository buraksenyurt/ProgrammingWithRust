/*
    Bu terminal uygulaması ile bazı çevre değişkenlerin(environment variables)
    değerlerini ekrana yazdırmayı planlıyoruz.
*/
use std::{env, process};

fn main() {
    // Command line argümanlarına ulaşmak için args() metodu
    // Sistem çevre değişkenlerine (key,value) çiftleri ulaimak için vars() metodu

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
        "-u" | "-usr" => {
            // TODO@buraksenyurt Bunu herhangibir key ile çalışacak hale getirelim
            // cargo run -- -v USERNAME
            print_user_name();
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
    println!("  -u, -usr  : Current root user\n");
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

// print_user_name aslında Environment key değerinin karşılı olan value bilgisi gösterecek
// şekilde hata kontrolünü de ele alan bir fonksiyon haline getirilebilir.
fn print_user_name() {
    if let Ok(user_name) = env::var("USERNAME") {
        println!("Username: {}", user_name);
    }
}
