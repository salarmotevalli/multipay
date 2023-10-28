use crate::{drivers::Driver, error::MultiPayErr, invoice::Invoice};

use super::ZarinPalStrategy;

pub(super) struct Zaringate {
    invoice: Invoice,
}

impl Driver for Zaringate {
    #[inline]
    fn new(invoice: Invoice) -> Self {
        Zaringate { invoice }
    }

    #[inline]
    fn purchase(&self) -> Result<String, MultiPayErr> {
        unimplemented!()
    }

    #[inline]
    fn amount(&mut self, amount: f64) {
        self.invoice.amount(amount);
    }

    #[inline]
    fn detail(&mut self, key: String, value: String) {
        self.invoice.detail(key, value);
    }

    #[inline]
    fn transaction_id(&mut self, id: &'static str) {
        self.invoice.transaction_id(id);
    }

    #[inline]
    fn pay(&self) {}

    #[inline]
    fn verify(&self) -> crate::receipt::Receipt {
        unimplemented!()
    }
}

impl ZarinPalStrategy for Zaringate {}
