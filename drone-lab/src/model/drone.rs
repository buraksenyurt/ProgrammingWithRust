use crate::model::location::Location;

#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
pub struct Drone<'a> {
    pub id: u32,
    pub model: &'a str,
    pub energy_level: f32,
    pub location: Location<'a>,
    pub is_alive: bool,
}
