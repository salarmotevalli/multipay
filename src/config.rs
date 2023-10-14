

use crate::drivers::{Driver, zarinpal::ZarinPal};

pub struct Config {
    default_driver: Box<dyn Driver>,
}

impl Config {
    pub fn load_config(init: &str) -> Self {
        Config {
            default_driver: Box::new(ZarinPal::new()),
        }
    }
}
