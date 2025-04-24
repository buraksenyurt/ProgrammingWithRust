use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Location<'a> {
    pub caption: &'a str, // 2nci Kat Güney Kanadı
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Display for Location<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}:{}:{})-{}", self.x, self.y, self.z, self.caption)
    }
}
