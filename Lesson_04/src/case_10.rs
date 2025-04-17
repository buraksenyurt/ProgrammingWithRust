pub fn run() {
    let mut super_mario = Player {
        id: 1,
        title: "Super Mario".to_string(),
        location: (0.0, 0.0),
    };
    jump(&mut super_mario, (1.0, 0.0));
    println!("{:#?}", super_mario);

    // let mut brother_mario = super_mario.clone();
    let mut brother_mario = super_mario;
    brother_mario.title = "Brother Mario".to_string();
    jump(&mut brother_mario, (1.0, 0.0));
    println!("{:#?}", brother_mario);
    // UYARI! Hatayı görmek için aşağıdaki kod satırını açarak ilerleyin
    // println!("{:#?}", super_mario); // Burada Value used after being moved [E0382] hatası oluşur.
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct Player {
    id: u32,
    title: String,
    location: (f32, f32),
}

fn jump(player: &mut Player, velocity: (f32, f32)) {
    player.location.0 += velocity.0;
    player.location.1 += velocity.1;
}
