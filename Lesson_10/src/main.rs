use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    run_rc()
}

#[derive(Debug)]
struct Player {
    id: u32,
    name: String,
    friends: RefCell<Vec<Rc<Player>>>,
}

impl Player {
    fn new(id: u32, name: &str) -> Rc<Self> {
        Rc::new(Player {
            id,
            name: name.to_string(),
            friends: RefCell::new(Vec::new()),
        })
    }

    fn add_friend(self: &Rc<Self>, friend: Rc<Player>) {
        self.friends.borrow_mut().push(friend);
    }

    fn print(&self) {
        println!("{}'s friends:", self.name);
        for friend in self.friends.borrow().iter() {
            println!("  {} (ID: {})", friend.name, friend.id);
        }
    }
}

pub fn run_rc() {
    let steve = Player::new(1, "Stivi Vondır");
    let lord = Player::new(2, "Lord veyda");
    let anakin = Player::new(3, "Anakin");

    steve.add_friend(Rc::clone(&lord));
    steve.add_friend(Rc::clone(&anakin));

    steve.print();

    println!("Lord Veyda's ID: {}", lord.id);
}
