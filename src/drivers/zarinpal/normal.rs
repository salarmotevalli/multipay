use crate::{drivers::Driver, invoice::Invoice};

pub(super) struct Normal {
    invoice: Invoice,
}

impl Normal {
    pub fn new(invoice: Invoice) -> Self {
        Normal { invoice }
    }
}

impl Driver for Normal {
    fn purchase(&self) -> String {
        unimplemented!()
    }

    fn pay(&self) {}

    fn verify(&self) -> crate::receipt::Receipt {
        unimplemented!()
    }
}
