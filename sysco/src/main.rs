use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        show_help();
        process::exit(1);
    }

    match args[1].as_str() {
        "-e" | "-env" => display_env(),
        "-w" | "-cwd" => display_cwd(),
        "-u" | "-usr" => display_user(),
        "-h" | "-help" => show_help(),
        _ => {
            eprintln!("Invalid option. Use '-help' for more information");
            process::exit(1);
        }
    }
}

/// Sistem değişkenlerini gösteren fonksiyon
fn display_env() {
    println!("System Environment Variables:");
    for (k, v) in env::vars() {
        println!("{}: {}", k, v);
    }
}

/// Şu an içinde bulunulan klasörü gösteren fonksiyon
fn display_cwd() {
    match env::current_dir() {
        Ok(path) => println!("Current Working Directory: {}", path.to_string_lossy()),
        Err(e) => {
            eprintln!("Error retrieving current directory: {}", e);
            process::exit(1);
        }
    }
}

/// Şu anki kullanıcıyı gösteren fonksiyon
fn display_user() {
    match env::var("USERNAME") {
        Ok(user) => println!("Current User: {}", user),
        Err(e) => eprintln!("Could not retrieve user information {}", e),
    }
}

fn show_help() {
    println!("SysCo - A lightweight system tool\n");
    println!("Usage: sysco <option>\n");
    println!("Valid Options");
    println!("  -h, -help : Display usage");
    println!("  -e, -env  : Show env values");
    println!("  -w, -cwd  : Show the current working directory");
    println!("  -u, -usr  : Current root user\n");
    println!("For details azon.com/sysco/help/readme.md");
}
