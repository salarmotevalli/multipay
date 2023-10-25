use crate::{receipt::Receipt, error::MultiPayErr};
pub mod zarinpal;

pub trait Driver {
    fn purchase(&self) -> Result<String, MultiPayErr>;
    fn pay(&self);
    fn verify(&self) -> Receipt;
}
