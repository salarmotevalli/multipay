use crate::{drivers::Driver, invoice::Invoice, error::MultiPayErr};

use super::ZarinPalConfig;

pub(super) struct Sandbox {
    config: ZarinPalConfig,
    invoice: Invoice,
}

impl Sandbox {
    pub fn new(config: ZarinPalConfig, invoice: Invoice) -> Self {
        Sandbox { config, invoice }
    }
}

impl Driver for Sandbox {
    fn purchase(&self) -> Result<String, MultiPayErr> {
        unimplemented!()
    }

    fn pay(&self) {}

    fn verify(&self) -> crate::receipt::Receipt {
        unimplemented!()
    }
}
