mod normal;
mod sandbox;
mod zaringate;

use std::collections::HashMap;

use self::{normal::Normal, sandbox::Sandbox, zaringate::Zaringate};

use super::{Driver, DriverConfig};
use crate::{error::MultiPayErr, invoice::Invoice, receipt::Receipt};

pub struct ZarinPalConfig {
    mode: &'static str, // can be normal, sandbox, zaringate
    merchant_id: &'static str,
    callback_url: &'static str,
    description: &'static str,
    currency: &'static str, //Can be R, T (Rial, Toman)
}

impl DriverConfig for ZarinPalConfig {
    fn load() -> Self {
        ZarinPalConfig {
            mode: "normal",
            merchant_id: "",
            callback_url: "",
            description: "",
            currency: "R",
        }
    }
}

impl ZarinPalConfig {
    #[inline]
    pub fn mode(&mut self, mode: &'static str) {
        self.mode = mode;
    }

    #[inline]
    pub fn merchent_id(&mut self, m_id: &'static str) {
        self.merchant_id = m_id;
    }

    #[inline]
    pub fn callback_url(&mut self, callback_url: &'static str) {
        self.callback_url = callback_url;
    }

    #[inline]
    pub fn description(&mut self, description: &'static str) {
        self.description = description;
    }

    #[inline]
    pub fn currency(&mut self, currency: &'static str) {
        self.currency = currency;
    }
}

pub struct ZarinPal {
    strategy: Box<dyn Driver>,
}

impl Driver for ZarinPal {
    #[inline]
    fn new(config: HashMap<&str, &str>, invoice: Invoice) -> Self {
        ZarinPal {
            strategy: match config.get("mode").unwrap() {
                &"normal" => Box::new(Normal::new(config, invoice)),
                &"sandbox" => Box::new(Sandbox::new(config, invoice)),
                &"zaringate" => Box::new(Zaringate::new(config, invoice)),
            },
        }
    }

    #[inline]
    fn pay(&self) {
        self.strategy.as_ref().pay();
    }

    #[inline]
    fn verify(&self) -> Receipt {
        self.strategy.as_ref().verify()
    }

    #[inline]
    fn purchase(&self) -> Result<String, MultiPayErr> {
        self.strategy.as_ref().purchase()
    }
}

pub trait ZarinPalStrategy {}
