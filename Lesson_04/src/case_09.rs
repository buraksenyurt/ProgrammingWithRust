pub fn run() {
    // let mut super_mario = Player {
    //     id: 1,
    //     title: "Super Mario",
    //     location: (0.0, 0.0),
    // };
    let mut super_mario = Player::new(13, "Super Mario", (0.0, 10.0));
    println!("{:#?}", super_mario);
    jump(&mut super_mario, (1.0, 0.0));
    println!("{:#?}", super_mario);
}

#[derive(Debug)]
struct Player<'level> {
    id: u32,
    title: &'level str,
    // title: &'static str, // static lifetime. Yani program çalıştığı müddetçe bu referans yaşayacak anlamında.
    location: (f32, f32),
}

impl<'level> Player<'level> {
    fn new(id: u32, title: &'level str, location: (f32, f32)) -> Self {
        Player {
            id,
            title,
            location,
        }
    }
}

fn jump(player: &mut Player, velocity: (f32, f32)) {
    player.location.0 += velocity.0;
    player.location.1 += velocity.1;
}
