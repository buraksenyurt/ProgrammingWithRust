use crate::data::load_games;
use crate::file_io_ops::*;
use crate::game::Game;

mod data;
mod file_io_ops;
mod game;
mod std_ops;

fn main() -> std::io::Result<()> {
    let mut classic_games = vec![
        "Pac-Man (1980)".to_string(),
        "Galaga (1981)".to_string(),
        "Donkey Kong (1981)".to_string(),
        "Tetris (1984)".to_string(),
    ];

    let new_game = read_game_from_stdin()?;
    classic_games.push(new_game);
    print_games_to_stdout(&classic_games);
    println!("\n\n");

    let games = load_games();
    write_games_to_file(&games)?;
    let games = read_games_from_file()?;
    for game in games {
        println!("{}", game);
    }

    println!("\nOyunları Vector İçerisine Okuma Sonuçları\n");
    let games = read_games_to_vec()?;
    for game in games {
        println!("{}", game);
    }

    let games = load_games();
    write_games_buffered("newGames.dat", &games)?;
    let games = read_games_from_file()?;
    for game in games {
        println!("{}", game);
    }

    let games = load_games();
    write_games_buffered_with_hof("newGames.dat", &games)?;
    let games = read_games_from_file()?;
    for game in games {
        println!("{}", game);
    }

    println!("\nBufReader ile Okuma Sonuçları\n");
    let games = read_games_buffered_into_vec("newGames.dat")?;
    for game in games {
        println!("{}", game);
    }

    println!("\nHOF Tekniği ile Okuma Sonuçları\n");
    let games = read_games_to_vec_with_hof()?;
    for game in games {
        println!("{}", game);
    }

    let shogun = Game {
        title: "Shogun Shawdown".to_string(),
        year: 2024,
        popularity: 8.5,
    };
    append_game_to_file("games.dat", &shogun)?;

    Ok(())
}
