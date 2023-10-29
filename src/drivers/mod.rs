use std::collections::HashMap;

use crate::{error::MultiPayErr, invoice::Invoice, receipt::Receipt};
pub mod zarinpal;

pub trait Driver {
    fn new(drv_cnf: HashMap<&'static str, &'static str>, invoice: Invoice) -> Self
    where
        Self: Sized;
    fn purchase(&self) -> Result<String, MultiPayErr>;
    fn pay(&self);
    fn verify(&self) -> Receipt;
    fn set_config(&mut self, config: HashMap<&'static str, &'static str>);
}

pub trait DriverConfig {
    fn load() -> Self
    where
        Self: Sized;
}
