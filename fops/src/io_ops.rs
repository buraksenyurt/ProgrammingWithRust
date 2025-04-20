use crate::game::Game;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::{BufRead, BufReader, BufWriter, Write, stdout};
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

/*
    Bu metot games.data dosyasındaki içeriği alıp, satırları | işaretine göre ayrıştırıp
    geriye Game türünden bir Vector nesnesi döndürür.
*/
pub fn read_games_to_vec() -> io::Result<Vec<Game>> {
    let mut games = Vec::new();

    for line in read_games_from_file()? {
        let cols: Vec<&str> = line.split('|').collect();
        if cols.len() != 3 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Beklenmeyen sütun sayısı: `{}`", line),
            ));
        }

        let title = cols[0].to_string();
        let year = cols[1]
            .parse::<u16>()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        let popularity = cols[2]
            .parse::<f32>()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        games.push(Game {
            title,
            year,
            popularity,
        });
    }

    Ok(games)
}

/*
    Aşağıdaki fonksiyon yine bir Game listesini alır ancak parametre olarak gelen
    dosyaya kaydetmek için, BufWriter kullanılır.
*/
pub fn write_games_buffered(path: &str, games: &[Game]) -> io::Result<()> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    for game in games {
        writeln!(writer, "{}", game)?;
    }
    writer.flush()?;
    Ok(())
}

/*
    Bu fonksiyon path parametresi ile gelen dosya içeriğini okuyup
    Game türünden bir vector olarak geriye döndürür.
*/
pub fn read_games_buffered_into_vec(path: &str) -> io::Result<Vec<Game>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut games = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if !line.is_empty() {
            let cols: Vec<&str> = line.split('|').collect();
            if cols.len() != 3 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Beklenmeyen sütun sayısı: `{}`", line),
                ));
            }
            let title = cols[0].to_string();
            let year = cols[1]
                .parse::<u16>()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
            let popularity = cols[2]
                .parse::<f32>()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

            games.push(Game {
                title,
                year,
                popularity,
            });
        }
    }
    Ok(games)
}

/*
    Aşağıdaki fonksiyon ise parametre olarak gelen dosyayı Append modunda açıp
    sonuna yine parametre olarak gelen game değişkenini ekler.
*/
pub fn append_game_to_file(path: &str, game: &Game) -> io::Result<()> {
    let mut file = OpenOptions::new().append(true).create(true).open(path)?;
    writeln!(file, "{}", game)?;
    Ok(())
}
