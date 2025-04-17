#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Location<'a> {
    pub caption: &'a str, // 2nci Kat Güney Kanadı
    pub x: f32,
    pub y: f32,
    pub z: f32,
}