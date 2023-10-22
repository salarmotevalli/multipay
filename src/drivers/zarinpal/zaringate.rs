use crate::{drivers::Driver, invoice::Invoice};

pub(super) struct Zaringate {
    invoice: Invoice,
}

impl Zaringate {
    pub fn new(invoice: Invoice) -> Self {
        Zaringate { invoice }
    }
}

impl Driver for Zaringate {
    fn purchase(&self) -> String {
        unimplemented!()
    }

    fn pay(&self) {}

    fn verify(&self) -> crate::receipt::Receipt {
        unimplemented!()
    }
}
