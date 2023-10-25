use crate::{drivers::Driver, invoice::Invoice, error::MultiPayErr};

use super::ZarinPalConfig;

pub(super) struct Zaringate {
    config: ZarinPalConfig,
    invoice: Invoice,
}

impl Zaringate {
    pub fn new(config: ZarinPalConfig, invoice: Invoice) -> Self {
        Zaringate { config, invoice }
    }
}

impl Driver for Zaringate {
    fn purchase(&self) -> Result<String, MultiPayErr>{
        unimplemented!()
    }

    fn pay(&self) {}

    fn verify(&self) -> crate::receipt::Receipt {
        unimplemented!()
    }
}
