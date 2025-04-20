use crate::game::Game;

pub fn load_games() -> Vec<Game> {
    vec![
        Game {
            title: String::from("The Legend of Zelda: Breath of the Wild"),
            year: 2017,
            popularity: 7.5,
        },
        Game {
            title: String::from("Half-Life 2"),
            year: 2004,
            popularity: 7.6,
        },
        Game {
            title: String::from("The Witcher 3: Wild Hunt"),
            year: 2015,
            popularity: 7.8,
        },
        Game {
            title: String::from("Dark Souls"),
            year: 2011,
            popularity: 7.1,
        },
        Game {
            title: String::from("God of War"),
            year: 2018,
            popularity: 7.25,
        },
        Game {
            title: String::from("Super Mario Odyssey"),
            year: 2017,
            popularity: 7.9,
        },
        Game {
            title: String::from("Hades"),
            year: 2020,
            popularity: 8.8,
        },
        Game {
            title: String::from("Resident Evil 4"),
            year: 2005,
            popularity: 5.9,
        },
        Game {
            title: String::from("Minecraft"),
            year: 2009,
            popularity: 8.73,
        },
        Game {
            title: String::from("Overwatch"),
            year: 2016,
            popularity: 8.25,
        },
        Game {
            title: String::from("Grand Theft Auto V"),
            year: 2013,
            popularity: 6.1,
        },
        Game {
            title: String::from("Animal Crossing: New Horizons"),
            year: 2020,
            popularity: 7.4,
        },
        Game {
            title: String::from("Elden Ring"),
            year: 2022,
            popularity: 9.98,
        },
        Game {
            title: String::from("Bloodborne"),
            year: 2015,
            popularity: 9.12,
        },
        Game {
            title: String::from("Sekiro: Shadows Die Twice"),
            year: 2019,
            popularity: 6.78,
        },
        Game {
            title: String::from("Mass Effect 2"),
            year: 2010,
            popularity: 7.98,
        },
    ]
}
