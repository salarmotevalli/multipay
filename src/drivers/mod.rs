use std::collections::HashMap;

use crate::{error::MultiPayErr, invoice::Invoice, receipt::Receipt};
pub mod zarinpal;

pub trait Driver {
    fn new(drv_cnf: HashMap<&str, &str>, invoice: Invoice) -> Self
    where
        Self: Sized;
    fn purchase(&self) -> Result<String, MultiPayErr>;
    fn pay(&self);
    fn verify(&self) -> Receipt;
}

pub trait DriverConfig {
    fn load() -> Self
    where
        Self: Sized;
}
