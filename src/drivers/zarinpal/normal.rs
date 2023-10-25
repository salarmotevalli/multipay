use crate::{drivers::Driver, invoice::Invoice, error::MultiPayErr};

use super::ZarinPalConfig;

pub(super) struct Normal {
    invoice: Invoice,
    config: ZarinPalConfig,
}

impl Normal {
    pub fn new(config: ZarinPalConfig, invoice: Invoice) -> Self {
        Normal { config, invoice }
    }
}

impl Driver for Normal {
    fn purchase(&self) -> Result<String, MultiPayErr> {
        
        unimplemented!()
    }

    fn pay(&self) {}

    fn verify(&self) -> crate::receipt::Receipt {
        unimplemented!()
    }
}
