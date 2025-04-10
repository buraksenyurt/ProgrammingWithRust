use crate::model::location::Location;

#[derive(Debug, Copy, Clone)]
pub struct Drone<'a> {
    pub id: u32,
    pub model: &'a str,
    pub energy_level: f32,
    pub location: Location<'a>,
}