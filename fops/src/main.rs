use crate::data::load_games;
use crate::io_ops::*;

mod data;
mod game;
mod io_ops;

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

    println!("\n\n");
    let games = read_games_to_vec()?;
    for game in games {
        println!("{}", game);
    }

    Ok(())
}
