use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum CommandError {
    Invalid(String),
}

// NOT: Daha büyük projelerde thiserror crate'inin kullanımı daha uygun olabilir

impl Display for CommandError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CommandError::Invalid(arg) => {
                write!(f, "Invalid Command. '{}' is invalid.", arg)
            }
        }
    }
}
