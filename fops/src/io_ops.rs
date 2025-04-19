use crate::game::Game;
use std::fs::File;
use std::io;
use std::io::{BufRead, Write, stdout};
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

/*
    Bu metot games.dat isimli bir dosyaya, parametre olarak gelen oyunlara ait bilgileri
    yazar.
*/
pub fn write_games_to_file(games: &[Game]) -> io::Result<()> {
    let mut contents = String::new();
    for g in games {
        contents.push_str(&g.to_string());
        contents.push_str("\n");
    }
    let mut f = File::create("games.dat")?;
    f.write_all(contents.as_bytes())?;
    Ok(())
}

/*
    Bu metot games.dat dosyasındaki satırları okuyup geriye String türünden değerler içeren
    bir vektör döndürür.
*/
pub fn read_games_from_file() -> io::Result<Vec<String>> {
    let mut games = Vec::new();
    let f = File::open("games.dat")?;
    for line in io::BufReader::new(f).lines() {
        games.push(line?);
    }
    Ok(games)
}
