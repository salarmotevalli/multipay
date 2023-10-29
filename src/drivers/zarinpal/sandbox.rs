use std::collections::HashMap;

use crate::{drivers::Driver, error::MultiPayErr, invoice::Invoice, DriverConfig};

use super::ZarinPalStrategy;

static SANDBOX_API_PURCHASE_URL: &str = "https://sandbox.zarinpal.com/pg/services/WebGate/wsdl";
static SANDBOX_API_PAYMENT_URL: &str = "https://sandbox.zarinpal.com/pg/StartPay";
static SANDBOX_API_VERIFICATION_URL: &str = "https://sandbox.zarinpal.com/pg/services/WebGate/wsdl";

pub(super) struct Sandbox {
    invoice: Invoice,
    config: HashMap<&'static str, &'static str>,
}

impl Driver for Sandbox {
    fn new(drv_cnf: HashMap<&'static str, &'static str>, invoice: Invoice) -> Self {
        Sandbox {
            invoice,
            config: drv_cnf,
        }
    }

    fn purchase(&self) -> Result<String, MultiPayErr> {
        unimplemented!()
    }

    fn pay(&self) {}

    fn verify(&self) -> crate::receipt::Receipt {
        unimplemented!()
    }

    #[inline]
    fn set_config(&mut self, config: HashMap<&'static str, &'static str>) {
        self.config = config;
    }
}

impl ZarinPalStrategy for Sandbox {}
