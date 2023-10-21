use crate::receipt::Receipt;
pub mod zarinpal;

pub trait Driver {
    fn purchase(&self) -> String;
    fn pay(&self);
    fn verify(&self) -> Receipt;
}
