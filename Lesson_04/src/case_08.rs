#[allow(dead_code)]
pub fn run() {
    let mut super_mario = Player {
        id: 1,
        title: "Super Mario".to_string(),
        location: (0.0, 0.0),
    };
    println!("{:#?}", super_mario);
    jump(&mut super_mario, (1.0, 0.0));
    println!("{:#?}", super_mario);
}

#[derive(Debug)]
#[allow(dead_code)]
struct Player {
    id: u32,
    title: String,
    location: (f32, f32),
}

#[allow(dead_code)]
fn jump(player: &mut Player, velocity: (f32, f32)) {
    player.location.0 += velocity.0;
    player.location.1 += velocity.1;
}
