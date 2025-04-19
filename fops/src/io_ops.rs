use std::io;
use std::io::{Write, stdout};
/*
    Bu fonksiyon stdin yardımıyla bilgi okur.
    Örneğin terminalden oyun bilgisi girilebilir.
    Metot geriye io modülünde yer alan Result türünden bir değer döndürür.
*/
pub fn read_game_from_stdin() -> io::Result<String> {
    let mut input = String::new();
    println!("Sevdiğin bilgisayar oyunlarından birisinin adını girer misin?");
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

/*
    Bu fonksiyon parametre olarak gelen String array'in içeriği
    stdout'a aktarır. Örnekte bu println ile ekrana yazdırmaktır.
*/
pub fn print_games_to_stdout(games: &[String]) {
    println!("--- En Sevdiğin Oyunlar ---");

    stdout().write_all(games.join("\n").as_bytes()).unwrap();
}
