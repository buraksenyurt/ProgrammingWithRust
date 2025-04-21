use crate::models::*;

pub fn update_players_system<F>(world: &mut GameWorld, f: F)
where
    F: Fn(&mut Player),
{
    for p in &mut world.players {
        f(p);
    }
}

pub fn update_score_system<F>(world: &GameWorld, mut f: F)
where
    F: FnMut(&Player),
{
    for p in &world.players {
        f(p);
    }
}
