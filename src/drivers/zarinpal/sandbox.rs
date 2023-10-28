use crate::{drivers::Driver, error::MultiPayErr, invoice::Invoice};

use super::ZarinPalStrategy;

pub(super) struct Sandbox {
    invoice: Invoice,
}

impl Driver for Sandbox {
    fn new(invoice: Invoice) -> Self {
        Sandbox { invoice }
    }

    fn amount(&mut self, amount: f64) {
        self.invoice.amount(amount);
    }

    fn detail(&mut self, key: String, value: String) {
        self.invoice.detail(key, value);
    }

    fn transaction_id(&mut self, id: &'static str) {
        self.invoice.transaction_id(id);
    }

    fn purchase(&self) -> Result<String, MultiPayErr> {
        unimplemented!()
    }

    fn pay(&self) {}

    fn verify(&self) -> crate::receipt::Receipt {
        unimplemented!()
    }
}

impl ZarinPalStrategy for Sandbox {}
