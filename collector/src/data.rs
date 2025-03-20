#[derive(Debug)]
pub enum Metric {
    Cpu,
    Memory,
    Both,
}

impl Metric {
    pub fn to_string(&self) -> &'static str {
        match self {
            Metric::Cpu => "CPU",
            Metric::Memory => "Memory",
            Metric::Both => "Cpu and Memory",
        }
    }
}

#[derive(Debug)]
pub struct Command {
    pub count: u32,
    pub period: u8,
    pub metric: Metric,
}

impl Command {
    pub fn to_string(&self) -> String {
        format!(
            "Metric {} Count : {} Period : {} secs",
            self.metric.to_string(),
            self.count,
            self.period
        )
    }
}
