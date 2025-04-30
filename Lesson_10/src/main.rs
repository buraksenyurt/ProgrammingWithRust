fn main() {
    recursive_sample()
}
use std::fmt::{Display, Formatter};

enum Server {
    Node(String, Box<Server>, Box<Server>),
    Empty,
}
impl Display for Server {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Server::Node(name, primary, backup) => {
                write!(
                    f,
                    "Server: {}\n  Primary: {}\n  Backup: {}",
                    name, primary, backup
                )
            }
            Server::Empty => write!(f, "None"),
        }
    }
}

pub fn recursive_sample() {
    let backup_server_blue = Server::Node(
        String::from("Backup Server Blue"),
        Box::new(Server::Empty),
        Box::new(Server::Empty),
    );

    let primary_server_green = Server::Node(
        String::from("Primary Server Green"),
        Box::new(Server::Empty),
        Box::new(backup_server_blue),
    );

    let root_server = Server::Node(
        String::from("Root Server"),
        Box::new(primary_server_green),
        Box::new(Server::Node(
            String::from("Backup Root"),
            Box::new(Server::Empty),
            Box::new(Server::Empty),
        )),
    );

    println!("{}", root_server);
}
