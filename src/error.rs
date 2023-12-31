use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct MultiPayErr {
    message: &'static str,
}

impl Display for MultiPayErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for MultiPayErr {}
