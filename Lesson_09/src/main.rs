mod models;
mod repository;
mod systems;

use crate::models::*;
use crate::systems::*;
use repository::*;

fn year_sorter(game: &Game) -> u16 {
    game.year
}

fn main() {
    let mut games = repository::load_games();

    // Yıla göre sıralama klasik fonksiyon ile
    games.sort_by_key(year_sorter);

    // Closure ile artan yıl sıralaması
    games.sort_by(|g1, g2| g1.year.cmp(&g2.year));
    println!("Yıla göre artan sıralama:");
    print_games(&games);

    // Closure ile azalan yıl sıralaması
    games.sort_by(|g1, g2| g2.year.cmp(&g1.year));
    println!("\nYıla göre azalan sıralama:");
    print_games(&games);

    // Popülaritesi 2.0'den yüksek olan oyunlar
    let popular_games: Vec<Game> = games.into_iter().filter(|g| g.popularity > 2.0).collect();
    println!("\nPopüler oyunlar (popularity > 2.0):");
    print_games(&popular_games);

    // Fn, FnMut Kullanımları
    let mut world = GameWorld {
        players: vec![
            Player {
                id: 1,
                position: (0.0, 0.0),
                velocity: (2.0, 0.0),
                score: 0,
            },
            Player {
                id: 2,
                position: (100.0, 0.0),
                velocity: (8.0, 0.0),
                score: 0,
            },
        ],
    };

    // let apply_gravity = |entity: &mut Player| {
    //     entity.position.0 += entity.velocity.0 * 0.9;
    //     entity.position.1 += entity.velocity.1 * 0.9;
    // };

    println!("Before Update: {:?}", world.players);
    // update_players_system(&mut world, apply_gravity);
    update_players_system(&mut world, |entity| {
        entity.position.0 += entity.velocity.0 * 0.9;
        entity.position.1 += entity.velocity.1 * 0.9;
    });
    println!("After Update: {:?}", world.players);

    // FnMut kullanımı ile ilgili bir örnek
    let mut total_team_score = 0;

    println!("Total score before update: {}", total_team_score);
    update_players_system(&mut world, |p| p.score += 2);
    update_score_system(&world, |p: &Player| {
        total_team_score += p.score;
    });
    println!("Total score after update: {}", total_team_score);
}
