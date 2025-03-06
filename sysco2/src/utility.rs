use crate::command_error::CommandError;
use std::{env, process};

pub struct Utility {}

impl Utility {
    pub fn help() {
        eprintln!(
            r#"A tiny system utility.
    Usage: sysco2 <option>

    Options:

    -e -env  -Show environment variables
    -w -cwd  -Show current working directory
    -u -usr -Show current user
    -h -help -Show usages
            "#
        );
    }

    /// Print some system information into terminal
    ///
    /// # Usage
    /// `run` fonksiyonu, verilen giriş parametresine göre belirli bir sistem bilgisini ekrana yazdırır.
    /// Supported arguments:
    /// - `"-env"`  : List of environment variables
    /// - `"-cwd"`  : Shows current working directory
    /// - `"-user"` : Current user info
    /// - `"-help"` : Show commands
    ///
    /// # Parameters
    /// - `input`: `&str` - Environment argument
    ///
    /// # Outputs
    /// - If commands valid shows the command's output
    ///
    /// # Example Usage
    /// ```
    /// Utility::run("env");    // All environment variables
    /// Utility::run("dir");   // Invalid command
    /// ```
    pub fn run(input: &str) -> Result<(), CommandError> {
        match input {
            "-e" | "-env" => Self::display_env(),
            "-w" | "-cwd" => Self::display_cwd(),
            "-u" | "-usr" => Self::display_user(),
            "-h" | "help" => Self::help(),
            _ => {
                return Err(CommandError::Invalid(input.to_string()));
            }
        }
        Ok(())
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
}
