use crate::{drivers::Driver, invoice::Invoice};

pub(super) struct Sandbox {
    invoice: Invoice,
}

impl Sandbox {
    pub fn new(invoice: Invoice) -> Self {
        Sandbox { invoice }
    }
}

impl Driver for Sandbox {
    fn purchase(&self) -> String {
        unimplemented!()
    }

    fn pay(&self) {}

    fn verify(&self) -> crate::receipt::Receipt {
        unimplemented!()
    }
}
