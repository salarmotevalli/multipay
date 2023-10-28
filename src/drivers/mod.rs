use crate::{error::MultiPayErr, invoice::Invoice, receipt::Receipt};
pub mod zarinpal;

pub trait Driver {
    fn new(invoice: Invoice) -> Self
    where
        Self: Sized;
    fn purchase(&self) -> Result<String, MultiPayErr>;
    fn pay(&self);
    fn verify(&self) -> Receipt;
    fn detail(&mut self, key: String, value: String);
    fn transaction_id(&mut self, id: &'static str);
    fn amount(&mut self, amount: f64);
}

pub trait DriverConfig {
    fn load() -> Self;
}
