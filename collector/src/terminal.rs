use std::env;

pub struct Command {
    pub count: u32,
    pub period: u8,
}
pub fn parse() -> Result<Command, String> {
    let _arguments: Vec<String> = env::args().collect();

    Ok(Command {
        count: 10,
        period: 1,
    })
}
