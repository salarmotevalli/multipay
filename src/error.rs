use std::{fmt::Display, error::Error};

#[derive(Debug)]
pub struct MultiPayErr {
    message: &'static str
}

impl MultiPayErr {
    pub fn new(message: &'static str) -> Self {
        MultiPayErr {
            message
        }
    }
}

impl Display for MultiPayErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for MultiPayErr {}

