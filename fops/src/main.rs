use crate::io_ops::*;

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

    Ok(())
}
