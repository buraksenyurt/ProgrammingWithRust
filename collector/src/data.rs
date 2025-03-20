#[derive(Debug)]
pub enum Metric {
    Cpu,
    Memory,
    Both,
}

#[derive(Debug)]
pub struct Command {
    pub count: u32,
    pub period: u8,
    pub metric: Metric,
}
